use async_trait::async_trait;
use orbis_domain::entities::organization::Organization;
use orbis_domain::entities::org_unit::OrgUnit;
use orbis_domain::entities::user::User;
use orbis_domain::entities::person::Person;
use orbis_domain::entities::session::Session;
use orbis_domain::entities::academic_year::AcademicYear;
use orbis_domain::entities::academic_period::AcademicPeriod;
use orbis_domain::entities::period_template::PeriodTemplate;
use orbis_domain::entities::subject::Subject;
use orbis_domain::entities::class::Class;
use orbis_domain::entities::class_subject::ClassSubject;
use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::entities::student_guardian::StudentGuardian;
use orbis_domain::entities::room::Room;
use orbis_domain::entities::time_slot::TimeSlot;
use orbis_domain::entities::scheduled_lesson::ScheduledLesson;
use orbis_domain::entities::assessment_scheme::AssessmentScheme;
use orbis_domain::entities::grade::Grade;
use orbis_domain::entities::attendance::Attendance;
use orbis_domain::entities::behavior_record::BehaviorRecord;
use orbis_domain::entities::exam_session::ExamSession;
use orbis_domain::entities::exam_schedule::ExamSchedule;
use orbis_domain::entities::seating_number::SeatingNumber;
use orbis_domain::entities::secret_number::SecretNumber;
use orbis_domain::entities::exam_proctor::ExamProctor;
use orbis_domain::entities::account::Account;
use orbis_domain::entities::journal_entry::JournalEntry;
use orbis_domain::entities::fee_template::FeeTemplate;
use orbis_domain::entities::student_fee::StudentFee;
use orbis_domain::entities::payment::Payment;
use orbis_domain::entities::document::Document;
use orbis_domain::errors::DomainError;
use uuid::Uuid;

#[async_trait]
pub trait SubjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Subject>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Subject>, DomainError>;
    async fn save(&self, subject: &Subject) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ClassRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Class>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<Class>, DomainError>;
    async fn save(&self, class: &Class) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ClassSubjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ClassSubject>, DomainError>;
    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<ClassSubject>, DomainError>;
    async fn save(&self, class_subject: &ClassSubject) -> Result<(), DomainError>;
}

#[async_trait]
pub trait EnrollmentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Enrollment>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Enrollment>, DomainError>;
    async fn save(&self, enrollment: &Enrollment) -> Result<(), DomainError>;
}

#[async_trait]
pub trait StudentGuardianRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<StudentGuardian>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<StudentGuardian>, DomainError>;
    async fn save(&self, guardian: &StudentGuardian) -> Result<(), DomainError>;
}

#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Room>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Room>, DomainError>;
    async fn save(&self, room: &Room) -> Result<(), DomainError>;
}

#[async_trait]
pub trait TimeSlotRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TimeSlot>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<TimeSlot>, DomainError>;
    async fn save(&self, time_slot: &TimeSlot) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ScheduledLessonRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ScheduledLesson>, DomainError>;
    async fn save(&self, lesson: &ScheduledLesson) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AssessmentSchemeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AssessmentScheme>, DomainError>;
    async fn save(&self, scheme: &AssessmentScheme) -> Result<(), DomainError>;
}

#[async_trait]
pub trait GradeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Grade>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Grade>, DomainError>;
    async fn save(&self, grade: &Grade) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AttendanceRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attendance>, DomainError>;
    async fn save(&self, attendance: &Attendance) -> Result<(), DomainError>;
}

#[async_trait]
pub trait BehaviorRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BehaviorRecord>, DomainError>;
    async fn save(&self, record: &BehaviorRecord) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamSessionRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSession>, DomainError>;
    async fn save(&self, session: &ExamSession) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamScheduleRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSchedule>, DomainError>;
    async fn save(&self, schedule: &ExamSchedule) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SeatingNumberRepository: Send + Sync {
    async fn save(&self, seating: &SeatingNumber) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SecretNumberRepository: Send + Sync {
    async fn save(&self, secret: &SecretNumber) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamProctorRepository: Send + Sync {
    async fn save(&self, proctor: &ExamProctor) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, DomainError>;
    async fn save(&self, account: &Account) -> Result<(), DomainError>;
}

#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<JournalEntry>, DomainError>;
    async fn save(&self, entry: &JournalEntry) -> Result<(), DomainError>;
}

#[async_trait]
pub trait FeeTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FeeTemplate>, DomainError>;
    async fn save(&self, template: &FeeTemplate) -> Result<(), DomainError>;
}

