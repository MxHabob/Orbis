//! Shared domain event loop — reactive pipeline, plugins, insights, optional projections.

use std::sync::Arc;

use async_trait::async_trait;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast;
use tracing::{info, warn, error};

use crate::event_bus::{EventBus, EventEnvelope};
use crate::event_store::EventStore;
use crate::plugin::PluginRegistry;
use crate::reactive::ReactivePipeline;

/// Optional handler for CQRS projections (e.g. DocForge).
#[async_trait]
pub trait EventProjectionHandler: Send + Sync {
    async fn on_event(&self, db: &DatabaseConnection, event: &EventEnvelope);
}

pub struct DomainEventLoop {
    pub event_bus: Arc<EventBus>,
    pub db: DatabaseConnection,
    pub reactive: Arc<ReactivePipeline>,
    pub plugins: Arc<PluginRegistry>,
    pub event_store: Arc<EventStore>,
    pub projection_handler: Option<Arc<dyn EventProjectionHandler>>,
}

impl DomainEventLoop {
    /// Start the background subscriber loop (idempotent per process — call once at startup).
    pub fn spawn(self) {
        tokio::spawn(async move {
            info!("Domain event loop started");
            let mut rx = self.event_bus.subscribe();
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        if event.is_chain_too_deep() {
                            warn!(
                                event_type = %event.event_type,
                                depth = event.chain_depth,
                                "Event chain too deep — skipping"
                            );
                            continue;
                        }

                        let org_id = event.org_id.clone();
                        let db = self.db.clone();

                        let insights = self.reactive.process(&db, &org_id, &event).await;
                        for insight in insights {
                            if let Err(e) = self.event_store.persist_insight(
                                &org_id,
                                &insight.insight_type,
                                insight.entity_type.as_deref(),
                                insight.entity_id.as_deref(),
                                &insight.title,
                                insight.title_ar.as_deref(),
                                &insight.summary,
                                insight.summary_ar.as_deref(),
                                &insight.severity,
                                insight.confidence,
                                insight.suggested_action.as_deref(),
                                "reactive_pipeline",
                            ).await {
                                error!(error = %e, "Failed to persist AI insight");
                            }
                        }

                        self.plugins.dispatch(&db, &org_id, &event).await;

                        if let Some(ref handler) = self.projection_handler {
                            handler.on_event(&db, &event).await;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        warn!(missed = n, "Event subscriber lagged — missed events");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        info!("Event bus closed — stopping domain event loop");
                        break;
                    }
                }
            }
        });
    }
}
