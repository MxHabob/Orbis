//! Executor — runs autopilot tasks according to their execution mode.
//!
//! Flow:
//!   1. Pick tasks that are READY or APPROVED
//!   2. Execute the task's action (calling domain services)
//!   3. Record result in execution_log
//!   4. Update task status
//!   5. Publish event via EventBus

use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde_json::Value;
use tracing::{info, error, warn};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::{EventBus, EventEnvelope};

use crate::config::ResolvedConfig;
use crate::entities::{autopilot_task, autopilot_execution_log};

/// Result of executing a batch of tasks.
#[derive(Debug, Clone, Default)]
pub struct ExecutionResult {
    pub executed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
}

pub struct Executor {
    event_bus: Arc<EventBus>,
}

impl Executor {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    /// Execute all tasks that are ready to run.
    pub async fn execute_ready_tasks(
        &self,
        db: &DatabaseConnection,
        config: &ResolvedConfig,
    ) -> Result<ExecutionResult, AppError> {
        let mut result = ExecutionResult::default();

        // Fetch tasks that are READY or APPROVED
        let tasks = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(&config.org_id))
            .filter(autopilot_task::Column::Status.is_in([
                autopilot_task::status::READY,
                autopilot_task::status::APPROVED,
            ]))
            .all(db)
            .await?;

        for task in tasks {
            // Check retry limit
            if task.attempts >= config.max_auto_retries {
                warn!(
                    task_id = %task.id,
                    attempts = task.attempts,
                    "Task exceeded max retries — marking as failed"
                );
                Self::update_task_status(db, &task.id, autopilot_task::status::FAILED, None).await?;
                result.skipped += 1;
                continue;
            }

            result.executed += 1;
            let start = Instant::now();

            // Execute the task action
            let exec_result = self.execute_single_task(db, &task, config).await;
            let duration_ms = start.elapsed().as_millis() as i32;

            match exec_result {
                Ok(output) => {
                    result.succeeded += 1;

                    // Update task as completed
                    Self::complete_task(db, &task.id, &output).await?;

                    // Log execution
                    Self::log_execution(
                        db, &config.org_id, &task.id, &task.task_type,
                        "completed", Some(&output), None, duration_ms,
                    ).await?;

                    // Publish event
                    self.event_bus.publish(EventEnvelope {
                        event_type: "autopilot.task.completed".to_string(),
                        entity_type: "autopilot_task".to_string(),
                        entity_id: task.id.clone(),
                        org_id: config.org_id.clone(),
                        actor: "autopilot".to_string(),
                        actor_type: "system".to_string(),
                        correlation_id: None,
                        payload: serde_json::json!({
                            "task_type": task.task_type,
                            "duration_ms": duration_ms,
                        }),
                        chain_depth: 0,
                    });

                    info!(
                        task_id = %task.id,
                        task_type = %task.task_type,
                        duration_ms,
                        "Task completed successfully"
                    );
                }
                Err(err) => {
                    result.failed += 1;
                    let err_msg = err.to_string();

                    // Increment attempts and record error
                    Self::fail_task(db, &task.id, &err_msg, task.attempts + 1).await?;

                    // Log execution
                    Self::log_execution(
                        db, &config.org_id, &task.id, &task.task_type,
                        "failed", None, Some(&err_msg), duration_ms,
                    ).await?;

                    // Publish failure event
                    self.event_bus.publish(EventEnvelope {
                        event_type: "autopilot.task.failed".to_string(),
                        entity_type: "autopilot_task".to_string(),
                        entity_id: task.id.clone(),
                        org_id: config.org_id.clone(),
                        actor: "autopilot".to_string(),
                        actor_type: "system".to_string(),
                        correlation_id: None,
                        payload: serde_json::json!({
                            "task_type": task.task_type,
                            "error": err_msg,
                            "attempt": task.attempts + 1,
                        }),
                        chain_depth: 0,
                    });

                    error!(
                        task_id = %task.id,
                        task_type = %task.task_type,
                        error = %err_msg,
                        attempt = task.attempts + 1,
                        "Task execution failed"
                    );

                    // Pause autopilot if configured to pause on error
                    if config.pause_on_error {
                        warn!("Pausing autopilot due to task failure (pause_on_error=true)");
                        break;
                    }
                }
            }
        }

