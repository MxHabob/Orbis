use std::sync::Arc;
use std::path::Path;
use sea_orm::DatabaseConnection;
use tokio::sync::RwLock;

// Re-export provider from infra-persistence
pub use infra_persistence::provider::DatabaseProvider;
pub use shared_kernel::config::Config;

use shared_kernel::context::ServiceContext;
use shared_kernel::event_bus::EventBus;
use shared_kernel::tenant_guard::TenantGuard;
use shared_kernel::period_gate::PeriodGate;
use shared_kernel::traits::Actor;
use svc_report::org_context::OrgContext;

/// Authenticated session context — set at login, cleared at logout.
///
/// The `org_id` is fixed for the session lifetime. This prevents the frontend
/// from passing arbitrary org_ids.
#[derive(Debug, Clone)]
pub struct SessionContext {
    pub user_id: String,
    pub org_id: String,
    pub org_unit_id: Option<String>,
    pub roles: Vec<String>,
    pub guard: TenantGuard,
}

/// Platform services container — holds all Arc'd trait implementations.
/// Created once at startup, reused for all requests.
pub struct PlatformServices {
    pub permissions: Arc<dyn shared_kernel::traits::PermissionChecker>,
    pub audit: Arc<dyn shared_kernel::traits::AuditLogger>,
    pub settings: Arc<dyn shared_kernel::traits::SettingsProvider>,
    pub policy: Arc<dyn shared_kernel::traits::PolicyEngine>,
    pub notifier: Arc<dyn shared_kernel::traits::NotificationSender>,
    pub fee_assigner: Arc<dyn shared_kernel::traits::FeeAssigner>,
    pub period_gate: Arc<PeriodGate>,
    pub event_bus: Arc<EventBus>,
    pub event_store: Arc<shared_kernel::event_store::EventStore>,
    pub reactive: Arc<shared_kernel::reactive::ReactivePipeline>,
    pub plugins: Arc<shared_kernel::plugin::PluginRegistry>,
    pub storage: Arc<dyn infra_integration::traits::StorageProvider>,
    pub job_queue: Arc<shared_kernel::job_queue::JobQueue>,
}

