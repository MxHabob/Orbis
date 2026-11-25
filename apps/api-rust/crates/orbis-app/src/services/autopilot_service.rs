use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::autopilot::engine::AutopilotEngine;
use crate::autopilot::evaluator::AutopilotEvaluator;
use crate::context::{ActionSource, RequestContext};
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{AutopilotTaskRepository, PolicyRepository};
use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskStatus, TaskType};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopilotDiagnostics {
    pub is_healthy: bool,
    pub status: String,
    pub mode: String,
    pub stuck_tasks: Vec<AutopilotTask>,
    pub overdue_tasks: Vec<AutopilotTask>,
    pub total_tasks_count: u64,
    pub pending_tasks_count: u64,
    pub completed_tasks_count: u64,
    pub failed_tasks_count: u64,
}

fn can_access_org(ctx: &RequestContext, org_id: Uuid) -> bool {
    (ctx.actor.is_system() && matches!(&ctx.source, ActionSource::System | ActionSource::Autopilot))
        || ctx.tenant_id == org_id
}

fn require_org_scope(ctx: &RequestContext, org_id: Uuid) -> Result<(), DomainError> {
    if can_access_org(ctx, org_id) {
        Ok(())
    } else {
        Err(DomainError::Forbidden(
            "The requested Autopilot data belongs to another organization".into(),
        ))
    }
}

fn require_task_scope(ctx: &RequestContext, task: &AutopilotTask) -> Result<(), DomainError> {
    if can_access_org(ctx, task.org_id) {
        Ok(())
    } else {
        Err(DomainError::NotFound("Autopilot task not found".into()))
    }
}

fn is_manually_executable(status: TaskStatus) -> bool {
    matches!(status, TaskStatus::Pending | TaskStatus::Suggested)
}

pub struct AutopilotService {
    pub evaluator: Arc<AutopilotEvaluator>,
    pub engine: Arc<AutopilotEngine>,
    pub task_repo: Arc<dyn AutopilotTaskRepository>,
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl AutopilotService {
    pub fn new(
        evaluator: Arc<AutopilotEvaluator>,
        engine: Arc<AutopilotEngine>,
        task_repo: Arc<dyn AutopilotTaskRepository>,
        policy_repo: Arc<dyn PolicyRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            evaluator,
            engine,
            task_repo,
            policy_repo,
            permission_checker,
        }
    }

    /// Triggers an immediate Autopilot evaluation tick for the organization.
    /// If `dry_run` is true, evaluates policies and returns proposed tasks without executing them.
    pub async fn run_tick(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        dry_run: bool,
    ) -> Result<Vec<AutopilotTask>, DomainError> {
        require_org_scope(ctx, org_id)?;
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let settings = self.policy_repo.get_settings(org_id).await?;
        let evaluated_tasks = self.evaluator.evaluate(ctx, org_id, &settings).await?;

        if dry_run {
            return Ok(evaluated_tasks
                .into_iter()
                .map(|mut task| {
                    task.status = TaskStatus::Suggested;
                    task
                })
                .collect());
        }

        // Capture previously due work before scheduling new occurrences so a
        // failed execution is retried on a future tick, never twice in one tick.
        let pending_tasks = if settings.autopilot.enabled {
            self.task_repo.find_pending_tasks(Some(org_id)).await?
        } else {
            Vec::new()
        };
        let should_execute = |task: &AutopilotTask| {
            let period_lifecycle = (settings.autopilot.auto_advance_periods
                && task.task_type == TaskType::PeriodActivate)
                || (settings.autopilot.auto_period_lock && task.task_type == TaskType::PeriodLock);
            period_lifecycle
                || (settings.autopilot.mode == "automatic"
                    && matches!(task.risk_level, RiskLevel::Low | RiskLevel::Medium))
        };

        let mut output_tasks = Vec::new();
        for mut task in evaluated_tasks {
            let execute = should_execute(&task);
            if !execute {
                task.status = TaskStatus::Suggested;
            }
            task.idempotency_key = Some(format!(
                "policy:{}:{}:{}",
                task.task_type.as_str(),
                task.target_id.unwrap_or_default(),
                task.scheduled_at.timestamp(),
            ));
            let proposed_id = task.id;
            let task = self.task_repo.schedule_task(&task).await?;
            if execute && (task.id == proposed_id || task.status == TaskStatus::Suggested) {
                output_tasks.push(self.execute_scheduled_task(ctx, task).await?);
            } else {
                output_tasks.push(task);
            }
        }

        for task in pending_tasks.into_iter().filter(should_execute) {
            let updated = self.execute_scheduled_task(ctx, task).await?;
            if let Some(output) = output_tasks.iter_mut().find(|task| task.id == updated.id) {
                *output = updated;
            }
        }

        Ok(output_tasks)
    }

