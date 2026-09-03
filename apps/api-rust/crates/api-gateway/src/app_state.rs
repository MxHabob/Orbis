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
use svc_report::service::ReportService;
use svc_report::registry::ReportRegistry;

/// Platform services â€” same as Desktop, shared between both.
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
    pub storage: Arc<dyn infra_integration::traits::StorageProvider>,
    pub job_queue: Arc<shared_kernel::job_queue::JobQueue>,
}

impl PlatformServices {
    pub fn new(db: DatabaseConnection, event_bus: Arc<EventBus>) -> Self {
        use svc_iam::ops::permission_checker::IamPermissionChecker;
        use svc_iam::ops::audit_logger::DbAuditLogger;
        use svc_organization::ops::settings::OrgSettingsProvider;
        use platform_policy::engine::PlatformPolicyEngine;
        use svc_finance::ops::fee_assigner::FinanceFeeAssigner;
        use infra_integration::notification_sender::DbNotificationSender;

        let mut reactive = shared_kernel::reactive::ReactivePipeline::new();
        reactive.register(Arc::new(
            svc_assessment::ops::at_risk_predictor::AtRiskPredictor::new(),
        ));
        reactive.register(Arc::new(
            svc_finance::ops::reactive_analyzers::FinanceAnalyzer::new(),
        ));

        let db_for_store = db.clone();
        
        let storage = infra_integration::adapters::storage::local::LocalStorageProvider::new("app_data/storage");

        Self {
            permissions: Arc::new(IamPermissionChecker::new(db.clone())),
            audit: Arc::new(DbAuditLogger::new(db.clone())),
            settings: Arc::new(OrgSettingsProvider::new(db.clone())),
            policy: Arc::new(PlatformPolicyEngine::new(db.clone())),
            notifier: Arc::new(DbNotificationSender::new(db.clone())),
            fee_assigner: Arc::new(FinanceFeeAssigner::new(db.clone())),
            period_gate: Arc::new(shared_kernel::period_gate::PeriodGate::new()),
            event_bus,
            event_store: Arc::new(shared_kernel::event_store::EventStore::new(db_for_store)),
            reactive: Arc::new(reactive),
            plugins: Arc::new(shared_kernel::plugin::PluginRegistry::new()),
            storage: Arc::new(storage),
            job_queue: Arc::new(shared_kernel::job_queue::JobQueue::new(1024)),
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
            self.job_queue.clone(),
        )
    }

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

    pub fn start_autopilot_catch_up(self: &Arc<Self>) {
        autopilot::resumption::ResumptionService::spawn_catch_up_all(
            self.event_store.db().clone(),
            self.event_bus.clone(),
            self.period_gate.clone(),
        );
    }
}

/// Shared application state for all Axum handlers.
#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub platform: Arc<PlatformServices>,
    pub config: Arc<shared_kernel::config::Config>,
    pub report_svc: Arc<ReportService>,
}

impl AppState {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        Self::init_with_database_url(None).await
    }

    pub async fn init_with_database_url(
        database_url: Option<&str>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut loaded_config = shared_kernel::config::Config::load();
        if let Some(database_url) = database_url.filter(|url| !url.trim().is_empty()) {
            loaded_config.database_url = database_url.to_string();
        }
        let config = Arc::new(loaded_config);
        config.validate()?;
        let db = infra_persistence::connection::connect_db(&config.database_url).await?;

        let baseline_sql = if config.database_url.starts_with("postgres://") || config.database_url.starts_with("postgresql://") {
            include_str!("../../infra-persistence/src/schemas/schema_baseline_pg.sql")
        } else {
            include_str!("../../infra-persistence/src/schemas/schema_baseline.sql")
        };
        infra_persistence::migrations::run_sql_migration(&db, baseline_sql).await?;
        infra_persistence::schema_migrations::run_outbox_migrations(&db).await?;
        infra_persistence::schema_migrations::run_p0_migrations(&db).await?;
        infra_persistence::schema_migrations::run_p4_migrations(&db).await?;
        docforge_orbis::schema::migrate_template_schema(&db).await?;
        infra_persistence::schema_migrations::run_postgres_security_migrations(&db).await?;

        let event_bus = Arc::new(EventBus::new());
        let platform = Arc::new(PlatformServices::new(db.clone(), event_bus));
        platform.start_event_loop();
        platform.start_autopilot_scheduler();
        platform.start_autopilot_catch_up();
        let recovered_jobs = svc_scheduling::ops::solver::recover_interrupted_jobs(db.clone()).await?;
        if recovered_jobs > 0 {
            tracing::info!(count = recovered_jobs, "Recovered interrupted schedule solve jobs");
        }

        // Initialize report service
        let registry = Arc::new(ReportRegistry::new());
        let report_svc = Arc::new(ReportService::new(registry, Arc::new(db.clone())));
        report_svc.register_builtin_reports();

        Ok(Self { db, platform, config, report_svc })
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

    /// Get the report service.
    pub fn report_service(&self) -> &ReportService {
        &self.report_svc
    }

    /// Get a reference to the database connection.
    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    /// Validate a Bearer token and return the user model.
    pub async fn validate_token(&self, token: &str) -> Result<crate::entities::user::Model, AppError> {
        use chrono::{DateTime, Utc};
        use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
        let session = svc_iam::entities::user_session::Entity::find()
            .filter(svc_iam::entities::user_session::Column::Token.eq(token))
            .filter(svc_iam::entities::user_session::Column::IsCurrent.eq(1))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::unauthorized("Invalid or expired token"))?;

        let expires_at = DateTime::parse_from_rfc3339(&session.expires_at)
            .map_err(|_| AppError::unauthorized("Invalid session expiry"))?
            .with_timezone(&Utc);
        if expires_at <= Utc::now() {
            return Err(AppError::unauthorized("Session expired"));
        }

        let user = svc_iam::entities::user::Entity::find_by_id(&session.user_id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::unauthorized("User not found"))?;

        if user.is_deleted != 0 || user.status.to_lowercase() != "active" {
            return Err(AppError::unauthorized("User account is inactive"));
        }

        Ok(user)
    }
}
