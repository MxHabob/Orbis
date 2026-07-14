//! ServiceContext — the unified context passed to every operation.
//!
//! Every `ops::` function receives a `ServiceContext` that carries:
//!   - Database connection
//!   - Organization identity (tenant boundary)
//!   - Actor identity (who is performing the operation)
//!   - All platform services (permissions, audit, settings, policies, etc.)
//!   - Event infrastructure (EventBus, EventStore, ReactivePipeline, Plugins)
//!
//! This ensures that every operation in the system respects:
//!   ✓ Permissions  ✓ Settings  ✓ Audit  ✓ Events  ✓ Policies

use std::sync::Arc;
use sea_orm::DatabaseConnection;

use crate::event_bus::EventBus;
use crate::event_store::EventStore;
use crate::reactive::ReactivePipeline;
use crate::plugin::PluginRegistry;
use crate::period_gate::PeriodGate;
use crate::traits::{
    Actor, PermissionChecker, AuditLogger, SettingsProvider,
    PolicyEngine, NotificationSender, FeeAssigner,
};

/// The unified context that every operation carries.
///
/// # Construction
///
/// Built once per request at the delivery layer (Tauri command or HTTP handler),
/// then passed down to `ops::` functions.
///
/// ```rust,ignore
/// let ctx = ServiceContext::new(db, org_id, actor)
///     .with_permissions(permission_checker)
///     .with_audit(audit_logger)
///     .with_settings(settings_provider)
///     .with_policy(policy_engine)
///     .with_events(event_bus, event_store, reactive, plugins);
/// ```
#[derive(Clone)]
pub struct ServiceContext {
    // ── Core ──
    pub db: DatabaseConnection,
    pub org_id: String,
    pub org_unit_id: Option<String>,
    pub period_id: String,
    pub actor: Actor,
    pub correlation_id: Option<String>,

    // ── Platform Services ──
    pub permissions: Arc<dyn PermissionChecker>,
    pub audit: Arc<dyn AuditLogger>,
    pub settings: Arc<dyn SettingsProvider>,
    pub policy: Arc<dyn PolicyEngine>,
    pub notifier: Arc<dyn NotificationSender>,
    pub fee_assigner: Arc<dyn FeeAssigner>,
    pub period_gate: Arc<PeriodGate>,

    // ── Event Infrastructure ──
    pub event_bus: Arc<EventBus>,
    pub event_store: Arc<EventStore>,
    pub reactive: Arc<ReactivePipeline>,
    pub plugins: Arc<PluginRegistry>,
}

impl ServiceContext {
    /// Create a new context with required core fields and platform services.
    pub fn new(
        db: DatabaseConnection,
        org_id: impl Into<String>,
        period_id: impl Into<String>,
        actor: Actor,
        permissions: Arc<dyn PermissionChecker>,
        audit: Arc<dyn AuditLogger>,
        settings: Arc<dyn SettingsProvider>,
        policy: Arc<dyn PolicyEngine>,
        notifier: Arc<dyn NotificationSender>,
        fee_assigner: Arc<dyn FeeAssigner>,
        period_gate: Arc<PeriodGate>,
        event_bus: Arc<EventBus>,
        event_store: Arc<EventStore>,
        reactive: Arc<ReactivePipeline>,
        plugins: Arc<PluginRegistry>,
    ) -> Self {
        Self {
            db,
            org_id: org_id.into(),
            org_unit_id: None,
            period_id: period_id.into(),
            actor,
            correlation_id: None,
            permissions,
            audit,
            settings,
            policy,
            notifier,
            fee_assigner,
            period_gate,
            event_bus,
            event_store,
            reactive,
            plugins,
        }
    }

    /// Attach a correlation ID to this context (for tracing related operations).
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }

    /// Create a child context with a different actor (e.g. system operations
    /// triggered by a user action).
    pub fn as_actor(&self, actor: Actor) -> Self {
        let mut child = self.clone();
        child.actor = actor;
        child
    }

    /// Switch to a different academic year within the same request.
    pub fn with_period_id(mut self, period_id: impl Into<String>) -> Self {
        self.period_id = period_id.into();
        self
    }

    /// Set the organizational unit context (branch/department).
    pub fn with_org_unit(mut self, unit_id: impl Into<String>) -> Self {
        self.org_unit_id = Some(unit_id.into());
        self
    }

    /// Generate a new correlation ID if none is set.
    pub fn ensure_correlation_id(&mut self) {
        if self.correlation_id.is_none() {
            self.correlation_id = Some(uuid::Uuid::new_v4().to_string());
        }
    }
}

impl std::fmt::Debug for ServiceContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceContext")
            .field("org_id", &self.org_id)
            .field("org_unit_id", &self.org_unit_id)
            .field("period_id", &self.period_id)
            .field("actor", &self.actor)
            .field("correlation_id", &self.correlation_id)
            .finish_non_exhaustive()
    }
}
