use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use orbis_domain::autopilot::task::{AutopilotTask, TaskStatus, RiskLevel};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{AutopilotTaskRepository, PolicyRepository};
use crate::autopilot::engine::AutopilotEngine;
use crate::autopilot::evaluator::AutopilotEvaluator;

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
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let settings = self.policy_repo.get_settings(org_id).await?;
        let evaluated_tasks = self.evaluator.evaluate(ctx, org_id, &settings).await?;

        let mut output_tasks = Vec::new();

        for mut task in evaluated_tasks {
            // Check if identical active task already exists to ensure idempotency
            let (existing, _) = self.task_repo.find_by_org(org_id, Some("pending"), Some(task.task_type.as_str()), 1, 0).await?;
            if let Some(first_existing) = existing.first() {
                if first_existing.target_id == task.target_id {
                    output_tasks.push(first_existing.clone());
                    continue;
                }
            }

            if dry_run || settings.autopilot.mode == "suggested" || settings.autopilot.mode == "manual" {
                task.status = TaskStatus::Suggested;
                self.task_repo.save_task(&task).await?;
                output_tasks.push(task);
            } else if settings.autopilot.mode == "automatic" {
                // In automatic mode, low/medium risk tasks execute immediately
                if matches!(task.risk_level, RiskLevel::Low | RiskLevel::Medium) {
                    self.task_repo.save_task(&task).await?;
                    match self.engine.process_task(ctx, task.clone()).await {
                        Ok(completed) => output_tasks.push(completed),
                        Err(_) => output_tasks.push(task),
                    }
                } else {
                    // High/Critical risk tasks require human confirmation (Suggested)
                    task.status = TaskStatus::Suggested;
                    self.task_repo.save_task(&task).await?;
                    output_tasks.push(task);
                }
            }
        }

        // Also process any pending scheduled tasks due for execution
        let pending_tasks = self.task_repo.find_pending_tasks(Some(org_id)).await?;
        for task in pending_tasks {
            if !dry_run && settings.autopilot.enabled && settings.autopilot.mode == "automatic" {
                let _ = self.engine.process_task(ctx, task).await;
            }
        }

        Ok(output_tasks)
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
        self.permission_checker.check(ctx, Permission::OrganizationRead).await?;
        self.task_repo.find_by_org(org_id, status, task_type, limit, offset).await
    }

    /// Get details of a single task
    pub async fn get_task(&self, ctx: &RequestContext, task_id: Uuid) -> Result<Option<AutopilotTask>, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationRead).await?;
        self.task_repo.find_by_id(task_id).await
    }

    /// Manually execute an Autopilot task (e.g. from UI confirmation or AI tool)
    pub async fn execute_task(&self, ctx: &RequestContext, task_id: Uuid) -> Result<AutopilotTask, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Autopilot task {} not found", task_id)))?;

        self.engine.process_task(ctx, task).await
    }

    /// Reset retries and re-execute a failed Autopilot task
    pub async fn retry_task(&self, ctx: &RequestContext, task_id: Uuid) -> Result<AutopilotTask, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let mut task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Autopilot task {} not found", task_id)))?;

        task.attempts = 0;
        task.status = TaskStatus::Pending;
        task.error_log = None;
        task.updated_at = Utc::now();
        self.task_repo.save_task(&task).await?;

        self.engine.process_task(ctx, task).await
    }

    /// Dismiss / Cancel a suggested task
    pub async fn dismiss_task(&self, ctx: &RequestContext, task_id: Uuid) -> Result<AutopilotTask, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let mut task = self.task_repo.find_by_id(task_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Autopilot task {} not found", task_id)))?;

        task.status = TaskStatus::Cancelled;
        task.updated_at = Utc::now();
        self.task_repo.save_task(&task).await?;

        Ok(task)
    }

    /// Return comprehensive diagnostics and health status for the Autopilot engine
    pub async fn get_diagnostics(&self, ctx: &RequestContext, org_id: Uuid) -> Result<AutopilotDiagnostics, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationRead).await?;

        let settings = self.policy_repo.get_settings(org_id).await?;
        let stuck_tasks = self.task_repo.find_stuck_tasks(org_id, 300).await?;
        let overdue_tasks = self.task_repo.find_overdue_tasks(org_id).await?;

        let (all_tasks, total_count) = self.task_repo.find_by_org(org_id, None, None, 100, 0).await?;

        let pending_count = all_tasks.iter().filter(|t| t.status == TaskStatus::Pending || t.status == TaskStatus::Suggested).count() as u64;
        let completed_count = all_tasks.iter().filter(|t| t.status == TaskStatus::Completed).count() as u64;
        let failed_count = all_tasks.iter().filter(|t| t.status == TaskStatus::Failed).count() as u64;

        let is_healthy = stuck_tasks.is_empty() && failed_count == 0;
        let status = if settings.autopilot.enabled { "active".to_string() } else { "standby".to_string() };

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
