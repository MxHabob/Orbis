use std::sync::Arc;
use sea_orm::DatabaseConnection;
use orbis_app::ports::repositories::{
    OrganizationRepository, OrgUnitRepository, UserRepository, PersonRepository,
    SessionRepository, AcademicYearRepository, AcademicPeriodRepository,
    PeriodTemplateRepository, SubjectRepository, ClassRepository, ClassSubjectRepository,
    EnrollmentRepository, StudentGuardianRepository, RoomRepository, TimeSlotRepository,
    ScheduledLessonRepository, AssessmentSchemeRepository, GradeRepository,
    AttendanceRepository, BehaviorRecordRepository, ExamSessionRepository,
    ExamScheduleRepository, SeatingNumberRepository, SecretNumberRepository,
    ExamProctorRepository, AccountRepository, JournalEntryRepository, FeeTemplateRepository,
    StudentFeeRepository, PaymentRepository, DocumentRepository, IntegrationRepository,
    WorkflowRepository, CurrencyRepository, PolicyRepository,
    SettingsRepository, DashboardRepository, OverrideRequestRepository, CoursePrerequisiteRepository,
    GpaRecordRepository, PromotionRepository, DiscountTemplateRepository,
    SystemRepository,
};
use orbis_app::ports::reporting::{
    ReportDefinitionRepository, ReportTemplateRepository, ReportJobRepository,
    ReportAuditRepository, DataProviderRegistry, RendererRegistry,
};
use orbis_app::ports::entitlement::EntitlementProvider;
use orbis_app::services::OrgContextResolver;
use orbis_app::ports::event_bus::EventBus;
use orbis_app::ports::security::{TokenService, PasswordHasher};
use orbis_app::ports::iam::PermissionCheckerPort;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActiveSession {
    pub user_id: Uuid,
    pub org_id: Uuid,
    pub active_unit_id: Option<Uuid>,
    pub token: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    /// The active session for the current desktop application.
    /// Set during login/session hydration and used by `make_ctx()` to scope all queries.
    pub active_session: Arc<std::sync::RwLock<Option<ActiveSession>>>,
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub period_template_repo: Arc<dyn PeriodTemplateRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub class_repo: Arc<dyn ClassRepository>,
    pub class_subject_repo: Arc<dyn ClassSubjectRepository>,
    pub student_repo: Arc<dyn orbis_app::ports::repositories::StudentRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_guardian_repo: Arc<dyn StudentGuardianRepository>,
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub prerequisite_repo: Arc<dyn CoursePrerequisiteRepository>,
    pub gpa_repo: Arc<dyn GpaRecordRepository>,
    pub promotion_repo: Arc<dyn PromotionRepository>,
    pub discount_template_repo: Arc<dyn DiscountTemplateRepository>,
    pub room_repo: Arc<dyn RoomRepository>,
    pub time_slot_repo: Arc<dyn TimeSlotRepository>,
    pub lesson_repo: Arc<dyn ScheduledLessonRepository>,
    pub assessment_scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub grade_repo: Arc<dyn GradeRepository>,
    pub attendance_repo: Arc<dyn AttendanceRepository>,
    pub behavior_record_repo: Arc<dyn BehaviorRecordRepository>,
    pub exam_session_repo: Arc<dyn ExamSessionRepository>,
    pub exam_schedule_repo: Arc<dyn ExamScheduleRepository>,
    pub seating_number_repo: Arc<dyn SeatingNumberRepository>,
    pub secret_number_repo: Arc<dyn SecretNumberRepository>,
    pub exam_proctor_repo: Arc<dyn ExamProctorRepository>,
    pub account_repo: Arc<dyn AccountRepository>,
    pub journal_entry_repo: Arc<dyn JournalEntryRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub payment_repo: Arc<dyn PaymentRepository>,
    pub document_repo: Arc<dyn DocumentRepository>,
    pub doc_req_repo: Arc<dyn orbis_app::ports::repositories::DocumentRequirementRepository>,
    pub sequence_repo: Arc<dyn orbis_app::ports::repositories::SequenceCounterRepository>,
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub workflow_repo: Arc<dyn WorkflowRepository>,
    pub report_definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub report_template_repo: Arc<dyn ReportTemplateRepository>,
    pub report_job_repo: Arc<dyn ReportJobRepository>,
    pub report_audit_repo: Arc<dyn ReportAuditRepository>,
    pub report_provider_registry: Arc<dyn DataProviderRegistry>,
    pub report_renderer_registry: Arc<dyn RendererRegistry>,
    pub report_org_context_resolver: Arc<dyn OrgContextResolver>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub dashboard_repo: Arc<dyn DashboardRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub integration_resolver: Arc<orbis_infra::adapters::resolver::IntegrationResolver>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    // ── Identity, MFA & Devices ─────────────────────────────────
    pub identity_repo: Arc<dyn orbis_app::ports::repositories::UserIdentityRepository>,
    pub mfa_factor_repo: Arc<dyn orbis_app::ports::repositories::MfaFactorRepository>,
    pub recovery_code_repo: Arc<dyn orbis_app::ports::repositories::MfaRecoveryCodeRepository>,
    pub device_repo: Arc<dyn orbis_app::ports::repositories::UserDeviceRepository>,
    pub totp_service: Arc<dyn orbis_app::ports::security::TotpService>,
    pub oidc_service: Arc<dyn orbis_app::ports::security::OidcService>,
    // ── Domain Services ─────────────────────────────────────────
    pub student_service: Arc<orbis_app::services::StudentService>,
    pub progression_service: Arc<orbis_app::services::ProgressionService>,
    pub academic_service: Arc<orbis_app::services::AcademicService>,
    pub finance_service: Arc<orbis_app::services::FinanceService>,
    pub assessment_service: Arc<orbis_app::services::AssessmentService>,
    pub auth_service: Arc<orbis_app::services::AuthService>,
    pub organization_service: Arc<orbis_app::services::OrganizationService>,
    pub calendar_service: Arc<orbis_app::services::CalendarService>,
    pub scheduling_service: Arc<orbis_app::services::SchedulingService>,
    pub document_service: Arc<orbis_app::services::DocumentService>,
    pub folder_repo: Arc<dyn orbis_app::ports::repositories::FolderRepository>,
    pub folder_service: Arc<orbis_app::services::FolderService>,
    pub file_storage_service: Arc<orbis_app::services::FileStorageService>,
    pub storage_resolver: Arc<orbis_infra::adapters::storage::StorageResolver>,
    pub storage_adapter: Arc<dyn orbis_app::ports::storage::StorageAdapter>,
    pub integration_service: Arc<orbis_app::services::IntegrationService>,
    pub workflow_service: Arc<orbis_app::services::WorkflowService>,
    pub report_service: Arc<orbis_app::services::ReportService>,
    pub ai_service: Arc<orbis_app::services::AiOrchestrationService>,
    pub autopilot_service: Arc<orbis_app::services::AutopilotService>,
    pub autopilot_task_repo: Arc<dyn orbis_app::ports::repositories::AutopilotTaskRepository>,
    // ── System Services ─────────────────────────────────────────
    pub system_repo: Arc<dyn SystemRepository>,
    pub system_service: Arc<orbis_app::services::SystemService>,
    pub entitlement_provider: Arc<dyn EntitlementProvider>,
    pub solver_sidecar: Arc<crate::features::solver_sidecar::SolverSidecarManager>,
    pub app_data_dir: std::path::PathBuf,
    // ── Employee Affairs & User Groups ─────────────────────────
    pub employee_repo: Arc<dyn orbis_app::ports::repositories::EmployeeRepository>,
    pub group_repo: Arc<dyn orbis_app::ports::repositories::GroupRepository>,
    pub identity_provisioning_service: Arc<orbis_app::services::IdentityProvisioningService>,
    pub employee_service: Arc<orbis_app::services::EmployeeService>,
    pub group_service: Arc<orbis_app::services::GroupService>,
    pub bootstrap_service: Arc<orbis_app::services::BootstrapService>,
}