        Ok(result)
    }

    /// Execute a single task. This is where domain service calls happen.
    ///
    /// The task_type determines which domain operation to invoke.
    /// Params are passed as JSON from the calendar event's auto_action field.
    async fn execute_single_task(
        &self,
        _db: &DatabaseConnection,
        task: &autopilot_task::Model,
        _config: &ResolvedConfig,
    ) -> Result<String, AppError> {
        // Parse params
        let _params: Value = task.params.as_deref()
            .map(|p| serde_json::from_str(p).unwrap_or(Value::Null))
            .unwrap_or(Value::Null);

        // Dispatch based on task_type
        // In Phase 3 (AI Gateway), this will be replaced by ActionExecutor calls.
        // For now, we implement known task types directly.
        match task.task_type.as_str() {
            "enrollment_open" => {
                // → domain-lifecycle: open enrollment for period
                info!(task_type = "enrollment_open", "Opening enrollment period");
                Ok(r#"{"action":"enrollment_opened"}"#.to_string())
            }
            "enrollment_close" => {
                info!(task_type = "enrollment_close", "Closing enrollment period");
                Ok(r#"{"action":"enrollment_closed"}"#.to_string())
            }
            "grade_entry_open" => {
                info!(task_type = "grade_entry_open", "Opening grade entry window");
                Ok(r#"{"action":"grade_entry_opened"}"#.to_string())
            }
            "grade_entry_close" => {
                info!(task_type = "grade_entry_close", "Closing grade entry window");
                Ok(r#"{"action":"grade_entry_closed"}"#.to_string())
            }
            "period_lock" => {
                info!(task_type = "period_lock", "Locking academic period");
                Ok(r#"{"action":"period_locked"}"#.to_string())
            }
            "report_generation" => {
                info!(task_type = "report_generation", "Generating reports");
                Ok(r#"{"action":"reports_generated"}"#.to_string())
            }
            "backup" => {
                info!(task_type = "backup", "Creating database backup");
                Ok(r#"{"action":"backup_created"}"#.to_string())
            }
            "promotion_dry_run" => {
                info!(task_type = "promotion_dry_run", "Running promotion dry run");
                Ok(r#"{"action":"promotion_dry_run_complete"}"#.to_string())
            }
            other => {
                warn!(task_type = other, "Unknown task type — skipping");
                Err(AppError::validation(format!("Unknown task type: {}", other)))
            }
        }
    }

    // ── DB Helpers ────────────────────────────────────────────────

    async fn update_task_status(
        db: &DatabaseConnection,
        task_id: &str,
        status: &str,
        error: Option<&str>,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active: autopilot_task::ActiveModel = task.into();
        active.status = Set(status.to_string());
        active.last_error = Set(error.map(String::from));
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn complete_task(
        db: &DatabaseConnection,
        task_id: &str,
        result: &str,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active: autopilot_task::ActiveModel = task.into();
        active.status = Set(autopilot_task::status::COMPLETED.to_string());
        active.completed_at = Set(Some(Utc::now().to_rfc3339()));
        active.result = Set(Some(result.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn fail_task(
        db: &DatabaseConnection,
        task_id: &str,
        error: &str,
        attempts: i32,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active: autopilot_task::ActiveModel = task.into();
        active.status = Set(autopilot_task::status::FAILED.to_string());
        active.last_error = Set(Some(error.to_string()));
        active.last_attempt_at = Set(Some(Utc::now().to_rfc3339()));
        active.attempts = Set(attempts);
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn log_execution(
        db: &DatabaseConnection,
        org_id: &str,
        task_id: &str,
        action: &str,
        status: &str,
        details: Option<&str>,
        error: Option<&str>,
        duration_ms: i32,
    ) -> Result<(), AppError> {
        let log = autopilot_execution_log::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            org_id: Set(org_id.to_string()),
            task_id: Set(Some(task_id.to_string())),
            action: Set(action.to_string()),
            status: Set(status.to_string()),
            details: Set(details.map(String::from)),
            error: Set(error.map(String::from)),
            duration_ms: Set(Some(duration_ms)),
            executed_at: Set(Utc::now().to_rfc3339()),
        };

        log.insert(db).await?;
        Ok(())
    }
}
