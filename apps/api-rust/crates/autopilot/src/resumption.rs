//! Autopilot v2 resumption — runs a catch-up tick after startup (never auto-executes missed past events).

use std::sync::Arc;

use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use serde::Serialize;
use tracing::{info, error};

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventBus;
use shared_kernel::period_gate::PeriodGate;

use crate::engine::{AutopilotEngine, TickResult};
use svc_organization::entities::organization;

#[derive(Debug, Clone, Serialize)]
pub struct CatchUpReport {
    pub org_id: String,
    pub tick_status: String,
    pub proposed_tasks: usize,
    pub tasks_created: usize,
    pub executed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub error: Option<String>,
}

pub struct ResumptionService;

impl ResumptionService {
    /// Run one autopilot tick on startup to plan/execute tasks in the current window.
    pub async fn catch_up(
        db: &DatabaseConnection,
        event_bus: Arc<EventBus>,
        period_gate: Arc<PeriodGate>,
        org_id: &str,
        org_unit_id: Option<&str>,
    ) -> Result<CatchUpReport, AppError> {
        info!(org_id, "Autopilot catch-up tick starting");

        let engine = AutopilotEngine::new(event_bus).with_period_gate(period_gate);
        let result = engine.tick(db, org_id, org_unit_id).await;

        Ok(CatchUpReport::from_tick(org_id, result))
    }

    /// Spawn a background task to catch up all active organizations asynchronously.
    pub fn spawn_catch_up_all(
        db: DatabaseConnection,
        event_bus: Arc<EventBus>,
        period_gate: Arc<PeriodGate>,
    ) {
        tokio::spawn(async move {
            info!("Starting global autopilot catch-up");
            let orgs = match organization::Entity::find()
                .filter(organization::Column::IsActive.eq(1))
                .filter(organization::Column::IsDeleted.eq(0))
                .all(&db)
                .await
            {
                Ok(o) => o,
                Err(e) => {
                    error!(error = %e, "Global catch-up: failed to list organizations");
                    return;
                }
            };

            for org in orgs {
                let _ = Self::catch_up(
                    &db,
                    event_bus.clone(),
                    period_gate.clone(),
                    &org.id,
                    None,
                ).await;
            }
            info!("Global autopilot catch-up completed");
        });
    }
}

impl CatchUpReport {
    fn from_tick(org_id: &str, tick: TickResult) -> Self {
        Self {
            org_id: org_id.to_string(),
            tick_status: format!("{:?}", tick.status).to_lowercase(),
            proposed_tasks: tick.evaluation.proposed_tasks,
            tasks_created: tick.plan.tasks_created,
            executed: tick.execution.executed,
            succeeded: tick.execution.succeeded,
            failed: tick.execution.failed,
            error: tick.error,
        }
    }
}
