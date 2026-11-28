use orbis_app::ports::event_bus::EventBus;
use orbis_app::ports::iam::PermissionCheckerPort;
use orbis_app::ports::reporting::{
    DataProviderRegistry, RendererRegistry, ReportAuditRepository, ReportDefinitionRepository,
    ReportJobRepository, ReportTemplateRepository,
};
use orbis_app::ports::repositories::{
    AcademicCalendarRepository, AcademicFrameworkRepository, AcademicPeriodRepository,
    AccountRepository, AssessmentSchemeRepository, AttendanceRepository, BehaviorRecordRepository,
    CurrencyRepository, DashboardRepository, DocumentRepository, EnrollmentRepository,
    ExamProctorRepository, ExamScheduleRepository, ExamSessionRepository, FeeTemplateRepository,
    GradeRepository, IntegrationRepository, JournalEntryRepository, OrgUnitRepository,
    OrganizationRepository, PaymentRepository, PeriodTypeRepository, PersonRepository,
    PolicyRepository, RoomRepository, ScheduledLessonRepository, SeatingNumberRepository,
    SecretNumberRepository, SectionRepository, SectionSubjectRepository, SessionRepository,
    SettingsRepository, StudentFeeRepository, StudentGuardianRepository, SubjectRepository,
    TimeSlotRepository, UserRepository, WorkflowRepository,
};
use orbis_app::ports::security::{PasswordHasher, TokenService};
use orbis_app::services::OrgContextResolver;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone, Debug, serde::Serialize)]
pub struct InvalidationEvent {
    pub tenant_id: uuid::Uuid,
    pub entity: String,
    pub path: String,
}

#[derive(Clone)]
pub struct AppState {
    pub invalidation_tx: tokio::sync::broadcast::Sender<InvalidationEvent>,
    pub license_runtime: Arc<dyn orbis_app::ports::license::LicenseRuntime>,
    pub mulverse_client: Arc<orbis_infra::adapters::mulverse_client::MulverseClient>,
    pub db: DatabaseConnection,
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub org_unit_repo: Arc<dyn OrgUnitRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub year_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub period_type_repo: Arc<dyn PeriodTypeRepository>,
    pub academic_framework_repo: Arc<dyn AcademicFrameworkRepository>,
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub section_repo: Arc<dyn SectionRepository>,
    pub section_subject_repo: Arc<dyn SectionSubjectRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_repo: Arc<dyn orbis_app::ports::repositories::StudentRepository>,
    pub student_guardian_repo: Arc<dyn StudentGuardianRepository>,
    pub override_repo: Arc<dyn orbis_app::ports::repositories::OverrideRequestRepository>,
    pub prerequisite_repo: Arc<dyn orbis_app::ports::repositories::PrerequisiteRepository>,
    pub gpa_repo: Arc<dyn orbis_app::ports::repositories::GpaRecordRepository>,
    pub promotion_repo: Arc<dyn orbis_app::ports::repositories::PromotionRepository>,
    pub discount_template_repo: Arc<dyn orbis_app::ports::repositories::DiscountTemplateRepository>,
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
    pub folder_repo: Arc<dyn orbis_app::ports::repositories::FolderRepository>,
    pub doc_requirement_repo:
        Arc<dyn orbis_app::ports::repositories::DocumentRequirementRepository>,
    pub sequence_repo: Arc<dyn orbis_app::ports::repositories::SequenceCounterRepository>,
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub workflow_repo: Arc<dyn WorkflowRepository>,
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
    pub academic_framework_service: Arc<orbis_app::services::AcademicFrameworkService>,
    pub finance_service: Arc<orbis_app::services::FinanceService>,
    pub assessment_service: Arc<orbis_app::services::AssessmentService>,
    pub auth_service: Arc<orbis_app::services::AuthService>,
    pub organization_service: Arc<orbis_app::services::OrganizationService>,
    pub calendar_service: Arc<orbis_app::services::CalendarService>,
    pub scheduling_service: Arc<orbis_app::services::SchedulingService>,
    pub document_service: Arc<orbis_app::services::DocumentService>,
    pub folder_service: Arc<orbis_app::services::FolderService>,
    pub file_storage_service: Arc<orbis_app::services::FileStorageService>,
    pub storage_resolver: Arc<orbis_infra::adapters::storage::StorageResolver>,
    pub integration_service: Arc<orbis_app::services::IntegrationService>,
    pub notification_adapter: Arc<orbis_infra::adapters::notification::NotificationAdapter>,
    pub notification_service: Arc<orbis_app::services::NotificationService>,
    pub workflow_service: Arc<orbis_app::services::WorkflowService>,
    pub report_service: Arc<orbis_app::services::ReportService>,
    pub ai_service: Arc<orbis_app::services::AiOrchestrationService>,
    pub autopilot_service: Arc<orbis_app::services::AutopilotService>,
    pub autopilot_task_repo: Arc<dyn orbis_app::ports::repositories::AutopilotTaskRepository>,
    // ── Reporting System ────────────────────────────────────────
    pub report_definition_repo: Arc<dyn ReportDefinitionRepository>,
    pub report_template_repo: Arc<dyn ReportTemplateRepository>,
    pub report_job_repo: Arc<dyn ReportJobRepository>,
    pub report_audit_repo: Arc<dyn ReportAuditRepository>,
    pub report_provider_registry: Arc<dyn DataProviderRegistry>,
    pub report_renderer_registry: Arc<dyn RendererRegistry>,
    pub report_org_context_resolver: Arc<dyn OrgContextResolver>,
    pub system_service: Arc<orbis_app::services::SystemService>,
    // ── Employee Affairs & User Groups ─────────────────────────
    pub employee_repo: Arc<dyn orbis_app::ports::repositories::EmployeeRepository>,
    pub group_repo: Arc<dyn orbis_app::ports::repositories::GroupRepository>,
    pub identity_provisioning_service: Arc<orbis_app::services::IdentityProvisioningService>,
    pub employee_service: Arc<orbis_app::services::EmployeeService>,
    pub group_service: Arc<orbis_app::services::GroupService>,
    pub bootstrap_service: Arc<orbis_app::services::BootstrapService>,
    pub onboarding_service: Arc<orbis_app::services::OnboardingService>,
}

