use axum::serve;
use orbis_infra::adapters::iam::db_permission_checker::DbPermissionChecker;
use orbis_infra::event_bus::MemoryEventBus;
use orbis_infra::repositories::{
    org_repo::OrgRepoImpl, person_repo::PersonRepoImpl, session_repo::SessionRepoImpl,
    user_repo::UserRepoImpl,
};
use orbis_infra::security::{bcrypt::BcryptHasher, jwt::JwtTokenService};
use orbis_server::app_router;
use orbis_server::state::AppState;
use sea_orm::Database;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // The API server owns its database configuration. Never discover a Desktop
    // user's local database or log credentials embedded in a database URL.
    let db_url = orbis_infra::config::resolve_database_url()?;
    tracing::info!("Connecting to configured server database");

    // Connect to database
    let db = Database::connect(&db_url).await?;

    use orbis_infra::migration::Migrator;
    use sea_orm_migration::{MigratorTrait, SchemaManager};
    let schema_manager = SchemaManager::new(&db);
    if !schema_manager.has_table("seaql_migrations").await? {
        return Err(std::io::Error::other(
            "Orbis database schema is not initialized. Connect the Desktop to this database and apply migrations before starting the API server.",
        )
        .into());
    }
    let pending = Migrator::get_pending_migrations(&db).await?;
    if !pending.is_empty() {
        return Err(std::io::Error::other(format!(
            "The Orbis database has {} pending migration(s). Apply them from the Desktop before starting the API server.",
            pending.len()
        ))
        .into());
    }

    let license_data_dir = std::env::var_os("ORBIS_LICENSE_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or(std::env::current_dir()?.join(".orbis-license"));
    let app_version = env!("CARGO_PKG_VERSION").to_string();
    let mulverse_url = std::env::var("MULVERSE_API_URL").ok();
    let mulverse_client = Arc::new(match mulverse_url.as_deref() {
        Some(url) => orbis_infra::adapters::mulverse_client::MulverseClient::with_base_url(url)
            .map_err(std::io::Error::other)?,
        None => orbis_infra::adapters::mulverse_client::MulverseClient::new()
            .map_err(std::io::Error::other)?,
    });
    let license_runtime: Arc<dyn orbis_app::ports::license::LicenseRuntime> = Arc::new(
        orbis_infra::adapters::license_runtime::MulverseLicenseRuntime::new(
            &license_data_dir,
            &app_version,
            mulverse_url.as_deref(),
        )?,
    );
    license_runtime.initialize().await?;
    let _license_sync_handle =
        orbis_infra::adapters::license_runtime::spawn_background_sync(license_runtime.clone(), 30);

    let org_repo = Arc::new(OrgRepoImpl { db: db.clone() });
    let org_unit_repo =
        Arc::new(orbis_infra::repositories::org_unit_repo::OrgUnitRepoImpl { db: db.clone() });
    let user_repo = Arc::new(UserRepoImpl { db: db.clone() });
    let person_repo = Arc::new(PersonRepoImpl { db: db.clone() });
    let session_repo = Arc::new(SessionRepoImpl { db: db.clone() });
    let calendar_repo =
        Arc::new(orbis_infra::repositories::calendar_repos::CalendarRepoImpl { db: db.clone() });
    let academic_repo =
        Arc::new(orbis_infra::repositories::academic_repos::AcademicRepoImpl { db: db.clone() });
    let academic_framework_repo = Arc::new(
        orbis_infra::repositories::academic_framework_repos::AcademicFrameworkRepoImpl {
            db: db.clone(),
        },
    );
    let student_repo =
        Arc::new(orbis_infra::repositories::student_repos::StudentRepoImpl { db: db.clone() });
    let employee_repo =
        Arc::new(orbis_infra::repositories::employee_repo::EmployeeRepoImpl { db: db.clone() });
    let scheduling_repo = Arc::new(
        orbis_infra::repositories::scheduling_repos::SchedulingRepoImpl { db: db.clone() },
    );
    let assessment_repo = Arc::new(
        orbis_infra::repositories::assessment_repos::AssessmentRepoImpl { db: db.clone() },
    );
    let exam_repo =
        Arc::new(orbis_infra::repositories::exam_repos::ExamRepoImpl { db: db.clone() });
    let finance_repo =
        Arc::new(orbis_infra::repositories::finance_repos::FinanceRepoImpl { db: db.clone() });
    let document_repo =
        Arc::new(orbis_infra::repositories::document_repos::DocumentRepoImpl { db: db.clone() });
    let folder_repo = Arc::new(orbis_infra::repositories::folder_repo::FolderRepoImpl::new(
        db.clone(),
    ));
    let integration_repo = Arc::new(
        orbis_infra::repositories::integration_repos::IntegrationRepoImpl { db: db.clone() },
    );
    let notification_repo = Arc::new(
        orbis_infra::repositories::notification_repos::NotificationRepoImpl { db: db.clone() },
    );
    let notification_pref_repo = Arc::new(
        orbis_infra::repositories::notification_repos::NotificationPreferenceRepoImpl {
            db: db.clone(),
        },
    );
    let workflow_repo =
        Arc::new(orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() });
    let override_repo =
        Arc::new(orbis_infra::repositories::override_repos::OverrideRepoImpl { db: db.clone() });
    let progression_repo = Arc::new(
        orbis_infra::repositories::progression_repos::ProgressionRepoImpl { db: db.clone() },
    );
    let policy_repo =
        Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() });
    let autopilot_task_repo = Arc::new(
        orbis_infra::autopilot::task_repo::PgAutopilotTaskRepo::new(db.clone()),
    );

    let settings_repo =
        Arc::new(orbis_infra::repositories::settings_repo::SettingsRepoImpl { db: db.clone() });
    let dashboard_repo =
        Arc::new(orbis_infra::repositories::dashboard_repo::DashboardRepoImpl { db: db.clone() });

    let event_bus = Arc::new(MemoryEventBus::new());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "orbis_dev_super_secret_jwt_key_2026_educational_platform".to_string());
    let mfa_encryption_key = match std::env::var("MFA_ENCRYPTION_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ if std::env::var("APP_ENV")
            .map(|environment| environment != "production")
            .unwrap_or(true) =>
        {
            tracing::warn!(
                "MFA_ENCRYPTION_KEY is unset; deriving the MFA key from JWT_SECRET for development"
            );
            jwt_secret.clone()
        }
        _ => return Err("MFA_ENCRYPTION_KEY is required in production".into()),
    };
    let token_service = Arc::new(JwtTokenService::new(jwt_secret));
    let password_hasher = Arc::new(BcryptHasher);

    let integration_resolver = Arc::new(orbis_infra::adapters::resolver::IntegrationResolver::new(
        integration_repo.clone(),
    ));
    let notification_adapter = Arc::new(
        orbis_infra::adapters::notification::NotificationAdapter::new(
            integration_repo.clone(),
            settings_repo.clone(),
        ),
    );
    let permission_checker = Arc::new(DbPermissionChecker::new(db.clone()));
    let academic_framework_service = Arc::new(orbis_app::services::AcademicFrameworkService::new(
        academic_framework_repo.clone(),
        student_repo.clone(),
        org_unit_repo.clone(),
        permission_checker.clone(),
    ));

    let identity_repo =
        Arc::new(orbis_infra::repositories::identity_repo::UserIdentityRepoImpl { db: db.clone() });
    let mfa_factor_repo =
        Arc::new(orbis_infra::repositories::mfa_repo::MfaFactorRepoImpl { db: db.clone() });
    let recovery_code_repo =
        Arc::new(orbis_infra::repositories::mfa_repo::MfaRecoveryCodeRepoImpl { db: db.clone() });
    let device_repo =
        Arc::new(orbis_infra::repositories::device_repo::UserDeviceRepoImpl { db: db.clone() });
    let totp_service = Arc::new(orbis_infra::security::totp::TotpServiceImpl::new(
        &mfa_encryption_key,
    ));
    let oidc_service = Arc::new(orbis_infra::adapters::identity::GoogleIdentityAdapter::new());

    // ── Reporting System Wiring ─────────────────────────────────
    use orbis_app::ports::reporting::{DataProviderRegistry, RendererRegistry};
    use orbis_infra::adapters::report_adapter::*;
    use orbis_infra::repositories::report_repos::*;

    let report_definition_repo = Arc::new(ReportDefinitionRepoImpl { db: db.clone() });
    let report_template_repo = Arc::new(ReportTemplateRepoImpl { db: db.clone() });
    let report_job_repo = Arc::new(ReportJobRepoImpl { db: db.clone() });
    let report_audit_repo = Arc::new(ReportAuditRepoImpl { db: db.clone() });

    // Build data provider registry — register all available providers
    let mut provider_registry = InMemoryDataProviderRegistry::new();
    provider_registry.register_provider(Box::new(FeeCollectionDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(StudentListDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(TermResultDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(AnnualResultDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(ExamSeatingPlanDataProvider { db: db.clone() }));
    provider_registry
        .register_provider(Box::new(ExamAttendanceSheetDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(PaymentReceiptDataProvider { db: db.clone() }));
    provider_registry.register_provider(Box::new(StudentStatementDataProvider { db: db.clone() }));
    provider_registry
        .register_provider(Box::new(AcademicCertificateDataProvider { db: db.clone() }));
    let provider_registry = Arc::new(provider_registry);

    // Build renderer registry — register all available renderers
    let mut renderer_registry = InMemoryRendererRegistry::new();
    renderer_registry.register_renderer(Box::new(DataOnlyRenderer));
    renderer_registry.register_renderer(Box::new(JsonRenderer));
    renderer_registry.register_renderer(Box::new(CsvRenderer));
    renderer_registry.register_renderer(Box::new(XlsxRenderer));
    renderer_registry.register_renderer(Box::new(HtmlRenderer));
    renderer_registry.register_renderer(Box::new(PdfRenderer));
    let renderer_registry = Arc::new(renderer_registry);

    let org_context_resolver = Arc::new(DbOrgContextResolver { db: db.clone() });

    let sequence_repo = Arc::new(
        orbis_infra::repositories::sequence_repo::SequenceCounterRepoImpl::new(db.clone()),
    );
    let doc_req_repo = Arc::new(
        orbis_infra::repositories::document_repos::DocumentRequirementRepoImpl::new(db.clone()),
    );

    // Build Domain Services
    let student_service = Arc::new(
        orbis_app::services::StudentService::new(
            student_repo.clone(),
            person_repo.clone(),
            student_repo.clone(),
            org_repo.clone(),
            calendar_repo.clone(),
            calendar_repo.clone(),
            student_repo.clone(),
            settings_repo.clone(),
            sequence_repo.clone(),
            academic_repo.clone(),
            finance_repo.clone(),
            finance_repo.clone(),
            override_repo.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        )
        .with_license_runtime(license_runtime.clone()),
    );

    let progression_service = Arc::new(orbis_app::services::ProgressionService::new(
        progression_repo.clone(),
        student_repo.clone(),
        student_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let academic_service = Arc::new(orbis_app::services::AcademicService::new(
        academic_repo.clone(),
        academic_repo.clone(),
        academic_repo.clone(),
        academic_repo.clone(),
        assessment_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let finance_service = Arc::new(orbis_app::services::FinanceService::new(
        finance_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        org_repo.clone(),
        finance_repo.clone(),
        policy_repo.clone(),
        event_bus.clone(),
        integration_resolver.clone(),
        notification_adapter.clone(),
        permission_checker.clone(),
    ));

    let assessment_service = Arc::new(
        orbis_app::services::AssessmentService::new(
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            academic_repo.clone(),
            academic_repo.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        )
        .with_academic_repos(
            Some(calendar_repo.clone()),
            Some(academic_repo.clone()),
            Some(student_repo.clone()),
            Some(calendar_repo.clone()),
            Some(student_repo.clone()),
            Some(settings_repo.clone()),
        )
        .with_attendance_scope_repos(employee_repo.clone(), scheduling_repo.clone()),
    );

    let auth_service = Arc::new(
        orbis_app::services::AuthService::new(
            user_repo.clone(),
            person_repo.clone(),
            session_repo.clone(),
            mfa_factor_repo.clone(),
            recovery_code_repo.clone(),
            device_repo.clone(),
            identity_repo.clone(),
            token_service.clone(),
            password_hasher.clone(),
            totp_service.clone(),
            oidc_service.clone(),
            Some(integration_repo.clone()),
            event_bus.clone(),
            permission_checker.clone(),
        )
        .with_context_repos(
            Some(calendar_repo.clone()),
            Some(student_repo.clone()),
            Some(student_repo.clone()),
            Some(student_repo.clone()),
        )
        .with_teaching_scope_repos(
            employee_repo.clone(),
            academic_repo.clone(),
            academic_repo.clone(),
        ),
    );

    let organization_service = Arc::new(orbis_app::services::OrganizationService::new(
        org_repo.clone(),
        org_unit_repo.clone(),
        settings_repo.clone(),
        dashboard_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let calendar_service = Arc::new(orbis_app::services::CalendarService::new(
        calendar_repo.clone(),
        calendar_repo.clone(),
        calendar_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let scheduling_service = Arc::new(orbis_app::services::SchedulingService::new(
        scheduling_repo.clone(),
        org_unit_repo.clone(),
        scheduling_repo.clone(),
        scheduling_repo.clone(),
        exam_repo.clone(),
        exam_repo.clone(),
        exam_repo.clone(),
        exam_repo.clone(),
        exam_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let document_service = Arc::new(orbis_app::services::DocumentService::new(
        document_repo.clone(),
        doc_req_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let folder_service = Arc::new(orbis_app::services::FolderService::new(
        folder_repo.clone(),
        document_repo.clone(),
        permission_checker.clone(),
    ));

    let storage_root = std::env::var("STORAGE_LOCAL_ROOT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("./storage"));
    let storage_resolver = Arc::new(orbis_infra::adapters::storage::StorageResolver::new(
        storage_root,
        integration_repo.clone(),
    ));
    let file_storage_service = Arc::new(orbis_app::services::FileStorageService::new(
        storage_resolver.clone(),
        document_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let integration_service = Arc::new(orbis_app::services::IntegrationService::new(
        integration_repo.clone(),
        permission_checker.clone(),
    ));

    let (notif_tx, _notif_rx) = tokio::sync::broadcast::channel(256);
    let notification_service = Arc::new(
        orbis_app::services::NotificationService::new(
            notification_repo.clone(),
            notification_pref_repo.clone(),
            permission_checker.clone(),
        )
        .with_broadcaster(notif_tx),
    );
    event_bus.set_notification_service(notification_service.clone());

    let workflow_service = Arc::new(orbis_app::services::WorkflowService::new(
        override_repo.clone(),
        workflow_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let report_service = Arc::new(orbis_app::services::ReportService::new(
        report_definition_repo.clone(),
        report_template_repo.clone(),
        report_job_repo.clone(),
        report_audit_repo.clone(),
        provider_registry.clone(),
        renderer_registry.clone(),
        org_context_resolver.clone(),
    ));

    let autopilot_engine = Arc::new(orbis_app::autopilot::engine::AutopilotEngine::new(
        policy_repo.clone(),
        autopilot_task_repo.clone(),
        calendar_service.clone(),
        progression_service.clone(),
        finance_service.clone(),
        report_service.clone(),
        workflow_service.clone(),
    ));
    event_bus.set_workflow_engine(autopilot_engine.clone());

    let autopilot_evaluator = Arc::new(orbis_app::autopilot::evaluator::AutopilotEvaluator::new(
        policy_repo.clone(),
        calendar_repo.clone(),
        calendar_repo.clone(),
        student_repo.clone(),
        finance_repo.clone(),
    ));

    let autopilot_service = Arc::new(
        orbis_app::services::AutopilotService::new(
            autopilot_evaluator.clone(),
            autopilot_engine.clone(),
            autopilot_task_repo.clone(),
            policy_repo.clone(),
            permission_checker.clone(),
        )
        .with_notification_port(notification_adapter.clone()),
    );

    let ai_service = Arc::new(
        orbis_app::services::AiOrchestrationService::new(
            integration_resolver.clone(),
            permission_checker.clone(),
            student_service.clone(),
            employee_repo.clone(),
            academic_service.clone(),
            assessment_service.clone(),
            finance_service.clone(),
            scheduling_service.clone(),
            report_service.clone(),
            organization_service.clone(),
        )
        .with_autopilot_service(autopilot_service.clone()),
    );

    // ── Employee Affairs & User Groups Wiring ───────────────────
    let group_repo =
        Arc::new(orbis_infra::repositories::group_repo::GroupRepoImpl { db: db.clone() });

    let identity_provisioning_service =
        Arc::new(orbis_app::services::IdentityProvisioningService::new(
            user_repo.clone(),
            person_repo.clone(),
            identity_repo.clone(),
            group_repo.clone(),
            Some(integration_repo.clone()),
            Some(session_repo.clone()),
            Some(org_repo.clone()),
            Some(notification_adapter.clone()),
            password_hasher.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

    let employee_service = Arc::new(
        orbis_app::services::EmployeeService::new(
            employee_repo.clone(),
            person_repo.clone(),
            group_repo.clone(),
            identity_provisioning_service.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        )
        .with_license_runtime(license_runtime.clone()),
    );

    let group_service = Arc::new(orbis_app::services::GroupService::new(
        group_repo.clone(),
        event_bus.clone(),
        permission_checker.clone(),
    ));

    let bootstrap_service = Arc::new(orbis_app::services::BootstrapService::new(
        org_repo.clone(),
        user_repo.clone(),
        person_repo.clone(),
        identity_repo.clone(),
        group_repo.clone(),
        Arc::new(orbis_infra::repositories::role_repo::RoleRepoImpl::new(
            db.clone(),
        )),
        settings_repo.clone(),
        Arc::new(orbis_infra::repositories::system_repo::SystemRepoImpl { db: db.clone() }),
        finance_repo.clone(),
        finance_repo.clone(),
        password_hasher.clone(),
        event_bus.clone(),
    ));

    let onboarding_service = Arc::new(orbis_app::services::OnboardingService::new(
        org_repo.clone(),
        org_unit_repo.clone(),
        calendar_repo.clone(),
        calendar_repo.clone(),
        academic_repo.clone(),
        academic_repo.clone(),
        academic_repo.clone(),
        assessment_repo.clone(),
        settings_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        finance_repo.clone(),
        scheduling_repo.clone(),
        scheduling_repo.clone(),
        doc_req_repo.clone(),
        report_definition_repo.clone(),
        report_template_repo.clone(),
        integration_repo.clone(),
        permission_checker.clone(),
    ));

    // Build AppState
    let state = AppState {
        mulverse_client,
        db: db.clone(),
        org_repo: org_repo.clone(),
        org_unit_repo,
        user_repo,
        person_repo,
        session_repo,
        year_repo: calendar_repo.clone(),
        period_repo: calendar_repo.clone(),
        period_type_repo: calendar_repo,
        academic_framework_repo,
        subject_repo: academic_repo.clone(),
        section_repo: academic_repo.clone(),
        section_subject_repo: academic_repo.clone(),
        enrollment_repo: student_repo.clone(),
        student_repo: student_repo.clone(),
        student_guardian_repo: student_repo,
        override_repo,
        prerequisite_repo: academic_repo,
        gpa_repo: assessment_repo.clone(),
        promotion_repo: progression_repo,
        discount_template_repo: finance_repo.clone(),
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
        document_repo: document_repo.clone(),
        folder_repo,
        doc_requirement_repo: doc_req_repo,
        sequence_repo,
        integration_repo,
        workflow_repo,
        currency_repo: finance_repo.clone(),
        policy_repo,
        settings_repo,
        dashboard_repo,
        event_bus,
        token_service: token_service.clone(),
        password_hasher: password_hasher.clone(),
        integration_resolver,
        permission_checker,
        // ── Identity, MFA & Devices ─────────────────────────────
        identity_repo,
        mfa_factor_repo,
        recovery_code_repo,
        device_repo,
        totp_service,
        oidc_service,
        // ── Domain Services ─────────────────────────────────────
        student_service,
        progression_service,
        academic_service,
        academic_framework_service,
        finance_service,
        assessment_service,
        auth_service,
        organization_service,
        calendar_service,
        scheduling_service,
        document_service,
        folder_service,
        file_storage_service,
        storage_resolver,
        integration_service,
        notification_adapter,
        notification_service: notification_service.clone(),
        workflow_service,
        report_service: report_service.clone(),
        ai_service,
        autopilot_service: autopilot_service.clone(),
        autopilot_task_repo: autopilot_task_repo.clone(),
        // ── Reporting System ────────────────────────────────────
        report_definition_repo,
        report_template_repo,
        report_job_repo,
        report_audit_repo,
        report_provider_registry: provider_registry,
        report_renderer_registry: renderer_registry,
        report_org_context_resolver: org_context_resolver,
        // ── System Services ────────────────────────────────────
        license_runtime,
        system_service: Arc::new(orbis_app::services::SystemService::new(Arc::new(
            orbis_infra::repositories::system_repo::SystemRepoImpl { db: db.clone() },
        ))),
        // ── Employee Affairs & User Groups ─────────────────────────
        employee_repo,
        group_repo,
        identity_provisioning_service,
        employee_service,
        group_service,
        bootstrap_service,
        onboarding_service,
    };

    // Spawn Autopilot Background Scheduler (checks every 60 seconds)
    let autopilot_scheduler = Arc::new(orbis_infra::autopilot::scheduler::AutopilotScheduler::new(
        autopilot_service.clone(),
        org_repo.clone(),
    ));
    let _scheduler_handle = autopilot_scheduler.spawn(60);

    let report_cleanup_service = report_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60 * 60));
        loop {
            interval.tick().await;
            match report_cleanup_service.cleanup_expired_reports().await {
                Ok(removed) if removed > 0 => {
                    tracing::info!(removed, "Expired report files cleaned up");
                }
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(error = %error, "Expired report cleanup failed");
                }
            }
        }
    });

    // Spawn Notification cleanup background task (every 6 hours)
    let notification_cleanup = notification_service.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(6 * 60 * 60));
        loop {
            interval.tick().await;
            match notification_cleanup.cleanup_expired().await {
                Ok(removed) if removed > 0 => {
                    tracing::info!(removed, "Expired notifications cleaned up");
                }
                Ok(_) => {}
                Err(error) => {
                    tracing::error!(error = %error, "Expired notification cleanup failed");
                }
            }
        }
    });
    // Build Axum Router
    let app = app_router(state);

    // Start Server
    let bind_host = std::env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", bind_host, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("Server running on http://{}", addr);

    serve(listener, app).await?;

    Ok(())
}
