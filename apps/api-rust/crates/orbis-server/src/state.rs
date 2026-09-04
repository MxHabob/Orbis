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
