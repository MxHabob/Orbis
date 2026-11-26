use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

use crate::context::RequestContext;
use crate::ports::repositories::{
    AutopilotTaskRepository, EmployeeRepository, PolicyRepository, SectionSubjectRepository,
    StudentFeeRepository, StudentRepository, UserRepository,
};
use crate::services::{
    calendar_service::CalendarService,
    notification_service::{CreateNotificationInput, NotificationService},
    progression_service::ProgressionService,
    report_service::{GenerateReportInput, ReportService},
};
use orbis_domain::autopilot::task::{AutopilotTask, TaskStatus, TaskType};
use orbis_domain::entities::academic_period::{AcademicPeriodState, PeriodKind};
use orbis_domain::entities::report::{OutputFormat, ParamValue, ReportLocale};
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::SectionSubjectFilter;

pub struct AutopilotEngine {
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub task_repo: Arc<dyn AutopilotTaskRepository>,
    pub calendar_service: Arc<CalendarService>,
    pub progression_service: Arc<ProgressionService>,
    pub report_service: Arc<ReportService>,
    pub notification_service: Option<Arc<NotificationService>>,
    pub user_repo: Option<Arc<dyn UserRepository>>,
    pub student_repo: Option<Arc<dyn StudentRepository>>,
    pub fee_repo: Option<Arc<dyn StudentFeeRepository>>,
    pub section_subject_repo: Option<Arc<dyn SectionSubjectRepository>>,
    pub employee_repo: Option<Arc<dyn EmployeeRepository>>,
}

impl AutopilotEngine {
    pub fn new(
        policy_repo: Arc<dyn PolicyRepository>,
        task_repo: Arc<dyn AutopilotTaskRepository>,
        calendar_service: Arc<CalendarService>,
        progression_service: Arc<ProgressionService>,
        report_service: Arc<ReportService>,
    ) -> Self {
        Self {
            policy_repo,
            task_repo,
            calendar_service,
            progression_service,
            report_service,
            notification_service: None,
            user_repo: None,
            student_repo: None,
            fee_repo: None,
            section_subject_repo: None,
            employee_repo: None,
        }
    }

    pub fn with_smart_services(
        mut self,
        notification_service: Arc<NotificationService>,
        user_repo: Arc<dyn UserRepository>,
        student_repo: Arc<dyn StudentRepository>,
        fee_repo: Arc<dyn StudentFeeRepository>,
        section_subject_repo: Arc<dyn SectionSubjectRepository>,
        employee_repo: Arc<dyn EmployeeRepository>,
    ) -> Self {
        self.notification_service = Some(notification_service);
        self.user_repo = Some(user_repo);
        self.student_repo = Some(student_repo);
        self.fee_repo = Some(fee_repo);
        self.section_subject_repo = Some(section_subject_repo);
        self.employee_repo = Some(employee_repo);
        self
    }

    /// Entry point for processing an individual Autopilot task.
    pub async fn process_task(
        &self,
        ctx: &RequestContext,
        mut task: AutopilotTask,
    ) -> Result<AutopilotTask, DomainError> {
        let expected_status = task.status;
        let settings = self.policy_repo.get_settings(task.org_id).await?;

        if !settings.autopilot.enabled {
            info!("Autopilot is disabled for org {}", task.org_id);
            task.status = TaskStatus::Skipped;
            task.reason = Some("Autopilot is disabled in organization settings".to_string());
            task.updated_at = Utc::now();
            self.task_repo
                .update_task_state(&task, expected_status)
                .await?;
            return Ok(task);
        }

        if !matches!(
            expected_status,
            TaskStatus::Pending | TaskStatus::Suggested | TaskStatus::Failed
        ) {
            return Err(DomainError::Conflict(format!(
                "Autopilot task cannot execute from the '{}' state",
                expected_status.as_str()
            )));
        }
        if !self
            .task_repo
            .claim_task(task.id, task.org_id, expected_status, task.attempts)
            .await?
        {
            return Err(DomainError::Conflict(
                "Autopilot task was already claimed or its state changed".into(),
            ));
        }

        task.status = TaskStatus::Executing;
        task.attempts = if expected_status == TaskStatus::Failed {
            1
        } else {
            task.attempts + 1
        };
        task.updated_at = Utc::now();

        let execution_result = self.dispatch_task(ctx, &task).await;

        let now = Utc::now();
        match execution_result {
            Ok(_) => {
                info!(
                    "Autopilot task {} ({:?}) completed successfully",
                    task.id, task.task_type
                );
                task.status = TaskStatus::Completed;
                task.executed_at = Some(now);
                task.error_log = None;
                task.updated_at = now;
                self.task_repo
                    .update_task_state(&task, TaskStatus::Executing)
                    .await?;
                Ok(task)
            }
            Err(e) => {
                error!(
                    "Autopilot task {} ({:?}) failed: {:?}",
                    task.id, task.task_type, e
                );
                task.executed_at = Some(now);
                task.error_log = Some(format!("{:?}", e));
                task.updated_at = now;

                if matches!(e, DomainError::Validation(_) | DomainError::BusinessRule(_))
                    || task.attempts >= task.max_retries
                {
                    task.status = TaskStatus::Failed;
                } else {
                    task.status = TaskStatus::Pending; // Eligible for retry
                }

                self.task_repo
                    .update_task_state(&task, TaskStatus::Executing)
                    .await?;
                Err(e)
            }
        }
    }

