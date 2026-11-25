use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

use crate::context::RequestContext;
use crate::ports::repositories::{AutopilotTaskRepository, PolicyRepository};
use crate::services::{
    calendar_service::CalendarService,
    progression_service::ProgressionService,
    report_service::{GenerateReportInput, ReportService},
};
use orbis_domain::autopilot::task::{AutopilotTask, TaskStatus, TaskType};
use orbis_domain::entities::academic_period::AcademicPeriodState;
use orbis_domain::entities::report::{OutputFormat, ReportLocale};
use orbis_domain::errors::DomainError;

pub struct AutopilotEngine {
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub task_repo: Arc<dyn AutopilotTaskRepository>,
    pub calendar_service: Arc<CalendarService>,
    pub progression_service: Arc<ProgressionService>,
    pub report_service: Arc<ReportService>,
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
        }
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
                self.calendar_service
                    .start_year_closing(ctx, year_id)
                    .await?;
            }
            TaskType::PeriodActivate => {
                let period_id = target()?;
                self.calendar_service
                    .transition_period_state(ctx, period_id, AcademicPeriodState::ActiveTeaching)
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
            TaskType::BackupDatabase => {
                return Err(DomainError::BusinessRule(
                    "Database backups must be run by the deployment backup service".into(),
                ));
            }
            TaskType::LateFeeReminder
            | TaskType::SendDelayNotification
            | TaskType::GenericWorkflow
            | TaskType::TransitionPeriod => {
                return Err(DomainError::BusinessRule(format!(
                    "Autopilot task '{}' has no execution handler",
                    task.task_type.as_str()
                )));
            }
        }

        Ok(())
    }
}
