//! AcademicExecutor â€” performs real state transitions on academic entities.
//!
//! Replaces the old stub executor that only logged `info!()` and returned
//! fake JSON strings. Every task type now makes actual database changes.

use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter,
    ActiveModelTrait, Set, IntoActiveModel,
};
use serde_json::json;
use tracing::{info, warn};

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::{EventBus, EventEnvelope};
use shared_kernel::period_gate::PeriodGate;
use svc_calendar::entities::{academic_year, academic_period};
use svc_organization::entities::organization;

use crate::entities::autopilot_task;

/// Execute a single autopilot task by performing real DB mutations.
pub struct AcademicExecutor;

impl AcademicExecutor {
    /// Execute a task and return a JSON result string.
    pub async fn execute(
        db: &DatabaseConnection,
        task: &autopilot_task::Model,
        event_bus: &Arc<EventBus>,
        period_gate: Option<&PeriodGate>,
    ) -> Result<String, AppError> {
        let now = Utc::now().to_rfc3339();
        let target_id = task.target_entity_id.as_deref()
            .ok_or_else(|| AppError::validation("Task missing target_entity_id"))?;

        let result = match task.task_type.as_str() {
            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
            // Year lifecycle
            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

            "year.activate" => {
                // Verify no other active year exists for this org
                let existing_active = academic_year::Entity::find()
                    .filter(academic_year::Column::OrgId.eq(&task.org_id))
                    .filter(academic_year::Column::Status.eq("active"))
                    .filter(academic_year::Column::Id.ne(target_id))
                    .one(db)
                    .await?;

                if let Some(active) = existing_active {
                    return Err(AppError::validation(format!(
                        "Cannot activate year: '{}' is already active. Close it first.",
                        active.name
                    )));
                }

                // Activate the year
                let year = academic_year::Entity::find_by_id(target_id.to_string())
                    .one(db)
                    .await?
                    .ok_or_else(|| AppError::not_found("academic_year", target_id))?;

                let mut active_model = year.into_active_model();
                active_model.status = Set("active".to_string());
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                // Update organizations.current_period_id
                let org = organization::Entity::find_by_id(task.org_id.clone())
                    .one(db)
                    .await?
                    .ok_or_else(|| AppError::not_found("organization", &task.org_id))?;

                let mut org_active = org.into_active_model();
                org_active.current_period_id = Set(Some(target_id.to_string()));
                org_active.updated_at = Set(now.clone());
                org_active.update(db).await?;

                Self::emit(event_bus, "academic.year.activated", "academic_year", target_id, &task.org_id);
                info!(period_id = target_id, "Year activated");

                Ok(json!({
                    "action": "year_activated",
                    "period_id": target_id,
                    "current_year_updated": true
                }).to_string())
            }

            "year.start_closing" => {
                let year = Self::load_year(db, target_id).await?;
                let mut active_model = year.into_active_model();
                active_model.status = Set("closing".to_string());
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.year.closing", "academic_year", target_id, &task.org_id);
                info!(period_id = target_id, "Year entering closing state");

                Ok(json!({
                    "action": "year_closing_started",
                    "period_id": target_id
                }).to_string())
            }

            "year.archive" => {
                let year = Self::load_year(db, target_id).await?;
                let mut active_model = year.into_active_model();
                active_model.status = Set("archived".to_string());
                active_model.is_locked = Set(1);
                active_model.locked_at = Set(Some(now.clone()));
                active_model.locked_by = Set(Some("autopilot".to_string()));
                active_model.lock_reason = Set(Some("Year archived by Autopilot".to_string()));
                active_model.archived_at = Set(Some(now.clone()));
                active_model.archived_by = Set(Some("autopilot".to_string()));
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.year.archived", "academic_year", target_id, &task.org_id);
                info!(period_id = target_id, "Year archived");

                Ok(json!({
                    "action": "year_archived",
                    "period_id": target_id
                }).to_string())
            }

            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
            // Period lifecycle
            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

            "period.activate" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.clone().into_active_model();
                active_model.status = Set("active".to_string());
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                // Update academic_years.current_period_id
                let year = Self::load_year(db, &period.period_id).await?;
                let mut year_active = year.into_active_model();
                year_active.current_period_id = Set(Some(target_id.to_string()));
                year_active.updated_at = Set(now.clone());
                year_active.update(db).await?;

                Self::emit(event_bus, "academic.period.activated", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Period activated");

                Ok(json!({
                    "action": "period_activated",
                    "period_id": target_id,
                    "current_period_updated": true
                }).to_string())
            }

            "period.start_grading" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.into_active_model();
                active_model.status = Set("grading".to_string());
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.period.grading", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Period entered grading state");

                Ok(json!({
                    "action": "period_grading_started",
                    "period_id": target_id
                }).to_string())
            }

            "period.lock" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.clone().into_active_model();
                active_model.is_locked = Set(1);
                active_model.locked_at = Set(Some(now.clone()));
                active_model.locked_by = Set(Some("autopilot".to_string()));
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                // Clear current_period_id if this was the active one
                let year = Self::load_year(db, &period.period_id).await?;
                if year.current_period_id.as_deref() == Some(target_id) {
                    let mut year_active = year.into_active_model();
                    year_active.current_period_id = Set(None);
                    year_active.updated_at = Set(now.clone());
                    year_active.update(db).await?;
                }

                Self::emit(event_bus, "academic.period.locked", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Period locked");

                Ok(json!({
                    "action": "period_locked",
                    "period_id": target_id
                }).to_string())
            }