impl PlatformServices {
    /// Build all platform services from a database connection.
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        Self::new_with_storage(db, event_bus, "app_data/storage")
    }

    pub fn new_with_storage(
        db: DatabaseConnection,
        event_bus: Arc<EventBus>,
        storage_path: impl AsRef<Path>,
    ) -> Self {
        use svc_iam::ops::permission_checker::IamPermissionChecker;
        use svc_iam::ops::audit_logger::DbAuditLogger;
        use svc_organization::ops::settings::OrgSettingsProvider;
        use platform_policy::engine::PlatformPolicyEngine;
        use svc_finance::ops::fee_assigner::FinanceFeeAssigner;
        use infra_integration::notification_sender::DbNotificationSender;

        // Build reactive pipeline with analyzers
        let mut reactive = shared_kernel::reactive::ReactivePipeline::new();
        reactive.register(Arc::new(
            svc_assessment::ops::at_risk_predictor::AtRiskPredictor::new(),
        ));
        reactive.register(Arc::new(
            svc_finance::ops::reactive_analyzers::FinanceAnalyzer::new(),
        ));

        let db_for_store = db.clone();
        
        let storage = infra_integration::adapters::storage::local::LocalStorageProvider::new(storage_path);

        Self {
            permissions: Arc::new(IamPermissionChecker::new(db.clone())),
            audit: Arc::new(DbAuditLogger::new(db.clone())),
            settings: Arc::new(OrgSettingsProvider::new(db.clone())),
            policy: Arc::new(PlatformPolicyEngine::new(db.clone())),
            notifier: Arc::new(DbNotificationSender::new(db.clone())),
            fee_assigner: Arc::new(FinanceFeeAssigner::new(db)),
            period_gate: Arc::new(PeriodGate::new()),
            event_bus,
            event_store: Arc::new(shared_kernel::event_store::EventStore::new(db_for_store)),
            reactive: Arc::new(reactive),
            plugins: Arc::new(shared_kernel::plugin::PluginRegistry::new()),
            storage: Arc::new(storage),
            job_queue: Arc::new(shared_kernel::job_queue::JobQueue::new(1024)),
        }
    }

    /// Start the background domain event loop (reactive + plugins + DocForge).
    pub fn start_event_loop(self: &Arc<Self>) {
        shared_kernel::outbox_relay::OutboxRelay::new(
            self.event_store.db().clone(),
            self.event_bus.clone(),
        ).spawn();

        shared_kernel::event_loop::DomainEventLoop {
            event_bus: self.event_bus.clone(),
            db: self.event_store.db().clone(),
            reactive: self.reactive.clone(),
            plugins: self.plugins.clone(),
            event_store: self.event_store.clone(),
            projection_handler: Some(docforge_orbis::projection_handler::handler()),
        }
        .spawn();
    }

    /// Start autopilot background scheduler (15-minute interval).
    pub fn start_autopilot_scheduler(self: &Arc<Self>) {
        autopilot::subscriber::AutopilotSubscriber::spawn(
            self.event_store.db().clone(),
            self.event_bus.clone(),
            self.period_gate.clone(),
        );

        autopilot::scheduler::spawn_scheduler(
            self.event_store.db().clone(),
            self.event_bus.clone(),
            self.period_gate.clone(),
            std::time::Duration::from_secs(15 * 60),
        );
    }

    /// Run global catch-up on startup.
    pub fn start_autopilot_catch_up(self: &Arc<Self>) {
        autopilot::resumption::ResumptionService::spawn_catch_up_all(
            self.event_store.db().clone(),
            self.event_bus.clone(),
            self.period_gate.clone(),
        );
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
            self.period_gate.clone(),
            self.event_bus.clone(),
            self.event_store.clone(),
            self.reactive.clone(),
            self.plugins.clone(),
            self.job_queue.clone(),
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

    /// Canonical backend report service shared by all Tauri report commands.
    pub report_service: Arc<svc_report::service::ReportService>,

    /// Authenticated session — set at login, cleared at logout.
    pub session: Arc<RwLock<Option<SessionContext>>>,
}

impl AppState {
    /// Initialize the AppState with full platform services.
    pub async fn init(app_data_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let provider = Arc::new(DatabaseProvider::init(app_data_dir).await?);
        let config = Arc::new(Config::load());
        
        // Initialize platform services with the active database
        let db = provider.db();
        let event_bus = Arc::new(EventBus::new());
        let platform = Arc::new(PlatformServices::new_with_storage(
            db,
            event_bus,
            app_data_dir.join("storage"),
        ));
        
        // Start background event loop + autopilot scheduler + catch-up
        platform.start_event_loop();
        platform.start_autopilot_scheduler();
        platform.start_autopilot_catch_up();

        let report_registry = Arc::new(svc_report::registry::ReportRegistry::new());
        let report_service = Arc::new(svc_report::service::ReportService::new(
            report_registry,
            Arc::new(provider.db()),
        ));
        report_service.register_builtin_reports();
        
        Ok(Self {
            provider,
            config,
            platform,
            report_service,
            session: Arc::new(RwLock::new(None)),
        })
    }

    /// Accessor for the database connection.
    pub fn db(&self) -> DatabaseConnection {
        self.provider.db()
    }

    /// Resolve report branding and locale from the authenticated tenant record.
    pub async fn report_org_context(&self, org_id: &str) -> Result<OrgContext, shared_kernel::errors::AppError> {
        use sea_orm::{ConnectionTrait, Statement};

        let row = self.db()
            .query_one(Statement::from_sql_and_values(
                self.db().get_database_backend(),
                "SELECT id, name, name_ar, code, logo, locale, timezone, hijri_enabled, settings FROM organizations WHERE id = $1",
                vec![org_id.to_string().into()],
            ))
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::not_found("organization", org_id))?;

