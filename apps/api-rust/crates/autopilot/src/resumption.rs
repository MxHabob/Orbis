//! Resumption — catches up on missed events after app restarts or interruptions.
//!
//! When the app starts, the ResumptionService:
//!   1. Finds calendar events that should have been executed but weren't
//!   2. Logs missed events as insights (never auto-executes past events)
//!   3. Creates tasks for events currently in their active window
//!   4. Syncs cycle state with reality (dates vs actual flags)

use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde::Serialize;
use tracing::{info, warn};

use shared_kernel::errors::AppError;
use domain_lifecycle::entities::academic_calendar_event;

use crate::config::ResolvedConfig;
use crate::evaluator::{Evaluator, SystemSnapshot};
use crate::planner::Planner;

/// Report of what happened during catch-up.
#[derive(Debug, Clone, Serialize)]
pub struct CatchUpReport {
    /// Events that were missed (past window, never executed).
    pub missed: Vec<MissedEvent>,
    /// Tasks created for events currently in their active window.
    pub activated: usize,
    /// Tasks that already existed (skipped).
    pub already_planned: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct MissedEvent {
    pub event_id: String,
    pub event_type: String,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
}

pub struct ResumptionService;

impl ResumptionService {
    /// Run catch-up after application startup.
    ///
    /// Safety: This never auto-executes missed events. It only:
    ///   1. Records missed events for admin review
    ///   2. Creates tasks for events currently in their active window
    pub async fn catch_up(
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        config: &ResolvedConfig,
    ) -> Result<CatchUpReport, AppError> {
        info!(org_id = %config.org_id, "Starting autopilot catch-up...");

        // 1. Evaluate current system state
        let snapshot = Evaluator::evaluate(registry_db, year_db, &config.org_id).await?;

        // 2. Record missed events (never auto-execute)
        let missed: Vec<MissedEvent> = snapshot.missed_events.iter().map(|e| {
            MissedEvent {
                event_id: e.id.clone(),
                event_type: e.event_type.clone(),
                title: e.title.clone(),
                start_date: e.start_date.clone(),
                end_date: e.end_date.clone(),
            }
        }).collect();

        if !missed.is_empty() {
            warn!(
                org_id = %config.org_id,
                count = missed.len(),
                "Found {} missed calendar events — recording for admin review",
                missed.len()
            );
        }

        // 3. Create tasks for currently active events
        let plan_result = Planner::plan(registry_db, &snapshot, config).await?;

        let report = CatchUpReport {
            missed,
            activated: plan_result.tasks_created.len(),
            already_planned: plan_result.already_planned,
        };

        info!(
            org_id = %config.org_id,
            missed = report.missed.len(),
            activated = report.activated,
            already_planned = report.already_planned,
            "Catch-up completed"
        );

        Ok(report)
    }
}
