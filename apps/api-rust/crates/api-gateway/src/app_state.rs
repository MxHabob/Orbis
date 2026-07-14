//! Application state for the API Gateway.
//!
//! Mirrors the Desktop's AppState but for HTTP context.
//! Provides `build_context()` to create a `ServiceContext` for any request.

use std::sync::Arc;
use sea_orm::DatabaseConnection;
use shared_kernel::context::ServiceContext;
use shared_kernel::traits::Actor;
use shared_kernel::event_bus::EventBus;
use shared_kernel::errors::AppError;

/// Platform services — same as Desktop, shared between both.
pub struct PlatformServices {
    pub permissions: Arc<dyn shared_kernel::traits::PermissionChecker>,
    pub audit: Arc<dyn shared_kernel::traits::AuditLogger>,
    pub settings: Arc<dyn shared_kernel::traits::SettingsProvider>,
    pub policy: Arc<dyn shared_kernel::traits::PolicyEngine>,
    pub notifier: Arc<dyn shared_kernel::traits::NotificationSender>,
    pub fee_assigner: Arc<dyn shared_kernel::traits::FeeAssigner>,
    pub period_gate: Arc<shared_kernel::period_gate::PeriodGate>,
    pub event_bus: Arc<EventBus>,
    pub event_store: Arc<shared_kernel::event_store::EventStore>,
    pub reactive: Arc<shared_kernel::reactive::ReactivePipeline>,
    pub plugins: Arc<shared_kernel::plugin::PluginRegistry>,
}

impl PlatformServices {
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        use svc_iam::ops::permission_checker::IamPermissionChecker;
        use svc_iam::ops::audit_logger::DbAuditLogger;
        use svc_organization::ops::settings::OrgSettingsProvider;
        use svc_organization::ops::policy::OrgPolicyEngine;
        use svc_finance::ops::fee_assigner::FinanceFeeAssigner;
        use infra_integration::notification_sender::LogNotificationSender;

        let mut reactive = shared_kernel::reactive::ReactivePipeline::new();
        reactive.register(Arc::new(
            svc_assessment::ops::at_risk_predictor::AtRiskPredictor::new(),
        ));
        reactive.register(Arc::new(
            svc_finance::ops::reactive_analyzers::FinanceAnalyzer::new(),
        ));

        let db_for_store = db.clone();

        Self {
            permissions: Arc::new(IamPermissionChecker::new(db.clone())),
            audit: Arc::new(DbAuditLogger::new(db.clone())),
            settings: Arc::new(OrgSettingsProvider::new(db.clone())),
            policy: Arc::new(OrgPolicyEngine::new(db.clone())),
            notifier: Arc::new(LogNotificationSender::new()),
            fee_assigner: Arc::new(FinanceFeeAssigner::new(db.clone())),
            period_gate: Arc::new(shared_kernel::period_gate::PeriodGate::new()),
            event_bus,
            event_store: Arc::new(shared_kernel::event_store::EventStore::new(db_for_store)),
            reactive: Arc::new(reactive),
            plugins: Arc::new(shared_kernel::plugin::PluginRegistry::new()),
        }
    }

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
            self.period_gate.clone(),
            self.event_bus.clone(),
            self.event_store.clone(),
            self.reactive.clone(),
            self.plugins.clone(),
        )
    }
}

/// Shared application state for all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub platform: Arc<PlatformServices>,
    pub config: Arc<shared_kernel::config::Config>,
}

impl AppState {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(shared_kernel::config::Config::load());
        let db = infra_persistence::connection::connect_db(&config.database_url).await?;
        let event_bus = Arc::new(EventBus::new());
        let platform = Arc::new(PlatformServices::new(db.clone(), event_bus));

        Ok(Self { db, platform, config })
    }

    /// Build a ServiceContext from the authenticated user.
    pub fn build_context(&self, org_id: &str, user_id: &str) -> ServiceContext {
        let actor = Actor::User(user_id.to_string());
        let mut ctx = self.platform.context_for(self.db.clone(), org_id, actor);
        ctx.ensure_correlation_id();
        ctx
    }

    /// Build a system-level ServiceContext.
    pub fn build_system_context(&self, org_id: &str) -> ServiceContext {
        let mut ctx = self.platform.context_for(self.db.clone(), org_id, Actor::System);
        ctx.ensure_correlation_id();
        ctx
    }

    /// Validate a Bearer token and return the user model.
    pub async fn validate_token(&self, token: &str) -> Result<crate::entities::user::Model, AppError> {
        use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
        let session = svc_iam::entities::user_session::Entity::find()
            .filter(svc_iam::entities::user_session::Column::Token.eq(token))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::unauthorized("Invalid or expired token"))?;

        let user = svc_iam::entities::user::Entity::find_by_id(&session.user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::unauthorized("User not found"))?;

        Ok(user)
    }
}