impl AppState {
    pub async fn init(app_data_dir: &std::path::Path) -> Result<Self, String> {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            let db_path = app_data_dir.join("orbis.db");
            format!("sqlite://{}?mode=rwc", db_path.display())
        });
        let db = sea_orm::Database::connect(&db_url).await.map_err(|e| e.to_string())?;

        // Run migrations
        use orbis_infra::migration::Migrator;
        use sea_orm_migration::MigratorTrait;
        Migrator::up(&db, None).await.map_err(|e| e.to_string())?;

        let active_session = Arc::new(std::sync::RwLock::new(None));

        let org_repo = Arc::new(orbis_infra::repositories::org_repo::OrgRepoImpl { db: db.clone() });
        let org_unit_repo = Arc::new(orbis_infra::repositories::org_unit_repo::OrgUnitRepoImpl { db: db.clone() });
        let user_repo = Arc::new(orbis_infra::repositories::user_repo::UserRepoImpl { db: db.clone() });
        let person_repo = Arc::new(orbis_infra::repositories::person_repo::PersonRepoImpl { db: db.clone() });
        let session_repo = Arc::new(orbis_infra::repositories::session_repo::SessionRepoImpl { db: db.clone() });
        let calendar_repo = Arc::new(orbis_infra::repositories::calendar_repos::CalendarRepoImpl { db: db.clone() });
        let academic_repo = Arc::new(orbis_infra::repositories::academic_repos::AcademicRepoImpl { db: db.clone() });
        let student_repo = Arc::new(orbis_infra::repositories::student_repos::StudentRepoImpl { db: db.clone() });
        let scheduling_repo = Arc::new(orbis_infra::repositories::scheduling_repos::SchedulingRepoImpl { db: db.clone() });
        let assessment_repo = Arc::new(orbis_infra::repositories::assessment_repos::AssessmentRepoImpl { db: db.clone() });
        let exam_repo = Arc::new(orbis_infra::repositories::exam_repos::ExamRepoImpl { db: db.clone() });
        let finance_repo = Arc::new(orbis_infra::repositories::finance_repos::FinanceRepoImpl { db: db.clone() });
        let document_repo = Arc::new(orbis_infra::repositories::document_repos::DocumentRepoImpl { db: db.clone() });
        let integration_repo = Arc::new(orbis_infra::repositories::integration_repos::IntegrationRepoImpl { db: db.clone() });
        let workflow_repo = Arc::new(orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() });
        let override_repo = Arc::new(orbis_infra::repositories::override_repos::OverrideRepoImpl { db: db.clone() });
        let progression_repo = Arc::new(orbis_infra::repositories::progression_repos::ProgressionRepoImpl { db: db.clone() });
        let report_definition_repo = Arc::new(orbis_infra::repositories::report_repos::ReportDefinitionRepoImpl { db: db.clone() });
        let report_template_repo = Arc::new(orbis_infra::repositories::report_repos::ReportTemplateRepoImpl { db: db.clone() });
        let report_job_repo = Arc::new(orbis_infra::repositories::report_repos::ReportJobRepoImpl { db: db.clone() });
        let report_audit_repo = Arc::new(orbis_infra::repositories::report_repos::ReportAuditRepoImpl { db: db.clone() });
        let policy_repo = Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() });
        let settings_repo = Arc::new(orbis_infra::repositories::settings_repo::SettingsRepoImpl { db: db.clone() });
        let dashboard_repo = Arc::new(orbis_infra::repositories::dashboard_repo::DashboardRepoImpl { db: db.clone() });

        use orbis_infra::adapters::report_adapter::*;
        let mut provider_registry = InMemoryDataProviderRegistry::new();
        provider_registry.register_provider(Box::new(FeeCollectionDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(StudentListDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(TermResultDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(AnnualResultDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(ExamSeatingPlanDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(ExamAttendanceSheetDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(PaymentReceiptDataProvider { db: db.clone() }));
        provider_registry.register_provider(Box::new(StudentStatementDataProvider { db: db.clone() }));
        let report_provider_registry = Arc::new(provider_registry);

        let mut renderer_registry = InMemoryRendererRegistry::new();
        renderer_registry.register_renderer(Box::new(DataOnlyRenderer));
        renderer_registry.register_renderer(Box::new(JsonRenderer));
        renderer_registry.register_renderer(Box::new(CsvRenderer));
        renderer_registry.register_renderer(Box::new(HtmlRenderer));
        renderer_registry.register_renderer(Box::new(PdfRenderer));
        let report_renderer_registry = Arc::new(renderer_registry);

        let report_org_context_resolver = Arc::new(DbOrgContextResolver { db: db.clone() });

        let permission_checker = Arc::new(orbis_infra::adapters::iam::db_permission_checker::DbPermissionChecker {
            db: db.clone(),
        });

        let autopilot_task_repo = Arc::new(orbis_infra::autopilot::task_repo::PgAutopilotTaskRepo::new(db.clone()));

        let event_bus = Arc::new(orbis_infra::event_bus::MemoryEventBus {
            workflow_engine: None,
        });
        // Generate or load a per-installation JWT secret.
        // Stored as a file in AppData — never hardcoded.
        let jwt_secret = Self::load_or_generate_jwt_secret(app_data_dir)?;
        let token_service = Arc::new(orbis_infra::security::jwt::JwtTokenService::new(jwt_secret));
        let password_hasher = Arc::new(orbis_infra::security::bcrypt::BcryptHasher);

        let integration_resolver = Arc::new(orbis_infra::adapters::resolver::IntegrationResolver::new(integration_repo.clone()));

        let identity_repo = Arc::new(orbis_infra::repositories::identity_repo::UserIdentityRepoImpl { db: db.clone() });
        let mfa_factor_repo = Arc::new(orbis_infra::repositories::mfa_repo::MfaFactorRepoImpl { db: db.clone() });
        let recovery_code_repo = Arc::new(orbis_infra::repositories::mfa_repo::MfaRecoveryCodeRepoImpl { db: db.clone() });
        let device_repo = Arc::new(orbis_infra::repositories::device_repo::UserDeviceRepoImpl { db: db.clone() });
        let totp_service = Arc::new(orbis_infra::security::totp::TotpServiceImpl);
        let oidc_service = Arc::new(orbis_infra::adapters::identity::GoogleIdentityAdapter::new());

        let sequence_repo = Arc::new(orbis_infra::repositories::sequence_repo::SequenceCounterRepoImpl::new(db.clone()));
        let doc_req_repo = Arc::new(orbis_infra::repositories::document_repos::DocumentRequirementRepoImpl::new(db.clone()));

        // Build Domain Services
        let student_service = Arc::new(orbis_app::services::StudentService::new(
            student_repo.clone(),
            person_repo.clone(),
            student_repo.clone(),
            student_repo.clone(),
            settings_repo.clone(),
            sequence_repo.clone(),
            academic_repo.clone(),
            finance_repo.clone(),
            finance_repo.clone(),
            override_repo.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

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
            finance_repo.clone(),
            policy_repo.clone(),
            event_bus.clone(),
            integration_resolver.clone(),
            integration_resolver.clone(),
            permission_checker.clone(),
        ));

        let assessment_service = Arc::new(orbis_app::services::AssessmentService::new(
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            assessment_repo.clone(),
            academic_repo.clone(),
            academic_repo.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

        let auth_service = Arc::new(orbis_app::services::AuthService::new(
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
        ).with_context_repos(
            Some(calendar_repo.clone()),
            Some(student_repo.clone()),
            Some(student_repo.clone()),
            Some(student_repo.clone()),
        ));

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

        let folder_repo = Arc::new(orbis_infra::repositories::folder_repo::FolderRepoImpl { db: db.clone() });
        let folder_service = Arc::new(orbis_app::services::FolderService::new(
            folder_repo.clone(),
            document_repo.clone(),
            permission_checker.clone(),
        ));

        let storage_root = app_data_dir.join("storage");
        let storage_resolver = Arc::new(orbis_infra::adapters::storage::StorageResolver::new(
            storage_root,
            integration_repo.clone(),
        ));
        let storage_adapter = storage_resolver.default_adapter();
        let file_storage_service = Arc::new(orbis_app::services::FileStorageService::new(
            storage_adapter.clone(),
            document_repo.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

        let integration_service = Arc::new(orbis_app::services::IntegrationService::new(
            integration_repo.clone(),
            permission_checker.clone(),
        ));

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
            report_provider_registry.clone(),
            report_renderer_registry.clone(),
            report_org_context_resolver.clone(),
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

        let autopilot_evaluator = Arc::new(orbis_app::autopilot::evaluator::AutopilotEvaluator::new(
            policy_repo.clone(),
            calendar_repo.clone(),
            calendar_repo.clone(),
            student_repo.clone(),
            finance_repo.clone(),
        ));

        let autopilot_service = Arc::new(orbis_app::services::AutopilotService::new(
            autopilot_evaluator.clone(),
            autopilot_engine.clone(),
            autopilot_task_repo.clone(),
            policy_repo.clone(),
            permission_checker.clone(),
        ));

        let ai_service = Arc::new(orbis_app::services::AiOrchestrationService::new(
            integration_resolver.clone(),
            permission_checker.clone(),
            student_service.clone(),
            academic_service.clone(),
            assessment_service.clone(),
            finance_service.clone(),
            scheduling_service.clone(),
            report_service.clone(),
            organization_service.clone(),
        ).with_autopilot_service(autopilot_service.clone()));

        // ── Bootstrap & System Services ─────────────────────────────
        let system_repo_impl = Arc::new(orbis_infra::repositories::system_repo::SystemRepoImpl { db: db.clone() });
        let entitlement_provider: Arc<dyn EntitlementProvider> = Arc::new(
            orbis_infra::adapters::entitlement_adapter::UnlicensedEntitlementProvider,
        );

        let system_service = Arc::new(orbis_app::services::SystemService::new(
            system_repo_impl.clone(),
        ));

        let solver_sidecar = Arc::new(crate::features::solver_sidecar::SolverSidecarManager::new());

        // ── Employee Affairs & User Groups Wiring ───────────────────
        let employee_repo = Arc::new(orbis_infra::repositories::employee_repo::EmployeeRepoImpl { db: db.clone() });
        let group_repo = Arc::new(orbis_infra::repositories::group_repo::GroupRepoImpl { db: db.clone() });

        let identity_provisioning_service = Arc::new(orbis_app::services::IdentityProvisioningService::new(
            user_repo.clone(),
            person_repo.clone(),
            identity_repo.clone(),
            group_repo.clone(),
            Some(integration_repo.clone()),
            Some(session_repo.clone()),
            Some(org_repo.clone()),
            None,
            password_hasher.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

        let employee_service = Arc::new(orbis_app::services::EmployeeService::new(
            employee_repo.clone(),
            person_repo.clone(),
            group_repo.clone(),
            identity_provisioning_service.clone(),
            event_bus.clone(),
            permission_checker.clone(),
        ));

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
            Arc::new(orbis_infra::repositories::role_repo::RoleRepoImpl::new(db.clone())),
            settings_repo.clone(),
            system_repo_impl.clone(),
            password_hasher.clone(),
            event_bus.clone(),
        ));

        let state = Self {
            db,
            active_session,
            org_repo: org_repo.clone(),
            org_unit_repo,
            user_repo,
            person_repo,
            session_repo,
            year_repo: calendar_repo.clone(),
            period_repo: calendar_repo.clone(),
            period_template_repo: calendar_repo,
            subject_repo: academic_repo.clone(),
            class_repo: academic_repo.clone(),
            class_subject_repo: academic_repo.clone(),
            student_repo: student_repo.clone(),
            enrollment_repo: student_repo.clone(),
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
            document_repo,
            doc_req_repo,
            sequence_repo,
            integration_repo,
            workflow_repo,
            report_definition_repo,
            report_template_repo,
            report_job_repo,
            report_audit_repo,
            report_provider_registry,
            report_renderer_registry,
            report_org_context_resolver,
            currency_repo: finance_repo.clone(),
            policy_repo,
            settings_repo,
            dashboard_repo,
            event_bus,
            token_service,
            password_hasher,
            integration_resolver,
            permission_checker,
            identity_repo,
            mfa_factor_repo,
            recovery_code_repo,
            device_repo,
            totp_service,
            oidc_service,
            student_service,
            progression_service,
            academic_service,
            finance_service,
            assessment_service,
            auth_service,
            organization_service,
            calendar_service,
            scheduling_service,
            document_service,
            folder_repo,
            folder_service: folder_service.clone(),
            file_storage_service,
            storage_resolver,
            storage_adapter,
            integration_service,
            workflow_service,
            report_service,
            ai_service,
            autopilot_service: autopilot_service.clone(),
            autopilot_task_repo: autopilot_task_repo.clone(),
            system_repo: system_repo_impl,
            system_service,
            entitlement_provider,
            solver_sidecar,
            app_data_dir: app_data_dir.to_path_buf(),
            // ── Employee Affairs & User Groups ─────────────────────────
            employee_repo,
            group_repo,
            identity_provisioning_service,
            employee_service,
            group_service,
            bootstrap_service,
        };

        // Spawn Autopilot Background Scheduler (checks every 60 seconds)
        let autopilot_scheduler = Arc::new(orbis_infra::autopilot::scheduler::AutopilotScheduler::new(
            autopilot_service.clone(),
            org_repo.clone(),
        ));
        let _scheduler_handle = autopilot_scheduler.spawn(60);

        Ok(state)
    }

    pub fn get_active_session(&self) -> Option<ActiveSession> {
        self.active_session.read().ok().and_then(|guard| guard.clone())
    }

    pub fn set_active_session(&self, session: ActiveSession) {
        if let Ok(mut guard) = self.active_session.write() {
            *guard = Some(session);
        }
    }

    pub fn set_active_unit(&self, unit_id: Option<Uuid>) {
        if let Ok(mut guard) = self.active_session.write() {
            if let Some(ref mut session) = *guard {
                session.active_unit_id = unit_id;
            }
        }
    }

    pub fn clear_active_session(&self) {
        if let Ok(mut guard) = self.active_session.write() {
            *guard = None;
        }
    }

    /// Load or generate a per-installation JWT secret.
    /// Stored in AppData/jwt_secret — never hardcoded.
    fn load_or_generate_jwt_secret(app_data_dir: &std::path::Path) -> Result<String, String> {
        let secret_path = app_data_dir.join("jwt_secret");
        if secret_path.exists() {
            std::fs::read_to_string(&secret_path)
                .map(|s| s.trim().to_string())
                .map_err(|e| format!("Failed to read JWT secret: {e}"))
        } else {
            use rand::Rng;
            let secret: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(64)
                .map(char::from)
                .collect();
            // Ensure parent directory exists
            if let Some(parent) = secret_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create AppData dir: {e}"))?;
            }
            std::fs::write(&secret_path, &secret)
                .map_err(|e| format!("Failed to write JWT secret: {e}"))?;
            Ok(secret)
        }
    }
}
