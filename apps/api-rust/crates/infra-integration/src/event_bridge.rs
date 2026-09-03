//! Event Bridge — routes events between external integrations and the core system.
//!
//! Architecture:
//!   External → [Adapter] → [EventBridge] → [EventBus] → Core System
//!   Core System → [EventBus] → [EventBridge] → [Adapter] → External
//!
//! The bridge:
//!   1. Receives inbound events from integration adapters
//!   2. Transforms them into domain EventEnvelopes
//!   3. Publishes to the shared-kernel EventBus
//!   4. Listens for outbound events and routes to appropriate adapters

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error, debug};

use shared_kernel::event_bus::{EventBus, EventEnvelope};
use crate::traits::{IntegrationEvent, IntegrationProvider, AdapterStatus};

/// Configuration for the event bridge.
#[derive(Debug, Clone)]
pub struct EventBridgeConfig {
    /// How often to poll adapters that don't push events (seconds).
    pub poll_interval_secs: u64,
    /// Maximum events to process per poll period.
    pub max_events_per_poll: usize,
    /// Whether to persist integration events for replay.
    pub persist_events: bool,
}

impl Default for EventBridgeConfig {
    fn default() -> Self {
        Self {
            poll_interval_secs: 30,
            max_events_per_poll: 100,
            persist_events: true,
        }
    }
}

/// The Event Bridge — central hub for external ↔ core event routing.
pub struct EventBridge {
    event_bus: Arc<EventBus>,
    adapters: RwLock<Vec<Box<dyn IntegrationProvider>>>,
    config: EventBridgeConfig,
}

impl EventBridge {
    pub fn new(event_bus: Arc<EventBus>, config: EventBridgeConfig) -> Self {
        Self {
            event_bus,
            adapters: RwLock::new(Vec::new()),
            config,
        }
    }

    /// Register an integration adapter.
    pub async fn register_adapter(&self, adapter: Box<dyn IntegrationProvider>) {
        info!(
            adapter = adapter.adapter_name(),
            category = adapter.category(),
            "Registering integration adapter"
        );
        self.adapters.write().await.push(adapter);
    }

    /// Connect all registered adapters.
    pub async fn connect_all(&self) {
        let mut adapters = self.adapters.write().await;
        for adapter in adapters.iter_mut() {
            match adapter.connect().await {
                Ok(()) => info!(
                    adapter = adapter.adapter_name(),
                    "Adapter connected"
                ),
                Err(e) => error!(
                    adapter = adapter.adapter_name(),
                    error = %e,
                    "Adapter connection failed"
                ),
            }
        }
    }

    /// Disconnect all adapters.
    pub async fn disconnect_all(&self) {
        let mut adapters = self.adapters.write().await;
        for adapter in adapters.iter_mut() {
            let _ = adapter.disconnect().await;
            info!(adapter = adapter.adapter_name(), "Adapter disconnected");
        }
    }

    /// Poll all adapters for new events and publish to EventBus.
    pub async fn poll_period(&self) {
        let adapters = self.adapters.read().await;
        let mut total_events = 0;

        for adapter in adapters.iter() {
            if adapter.status() != AdapterStatus::Connected {
                continue;
            }

            match adapter.poll_events().await {
                Ok(events) => {
                    let count = events.len().min(self.config.max_events_per_poll);
                    for event in events.into_iter().take(count) {
                        self.route_inbound(event);
                        total_events += 1;
                    }
                }
                Err(e) => {
                    warn!(
                        adapter = adapter.adapter_name(),
                        error = %e,
                        "Failed to poll adapter"
                    );
                }
            }
        }

        if total_events > 0 {
            debug!(events = total_events, "Poll period completed");
        }
    }

    /// Route an inbound integration event to the EventBus.
    fn route_inbound(&self, event: IntegrationEvent) {
        let envelope = EventEnvelope {
            event_type: format!("integration.{}.{}", event.source, event.event_type),
            entity_type: event.source.clone(),
            entity_id: event.id.clone(),
            org_id: event.org_id,
            actor: format!("integration:{}", event.source),
            actor_type: "integration".to_string(),
            correlation_id: None,
            payload: event.payload,
            chain_depth: 0,
        };

        self.event_bus.publish(envelope);
    }

    /// Start the polling loop (runs in background via tokio::spawn).
    pub async fn start_polling(self: Arc<Self>) {
        let interval = self.config.poll_interval_secs;
        info!(interval_secs = interval, "Starting event bridge polling");

        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(
                tokio::time::Duration::from_secs(interval),
            );

            loop {
                ticker.tick().await;
                self.poll_period().await;
            }
        });
    }

    /// Get status of all registered adapters.
    pub async fn adapter_statuses(&self) -> Vec<(String, String, AdapterStatus)> {
        let adapters = self.adapters.read().await;
        adapters.iter()
            .map(|a| (
                a.adapter_name().to_string(),
                a.category().to_string(),
                a.status(),
            ))
            .collect()
    }
}
