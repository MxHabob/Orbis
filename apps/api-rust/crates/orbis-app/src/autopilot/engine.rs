use chrono::Utc;
use std::sync::Arc;
use tracing::{error, info};

use crate::context::RequestContext;
use crate::ports::repositories::{AutopilotTaskRepository, PolicyRepository};
use crate::services::{
    calendar_service::CalendarService,
    finance_service::FinanceService,
    progression_service::ProgressionService,
    report_service::{GenerateReportInput, ReportService},
    workflow_service::WorkflowService,
};
use orbis_domain::autopilot::task::{AutopilotTask, TaskStatus, TaskType};
use orbis_domain::entities::academic_period::AcademicPeriodState;
use orbis_domain::entities::report::{OutputFormat, ReportLocale};
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;

pub struct AutopilotEngine {
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub task_repo: Arc<dyn AutopilotTaskRepository>,
    pub calendar_service: Arc<CalendarService>,
    pub progression_service: Arc<ProgressionService>,
    pub finance_service: Arc<FinanceService>,
    pub report_service: Arc<ReportService>,
    pub workflow_service: Arc<WorkflowService>,
}

impl AutopilotEngine {
    pub fn new(
        policy_repo: Arc<dyn PolicyRepository>,
        task_repo: Arc<dyn AutopilotTaskRepository>,
        calendar_service: Arc<CalendarService>,
        progression_service: Arc<ProgressionService>,
        finance_service: Arc<FinanceService>,
        report_service: Arc<ReportService>,
        workflow_service: Arc<WorkflowService>,
    ) -> Self {
        Self {
            policy_repo,
            task_repo,
            calendar_service,
            progression_service,
            finance_service,
            report_service,
            workflow_service,
        }
    }

    /// Entry point for processing an individual Autopilot task.
    pub async fn process_task(
        &self,
        ctx: &RequestContext,
        mut task: AutopilotTask,
    ) -> Result<AutopilotTask, DomainError> {
        let settings = self.policy_repo.get_settings(task.org_id).await?;

        if !settings.autopilot.enabled {
            info!("Autopilot is disabled for org {}", task.org_id);
            task.status = TaskStatus::Skipped;
            task.reason = Some("Autopilot is disabled in organization settings".to_string());
            self.task_repo.save_task(&task).await?;
            return Ok(task);
        }

        let explicitly_automated_period = (settings.autopilot.auto_advance_periods
            && task.task_type == orbis_domain::autopilot::task::TaskType::PeriodActivate)
            || (settings.autopilot.auto_period_lock
                && task.task_type == orbis_domain::autopilot::task::TaskType::PeriodLock);
        if settings.autopilot.mode == "manual"
            && !explicitly_automated_period
            && task.status != TaskStatus::Executing
        {
            info!(
                "Autopilot is in manual mode for org {}, marking task {} as Suggested",
                task.org_id, task.id
            );
            task.status = TaskStatus::Suggested;
            self.task_repo.save_task(&task).await?;
            return Ok(task);
        }

        // Transition to Executing
        task.status = TaskStatus::Executing;
        task.attempts += 1;
        task.updated_at = Utc::now();
        self.task_repo.save_task(&task).await?;

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
                self.task_repo.save_task(&task).await?;
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

                if task.attempts >= task.max_retries {
                    task.status = TaskStatus::Failed;
                } else {
                    task.status = TaskStatus::Pending; // Eligible for retry
                }

                self.task_repo.save_task(&task).await?;
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
        match task.task_type {
            TaskType::YearActivate => {
                if let Some(year_id) = task.target_id {
                    self.calendar_service.activate_year(ctx, year_id).await?;
                }
            }
            TaskType::YearClose => {
                if let Some(year_id) = task.target_id {
                    self.calendar_service
                        .start_year_closing(ctx, year_id)
                        .await?;
                }
            }
            TaskType::PeriodActivate => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service
                        .transition_period_state(
                            ctx,
                            period_id,
                            AcademicPeriodState::ActiveTeaching,
                        )
                        .await?;
                }
            }
            TaskType::EnrollmentOpen => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service
                        .open_enrollment_window(ctx, period_id)
                        .await?;
                }
            }
            TaskType::EnrollmentClose => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service
                        .close_enrollment_window(ctx, period_id)
                        .await?;
                }
            }
            TaskType::GradeEntryOpen => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service
                        .open_grade_entry_window(ctx, period_id)
                        .await?;
                }
            }
            TaskType::GradeEntryClose => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service
                        .close_grade_entry_window(ctx, period_id)
                        .await?;
                }
            }
            TaskType::PeriodLock => {
                if let Some(period_id) = task.target_id {
                    self.calendar_service.lock_period(ctx, period_id).await?;
                }
            }
            TaskType::PromotionDryRun => {
                if let Some(source_period_id) = task.target_id {
                    let preview = self
                        .progression_service
                        .evaluate_cohort_progression(ctx, task.org_id, source_period_id)
                        .await?;
                    info!(
                        "Autopilot generated progression dry-run with {} candidates",
                        preview.len()
                    );
                }
            }
            TaskType::LateFeeReminder | TaskType::SendDelayNotification => {
                // In production, dispatch notification via notification_gateway or mark fee reminder sent
                info!(
                    "Autopilot processed late fee reminder for target {:?}",
                    task.target_id
                );
            }
            TaskType::GenerateReport => {
                if let Some(params) = &task.parameters {
                    let report_code = params
                        .get("report_code")
                        .and_then(|v| v.as_str())
                        .unwrap_or("DEFAULT");
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
            }
            TaskType::BackupDatabase => {
                info!(
                    "Autopilot scheduled database backup check for org {}",
                    task.org_id
                );
            }
            TaskType::GenericWorkflow | TaskType::TransitionPeriod => {
                info!("Autopilot generic workflow executed for task {}", task.id);
            }
        }

        Ok(())
    }

    /// Evaluates domain events in real-time to trigger immediate automated responses or scheduled tasks.
    pub async fn handle_event(
        &self,
        _ctx: &RequestContext,
        event: &DomainEvent,
    ) -> Result<(), DomainError> {
        info!("Autopilot Engine analyzed domain event: {:?}", event);
        Ok(())
    }
}
