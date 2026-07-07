//! Domain event traits and handler contracts.
//!
//! These traits define the contracts that domain crates implement.
//! They live in `shared-kernel` so that domain crates depend only on
//! abstractions, not on each other.

use serde::Serialize;
use std::any::Any;

use crate::errors::AppError;

// ── Domain Event ────────────────────────────────────────────────

/// A domain event represents something that happened in the system.
///
/// Events are published to the `EventBus` after a successful command.
/// Consumers (Autopilot, AI Gateway, Notifications) subscribe to relevant events.
///
/// # Design
/// - Events are immutable — they represent facts.
/// - Events are serializable — for persistence in `audit_log` and `integration_events`.
/// - Events carry a `correlation_id` to link related operations.
pub trait DomainEvent: Any + Send + Sync + 'static {
    /// Machine-readable event type, e.g. `enrollment.created`.
    fn event_type(&self) -> &'static str;

    /// The entity type that changed, e.g. `enrollment`.
    fn entity_type(&self) -> &'static str;

    /// The ID of the entity that changed.
    fn entity_id(&self) -> &str;

    /// The organization that owns this entity.
    fn org_id(&self) -> &str;

    /// Optional correlation ID to link related events in a chain.
    fn correlation_id(&self) -> Option<&str> {
        None
    }

    /// Who caused this event — user ID, "system", or agent session ID.
    fn actor(&self) -> &str;

    /// Serialize the event payload for storage/transport.
    fn payload(&self) -> serde_json::Value;
}

// ── Command / Query Contracts ───────────────────────────────────

/// A command handler processes a write operation and may emit domain events.
///
/// Commands mutate state. They must be:
/// - Validated before execution
/// - Executed within a transaction (if applicable)
/// - Followed by event emission on success
#[allow(async_fn_in_trait)]
pub trait CommandHandler<C, R> {
    async fn handle(&self, command: C) -> Result<R, AppError>;
}

/// A query handler processes a read operation.
///
/// Queries are pure reads — they never mutate state.
/// They can bypass domain logic and use optimized SQL directly (CQRS).
#[allow(async_fn_in_trait)]
pub trait QueryHandler<Q, R> {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}

// ── Actor Type ──────────────────────────────────────────────────

/// Identifies who performed an operation — used in audit logging.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum Actor {
    /// A human user.
    User(String),
    /// The system (migrations, scheduled tasks).
    System,
    /// An AI agent session.
    Agent(String),
    /// The Autopilot engine.
    Autopilot,
}

impl Actor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User(id) => id,
            Self::System => "system",
            Self::Agent(id) => id,
            Self::Autopilot => "autopilot",
        }
    }

    pub fn actor_type(&self) -> &'static str {
        match self {
            Self::User(_) => "user",
            Self::System => "system",
            Self::Agent(_) => "ai_agent",
            Self::Autopilot => "autopilot",
        }
    }
}
