//! In-process event bus built on `tokio::broadcast`.
//!
//! # Design Decisions (ADQ-5)
//!
//! - **Async, at-most-once delivery** — if a consumer is slow or absent,
//!   events are dropped (ring buffer). This is acceptable for a desktop app.
//! - **No external message broker** — Tauri desktop app runs locally.
//! - **Bounded channel (1024)** — prevents unbounded memory growth.
//! - **Critical operations use outbox pattern** — they write to
//!   `autopilot_execution_log` before execution for guaranteed delivery.
//!
//! # Event Chain Safety
//!
//! Each event carries a `correlation_id`. Consumers must check this to prevent
//! infinite loops (max chain depth = 5).

use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, warn};

use serde::{Serialize, Deserialize};

/// Maximum events in the ring buffer before oldest are dropped.
const CHANNEL_CAPACITY: usize = 1024;

/// Maximum depth of event chains to prevent infinite loops.
pub const MAX_EVENT_CHAIN_DEPTH: u8 = 5;

/// A serialized domain event for broadcasting.
///
/// We use a serialized form instead of trait objects because `broadcast::Sender`
/// requires `Clone`, and trait objects don't implement `Clone` easily.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub org_id: String,
    pub actor: String,
    pub actor_type: String,
    pub correlation_id: Option<String>,
    pub payload: serde_json::Value,
    pub chain_depth: u8,
}

impl EventEnvelope {
    /// Check if this event chain has exceeded the maximum depth.
    pub fn is_chain_too_deep(&self) -> bool {
        self.chain_depth >= MAX_EVENT_CHAIN_DEPTH
    }

    /// Create a child envelope for an event triggered by this one.
    pub fn child_envelope(
        &self,
        event_type: impl Into<String>,
        entity_type: impl Into<String>,
        entity_id: impl Into<String>,
        actor: impl Into<String>,
        actor_type: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            event_type: event_type.into(),
            entity_type: entity_type.into(),
            entity_id: entity_id.into(),
            org_id: self.org_id.clone(),
            actor: actor.into(),
            actor_type: actor_type.into(),
            correlation_id: self.correlation_id.clone(),
            payload,
            chain_depth: self.chain_depth + 1,
        }
    }
}

/// In-process event bus using `tokio::broadcast`.
#[derive(Clone)]
pub struct EventBus {
    sender: broadcast::Sender<Arc<EventEnvelope>>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(CHANNEL_CAPACITY);
        Self { sender }
    }

    /// Publish an event. Fire-and-forget — never blocks the caller.
    pub fn publish(&self, envelope: EventEnvelope) {
        if envelope.is_chain_too_deep() {
            warn!(
                event_type = %envelope.event_type,
                chain_depth = envelope.chain_depth,
                "Event chain too deep — dropping to prevent infinite loop"
            );
            return;
        }

        match self.sender.send(Arc::new(envelope)) {
            Ok(receivers) => {
                debug!("Event published to {receivers} receiver(s)");
            }
            Err(_) => {
                // No receivers subscribed — this is normal during startup
                debug!("Event published but no receivers subscribed");
            }
        }
    }

    /// Subscribe to receive events. Returns a receiver that can be polled.
    pub fn subscribe(&self) -> broadcast::Receiver<Arc<EventEnvelope>> {
        self.sender.subscribe()
    }

    /// Number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_publish_subscribe() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();

        bus.publish(EventEnvelope {
            event_type: "enrollment.created".into(),
            entity_type: "enrollment".into(),
            entity_id: "e123".into(),
            org_id: "org1".into(),
            actor: "user1".into(),
            actor_type: "user".into(),
            correlation_id: None,
            payload: serde_json::json!({}),
            chain_depth: 0,
        });

        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_type, "enrollment.created");
        assert_eq!(received.entity_id, "e123");
    }

    #[tokio::test]
    async fn test_chain_depth_guard() {
        let bus = EventBus::new();
        let mut rx = bus.subscribe();

        // Event at max depth should be dropped
        bus.publish(EventEnvelope {
            event_type: "test.deep".into(),
            entity_type: "test".into(),
            entity_id: "t1".into(),
            org_id: "org1".into(),
            actor: "system".into(),
            actor_type: "system".into(),
            correlation_id: None,
            payload: serde_json::json!({}),
            chain_depth: MAX_EVENT_CHAIN_DEPTH,
        });

        // Should timeout — event was dropped
        let result = tokio::time::timeout(
            std::time::Duration::from_millis(50),
            rx.recv(),
        ).await;
        assert!(result.is_err());
    }
}
