use orbis_domain::iam::actor::Actor;
use uuid::Uuid;

/// Describes the originating transport/subsystem for an action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionSource {
    /// HTTP API request (Axum handler).
    Api,
    /// Tauri IPC command from the desktop shell.
    TauriIpc,
    /// Background autopilot daemon.
    Autopilot,
    /// System-level operations (migrations, bootstrap, internal triggers).
    System,
    /// External integration (provider name attached).
    Integration(String),
}

impl Default for ActionSource {
    fn default() -> Self {
        ActionSource::Api
    }
}

impl ActionSource {
    pub fn as_str(&self) -> &str {
        match self {
            ActionSource::Api => "api",
            ActionSource::TauriIpc => "tauri_ipc",
            ActionSource::Autopilot => "autopilot",
            ActionSource::System => "system",
            ActionSource::Integration(name) => name.as_str(),
        }
    }
}

/// Unified execution context that flows through every application service call.
///
/// Constructed once per-request at the transport boundary (Axum extractor, Tauri
/// command, Autopilot scheduler) and passed through to services, repositories,
/// and the event bus. Services use it for authorization, audit attribution, and
/// OrgUnit scope enforcement.
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Organization ID. Derived from JWT claims (HTTP) or active session (Tauri).
    pub tenant_id: Uuid,
    /// Authenticated actor identity.
    pub actor: Actor,
    /// Unique correlation ID for distributed tracing / audit correlation.
    pub request_id: String,
    /// Originating transport/subsystem.
    pub source: ActionSource,
    /// Materialized OrgUnit path for branch-scoped queries (e.g., "/1/4/12/").
    /// `None` means org-wide access (no branch restriction).
    pub org_unit_path: Option<String>,
}

impl RequestContext {
    /// Returns the actor's user/agent UUID, or `None` for System/Autopilot.
    pub fn user_id(&self) -> Option<Uuid> {
        self.actor.id()
    }

    /// Returns a static string representing the actor type, suitable for audit
    /// trail `actor_type` column.
    pub fn actor_type_str(&self) -> &'static str {
        match &self.actor {
            Actor::User(_) => "user",
            Actor::Agent(_) => "agent",
            Actor::Autopilot => "autopilot",
            Actor::System => "system",
        }
    }

    /// Returns the source as a string slice for serialization.
    pub fn source_str(&self) -> &str {
        self.source.as_str()
    }
}
