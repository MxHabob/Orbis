//! Evaluator — reads system state to determine what the autopilot should do.
//!
//! The evaluator examines:
//!   • Current date vs calendar events (what's due today?)
//!   • Academic cycle phase (enrollment open? grade entry? exam period?)
//!   • System health (pending tasks, failed tasks, paused state)
//!
//! It produces a `SystemSnapshot` consumed by the Planner.

use chrono::{NaiveDate, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder};
use serde::Serialize;
use tracing::debug;

use shared_kernel::errors::AppError;
use svc_organization::entities::{academic_calendar_event, academic_cycle};
use crate::entities::{autopilot_task, autopilot_config};

/// Snapshot of the system state at a point in time.
#[derive(Debug, Clone, Serialize)]
pub struct SystemSnapshot {
    pub today: NaiveDate,
    pub org_id: String,

    /// Calendar events whose window includes today.
    pub active_events: Vec<academic_calendar_event::Model>,

    /// Calendar events whose window has passed without execution.
    pub missed_events: Vec<academic_calendar_event::Model>,

    /// Calendar events coming up within the next 7 days.
    pub upcoming_events: Vec<academic_calendar_event::Model>,

    /// Currently active academic cycle (is_current = 1).
    pub current_cycle: Option<academic_cycle::Model>,

    /// Tasks that are pending or ready for execution.
    pub pending_tasks: Vec<autopilot_task::Model>,

    /// Tasks that failed and may need retry.
    pub failed_tasks: Vec<autopilot_task::Model>,

    /// Whether the autopilot is paused.
    pub is_paused: bool,
}

pub struct Evaluator;

impl Evaluator {
    /// Evaluate the current state of the system for a given organization.
    pub async fn evaluate(
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<SystemSnapshot, AppError> {
        let today = Utc::now().date_naive();
        let today_str = today.to_string();
        let week_ahead = (today + chrono::Duration::days(7)).to_string();

        // ── Parallel queries ──────────────────────────────────────
        let (calendar_events, current_cycle, all_tasks, config) = tokio::join!(
            // All calendar events for the org (from year DB)
            academic_calendar_event::Entity::find()
                .filter(academic_calendar_event::Column::OrgId.eq(org_id))
                .filter(academic_calendar_event::Column::IsActive.eq(1))
                .order_by_asc(academic_calendar_event::Column::StartDate)
                .all(year_db),

            // Current active cycle (from registry)
            academic_cycle::Entity::find()
                .filter(academic_cycle::Column::OrgId.eq(org_id))
                .filter(academic_cycle::Column::IsCurrent.eq(1))
                .one(registry_db),

            // All autopilot tasks (from registry)
            autopilot_task::Entity::find()
                .filter(autopilot_task::Column::OrgId.eq(org_id))
                .all(registry_db),

            // Config (from registry)
            autopilot_config::Entity::find()
                .filter(autopilot_config::Column::OrgId.eq(org_id))
                .one(registry_db),
        );

        let calendar_events = calendar_events?;
        let current_cycle = current_cycle?;
        let all_tasks = all_tasks?;
        let config = config?;

        // ── Filter tasks in memory ────────────────────────────────
        let pending_tasks: Vec<autopilot_task::Model> = all_tasks
            .iter()
            .filter(|t| {
                t.status == autopilot_task::status::PENDING
                    || t.status == autopilot_task::status::READY
                    || t.status == autopilot_task::status::APPROVED
            })
            .cloned()
            .collect();

        let failed_tasks: Vec<autopilot_task::Model> = all_tasks
            .iter()
            .filter(|t| t.status == autopilot_task::status::FAILED)
            .cloned()
            .collect();

        // ── Classify calendar events ──────────────────────────────
        let mut active_events = Vec::new();
        let mut missed_events = Vec::new();
        let mut upcoming_events = Vec::new();

        for event in &calendar_events {
            // Check if there is a completed task for this calendar event
            let is_completed = all_tasks.iter().any(|t| {
                t.calendar_event_id.as_ref() == Some(&event.id)
                    && t.status == autopilot_task::status::COMPLETED
            });

            if event.end_date < today_str && !is_completed {
                missed_events.push(event.clone());
            } else if event.start_date <= today_str && event.end_date >= today_str {
                active_events.push(event.clone());
            } else if event.start_date > today_str && event.start_date <= week_ahead {
                upcoming_events.push(event.clone());
            }
        }

        let is_paused = config
            .as_ref()
            .map(|c| c.paused_at.is_some())
            .unwrap_or(false);

        debug!(
            org_id,
            active = active_events.len(),
            missed = missed_events.len(),
            upcoming = upcoming_events.len(),
            pending = pending_tasks.len(),
            failed = failed_tasks.len(),
            is_paused,
            "System state evaluated"
        );

        Ok(SystemSnapshot {
            today,
            org_id: org_id.to_string(),
            active_events,
            missed_events,
            upcoming_events,
            current_cycle,
            pending_tasks,
            failed_tasks,
            is_paused,
        })
    }
}
