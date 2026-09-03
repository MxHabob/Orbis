//! Process Manager — in-memory multi-step process coordination.
//!
//! Used for complex operations that span multiple aggregates/services:
//!   • Student registration (6 steps: validate → user → number → enrollment → guardian → fees)
//!   • Promotion (create new year → move students → archive old year)
//!   • Initial setup (org → admin user → year → classes → subjects)
//!
//! **In-memory only** — if the process fails, compensation runs immediately
//! and state is discarded. No persistence of process state.
//!
//! Compensation runs in reverse order: if step 4 fails, steps 3→2→1 are compensated.

use serde::{Deserialize, Serialize};
use sea_orm::{ConnectionTrait, Statement, DbBackend};
use tracing::{info, error};

use crate::errors::AppError;

/// Current status of a multi-step process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessStatus {
    /// Process is currently executing steps.
    InProgress,
    /// Process is waiting for external approval to continue.
    AwaitingApproval {
        step: String,
        approval_id: String,
    },
    /// All steps completed successfully.
    Completed,
    /// A step failed — compensation may be needed.
    Failed {
        step: String,
        error: String,
    },
    /// Compensation is currently running (rolling back completed steps).
    Compensating,
    /// Compensation completed — all completed steps have been rolled back.
    Compensated,
}

/// Status of an individual step within a process.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Compensated,
}

/// A compensation action to undo a completed step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompensationAction {
    /// Delete an entity by type and ID.
    DeleteEntity {
        entity_type: String,
        id: String,
    },
    /// Update an entity's status (e.g. soft-delete).
    UpdateStatus {
        entity_type: String,
        id: String,
        field: String,
        value: String,
    },
    /// Custom compensation with arbitrary SQL.
    Custom {
        description: String,
        sql: String,
        params: Vec<String>,
    },
}

/// A single step in a multi-step process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub name: String,
    pub status: StepStatus,
    pub result: Option<serde_json::Value>,
    pub compensation: Option<CompensationAction>,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Tracks the state of a multi-step process.
///
/// Lives entirely in memory. On failure, `compensate()` rolls back
/// completed steps in reverse order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessState {
    pub id: String,
    pub process_type: String,
    pub org_id: String,
    pub status: ProcessStatus,
    pub steps: Vec<ProcessStep>,
    pub current_step: usize,
    pub started_at: String,
    pub completed_at: Option<String>,
}

impl ProcessState {
    /// Create a new process and persist it immediately.
    pub async fn start<C: ConnectionTrait>(db: &C, process_type: &str, org_id: &str, step_names: &[&str]) -> Result<Self, AppError> {
        let state = Self {
            id: uuid::Uuid::new_v4().to_string(),
            process_type: process_type.to_string(),
            org_id: org_id.to_string(),
            status: ProcessStatus::InProgress,
            steps: step_names
                .iter()
                .map(|name| ProcessStep {
                    name: name.to_string(),
                    status: StepStatus::Pending,
                    result: None,
                    compensation: None,
                    started_at: None,
                    completed_at: None,
                })
                .collect(),
            current_step: 0,
            started_at: chrono::Utc::now().to_rfc3339(),
            completed_at: None,
        };
        state.persist(db).await?;
        Ok(state)
    }