impl AppState {
    pub fn build(
        db: DatabaseConnection,
        license_runtime: Arc<dyn orbis_app::ports::license::LicenseRuntime>,
        mulverse_client: Arc<orbis_infra::adapters::mulverse_client::MulverseClient>,
        jwt_secret: String,
        mfa_encryption_key: String,
    ) -> Self {
        use orbis_app::ports::reporting::RendererRegistry;
        use orbis_infra::adapters::iam::db_permission_checker::DbPermissionChecker;
        use orbis_infra::adapters::report_adapter::*;
        use orbis_infra::event_bus::MemoryEventBus;
        use orbis_infra::repositories::report_repos::*;
        use orbis_infra::repositories::{
            org_repo::OrgRepoImpl, person_repo::PersonRepoImpl, session_repo::SessionRepoImpl,
            user_repo::UserRepoImpl,
        };
        use orbis_infra::security::{bcrypt::BcryptHasher, jwt::JwtTokenService};

        let org_repo = Arc::new(OrgRepoImpl { db: db.clone() });
        let org_unit_repo =
            Arc::new(orbis_infra::repositories::org_unit_repo::OrgUnitRepoImpl { db: db.clone() });
        let user_repo = Arc::new(UserRepoImpl { db: db.clone() });
        let person_repo = Arc::new(PersonRepoImpl { db: db.clone() });
        let session_repo = Arc::new(SessionRepoImpl { db: db.clone() });
        let calendar_repo = Arc::new(
            orbis_infra::repositories::calendar_repos::CalendarRepoImpl { db: db.clone() },
        );
        let academic_repo = Arc::new(
            orbis_infra::repositories::academic_repos::AcademicRepoImpl { db: db.clone() },
        );
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
        let document_repo = Arc::new(
            orbis_infra::repositories::document_repos::DocumentRepoImpl { db: db.clone() },
        );
        let folder_repo = Arc::new(orbis_infra::repositories::folder_repo::FolderRepoImpl::new(
            db.clone(),
        ));
        let notification_repo = Arc::new(
            orbis_infra::repositories::notification_repos::NotificationRepoImpl { db: db.clone() },
        );
        let notification_pref_repo = Arc::new(
            orbis_infra::repositories::notification_repos::NotificationPreferenceRepoImpl {
                db: db.clone(),
            },
        );
        let workflow_repo = Arc::new(
            orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() },
        );
        let override_repo = Arc::new(
            orbis_infra::repositories::override_repos::OverrideRepoImpl { db: db.clone() },
        );
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
        let dashboard_repo = Arc::new(
            orbis_infra::repositories::dashboard_repo::DashboardRepoImpl { db: db.clone() },
        );

        let event_bus = Arc::new(MemoryEventBus::new());
        let integration_repo = Arc::new(
            orbis_infra::repositories::integration_repos::IntegrationRepoImpl::new(
                db.clone(),
                &mfa_encryption_key,
            ),
        );
        let token_service = Arc::new(JwtTokenService::new(jwt_secret));
        let password_hasher = Arc::new(BcryptHasher);

        let integration_resolver = Arc::new(
            orbis_infra::adapters::resolver::IntegrationResolver::new(integration_repo.clone()),
        );
        let notification_adapter = Arc::new(
            orbis_infra::adapters::notification::NotificationAdapter::new(
                integration_repo.clone(),
                settings_repo.clone(),
            ),
        );
        let permission_checker = Arc::new(DbPermissionChecker::new(db.clone()));
        let academic_framework_service =
            Arc::new(orbis_app::services::AcademicFrameworkService::new(
                academic_framework_repo.clone(),
                student_repo.clone(),
                org_unit_repo.clone(),
                permission_checker.clone(),
            ));

