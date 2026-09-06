use axum::serve;
use orbis_server::app_router;
use orbis_server::state::AppState;
use orbis_infra::repositories::{org_repo::OrgRepoImpl, user_repo::UserRepoImpl, person_repo::PersonRepoImpl, session_repo::SessionRepoImpl};
use orbis_infra::event_bus::MemoryEventBus;
use orbis_infra::security::{jwt::JwtTokenService, bcrypt::BcryptHasher};
use orbis_infra::adapters::iam::db_permission_checker::DbPermissionChecker;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Use an in-memory SQLite DB for now, or real DB if provided
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());
    
    // Connect to database
    let db = Database::connect(&db_url).await?;

    // Run Migrations
    use orbis_infra::migration::Migrator;
    use sea_orm_migration::MigratorTrait;
    Migrator::up(&db, None).await?;

    let org_repo = Arc::new(OrgRepoImpl { db: db.clone() });
    let org_unit_repo = Arc::new(orbis_infra::repositories::org_unit_repo::OrgUnitRepoImpl { db: db.clone() });
    let user_repo = Arc::new(UserRepoImpl { db: db.clone() });
    let person_repo = Arc::new(PersonRepoImpl { db: db.clone() });
    let session_repo = Arc::new(SessionRepoImpl { db: db.clone() });
    let calendar_repo = Arc::new(orbis_infra::repositories::calendar_repos::CalendarRepoImpl { db: db.clone() });
    let academic_repo = Arc::new(orbis_infra::repositories::academic_repos::AcademicRepoImpl { db: db.clone() });
    let student_repo = Arc::new(orbis_infra::repositories::student_repos::StudentRepoImpl { db: db.clone() });
    let scheduling_repo = Arc::new(orbis_infra::repositories::scheduling_repos::SchedulingRepoImpl { db: db.clone() });
    let assessment_repo = Arc::new(orbis_infra::repositories::assessment_repos::AssessmentRepoImpl { db: db.clone() });
    let exam_repo = Arc::new(orbis_infra::repositories::exam_repos::ExamRepoImpl { db: db.clone() });
    let finance_repo = Arc::new(orbis_infra::repositories::finance_repos::FinanceRepoImpl { db: db.clone() });
    let document_repo = Arc::new(orbis_infra::repositories::document_repos::DocumentRepoImpl { db: db.clone() });
    let integration_repo = Arc::new(orbis_infra::repositories::integration_repos::IntegrationRepoImpl { db: db.clone() });
    let ai_repo = Arc::new(orbis_infra::repositories::ai_repos::AiRepoImpl { db: db.clone() });
    let workflow_repo = Arc::new(orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() });
    
    let autopilot_engine = Arc::new(orbis_app::autopilot::engine::AutopilotEngine {
        policy_repo: Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() }),
    });
    let policy_repo = Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() });

    let settings_repo = Arc::new(orbis_infra::repositories::settings_repo::SettingsRepoImpl { db: db.clone() });
    let dashboard_repo = Arc::new(orbis_infra::repositories::dashboard_repo::DashboardRepoImpl { db: db.clone() });

    let event_bus = Arc::new(MemoryEventBus {
        workflow_engine: Some(autopilot_engine),
    });
    let token_service = Arc::new(JwtTokenService::new("super-secret-key-for-dev".to_string()));
    let password_hasher = Arc::new(BcryptHasher);

    let integration_resolver = Arc::new(orbis_infra::adapters::resolver::IntegrationResolver::new(integration_repo.clone()));
    let permission_checker = Arc::new(DbPermissionChecker::new(db.clone()));

    let identity_repo = Arc::new(orbis_infra::repositories::identity_repo::UserIdentityRepoImpl { db: db.clone() });
    let mfa_factor_repo = Arc::new(orbis_infra::repositories::mfa_repo::MfaFactorRepoImpl { db: db.clone() });
    let recovery_code_repo = Arc::new(orbis_infra::repositories::mfa_repo::MfaRecoveryCodeRepoImpl { db: db.clone() });
    let device_repo = Arc::new(orbis_infra::repositories::device_repo::UserDeviceRepoImpl { db: db.clone() });
    let totp_service = Arc::new(orbis_infra::security::totp::TotpServiceImpl);
    let oidc_service = Arc::new(orbis_infra::security::google_oidc::GoogleOidcService::new());

    // ── Reporting System Wiring ─────────────────────────────────
    use orbis_infra::repositories::report_repos::*;
    use orbis_infra::adapters::report_adapter::*;
    use orbis_app::ports::reporting::{DataProviderRegistry, RendererRegistry};

    let report_definition_repo = Arc::new(ReportDefinitionRepoImpl { db: db.clone() });
    let report_template_repo = Arc::new(ReportTemplateRepoImpl { db: db.clone() });
    let report_job_repo = Arc::new(ReportJobRepoImpl { db: db.clone() });
    let report_audit_repo = Arc::new(ReportAuditRepoImpl { db: db.clone() });

    // Build data provider registry — register all available providers
    let mut provider_registry = InMemoryDataProviderRegistry::new();
    provider_registry.register_provider(Box::new(FeeCollectionDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(StudentListDataProvider { db: db.clone() }));
    let provider_registry = Arc::new(provider_registry);

    // Build renderer registry — register all available renderers
    let mut renderer_registry = InMemoryRendererRegistry::new();
    renderer_registry.register_renderer(Box::new(DataOnlyRenderer));
    renderer_registry.register_renderer(Box::new(JsonRenderer));
    renderer_registry.register_renderer(Box::new(CsvRenderer));
    renderer_registry.register_renderer(Box::new(HtmlRenderer));
    renderer_registry.register_renderer(Box::new(PdfRenderer));
    let renderer_registry = Arc::new(renderer_registry);

    let org_context_resolver = Arc::new(DbOrgContextResolver { db: db.clone() });

    // Build AppState
    let state = AppState {
        db,
        org_repo,
        org_unit_repo,
        user_repo,
        person_repo,
        session_repo,
        year_repo: calendar_repo.clone(),
        period_repo: calendar_repo.clone(),
        period_template_repo: calendar_repo,
        subject_repo: academic_repo.clone(),
        class_repo: academic_repo.clone(),
        class_subject_repo: academic_repo,
        enrollment_repo: student_repo.clone(),
        student_guardian_repo: student_repo,
        room_repo: scheduling_repo.clone(),
        time_slot_repo: scheduling_repo.clone(),
        lesson_repo: scheduling_repo,
        assessment_scheme_repo: assessment_repo.clone(),
        grade_repo: assessment_repo.clone(),
        attendance_repo: assessment_repo.clone(),
        behavior_record_repo: assessment_repo,
        exam_session_repo: exam_repo.clone(),
        exam_schedule_repo: exam_repo.clone(),
        seating_number_repo: exam_repo.clone(),
        secret_number_repo: exam_repo.clone(),
        exam_proctor_repo: exam_repo,
        account_repo: finance_repo.clone(),
        journal_entry_repo: finance_repo.clone(),
        fee_template_repo: finance_repo.clone(),
        student_fee_repo: finance_repo.clone(),
        payment_repo: finance_repo.clone(),
        document_repo,
        integration_repo,
        ai_repo,
        workflow_repo,
        currency_repo: finance_repo.clone(),
        policy_repo,
        settings_repo,
        dashboard_repo,
        event_bus,
        token_service,
        password_hasher,
        integration_resolver,
        permission_checker,
        // ── Identity, MFA & Devices ─────────────────────────────
        identity_repo,
        mfa_factor_repo,
        recovery_code_repo,
        device_repo,
        totp_service,
        oidc_service,
        // ── Reporting System ────────────────────────────────────
        report_definition_repo,
        report_template_repo,
        report_job_repo,
        report_audit_repo,
        report_provider_registry: provider_registry,
        report_renderer_registry: renderer_registry,
        report_org_context_resolver: org_context_resolver,
    };

    // Build Axum Router
    let app = app_router(state);

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    println!("Server running on http://0.0.0.0:3000");
    
    serve(listener, app).await?;

    Ok(())
}
