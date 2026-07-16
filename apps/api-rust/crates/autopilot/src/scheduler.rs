//! Background autopilot scheduler — periodic tick for all active organizations.

use std::sync::Arc;
use std::time::Duration;

use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use tracing::{info, warn, error};

use shared_kernel::event_bus::EventBus;
use shared_kernel::period_gate::PeriodGate;

use crate::engine::AutopilotEngine;
use svc_organization::entities::organization;

/// Spawn a background task that runs autopilot ticks on an interval.
pub fn spawn_scheduler(
    db: DatabaseConnection,
    event_bus: Arc<EventBus>,
    period_gate: Arc<PeriodGate>,
    interval: Duration,
) {
    tokio::spawn(async move {
        info!(?interval, "Autopilot scheduler started");
        let mut ticker = tokio::time::interval(interval);

        loop {
            ticker.tick().await;

            let orgs = match organization::Entity::find()
                .filter(organization::Column::IsActive.eq(1))
                .filter(organization::Column::IsDeleted.eq(0))
                .all(&db)
                .await
            {
                Ok(o) => o,
                Err(e) => {
                    error!(error = %e, "Autopilot scheduler: failed to list organizations");
                    continue;
                }
            };

            let engine = AutopilotEngine::new(event_bus.clone()).with_period_gate(period_gate.clone());

            for org in orgs {
                let result = engine.tick(&db, &org.id, None).await;
                if result.status == crate::engine::TickStatus::Error {
                    warn!(org_id = %org.id, error = ?result.error, "Autopilot tick error");
                }
            }
        }
    });
}