#[async_trait]
pub trait StudentFeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<StudentFee>, DomainError>;
    async fn save(&self, fee: &StudentFee) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Payment>, DomainError>;
    async fn save(&self, payment: &Payment) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AcademicYearRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicYear>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<AcademicYear>, DomainError>;
    async fn save(&self, year: &AcademicYear) -> Result<(), DomainError>;
}

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, DomainError>;
    async fn save(&self, document: &Document) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait IntegrationRepository: Send + Sync {
    async fn find_configs_by_org(&self, org_id: Uuid) -> Result<Vec<orbis_domain::entities::integration::IntegrationConfig>, DomainError>;
    async fn get_secrets_by_config(&self, config_id: Uuid) -> Result<Vec<orbis_domain::entities::integration::IntegrationSecret>, DomainError>;
    async fn save_config(&self, config: &orbis_domain::entities::integration::IntegrationConfig) -> Result<(), DomainError>;
    async fn save_secret(&self, secret: &orbis_domain::entities::integration::IntegrationSecret) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AiRepository: Send + Sync {
    async fn find_agent_by_id(&self, id: uuid::Uuid) -> Result<Option<orbis_domain::entities::ai::AiAgent>, DomainError>;
    async fn find_active_agents(&self, org_id: uuid::Uuid) -> Result<Vec<orbis_domain::entities::ai::AiAgent>, DomainError>;
    async fn save_insight(&self, insight: &orbis_domain::entities::ai::AiInsight) -> Result<(), DomainError>;
}

#[async_trait]
pub trait WorkflowRepository: Send + Sync {
    async fn find_rules_by_trigger(&self, org_id: Uuid, trigger_event: &str) -> Result<Vec<orbis_domain::entities::workflow::WorkflowRule>, DomainError>;
    async fn find_actions_by_rule(&self, rule_id: Uuid) -> Result<Vec<orbis_domain::entities::workflow::WorkflowAction>, DomainError>;
    async fn save_execution(&self, execution: &orbis_domain::entities::workflow::WorkflowExecution) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ReportRepository: Send + Sync {
    async fn find_template_by_id(&self, id: Uuid) -> Result<Option<orbis_domain::entities::report::ReportTemplate>, DomainError>;
    async fn save_job(&self, job: &orbis_domain::entities::report::ReportJob) -> Result<(), DomainError>;
    async fn update_job_status(&self, job_id: Uuid, status: &str, file_path: Option<String>) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AcademicPeriodRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicPeriod>, DomainError>;
    async fn find_by_year(&self, year_id: Uuid) -> Result<Vec<AcademicPeriod>, DomainError>;
    async fn save(&self, period: &AcademicPeriod) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PeriodTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PeriodTemplate>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<PeriodTemplate>, DomainError>;
    async fn save(&self, template: &PeriodTemplate) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Organization>, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Organization>, DomainError>;
    async fn save(&self, org: &Organization) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OrgUnitRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<OrgUnit>, DomainError>;
    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<OrgUnit>, DomainError>;
    async fn find_descendants(&self, org_id: Uuid, path_prefix: &str) -> Result<Vec<OrgUnit>, DomainError>;
    async fn count_branches(&self, org_id: Uuid) -> Result<u32, DomainError>;
    async fn save(&self, unit: &OrgUnit) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, DomainError>;
    async fn save(&self, person: &Person) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_token(&self, token: &str) -> Result<Option<Session>, DomainError>;
    async fn save(&self, session: &Session) -> Result<(), DomainError>;
}