        let identity_repo = Arc::new(
            orbis_infra::repositories::identity_repo::UserIdentityRepoImpl { db: db.clone() },
        );
        let mfa_factor_repo =
            Arc::new(orbis_infra::repositories::mfa_repo::MfaFactorRepoImpl { db: db.clone() });
        let recovery_code_repo = Arc::new(
            orbis_infra::repositories::mfa_repo::MfaRecoveryCodeRepoImpl { db: db.clone() },
        );
        let device_repo =
            Arc::new(orbis_infra::repositories::device_repo::UserDeviceRepoImpl { db: db.clone() });
        let totp_service = Arc::new(orbis_infra::security::totp::TotpServiceImpl::new(
            &mfa_encryption_key,
        ));
        let oidc_service = Arc::new(orbis_infra::adapters::identity::GoogleIdentityAdapter::new());

        let report_definition_repo = Arc::new(ReportDefinitionRepoImpl { db: db.clone() });
        let report_template_repo = Arc::new(ReportTemplateRepoImpl { db: db.clone() });
        let report_job_repo = Arc::new(ReportJobRepoImpl { db: db.clone() });
        let report_audit_repo = Arc::new(ReportAuditRepoImpl { db: db.clone() });

        let mut provider_registry = InMemoryDataProviderRegistry::new();
        orbis_infra::adapters::report_adapter::register_default_providers(
            &mut provider_registry,
            db.clone(),
        );
        let provider_registry = Arc::new(provider_registry);

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

        let finance_service = Arc::new(
            orbis_app::services::FinanceService::new(
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
            )
            .with_student_repo(student_repo.clone()),
        );

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
            .with_license_runtime(license_runtime.clone())
            .with_finance_service(finance_service.clone()),
        );

        let progression_service = Arc::new(
            orbis_app::services::ProgressionService::new(
                progression_repo.clone(),
                student_repo.clone(),
                student_repo.clone(),
                event_bus.clone(),
                permission_checker.clone(),
            )
            .with_erp_dependencies(
                org_repo.clone(),
                calendar_repo.clone(),
                academic_repo.clone(),
                policy_repo.clone(),
                assessment_repo.clone(),
                assessment_repo.clone(),
                assessment_repo.clone(),
                finance_repo.clone(),
                finance_service.clone(),
            ),
        );

        let academic_service = Arc::new(orbis_app::services::AcademicService::new(
            academic_repo.clone(),
            academic_repo.clone(),
            academic_repo.clone(),
            academic_repo.clone(),
            assessment_repo.clone(),
            event_bus.clone(),
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
            .with_attendance_scope_repos(employee_repo.clone(), scheduling_repo.clone())
            .with_override_repo(override_repo.clone()),
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
            calendar_repo.clone(),
            academic_repo.clone(),
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
            org_unit_repo.clone(),
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

        let autopilot_engine = Arc::new(
            orbis_app::autopilot::engine::AutopilotEngine::new(
                policy_repo.clone(),
                autopilot_task_repo.clone(),
                calendar_service.clone(),
                progression_service.clone(),
                report_service.clone(),
            )
            .with_smart_services(
                notification_service.clone(),
                user_repo.clone(),
                student_repo.clone(),
                finance_repo.clone(),
                academic_repo.clone(),
                employee_repo.clone(),
            )
            .with_scheduling_service(scheduling_service.clone(), exam_repo.clone()),
        );

        let autopilot_evaluator = Arc::new(
            orbis_app::autopilot::evaluator::AutopilotEvaluator::new(
                calendar_repo.clone(),
                calendar_repo.clone(),
            )
            .with_fee_repo(finance_repo.clone())
            .with_exam_session_repo(exam_repo.clone()),
        );

        let autopilot_service = Arc::new(orbis_app::services::AutopilotService::new(
            autopilot_evaluator,
            autopilot_engine,
            autopilot_task_repo.clone(),
            policy_repo.clone(),
            permission_checker.clone(),
        ));

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

        let (invalidation_tx, _) = tokio::sync::broadcast::channel(512);

        Self {
            invalidation_tx,
            mulverse_client,
            db: db.clone(),
            org_repo,
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
            document_repo,
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
            notification_service,
            workflow_service,
            report_service,
            ai_service,
            autopilot_service,
            autopilot_task_repo,
            report_definition_repo,
            report_template_repo,
            report_job_repo,
            report_audit_repo,
            report_provider_registry: provider_registry,
            report_renderer_registry: renderer_registry,
            report_org_context_resolver: org_context_resolver,
            license_runtime,
            system_service: Arc::new(orbis_app::services::SystemService::new(Arc::new(
                orbis_infra::repositories::system_repo::SystemRepoImpl { db },
            ))),
            employee_repo,
            group_repo,
            identity_provisioning_service,
            employee_service,
            group_service,
            bootstrap_service,
            onboarding_service,
        }
    }
}