        let settings_json: String = row.try_get("", "settings").unwrap_or_default();
        let settings: shared_kernel::settings::OrgSettings = serde_json::from_str(&settings_json).unwrap_or_default();
        let currency = settings.finance.default_currency;
        let currency_symbol = match currency.as_str() {
            "SAR" => "ر.س".to_string(),
            "YER" => "ر.ي".to_string(),
            "USD" => "$".to_string(),
            "EUR" => "€".to_string(),
            _ => currency.clone(),
        };

        Ok(OrgContext {
            id: row.try_get("", "id").unwrap_or_else(|_| org_id.to_string()),
            name: row.try_get("", "name").unwrap_or_default(),
            name_ar: row.try_get("", "name_ar").unwrap_or_default(),
            code: row.try_get("", "code").unwrap_or_default(),
            logo_url: row.try_get("", "logo").unwrap_or_default(),
            locale: row.try_get("", "locale").unwrap_or_else(|_| "ar".into()),
            timezone: row.try_get("", "timezone").unwrap_or_else(|_| "Asia/Riyadh".into()),
            hijri_enabled: row.try_get::<i32>("", "hijri_enabled").unwrap_or(0) != 0,
            currency,
            currency_symbol,
            ..Default::default()
        })
    }

    // ── Session Management ──────────────────────────────────────────

    /// Set the authenticated session after login.
    pub async fn set_session(&self, user_id: String, org_id: String, roles: Vec<String>) {
        let guard = TenantGuard::new(&org_id);
        let ctx = SessionContext {
            user_id,
            org_id,
            org_unit_id: None,
            roles,
            guard,
        };
        *self.session.write().await = Some(ctx);
    }

    /// Select the active branch for the authenticated session.
    ///
    /// Branch access is resolved from the user's organization-scoped role
    /// assignments. Organization administrators may select any active unit;
    /// other users require a global, direct, or subtree assignment.
    pub async fn set_active_org_unit(
        &self,
        org_unit_id: Option<String>,
    ) -> Result<(), shared_kernel::errors::AppError> {
        use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

        let mut session = self
            .session
            .write()
            .await;
        let current = session
            .as_mut()
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session"))?;

        let Some(org_unit_id) = org_unit_id else {
            current.org_unit_id = None;
            return Ok(());
        };

        let unit = svc_organization::entities::org_unit::Entity::find()
            .filter(svc_organization::entities::org_unit::Column::Id.eq(&org_unit_id))
            .filter(svc_organization::entities::org_unit::Column::OrgId.eq(&current.org_id))
            .filter(svc_organization::entities::org_unit::Column::IsActive.eq(1))
            .one(&self.db())
            .await?
            .ok_or_else(|| shared_kernel::errors::AppError::not_found("org_unit", &org_unit_id))?;

        let is_org_admin = current.roles.iter().any(|role| {
            matches!(role.to_ascii_lowercase().as_str(), "admin" | "super_admin")
        });

        if !is_org_admin {
            let assignments = svc_iam::entities::user_role::Entity::find()
                .filter(svc_iam::entities::user_role::Column::UserId.eq(&current.user_id))
                .filter(svc_iam::entities::user_role::Column::OrgId.eq(&current.org_id))
                .all(&self.db())
                .await?;

            let allowed = assignments.iter().any(|assignment| {
                match assignment.scope_type.to_ascii_lowercase().as_str() {
                    "global" | "organization" | "org" => true,
                    "branch" | "org_unit" | "unit" => {
                        assignment.scope_id.as_deref() == Some(org_unit_id.as_str())
                    }
                    "subtree" => assignment.scope_id.as_deref().is_some_and(|scope_id| {
                        unit.path == scope_id
                            || unit.path.starts_with(&format!("{}/", scope_id.trim_end_matches('/')))
                    }),
                    _ => false,
                }
            });

            if !allowed {
                return Err(shared_kernel::errors::AppError::unauthorized(
                    "You are not assigned to this organizational unit",
                ));
            }
        }

        current.org_unit_id = Some(org_unit_id);
        Ok(())
    }

    /// Set a Super Admin session with access to multiple orgs.
    pub async fn set_super_admin_session(
        &self,
        user_id: String,
        org_id: String,
        roles: Vec<String>,
        allowed_org_ids: Vec<String>,
    ) {
        let guard = TenantGuard::with_allowed_orgs(&org_id, allowed_org_ids);
        let ctx = SessionContext {
            user_id,
            org_id,
            org_unit_id: None,
            roles,
            guard,
        };
        *self.session.write().await = Some(ctx);
    }

    /// Clear the session on logout.
    pub async fn clear_session(&self) {
        *self.session.write().await = None;
    }

    // ── Context Building ────────────────────────────────────────────

    /// Build a ServiceContext from the active session.
    ///
    /// This is the **primary** context builder. It reads `org_id` from the
    /// session — the frontend never passes it.
    pub async fn build_guarded_context(
        &self,
    ) -> Result<ServiceContext, shared_kernel::errors::AppError> {
        let session_lock = self.session.read().await;
        let session = session_lock
            .as_ref()
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session"))?;

        let user_id = session.user_id.clone();
        let org_id = session.org_id.clone();
        let active_org_unit_id = session.org_unit_id.clone();
        let roles = session.roles.clone();
        drop(session_lock);

        let settings = self.platform.settings.get_org_settings(&org_id).await?;

        let is_org_admin = roles.iter().any(|role| {
            matches!(role.to_ascii_lowercase().as_str(), "admin" | "super_admin")
        });

        if active_org_unit_id.is_none() && !is_org_admin {
            use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

            let assignments = svc_iam::entities::user_role::Entity::find()
                .filter(svc_iam::entities::user_role::Column::UserId.eq(&user_id))
                .filter(svc_iam::entities::user_role::Column::OrgId.eq(&org_id))
                .all(&self.db())
                .await?;

            let has_restricted_scope = assignments.iter().any(|assignment| {
                !matches!(
                    assignment.scope_type.to_ascii_lowercase().as_str(),
                    "global" | "organization" | "org"
                )
            });

            let branch_selection_required = settings.branch_policy.selection_required
                || settings.branch_policy.data_isolation.eq_ignore_ascii_case("isolated");
            if has_restricted_scope && branch_selection_required {
                return Err(shared_kernel::errors::AppError::unauthorized(
                    "Select an organizational unit before accessing scoped data",
                ));
            }
        }

        let db = self.db();
        let actor = Actor::User(user_id);
        let mut ctx = self.platform.context_for(db, &org_id, actor);
        ctx.org_unit_id = active_org_unit_id;
        ctx.ensure_correlation_id();
        Ok(ctx)
    }

    /// Build a ServiceContext for a **specific** org_id (Super Admin only).
    ///
    /// Validates against the TenantGuard before building.
    pub async fn build_context_for_org(
        &self,
        target_org_id: &str,
    ) -> Result<ServiceContext, shared_kernel::errors::AppError> {
        let session_lock = self.session.read().await;
        let session = session_lock
            .as_ref()
            .ok_or_else(|| shared_kernel::errors::AppError::unauthorized("No active session"))?;

        // Validate tenant boundary
        session.guard.validate(target_org_id)?;

        let db = self.db();
        let actor = Actor::User(session.user_id.clone());
        let mut ctx = self.platform.context_for(db, target_org_id, actor);
        ctx.ensure_correlation_id();
        Ok(ctx)
    }

    /// Build a ServiceContext for the current session (legacy — delegates to guarded).
    ///
    /// Kept for backward compatibility during migration. New code should use
    /// `build_guarded_context()`.
    pub async fn build_context(
        &self,
        org_id: &str,
    ) -> Result<ServiceContext, shared_kernel::errors::AppError> {
        // If there's a session, validate against it
        let session_lock = self.session.read().await;
        if let Some(ref session) = *session_lock {
            session.guard.validate(org_id)?;
        }
        drop(session_lock);

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
        permission: shared_kernel::traits::Permission,
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

