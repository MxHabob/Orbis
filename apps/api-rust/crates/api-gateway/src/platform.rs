//! ServiceContext Bootstrap — creates a fully-wired ServiceContext.
//!
//! This module provides the factory function that assembles all
//! Platform Trait implementations into a single `ServiceContext`.
//!
//! Called once at application startup (Tauri or API server).

use std::sync::Arc;
use sea_orm::DatabaseConnection;

use shared_kernel::context::ServiceContext;
use shared_kernel::event_bus::EventBus;
use shared_kernel::event_store::EventStore;
use shared_kernel::plugin::PluginRegistry;
use shared_kernel::reactive::ReactivePipeline;
use shared_kernel::traits::Actor;

// Re-export concrete implementations for direct construction
pub use svc_iam::ops::permission_checker::IamPermissionChecker;
pub use svc_iam::ops::audit_logger::DbAuditLogger;
pub use svc_organization::ops::settings::OrgSettingsProvider;
pub use svc_organization::ops::policy::OrgPolicyEngine;
pub use svc_finance::ops::fee_assigner::FinanceFeeAssigner;

/// Platform services container — holds all Arc'd trait implementations.
///
/// Created once at startup, then used to build per-request `ServiceContext` instances.
pub struct PlatformServices {
    pub permissions: Arc<dyn shared_kernel::traits::PermissionChecker>,
    pub audit: Arc<dyn shared_kernel::traits::AuditLogger>,
    pub settings: Arc<dyn shared_kernel::traits::SettingsProvider>,
    pub policy: Arc<dyn shared_kernel::traits::PolicyEngine>,
    pub notifier: Arc<dyn shared_kernel::traits::NotificationSender>,
    pub fee_assigner: Arc<dyn shared_kernel::traits::FeeAssigner>,
    pub event_bus: Arc<EventBus>,
    pub event_store: Arc<EventStore>,
    pub reactive: Arc<ReactivePipeline>,
    pub plugins: Arc<PluginRegistry>,
}

impl PlatformServices {
    /// Build all platform services from a database connection.
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        use infra_integration::notification_sender::LogNotificationSender;

        Self {
            permissions: Arc::new(IamPermissionChecker::new(db.clone())),
            audit: Arc::new(DbAuditLogger::new(db.clone())),
            settings: Arc::new(OrgSettingsProvider::new(db.clone())),
            policy: Arc::new(OrgPolicyEngine::new(db.clone())),
            notifier: Arc::new(LogNotificationSender::new()),
            fee_assigner: Arc::new(FinanceFeeAssigner::new(db.clone())),
            event_bus,
            event_store: Arc::new(EventStore::new(db)),
            reactive: Arc::new(ReactivePipeline::new()),
            plugins: Arc::new(PluginRegistry::new()),
        }
    }

    /// Create a `ServiceContext` for a specific request.
    pub fn context_for(&self, db: DatabaseConnection, org_id: &str, actor: Actor) -> ServiceContext {
        ServiceContext::new(
            db,
            org_id,
            "",
            actor,
            self.permissions.clone(),
            self.audit.clone(),
            self.settings.clone(),
            self.policy.clone(),
            self.notifier.clone(),
            self.fee_assigner.clone(),
            self.event_bus.clone(),
            self.event_store.clone(),
            self.reactive.clone(),
            self.plugins.clone(),
        )
    }
}
