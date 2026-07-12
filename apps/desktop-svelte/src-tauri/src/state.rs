use std::sync::Arc;
use std::path::Path;
use sea_orm::DatabaseConnection;

// Re-export provider from infra-persistence
pub use infra_persistence::provider::DatabaseProvider;
pub use shared_kernel::config::Config;

use shared_kernel::context::ServiceContext;
use shared_kernel::event_bus::EventBus;
use shared_kernel::traits::Actor;

/// Platform services container — holds all Arc'd trait implementations.
/// Created once at startup, reused for all requests.
pub struct PlatformServices {
    pub permissions: Arc<dyn shared_kernel::traits::PermissionChecker>,
    pub audit: Arc<dyn shared_kernel::traits::AuditLogger>,
    pub settings: Arc<dyn shared_kernel::traits::SettingsProvider>,
    pub policy: Arc<dyn shared_kernel::traits::PolicyEngine>,
    pub notifier: Arc<dyn shared_kernel::traits::NotificationSender>,
    pub fee_assigner: Arc<dyn shared_kernel::traits::FeeAssigner>,
    pub event_bus: Arc<EventBus>,
    pub event_store: Arc<shared_kernel::event_store::EventStore>,
    pub reactive: Arc<shared_kernel::reactive::ReactivePipeline>,
    pub plugins: Arc<shared_kernel::plugin::PluginRegistry>,
}

impl PlatformServices {
    /// Build all platform services from a database connection.
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        use svc_iam::ops::permission_checker::IamPermissionChecker;
        use svc_iam::ops::audit_logger::DbAuditLogger;
        use svc_organization::ops::settings::OrgSettingsProvider;
        use svc_organization::ops::policy::OrgPolicyEngine;
        use svc_finance::ops::fee_assigner::FinanceFeeAssigner;
        use infra_integration::notification_sender::LogNotificationSender;

        // Build reactive pipeline with analyzers
        let mut reactive = shared_kernel::reactive::ReactivePipeline::new();
        reactive.register(Arc::new(
            svc_assessment::ops::at_risk_predictor::AtRiskPredictor::new(),
        ));

        let db_for_store = db.clone();

        Self {
            permissions: Arc::new(IamPermissionChecker::new(db.clone())),
            audit: Arc::new(DbAuditLogger::new(db.clone())),
            settings: Arc::new(OrgSettingsProvider::new(db.clone())),
            policy: Arc::new(OrgPolicyEngine::new(db.clone())),
            notifier: Arc::new(LogNotificationSender::new()),
            fee_assigner: Arc::new(FinanceFeeAssigner::new(db)),
            event_bus,
            event_store: Arc::new(shared_kernel::event_store::EventStore::new(db_for_store)),
            reactive: Arc::new(reactive),
            plugins: Arc::new(shared_kernel::plugin::PluginRegistry::new()),
        }
    }

    /// Start the background event subscriber loop.
    ///
    /// Subscribes to EventBus and feeds every event through:
    ///   1. ReactivePipeline (analyzers → DetectedInsight)
    ///   2. PluginRegistry (plugins react to events)
    ///   3. Insights are logged (future: persisted to ai_insight table)
    pub fn start_event_loop(self: &Arc<Self>) {
        let platform = Arc::clone(self);
        let mut rx = platform.event_bus.subscribe();

        tokio::spawn(async move {
            tracing::info!("Event loop started — listening for domain events");
            loop {
                match rx.recv().await {
                    Ok(event) => {
                        // Skip events that are too deep in the chain
                        if event.is_chain_too_deep() {
                            tracing::warn!(
                                event_type = %event.event_type,
                                depth = event.chain_depth,
                                "Event chain too deep — skipping"
                            );
                            continue;
                        }

                        let org_id = &event.org_id;

                        // 1. Run through reactive pipeline (produces insights)
                        let db = platform.event_store.db();
                        let insights = platform.reactive.process(db, org_id, &event).await;
                        if !insights.is_empty() {
                            tracing::info!(
                                event = %event.event_type,
                                insights = insights.len(),
                                "Reactive pipeline produced insights"
                            );
                            // TODO: persist insights to ai_insight table
                        }

                        // 2. Notify plugins
                        platform.plugins.dispatch(db, org_id, &event).await;
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!(missed = n, "Event subscriber lagged — missed events");
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        tracing::info!("Event bus closed — stopping event loop");
                        break;
                    }
                }
            }
        });
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

/// Shared application state passed to all Tauri commands.
#[derive(Clone)]
pub struct AppState {
    /// Database provider.
    pub provider: Arc<DatabaseProvider>,
    
    /// Global application configuration.
    pub config: Arc<Config>,

    /// Platform services (new architecture).
    pub platform: Arc<PlatformServices>,
}

impl AppState {
    /// Initialize the AppState with full platform services.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Arc::new(DatabaseProvider::init(app_data_dir).await?);
        let config = Arc::new(Config::load());
        
        // Initialize platform services with the active database
        let db = provider.db();
        let event_bus = Arc::new(EventBus::new());
        let platform = Arc::new(PlatformServices::new(db, event_bus));
        
        // Start background event loop (reactive pipeline + plugins)
        platform.start_event_loop();
        
        Ok(Self { provider, config, platform })
    }

    /// Accessor for the database connection.
    pub fn db(&self) -> DatabaseConnection {
        self.provider.db()
    }

    /// Build a ServiceContext for the current session.
    ///
    /// Resolves the active user from the latest session and creates
    /// a fully-wired context for domain operations.
    pub async fn build_context(&self, org_id: &str) -> Result<ServiceContext, shared_kernel::errors::AppError> {
        let db = self.db();
        
        // Resolve current actor from the latest session
        use sea_orm::{EntityTrait, QueryOrder};
        let session = svc_iam::entities::user_session::Entity::find()
            .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
            .one(&db)
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session"))?;

        let actor = Actor::User(session.user_id);
        let mut ctx = self.platform.context_for(db, org_id, actor);
        ctx.ensure_correlation_id();
        Ok(ctx)
    }

    /// Build a system-level ServiceContext (for background tasks, events).
    pub async fn build_system_context(&self, org_id: &str) -> ServiceContext {
        let db = self.db();
        let mut ctx = self.platform.context_for(db, org_id, Actor::System);
        ctx.ensure_correlation_id();
        ctx
    }

    /// Legacy: verify RBAC permissions for the current active user session.
    pub async fn check_tauri_permission(
        &self,
        db: &DatabaseConnection,
        permission: svc_iam::services::authz::Permission,
        context: Option<&svc_iam::services::authz::AuthzContext>,
    ) -> Result<(), shared_kernel::errors::AppError> {
        use sea_orm::{EntityTrait, QueryOrder};
        
        let latest_session = svc_iam::entities::user_session::Entity::find()
            .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
            .one(db)
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session found"))?;

        let user = svc_iam::entities::user::Entity::find_by_id(latest_session.user_id)
            .one(db)
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("Active session user not found"))?;

        let authz = svc_iam::services::authz::AuthzService::new();
        authz.check_permission(db, &user, permission, context).await?;

        Ok(())
    }
}