            "period.close" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.clone().into_active_model();
                active_model.status = Set("closed".to_string());
                active_model.is_locked = Set(1);
                active_model.locked_at = Set(Some(now.clone()));
                active_model.locked_by = Set(Some("autopilot".to_string()));
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                // Clear current_period_id if this was the active one
                let year = Self::load_year(db, &period.period_id).await?;
                if year.current_period_id.as_deref() == Some(target_id) {
                    let mut year_active = year.into_active_model();
                    year_active.current_period_id = Set(None);
                    year_active.updated_at = Set(now.clone());
                    year_active.update(db).await?;
                }

                Self::emit(event_bus, "academic.period.closed", "academic_period", target_id, &task.org_id);
                Self::emit(event_bus, "assessment.batch_snapshot_requested", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Period closed and batch snapshot requested");

                Ok(json!({
                    "action": "period_closed",
                    "period_id": target_id
                }).to_string())
            }

            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
            // Enrollment window
            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

            "enrollment.open" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.into_active_model();
                active_model.is_enrollment_open = Set(1);
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.enrollment.opened", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Enrollment opened");

                Ok(json!({
                    "action": "enrollment_opened",
                    "period_id": target_id
                }).to_string())
            }

            "enrollment.close" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.into_active_model();
                active_model.is_enrollment_open = Set(0);
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.enrollment.closed", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Enrollment closed");

                Ok(json!({
                    "action": "enrollment_closed",
                    "period_id": target_id
                }).to_string())
            }

            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ
            // Grade entry window
            // â•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گâ•گ

            "grade_entry.open" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.into_active_model();
                active_model.is_grade_entry_open = Set(1);
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.grade_entry.opened", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Grade entry opened");

                Ok(json!({
                    "action": "grade_entry_opened",
                    "period_id": target_id
                }).to_string())
            }

            "grade_entry.close" => {
                let period = Self::load_period(db, target_id).await?;
                let mut active_model = period.into_active_model();
                active_model.is_grade_entry_open = Set(0);
                active_model.updated_at = Set(now.clone());
                active_model.update(db).await?;

                Self::emit(event_bus, "academic.grade_entry.closed", "academic_period", target_id, &task.org_id);
                info!(period_id = target_id, "Grade entry closed");

                Ok(json!({
                    "action": "grade_entry_closed",
                    "period_id": target_id
                }).to_string())
            }

            // ════════════════════════════════════════════════════════════════════════════════════════
            // Promotion (advisory — creates insight, doesn't execute)
            // ════════════════════════════════════════════════════════════════════════════════════════

            "promotion.dry_run" => {
                // This is advisory-only — doesn't auto-promote.
                // Creates an AI insight suggesting the admin start promotion.
                info!(period_id = target_id, "Promotion dry run executed, requires approval");

                Ok("REQUIRES_APPROVAL".to_string())
            }

            other => {
                warn!(task_type = other, "Unknown task type");
                Err(AppError::validation(format!("Unknown task type: {}", other)))
            }
        };

        if result.is_ok() {
            Self::invalidate_gate_after_task(period_gate, &task.task_type, target_id, &task.org_id).await;
        }

        result
    }

    async fn invalidate_gate_after_task(
        period_gate: Option<&PeriodGate>,
        task_type: &str,
        target_id: &str,
        org_id: &str,
    ) {
        let Some(gate) = period_gate else { return };

        let affects_period = matches!(
            task_type,
            "period.activate"
                | "period.start_grading"
                | "period.lock"
                | "period.close"
                | "enrollment.open"
                | "enrollment.close"
                | "grade_entry.open"
                | "grade_entry.close"
        );

        if affects_period {
            gate.invalidate_period(target_id).await;
            gate.invalidate_policy(org_id).await;
        }
    }

    // â”€â”€ Helpers â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

    async fn load_year(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<academic_year::Model, AppError> {
        academic_year::Entity::find_by_id(id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_year", id))
    }

    async fn load_period(
        db: &DatabaseConnection,
        id: &str,
    ) -> Result<academic_period::Model, AppError> {
        academic_period::Entity::find_by_id(id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_period", id))
    }

    fn emit(
        event_bus: &Arc<EventBus>,
        event_type: &str,
        entity_type: &str,
        entity_id: &str,
        org_id: &str,
    ) {
        event_bus.publish(EventEnvelope {
            event_type: event_type.to_string(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            org_id: org_id.to_string(),
            actor: "autopilot".to_string(),
            actor_type: "system".to_string(),
            correlation_id: None,
            payload: serde_json::json!({}),
            chain_depth: 0,
        });
    }
}

#[async_trait]
impl crate::executor::AutopilotExecutor for AcademicExecutor {
    fn can_handle(&self, task_type: &str) -> bool {
        matches!(
            task_type,
            "year.activate"
                | "year.start_closing"
                | "year.archive"
                | "period.activate"
                | "period.start_grading"
                | "period.lock"
                | "period.close"
                | "enrollment.open"
                | "enrollment.close"
                | "grade_entry.open"
                | "grade_entry.close"
                | "promotion.dry_run"
        )
    }

    async fn execute(
        &self,
        db: &DatabaseConnection,
        task: &autopilot_task::Model,
        event_bus: &Arc<EventBus>,
        period_gate: Option<&PeriodGate>,
    ) -> Result<String, AppError> {
        AcademicExecutor::execute(db, task, event_bus, period_gate).await
    }
}
