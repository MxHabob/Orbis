//! Planner — creates autopilot tasks from calendar events and system state.
//!
//! The planner examines the `SystemSnapshot` from the Evaluator and:
//!   1. Creates tasks for active calendar events that don't have tasks yet
//!   2. Generates insights for missed events
//!   3. Sets task prerequisites and execution order

use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use tracing::{info, warn};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use shared_kernel::types::ExecutionMode;

use crate::config::ResolvedConfig;
use crate::entities::autopilot_task;
use crate::academic_evaluator::SystemSnapshot;

/// Result of a planning period.
#[derive(Debug, Clone, Default)]
pub struct PlanResult {
    /// Tasks created for active events.
    pub tasks_created: Vec<autopilot_task::Model>,
    /// Events that were missed (past window, never executed).
    pub missed_event_count: usize,
    /// Events that already have tasks (skipped).
    pub already_planned: usize,
}

pub struct Planner;

impl Planner {
    /// Generate tasks from the current system snapshot.
    pub async fn plan(
        db: &DatabaseConnection,
        snapshot: &SystemSnapshot,
        config: &ResolvedConfig,
    ) -> Result<PlanResult, AppError> {
        let mut result = PlanResult::default();

        // ── 1. Handle active events → create tasks ───────────────
        for event in &snapshot.active_events {
            // Check if a task already exists for this event
            let existing = autopilot_task::Entity::find()
                .filter(autopilot_task::Column::CalendarEventId.eq(&event.id))
                .filter(autopilot_task::Column::Status.is_not_in([
                    autopilot_task::status::CANCELLED,
                    autopilot_task::status::SKIPPED,
                ]))
                .one(db)
                .await?;

            if existing.is_some() {
                result.already_planned += 1;
                continue;
            }

            // Resolve execution details by convention based on event_type
            let (event_mode_override, auto_action, prerequisites) = match event.event_type.as_str() {
                "enrollment_open" => (None, Some(r#"{"action":"enrollment_opened"}"#.to_string()), None),
                "enrollment_close" => (None, Some(r#"{"action":"enrollment_closed"}"#.to_string()), Some(r#"["enrollment_open"]"#.to_string())),
                "grade_entry_open" => (None, Some(r#"{"action":"grade_entry_opened"}"#.to_string()), None),
                "grade_entry_close" => (None, Some(r#"{"action":"grade_entry_closed"}"#.to_string()), Some(r#"["grade_entry_open"]"#.to_string())),
                "period_lock" => (None, Some(r#"{"action":"period_locked"}"#.to_string()), Some(r#"["grade_entry_close"]"#.to_string())),
                "report_generation" => (None, Some(r#"{"action":"reports_generated"}"#.to_string()), Some(r#"["period_lock"]"#.to_string())),
                "backup" => (Some("automatic"), Some(r#"{"action":"backup_created"}"#.to_string()), None),
                "promotion_dry_run" => (None, Some(r#"{"action":"promotion_dry_run_complete"}"#.to_string()), Some(r#"["period_lock"]"#.to_string())),
                _ => (None, None, None),
            };

            let effective_mode = config.resolve_mode(event_mode_override);

            // Determine initial task status based on mode
            let initial_status = match effective_mode {
                ExecutionMode::Automatic => autopilot_task::status::READY,
                ExecutionMode::Suggested => autopilot_task::status::SUGGESTED,
                ExecutionMode::Manual => autopilot_task::status::PENDING,
            };

            let now = Utc::now().to_rfc3339();
            let idempotency_key = format!("event:{}:{}", event.id, snapshot.today);

            let task = autopilot_task::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                org_id: Set(snapshot.org_id.clone()),
                calendar_event_id: Set(Some(event.id.clone())),
                task_type: Set(event.event_type.clone()),
                scheduled_at: Set(event.start_date.clone()),
                deadline_at: Set(Some(event.end_date.clone())),
                target_entity_type: Set(Some("academic_calendar_event".to_string())),
                target_entity_id: Set(Some(event.id.clone())),
                params: Set(auto_action),
                prerequisites: Set(prerequisites),
                status: Set(initial_status.to_string()),
                execution_mode: Set(effective_mode.as_str().to_string()),
                attempts: Set(0),
                last_attempt_at: Set(None),
                last_error: Set(None),
                suggested_at: Set(if effective_mode == ExecutionMode::Suggested {
                    Some(now.clone())
                } else {
                    None
                }),
                approved_by: Set(None),
                approved_at: Set(None),
                rejection_reason: Set(None),
                completed_at: Set(None),
                result: Set(None),
                idempotency_key: Set(idempotency_key),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            let inserted = task.insert(db).await?;

            info!(
                task_id = %inserted.id,
                event_type = %event.event_type,
                mode = %effective_mode,
                status = initial_status,
                "Autopilot task created"
            );

            result.tasks_created.push(inserted);
        }

        // ── 2. Log missed events ─────────────────────────────────
        result.missed_event_count = snapshot.missed_events.len();
        if !snapshot.missed_events.is_empty() {
            warn!(
                org_id = %snapshot.org_id,
                count = snapshot.missed_events.len(),
                "Missed calendar events detected"
            );
        }

        Ok(result)
    }
}