    async fn execute_scheduled_task(
        &self,
        ctx: &RequestContext,
        task: AutopilotTask,
    ) -> Result<AutopilotTask, DomainError> {
        let task_id = task.id;
        match self.engine.process_task(ctx, task).await {
            Ok(updated) => Ok(updated),
            Err(error) => {
                if !matches!(error, DomainError::Conflict(_)) {
                    warn!(%task_id, %error, "Scheduled Autopilot execution failed");
                }
                self.task_repo.find_by_id(task_id).await?.ok_or(error)
            }
        }
    }

    /// List tasks for the organization with optional status/type filters and pagination
    pub async fn list_tasks(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        status: Option<&str>,
        task_type: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<AutopilotTask>, u64), DomainError> {
        require_org_scope(ctx, org_id)?;
        self.permission_checker
            .check(ctx, Permission::OrganizationRead)
            .await?;
        self.task_repo
            .find_by_org(org_id, status, task_type, limit, offset)
            .await
    }

    /// Get details of a single task
    pub async fn get_task(
        &self,
        ctx: &RequestContext,
        task_id: Uuid,
    ) -> Result<Option<AutopilotTask>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationRead)
            .await?;
        let task = self.task_repo.find_by_id(task_id).await?;
        if let Some(task) = task.as_ref() {
            if !can_access_org(ctx, task.org_id) {
                return Ok(None);
            }
        }
        Ok(task)
    }

    /// Manually execute an Autopilot task (e.g. from UI confirmation or AI tool)
    pub async fn execute_task(
        &self,
        ctx: &RequestContext,
        task_id: Uuid,
    ) -> Result<AutopilotTask, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let task = self.task_repo.find_by_id(task_id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Autopilot task {} not found", task_id))
        })?;
        require_task_scope(ctx, &task)?;
        if !is_manually_executable(task.status) {
            return Err(DomainError::Conflict(format!(
                "Autopilot task cannot be executed from the '{}' state",
                task.status.as_str()
            )));
        }

        self.engine.process_task(ctx, task).await
    }

    /// Reset retries and re-execute a failed Autopilot task
    pub async fn retry_task(
        &self,
        ctx: &RequestContext,
        task_id: Uuid,
    ) -> Result<AutopilotTask, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let mut task = self.task_repo.find_by_id(task_id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Autopilot task {} not found", task_id))
        })?;
        require_task_scope(ctx, &task)?;
        if task.status != TaskStatus::Failed {
            return Err(DomainError::Conflict(format!(
                "Only failed Autopilot tasks can be retried; current state is '{}'",
                task.status.as_str()
            )));
        }

        task.error_log = None;
        task.updated_at = Utc::now();

        self.engine.process_task(ctx, task).await
    }

    /// Dismiss / Cancel a suggested task
    pub async fn dismiss_task(
        &self,
        ctx: &RequestContext,
        task_id: Uuid,
    ) -> Result<AutopilotTask, DomainError> {
        self.permission_checker
            .check(ctx, Permission::OrganizationWrite)
            .await?;

        let mut task = self.task_repo.find_by_id(task_id).await?.ok_or_else(|| {
            DomainError::NotFound(format!("Autopilot task {} not found", task_id))
        })?;
        require_task_scope(ctx, &task)?;
        if task.status == TaskStatus::Cancelled {
            return Ok(task);
        }
        if !is_manually_executable(task.status) {
            return Err(DomainError::Conflict(format!(
                "Autopilot task cannot be dismissed from the '{}' state",
                task.status.as_str()
            )));
        }

        let expected_status = task.status;
        task.status = TaskStatus::Cancelled;
        task.updated_at = Utc::now();
        self.task_repo
            .update_task_state(&task, expected_status)
            .await?;

        Ok(task)
    }

    /// Return comprehensive diagnostics and health status for the Autopilot engine
    pub async fn get_diagnostics(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
    ) -> Result<AutopilotDiagnostics, DomainError> {
        require_org_scope(ctx, org_id)?;
        self.permission_checker
            .check(ctx, Permission::OrganizationRead)
            .await?;

        let settings = self.policy_repo.get_settings(org_id).await?;
        let stuck_tasks = self.task_repo.find_stuck_tasks(org_id, 300).await?;
        let overdue_tasks = self.task_repo.find_overdue_tasks(org_id).await?;

        let counts = self.task_repo.count_by_status(org_id).await?;
        let count = |status| {
            counts
                .iter()
                .filter(|(state, _)| *state == status)
                .map(|(_, count)| count)
                .sum::<u64>()
        };
        let total_count = counts.iter().map(|(_, count)| count).sum();
        let pending_count = count(TaskStatus::Pending) + count(TaskStatus::Suggested);
        let completed_count = count(TaskStatus::Completed);
        let failed_count = count(TaskStatus::Failed);

        let is_healthy = stuck_tasks.is_empty() && failed_count == 0;
        let status = if settings.autopilot.enabled {
            "active".to_string()
        } else {
            "standby".to_string()
        };

        Ok(AutopilotDiagnostics {
            is_healthy,
            status,
            mode: settings.autopilot.mode,
            stuck_tasks,
            overdue_tasks,
            total_tasks_count: total_count,
            pending_tasks_count: pending_count,
            completed_tasks_count: completed_count,
            failed_tasks_count: failed_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use orbis_domain::iam::actor::Actor;

    fn context(tenant_id: Uuid, actor: Actor, source: ActionSource) -> RequestContext {
        RequestContext {
            tenant_id,
            actor,
            request_id: "test-request".into(),
            source,
            org_unit_path: None,
        }
    }

    #[test]
    fn user_cannot_access_another_organizations_autopilot_tasks() {
        let ctx = context(
            Uuid::new_v4(),
            Actor::User(Uuid::new_v4()),
            ActionSource::Api,
        );

        assert!(require_org_scope(&ctx, Uuid::new_v4()).is_err());
    }

    #[test]
    fn system_actor_requires_an_internal_execution_source_for_cross_org_access() {
        let org_id = Uuid::new_v4();
        let internal = context(Uuid::nil(), Actor::System, ActionSource::Autopilot);
        let transport = context(Uuid::nil(), Actor::System, ActionSource::TauriIpc);

        assert!(require_org_scope(&internal, org_id).is_ok());
        assert!(require_org_scope(&transport, org_id).is_err());
    }

    #[test]
    fn only_pending_or_suggested_tasks_can_be_manually_executed() {
        assert!(is_manually_executable(TaskStatus::Pending));
        assert!(is_manually_executable(TaskStatus::Suggested));
        assert!(!is_manually_executable(TaskStatus::Executing));
        assert!(!is_manually_executable(TaskStatus::Completed));
        assert!(!is_manually_executable(TaskStatus::Failed));
        assert!(!is_manually_executable(TaskStatus::Cancelled));
    }
}
