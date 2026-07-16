use std::sync::Arc;
use sea_orm::DatabaseConnection;
use tracing::{info, warn, debug};
use shared_kernel::event_bus::EventBus;
use shared_kernel::period_gate::PeriodGate;
use crate::engine::AutopilotEngine;

pub struct AutopilotSubscriber;

impl AutopilotSubscriber {
    /// Start a background task that listens to the EventBus and triggers autopilot evaluations
    /// when relevant events occur.
    pub fn spawn(
        db: DatabaseConnection,
        event_bus: Arc<EventBus>,
        period_gate: Arc<PeriodGate>,
    ) {
        let mut rx = event_bus.subscribe();
        let engine = AutopilotEngine::new(event_bus.clone()).with_period_gate(period_gate);

        tokio::spawn(async move {
            info!("Autopilot event subscriber started");
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        // For Phase 3, we trigger a tick if a period lock or closure event occurs,
                        // or any enrollment change that could alter fee calculations.
                        let trigger_tick = match event.event_type.as_str() {
                            "period.locked" | "period.closed" | "period.activated" => true,
                            "enrollment.created" | "enrollment.updated" | "enrollment.deleted" => true,
                            "assessment.snapshot_created" => true,
                            _ => false,
                        };

                        if trigger_tick {
                            debug!("Autopilot reacting to event: {}", event.event_type);
                            let _result = engine.tick(&db, &event.org_id, None).await;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        warn!("Event bus channel closed, Autopilot subscriber exiting");
                        break;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(missed)) => {
                        warn!("Autopilot subscriber lagged, missed {} events", missed);
                    }
                }
            }
        });
    }
}
