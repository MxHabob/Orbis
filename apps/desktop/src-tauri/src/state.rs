use std::sync::Arc;
use sea_orm::DatabaseConnection;
use orbis_app::ports::repositories::{OrganizationRepository, OrgUnitRepository, UserRepository, PersonRepository, SessionRepository, AcademicYearRepository, AcademicPeriodRepository, PeriodTemplateRepository, SubjectRepository, ClassRepository, ClassSubjectRepository, EnrollmentRepository, StudentGuardianRepository, RoomRepository, TimeSlotRepository, ScheduledLessonRepository, AssessmentSchemeRepository, GradeRepository, AttendanceRepository, BehaviorRecordRepository, ExamSessionRepository, ExamScheduleRepository, SeatingNumberRepository, SecretNumberRepository, ExamProctorRepository, AccountRepository, JournalEntryRepository, FeeTemplateRepository, StudentFeeRepository, PaymentRepository, DocumentRepository, IntegrationRepository, AiRepository, WorkflowRepository, ReportRepository};
use orbis_app::ports::event_bus::EventBus;
use orbis_app::ports::security::{TokenService, PasswordHasher};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
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
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_guardian_repo: Arc<dyn StudentGuardianRepository>,
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
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub ai_repo: Arc<dyn AiRepository>,
    pub workflow_repo: Arc<dyn WorkflowRepository>,
    pub report_repo: Arc<dyn ReportRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    // Plug and Play Gateway / Resolver
    pub integration_resolver: Arc<orbis_infra::adapters::resolver::IntegrationResolver>,
}

impl AppState {
    pub async fn init(app_data_dir: &std::path::Path) -> Result<Self, String> {
        let db_url = format!("sqlite://{}/orbis_local.db?mode=rwc", app_data_dir.display());
        let db = sea_orm::Database::connect(&db_url).await.map_err(|e| e.to_string())?;

        // Run migrations
        use orbis_infra::migration::Migrator;
        use sea_orm_migration::MigratorTrait;
        Migrator::up(&db, None).await.map_err(|e| e.to_string())?;

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
        let ai_repo = Arc::new(orbis_infra::repositories::ai_repos::AiRepoImpl { db: db.clone() });
        let workflow_repo = Arc::new(orbis_infra::repositories::workflow_repos::WorkflowRepoImpl { db: db.clone() });
        let report_repo = Arc::new(orbis_infra::repositories::report_repos::ReportRepoImpl { db: db.clone() });
        let policy_repo = Arc::new(orbis_infra::repositories::policy_repo::PgPolicyRepo { db: db.clone() });
        
        let autopilot_engine = Arc::new(orbis_app::autopilot::engine::AutopilotEngine {
            policy_repo: policy_repo.clone(),
        });

        let event_bus = Arc::new(orbis_infra::event_bus::MemoryEventBus {
            workflow_engine: Some(autopilot_engine),
        });
        let token_service = Arc::new(orbis_infra::security::jwt::JwtTokenService::new("desktop-secret".to_string()));
        let password_hasher = Arc::new(orbis_infra::security::bcrypt::BcryptHasher);

        let integration_resolver = Arc::new(orbis_infra::adapters::resolver::IntegrationResolver::new(integration_repo.clone()));

        Ok(Self {
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
            payment_repo: finance_repo,
            document_repo,
            integration_repo,
            ai_repo,
            workflow_repo,
            report_repo,
            event_bus,
            token_service,
            password_hasher,
            integration_resolver,
        })
    }
}