    /// Persist the current state to the `process_log` table.
    pub async fn persist<C: ConnectionTrait>(&self, db: &C) -> Result<(), AppError> {
        let steps_json = serde_json::to_string(&self.steps)
            .map_err(|e| AppError::internal(format!("Failed to serialize process steps: {}", e)))?;
        
        let status_str = serde_json::to_string(&self.status)
            .unwrap_or_else(|_| "\"Unknown\"".to_string());

        let sql = r#"
            INSERT INTO process_log (id, process_type, org_id, status, steps, current_step, started_at, completed_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT(id) DO UPDATE SET
                status = excluded.status,
                steps = excluded.steps,
                current_step = excluded.current_step,
                completed_at = excluded.completed_at
        "#;

        db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                self.id.clone().into(),
                self.process_type.clone().into(),
                self.org_id.clone().into(),
                status_str.into(),
                steps_json.into(),
                (self.current_step as i32).into(),
                self.started_at.clone().into(),
                self.completed_at.clone().map(|s| s.into()).unwrap_or(sea_orm::Value::String(None)),
            ],
        )).await?;

        Ok(())
    }

    /// Mark a step as in-progress.
    pub async fn begin_step<C: ConnectionTrait>(&mut self, db: &C, name: &str) -> Result<(), AppError> {
        let step = self.find_step_mut(name)?;
        step.status = StepStatus::InProgress;
        step.started_at = Some(chrono::Utc::now().to_rfc3339());
        self.persist(db).await?;
        Ok(())
    }

    /// Mark a step as completed with its result.
    pub async fn complete_step<C: ConnectionTrait>(&mut self, db: &C, name: &str, result: serde_json::Value) -> Result<(), AppError> {
        let step = self.find_step_mut(name)?;
        step.status = StepStatus::Completed;
        step.result = Some(result);
        step.completed_at = Some(chrono::Utc::now().to_rfc3339());
        self.current_step += 1;
        self.persist(db).await?;
        Ok(())
    }

    /// Mark a step as failed and update process status.
    pub async fn fail_step<C: ConnectionTrait>(&mut self, db: &C, name: &str, error_msg: &str) -> Result<(), AppError> {
        if let Ok(step) = self.find_step_mut(name) {
            step.status = StepStatus::Failed;
            step.completed_at = Some(chrono::Utc::now().to_rfc3339());
        }
        self.status = ProcessStatus::Failed {
            step: name.to_string(),
            error: error_msg.to_string(),
        };
        self.persist(db).await?;
        Ok(())
    }

    /// Register a compensation action for a completed step.
    pub async fn add_compensation<C: ConnectionTrait>(&mut self, db: &C, step_name: &str, action: CompensationAction) -> Result<(), AppError> {
        if let Ok(step) = self.find_step_mut(step_name) {
            step.compensation = Some(action);
            self.persist(db).await?;
        }
        Ok(())
    }

    /// Mark the entire process as completed.
    pub async fn complete<C: ConnectionTrait>(&mut self, db: &C) -> Result<(), AppError> {
        self.status = ProcessStatus::Completed;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
        self.persist(db).await?;
        Ok(())
    }

    /// Compensate all completed steps in reverse order.
    ///
    /// This rolls back the effects of completed steps by executing their
    /// compensation actions in reverse.
    pub async fn compensate<C: ConnectionTrait>(&mut self, db: &C) -> Result<(), AppError> {
        self.status = ProcessStatus::Compensating;

        // Walk steps in reverse, compensating completed ones
        for i in (0..self.steps.len()).rev() {
            if self.steps[i].status != StepStatus::Completed {
                continue;
            }

            if let Some(ref action) = self.steps[i].compensation.clone() {
                let step_name = self.steps[i].name.clone();
                info!(process = %self.process_type, step = %step_name, "Compensating step");

                match execute_compensation(db, action).await {
                    Ok(()) => {
                        self.steps[i].status = StepStatus::Compensated;
                        info!(step = %step_name, "Step compensated successfully");
                        let _ = self.persist(db).await; // ignore persist errors during compensation
                    }
                    Err(e) => {
                        error!(step = %step_name, error = %e, "Compensation failed — manual intervention required");
                        // Continue compensating remaining steps even if one fails
                    }
                }
            }
        }

        self.status = ProcessStatus::Compensated;
        self.completed_at = Some(chrono::Utc::now().to_rfc3339());
        self.persist(db).await?;
        Ok(())
    }

    /// Check if the process is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            ProcessStatus::Completed | ProcessStatus::Compensated
        )
    }

    fn find_step_mut(&mut self, name: &str) -> Result<&mut ProcessStep, AppError> {
        self.steps
            .iter_mut()
            .find(|s| s.name == name)
            .ok_or_else(|| AppError::internal(format!("Process step '{}' not found", name)))
    }
}

/// Execute a single compensation action against the database.
async fn execute_compensation<C: ConnectionTrait>(
    db: &C,
    action: &CompensationAction,
) -> Result<(), AppError> {
    match action {
        CompensationAction::DeleteEntity { entity_type, id } => {
            let sql = format!("DELETE FROM {} WHERE id = $1", entity_type);
            db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &sql,
                vec![id.clone().into()],
            ))
            .await?;
            Ok(())
        }
        CompensationAction::UpdateStatus { entity_type, id, field, value } => {
            let sql = format!("UPDATE {} SET {} = $1 WHERE id = $2", entity_type, field);
            db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                &sql,
                vec![value.clone().into(), id.clone().into()],
            ))
            .await?;
            Ok(())
        }
        CompensationAction::Custom { description, sql, params } => {
            tracing::info!(description = %description, "Executing custom compensation");
            let values: Vec<sea_orm::Value> = params.iter().map(|p| p.clone().into()).collect();
            db.execute(Statement::from_sql_and_values(
                DbBackend::Sqlite,
                sql,
                values,
            ))
            .await?;
            Ok(())
        }
    }
}