    /// Dispatches task execution to the appropriate domain service
    async fn dispatch_task(
        &self,
        ctx: &RequestContext,
        task: &AutopilotTask,
    ) -> Result<(), DomainError> {
        let target = || {
            task.target_id.ok_or_else(|| {
                DomainError::Validation(format!(
                    "Autopilot task '{}' requires a target",
                    task.task_type.as_str()
                ))
            })
        };

        match task.task_type {
            TaskType::YearActivate => {
                let year_id = target()?;
                self.calendar_service.activate_year(ctx, year_id).await?;
            }
            TaskType::YearClose => {
                let year_id = target()?;
                let year = self
                    .calendar_service
                    .year_repo
                    .find_by_id(year_id)
                    .await?
                    .ok_or_else(|| {
                        DomainError::NotFound(format!("Academic year {} not found", year_id))
                    })?;
                if year.status
                    == orbis_domain::entities::academic_calendar::AcademicCalendarStatus::Closing
                {
                    self.calendar_service.close_year(ctx, year_id).await?;
                } else {
                    self.calendar_service
                        .start_year_closing(ctx, year_id)
                        .await?;
                }
            }
            TaskType::PeriodActivate => {
                let period_id = target()?;
                let period = self
                    .calendar_service
                    .period_repo
                    .find_by_id(period_id)
                    .await?
                    .ok_or_else(|| {
                        DomainError::NotFound(format!("Academic period {} not found", period_id))
                    })?;
                let next_state = if period.period_kind == PeriodKind::Examination {
                    AcademicPeriodState::FinalExams
                } else {
                    AcademicPeriodState::ActiveTeaching
                };
                self.calendar_service
                    .transition_period_state(ctx, period_id, next_state)
                    .await?;
            }
            TaskType::TransitionPeriod => {
                let period_id = target()?;
                let target_state = task
                    .parameters
                    .as_ref()
                    .and_then(|p| p.get("target_state"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("grade_submission");
                let state = match target_state {
                    "planning" => AcademicPeriodState::Planning,
                    "enrollment_open" => AcademicPeriodState::EnrollmentOpen,
                    "active_teaching" => AcademicPeriodState::ActiveTeaching,
                    "final_exams" => AcademicPeriodState::FinalExams,
                    "grade_submission" => AcademicPeriodState::GradeSubmission,
                    "closed" => AcademicPeriodState::Closed,
                    "archived" => AcademicPeriodState::Archived,
                    other => {
                        return Err(DomainError::Validation(format!(
                            "Invalid target period state '{}'",
                            other
                        )));
                    }
                };
                self.calendar_service
                    .transition_period_state(ctx, period_id, state)
                    .await?;
            }
            TaskType::EnrollmentOpen => {
                let period_id = target()?;
                self.calendar_service
                    .open_enrollment_window(ctx, period_id)
                    .await?;
            }
            TaskType::EnrollmentClose => {
                let period_id = target()?;
                self.calendar_service
                    .close_enrollment_window(ctx, period_id)
                    .await?;
            }
            TaskType::GradeEntryOpen => {
                let period_id = target()?;
                self.calendar_service
                    .open_grade_entry_window(ctx, period_id)
                    .await?;
            }
            TaskType::GradeEntryClose => {
                let period_id = target()?;
                self.calendar_service
                    .close_grade_entry_window(ctx, period_id)
                    .await?;
            }
            TaskType::PeriodLock => {
                let period_id = target()?;
                self.calendar_service.lock_period(ctx, period_id).await?;
                let settings = self.policy_repo.get_settings(task.org_id).await?;
                if settings.autopilot.auto_report_generation {
                    let mut params = std::collections::HashMap::new();
                    params.insert(
                        "period_id".to_string(),
                        ParamValue::Period(period_id.to_string()),
                    );
                    let _ = self
                        .report_service
                        .generate_report(
                            ctx,
                            GenerateReportInput {
                                report_code: "rep.grade_summary".to_string(),
                                parameters: params,
                                output_format: OutputFormat::Pdf,
                                locale: ReportLocale::Ar,
                                title: None,
                                selected_columns: None,
                                template_id: None,
                            },
                        )
                        .await;
                }
            }
            TaskType::PromotionDryRun => {
                let source_period_id = target()?;
                let preview = self
                    .progression_service
                    .evaluate_cohort_progression(ctx, task.org_id, source_period_id)
                    .await?;
                info!(
                    "Autopilot generated progression dry-run with {} candidates",
                    preview.len()
                );
            }
            TaskType::GenerateReport => {
                let report_code = task
                    .parameters
                    .as_ref()
                    .and_then(|params| params.get("report_code"))
                    .and_then(|value| value.as_str())
                    .filter(|code| !code.trim().is_empty())
                    .ok_or_else(|| {
                        DomainError::Validation("Autopilot report requires a report_code".into())
                    })?;
                let input = GenerateReportInput {
                    report_code: report_code.to_string(),
                    parameters: std::collections::HashMap::new(),
                    output_format: OutputFormat::Pdf,
                    locale: ReportLocale::Ar,
                    title: None,
                    selected_columns: None,
                    template_id: None,
                };
                self.report_service.generate_report(ctx, input).await?;
            }
            TaskType::SendDelayNotification => {
                let period_id = target()?;
                let Some(notif) = &self.notification_service else {
                    return Ok(());
                };
                let period = self
                    .calendar_service
                    .period_repo
                    .find_by_id(period_id)
                    .await?
                    .ok_or_else(|| {
                        DomainError::NotFound(format!("Academic period {} not found", period_id))
                    })?;
                let mut recipient_ids = std::collections::HashSet::new();
                if let (Some(ss_repo), Some(emp_repo)) =
                    (&self.section_subject_repo, &self.employee_repo)
                {
                    let offerings = ss_repo
                        .list(task.org_id, SectionSubjectFilter::default())
                        .await?;
                    let instructor_ids: std::collections::HashSet<_> = offerings
                        .items
                        .into_iter()
                        .filter(|o| o.period_id == period_id)
                        .filter_map(|o| o.instructor_id)
                        .collect();
                    for inst_id in instructor_ids {
                        if let Some(emp) = emp_repo.find_by_id(inst_id).await? {
                            if let Some(user_id) = emp.user_id {
                                recipient_ids.insert(user_id);
                            }
                        }
                    }
                }
                if recipient_ids.is_empty() {
                    if let Some(user_repo) = &self.user_repo {
                        for u in user_repo.find_by_org(task.org_id).await? {
                            if !u.is_deleted && u.status == "active" {
                                recipient_ids.insert(u.id);
                                break;
                            }
                        }
                    }
                }
                for user_id in recipient_ids {
                    let _ = notif
                        .create(CreateNotificationInput {
                            org_id: task.org_id,
                            recipient_user_id: user_id,
                            category: "academics".to_string(),
                            priority: "high".to_string(),
                            title: format!("Grade entry closing soon: {}", period.name),
                            title_ar: Some(format!(
                                "تنبيه: اقترب إغلاق رصد الدرجات للفترة {}",
                                period.name_ar.as_deref().unwrap_or(&period.name)
                            )),
                            body: format!(
                                "Please finalize all grade entries for period '{}' before the window closes.",
                                period.name
                            ),
                            body_ar: Some(format!(
                                "يرجى استكمال رصد جميع الدرجات للفترة ({}) قبل إغلاق نافذة الإدخال.",
                                period.name_ar.as_deref().unwrap_or(&period.name)
                            )),
                            entity_type: Some("academic_period".to_string()),
                            entity_id: Some(period_id),
                            action_route: Some(format!("/{}/academics/grades", task.org_id)),
                            source_event: "autopilot.grade_entry_reminder".to_string(),
                            correlation_id: format!(
                                "autopilot:grade_reminder:{}:{}",
                                period_id, user_id
                            ),
                            expires_at: None,
                        })
                        .await;
                }
            }
            TaskType::LateFeeReminder => {
                let fee_id = target()?;
                let (Some(notif), Some(fee_repo), Some(student_repo)) =
                    (&self.notification_service, &self.fee_repo, &self.student_repo)
                else {
                    return Ok(());
                };
                let fee = fee_repo
                    .find_by_id(fee_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound(format!("Student fee {} not found", fee_id)))?;
                if let Some(student) = student_repo.find_by_id(fee.student_id).await? {
                    if let Some(user_id) = student.user_id {
                        let _ = notif
                            .create(CreateNotificationInput {
                                org_id: task.org_id,
                                recipient_user_id: user_id,
                                category: "finance".to_string(),
                                priority: "normal".to_string(),
                                title: "Overdue tuition fee reminder".to_string(),
                                title_ar: Some("تذكير برسوم دراسية مستحقة".to_string()),
                                body: format!(
                                    "You have an overdue balance of {} due on {}.",
                                    fee.balance_due, fee.due_date
                                ),
                                body_ar: Some(format!(
                                    "يوجد رصيد مستحق بقيمة {} كان موعد استحقاقه {}.",
                                    fee.balance_due, fee.due_date
                                )),
                                entity_type: Some("student_fee".to_string()),
                                entity_id: Some(fee_id),
                                action_route: Some(format!("/{}/finance", task.org_id)),
                                source_event: "autopilot.late_fee_reminder".to_string(),
                                correlation_id: format!("autopilot:fee_reminder:{}", fee_id),
                                expires_at: None,
                            })
                            .await;
                    }
                }
            }
            TaskType::BackupDatabase => {
                return Err(DomainError::BusinessRule(
                    "Database backups must be run by the deployment backup service".into(),
                ));
            }
            TaskType::GenericWorkflow => {
                return Err(DomainError::BusinessRule(format!(
                    "Autopilot task '{}' has no execution handler",
                    task.task_type.as_str()
                )));
            }
        }

        Ok(())
    }
}
