use std::sync::Arc;
use sea_orm::DatabaseConnection;
use orbis_app::ports::repositories::{OrganizationRepository, OrgUnitRepository, UserRepository, PersonRepository, SessionRepository, AcademicYearRepository, AcademicPeriodRepository, PeriodTemplateRepository, SubjectRepository, ClassRepository, ClassSubjectRepository, EnrollmentRepository, StudentGuardianRepository, RoomRepository, TimeSlotRepository, ScheduledLessonRepository, AssessmentSchemeRepository, GradeRepository, AttendanceRepository, BehaviorRecordRepository};
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
    pub event_bus: Arc<dyn EventBus>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
}
