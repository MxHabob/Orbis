use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use orbis_domain::{
    autopilot::task::AutopilotTask,
    entities::{
        academic_period::AcademicPeriod,
        academic_year::AcademicYear,
        account::Account,
        assessment_scheme::AssessmentScheme,
        attendance::Attendance,
        behavior_record::BehaviorRecord,
        class::Class,
        class_subject::ClassSubject,
        course_prerequisite::CoursePrerequisite,
        currency::Currency,
        device::UserDevice,
        discount_template::DiscountTemplate,
        document::Document,
        document_requirement::DocumentRequirement,
        employee::{Employee, EmployeeDetail},
        enrollment::{Enrollment, EnrollmentStatus},
        exam_proctor::ExamProctor,
        exam_schedule::ExamSchedule,
        exam_session::ExamSession,
        exchange_rate::ExchangeRate,
        fee_template::FeeTemplate,
        folder::Folder,
        grade::Grade,
        group::{Group, GroupDetail, GroupMemberInfo, UserGroupMembership},
        identity::UserIdentity,
        integration::{IntegrationConfig, IntegrationSecret},
        journal_entry::JournalEntry,
        mfa::{MfaFactor, MfaRecoveryCode},
        org_unit::OrgUnit,
        organization::Organization,
        override_request::AcademicOverrideRequest,
        payment::Payment,
        period_template::PeriodTemplate,
        person::Person,
        progression::{PromotionDetail, PromotionJob},
        role::Role,
        role_permission::RolePermission,
        room::Room,
        scheduled_lesson::ScheduledLesson,
        seating_number::SeatingNumber,
        secret_number::SecretNumber,
        settings::OrganizationSettings,
        student::{Student, StudentDetail},
        student_fee::StudentFee,
        student_guardian::StudentGuardian,
        subject::Subject,
        time_slot::TimeSlot,
        user::User,
        user_role_assignment::UserRoleAssignment,
        workflow::{WorkflowAction, WorkflowExecution, WorkflowRule},
    },
    errors::DomainError,
    queries::filters::{EnrollmentFilter, PaginatedList, StudentFilter},
};

// =========================================================================
// Organization & Unit
// =========================================================================

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Organization>, DomainError>;
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Organization>, DomainError>;
    async fn find_first(&self) -> Result<Option<Organization>, DomainError>;
    async fn list_all(&self) -> Result<Vec<Organization>, DomainError>;
    async fn save(&self, org: &Organization) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OrgUnitRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<OrgUnit>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<OrgUnit>, DomainError>;
    async fn find_children(&self, parent_id: Uuid) -> Result<Vec<OrgUnit>, DomainError>;
    async fn find_descendants(
        &self,
        org_id: Uuid,
        path_prefix: &str,
    ) -> Result<Vec<OrgUnit>, DomainError>;
    async fn count_branches(&self, org_id: Uuid) -> Result<u32, DomainError>;
    async fn save(&self, unit: &OrgUnit) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// User, Identity, MFA & Device
// =========================================================================

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, DomainError>;
    async fn find_by_national_id(
        &self,
        org_id: Option<Uuid>,
        national_id: &str,
    ) -> Result<Option<Person>, DomainError> {
        let _ = (org_id, national_id);
        Ok(None)
    }
    async fn find_by_email(
        &self,
        org_id: Option<Uuid>,
        email: &str,
    ) -> Result<Option<Person>, DomainError> {
        let _ = (org_id, email);
        Ok(None)
    }
    async fn find_by_phone(
        &self,
        org_id: Option<Uuid>,
        phone: &str,
    ) -> Result<Option<Person>, DomainError> {
        let _ = (org_id, phone);
        Ok(None)
    }
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Person>, DomainError> {
        let _ = org_id;
        Ok(vec![])
    }
    async fn save(&self, person: &Person) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserIdentityRepository: Send + Sync {
    async fn find_by_provider(
        &self,
        provider: &str,
        provider_user_id: &str,
    ) -> Result<Option<UserIdentity>, DomainError>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<UserIdentity>, DomainError>;
    async fn save(&self, identity: &UserIdentity) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserDeviceRepository: Send + Sync {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<UserDevice>, DomainError>;
    async fn find_by_device_id(
        &self,
        user_id: Uuid,
        device_id: &str,
    ) -> Result<Option<UserDevice>, DomainError>;
    async fn save(&self, device: &UserDevice) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn find_by_token(
        &self,
        token: &str,
    ) -> Result<Option<orbis_domain::entities::session::Session>, DomainError>;
    async fn save(
        &self,
        session: &orbis_domain::entities::session::Session,
    ) -> Result<(), DomainError>;
    async fn invalidate(&self, token: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait MfaFactorRepository: Send + Sync {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<MfaFactor>, DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<MfaFactor>, DomainError>;
    async fn save(&self, factor: &MfaFactor) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait MfaRecoveryCodeRepository: Send + Sync {
    async fn find_active_by_user(&self, user_id: Uuid)
    -> Result<Vec<MfaRecoveryCode>, DomainError>;
    async fn save_all(&self, codes: &[MfaRecoveryCode]) -> Result<(), DomainError>;
    async fn mark_used(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Calendar
// =========================================================================

#[async_trait]
pub trait AcademicYearRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicYear>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<AcademicYear>, DomainError>;
    async fn find_current(&self, org_id: Uuid) -> Result<Option<AcademicYear>, DomainError>;
    async fn save(&self, year: &AcademicYear) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AcademicPeriodRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicPeriod>, DomainError>;
    async fn find_by_year(&self, year_id: Uuid) -> Result<Vec<AcademicPeriod>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<AcademicPeriod>, DomainError>;
    async fn find_current(&self, year_id: Uuid) -> Result<Option<AcademicPeriod>, DomainError>;
    async fn save(&self, period: &AcademicPeriod) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PeriodTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<PeriodTemplate>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<PeriodTemplate>, DomainError>;
    async fn save(&self, template: &PeriodTemplate) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AcademicFrameworkRepository: Send + Sync {
    async fn list_calendars(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::academic_calendar::AcademicCalendar>, DomainError>;
    async fn find_calendar(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<orbis_domain::entities::academic_calendar::AcademicCalendar>, DomainError>;
    async fn find_calendar_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<orbis_domain::entities::academic_calendar::AcademicCalendar>, DomainError>;
    async fn save_calendar(
        &self,
        calendar: &orbis_domain::entities::academic_calendar::AcademicCalendar,
    ) -> Result<(), DomainError>;
    async fn list_period_types(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::academic_calendar::AcademicPeriodType>, DomainError>;
    async fn find_period_type(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<orbis_domain::entities::academic_calendar::AcademicPeriodType>, DomainError>;
    async fn find_period_type_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<orbis_domain::entities::academic_calendar::AcademicPeriodType>, DomainError>;
    async fn save_period_type(
        &self,
        period_type: &orbis_domain::entities::academic_calendar::AcademicPeriodType,
    ) -> Result<(), DomainError>;
    async fn list_calendar_periods(
        &self,
        org_id: Uuid,
        calendar_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::academic_calendar::AcademicCalendarPeriod>, DomainError>;
    async fn find_calendar_period(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<
        Option<orbis_domain::entities::academic_calendar::AcademicCalendarPeriod>,
        DomainError,
    >;
    async fn save_calendar_period(
        &self,
        period: &orbis_domain::entities::academic_calendar::AcademicCalendarPeriod,
    ) -> Result<(), DomainError>;
    async fn find_course(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<orbis_domain::entities::course::Course>, DomainError>;
    async fn list_courses(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::course::Course>, DomainError>;
    async fn find_course_by_code(
        &self,
        org_id: Uuid,
        code: &str,
    ) -> Result<Option<orbis_domain::entities::course::Course>, DomainError>;
    async fn save_course(
        &self,
        course: &orbis_domain::entities::course::Course,
    ) -> Result<(), DomainError>;
    async fn find_course_offering(
        &self,
        org_id: Uuid,
        id: Uuid,
    ) -> Result<Option<orbis_domain::entities::course_offering::CourseOffering>, DomainError>;
    async fn list_course_offerings(
        &self,
        org_id: Uuid,
        course_id: Option<Uuid>,
    ) -> Result<Vec<orbis_domain::entities::course_offering::CourseOffering>, DomainError>;
    async fn save_course_offering(
        &self,
        offering: &orbis_domain::entities::course_offering::CourseOffering,
    ) -> Result<(), DomainError>;
    async fn find_course_enrollment_attempt(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
        student_id: Uuid,
        attempt_number: u32,
    ) -> Result<Option<orbis_domain::entities::course_enrollment::CourseEnrollment>, DomainError>;
    async fn list_course_enrollments(
        &self,
        org_id: Uuid,
        offering_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::course_enrollment::CourseEnrollment>, DomainError>;
    async fn save_course_enrollment(
        &self,
        enrollment: &orbis_domain::entities::course_enrollment::CourseEnrollment,
    ) -> Result<(), DomainError>;
}

// =========================================================================
// Student & Enrollment
// =========================================================================

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Student>, DomainError>;
    async fn find_by_user_id(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Student>, DomainError> {
        let _ = (org_id, user_id);
        Ok(None)
    }
    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<StudentDetail>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Student>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: StudentFilter,
    ) -> Result<PaginatedList<StudentDetail>, DomainError>;
    async fn find_by_student_number(
        &self,
        org_id: Uuid,
        number: &str,
    ) -> Result<Option<Student>, DomainError>;
    async fn find_by_national_id(
        &self,
        org_id: Uuid,
        national_id: &str,
    ) -> Result<Option<Student>, DomainError>;
    async fn find_by_person_id(&self, person_id: Uuid) -> Result<Option<Student>, DomainError> {
        let _ = person_id;
        Ok(None)
    }
    async fn save(&self, student: &Student) -> Result<(), DomainError>;
    async fn create_with_license_limit(
        &self,
        student: &Student,
        max_students: Option<u64>,
    ) -> Result<(), DomainError> {
        if let Some(max_students) = max_students {
            let current = self.find_by_org(student.org_id).await?.len() as u64;
            if current.saturating_add(1) > max_students {
                return Err(DomainError::LicenseRequired(format!(
                    "The student quota of {max_students} has been reached"
                )));
            }
        }
        self.save(student).await
    }
    async fn update_status(&self, id: Uuid, status: &str) -> Result<(), DomainError>;
    async fn update_status_batch(&self, ids: &[Uuid], status: &str) -> Result<usize, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait EnrollmentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Enrollment>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: EnrollmentFilter,
    ) -> Result<PaginatedList<Enrollment>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Enrollment>, DomainError>;
    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<Enrollment>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<Enrollment>, DomainError>;
    async fn save(&self, enrollment: &Enrollment) -> Result<(), DomainError>;
    async fn save_batch(&self, enrollments: &[Enrollment]) -> Result<(), DomainError>;
    async fn update_status_batch(
        &self,
        enrollment_ids: &[Uuid],
        status: EnrollmentStatus,
    ) -> Result<(), DomainError>;
}

#[async_trait]
pub trait StudentGuardianRepository: Send + Sync {
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<StudentGuardian>, DomainError>;
    async fn find_by_guardian(
        &self,
        guardian_id: Uuid,
    ) -> Result<Vec<StudentGuardian>, DomainError>;
    async fn save(&self, guardian: &StudentGuardian) -> Result<(), DomainError>;
    async fn delete(&self, student_id: Uuid, guardian_id: Uuid) -> Result<(), DomainError>;
}

pub use StudentGuardianRepository as GuardianRepository;

#[async_trait]
pub trait GpaRecordRepository: Send + Sync {
    async fn find_by_student(
        &self,
        student_id: Uuid,
    ) -> Result<Vec<orbis_domain::entities::gpa_record::CumulativeGpaRecord>, DomainError>;
    async fn find_by_student_and_period(
        &self,
        student_id: Uuid,
        period_id: Uuid,
    ) -> Result<Option<orbis_domain::entities::gpa_record::CumulativeGpaRecord>, DomainError>;
    async fn save(
        &self,
        record: &orbis_domain::entities::gpa_record::CumulativeGpaRecord,
    ) -> Result<(), DomainError>;
}

// =========================================================================
// Academic
// =========================================================================

#[async_trait]
pub trait SubjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Subject>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Subject>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::SubjectFilter,
    ) -> Result<PaginatedList<Subject>, DomainError>;
    async fn save(&self, subject: &Subject) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ClassRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Class>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<Class>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Class>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::ClassFilter,
    ) -> Result<PaginatedList<Class>, DomainError>;
    async fn save(&self, class: &Class) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ClassSubjectRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ClassSubject>, DomainError>;
    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<ClassSubject>, DomainError>;
    async fn find_by_class_and_period(
        &self,
        class_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<ClassSubject>, DomainError>;
    async fn find_by_teacher(
        &self,
        teacher_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<ClassSubject>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::ClassSubjectFilter,
    ) -> Result<PaginatedList<ClassSubject>, DomainError>;
    async fn save(&self, class_subject: &ClassSubject) -> Result<(), DomainError>;
}

#[async_trait]
pub trait CoursePrerequisiteRepository: Send + Sync {
    async fn find_by_subject(
        &self,
        subject_id: Uuid,
    ) -> Result<Vec<CoursePrerequisite>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<CoursePrerequisite>, DomainError>;
    async fn save(&self, prereq: &CoursePrerequisite) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Assessment
// =========================================================================

#[async_trait]
pub trait AssessmentSchemeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AssessmentScheme>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<AssessmentScheme>, DomainError>;
    async fn save(&self, scheme: &AssessmentScheme) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait GradeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Grade>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Grade>, DomainError>;
    async fn find_by_student_and_period(
        &self,
        student_id: Uuid,
        period_id: Uuid,
    ) -> Result<Vec<Grade>, DomainError>;
    async fn find_by_student_and_subject(
        &self,
        student_id: Uuid,
        class_subject_id: Uuid,
    ) -> Result<Vec<Grade>, DomainError>;
    async fn find_by_enrollment(&self, enrollment_id: Uuid) -> Result<Vec<Grade>, DomainError>;
    async fn find_by_class_subject(
        &self,
        class_subject_id: Uuid,
    ) -> Result<Vec<Grade>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::GradeFilter,
    ) -> Result<PaginatedList<Grade>, DomainError>;
    async fn save(&self, grade: &Grade) -> Result<(), DomainError>;
    async fn save_batch(&self, grades: &[Grade]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AttendanceRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Attendance>, DomainError>;
    async fn find_by_class_and_date(
        &self,
        class_id: Uuid,
        date: NaiveDate,
    ) -> Result<Vec<Attendance>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Attendance>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::AttendanceFilter,
    ) -> Result<PaginatedList<Attendance>, DomainError>;
    async fn save(&self, attendance: &Attendance) -> Result<(), DomainError>;
    async fn save_batch(&self, attendances: &[Attendance]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait BehaviorRecordRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BehaviorRecord>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<BehaviorRecord>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<BehaviorRecord>, DomainError>;
    async fn save(&self, record: &BehaviorRecord) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Scheduling & Exam
// =========================================================================

#[async_trait]
pub trait RoomRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Room>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Room>, DomainError>;
    async fn save(&self, room: &Room) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait TimeSlotRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<TimeSlot>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<TimeSlot>, DomainError>;
    async fn save(&self, slot: &TimeSlot) -> Result<(), DomainError>;
    async fn save_bulk(&self, slots: &[TimeSlot]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_period(&self, period_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ScheduledLessonRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ScheduledLesson>, DomainError>;
    async fn find_by_class(&self, class_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError>;
    async fn find_by_teacher(&self, teacher_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError>;
    async fn find_by_class_subject(
        &self,
        class_subject_id: Uuid,
    ) -> Result<Vec<ScheduledLesson>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<ScheduledLesson>, DomainError>;
    async fn save(&self, lesson: &ScheduledLesson) -> Result<(), DomainError>;
    async fn save_bulk(&self, lessons: &[ScheduledLesson]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_period(&self, period_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamSessionRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSession>, DomainError>;
    async fn find_by_period(&self, period_id: Uuid) -> Result<Vec<ExamSession>, DomainError>;
    async fn save(&self, session: &ExamSession) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamScheduleRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamSchedule>, DomainError>;
    async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<ExamSchedule>, DomainError>;
    async fn save(&self, schedule: &ExamSchedule) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SeatingNumberRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SeatingNumber>, DomainError>;
    async fn find_by_session(&self, session_id: Uuid) -> Result<Vec<SeatingNumber>, DomainError>;
    async fn find_by_session_and_student(
        &self,
        session_id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<SeatingNumber>, DomainError>;
    async fn save(&self, seating: &SeatingNumber) -> Result<(), DomainError>;
    async fn save_bulk(&self, seatings: &[SeatingNumber]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_session(&self, session_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SecretNumberRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SecretNumber>, DomainError>;
    async fn find_by_schedule(&self, schedule_id: Uuid) -> Result<Vec<SecretNumber>, DomainError>;
    async fn find_by_schedule_and_seating(
        &self,
        schedule_id: Uuid,
        student_id: Uuid,
    ) -> Result<Option<SecretNumber>, DomainError>;
    async fn save(&self, secret: &SecretNumber) -> Result<(), DomainError>;
    async fn save_bulk(&self, secrets: &[SecretNumber]) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_schedule(&self, schedule_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ExamProctorRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ExamProctor>, DomainError>;
    async fn find_by_schedule(&self, schedule_id: Uuid) -> Result<Vec<ExamProctor>, DomainError>;
    async fn save(&self, proctor: &ExamProctor) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_schedule(&self, schedule_id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Finance
// =========================================================================

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Account>, DomainError>;
    async fn find_by_code(&self, org_id: Uuid, code: &str) -> Result<Option<Account>, DomainError>;
    async fn save(&self, account: &Account) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<JournalEntry>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<JournalEntry>, DomainError>;
    async fn save(&self, entry: &JournalEntry) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait FeeTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<FeeTemplate>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<FeeTemplate>, DomainError>;
    async fn save(&self, template: &FeeTemplate) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait DiscountTemplateRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DiscountTemplate>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DiscountTemplate>, DomainError>;
    async fn save(&self, template: &DiscountTemplate) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait StudentFeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<StudentFee>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<StudentFee>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<StudentFee>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::StudentFeeFilter,
    ) -> Result<PaginatedList<StudentFee>, DomainError>;
    async fn save(&self, fee: &StudentFee) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PaymentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Payment>, DomainError>;
    async fn find_by_student(&self, student_id: Uuid) -> Result<Vec<Payment>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Payment>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: orbis_domain::queries::filters::PaymentFilter,
    ) -> Result<PaginatedList<Payment>, DomainError>;
    async fn save(&self, payment: &Payment) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait CurrencyRepository: Send + Sync {
    async fn find_by_code(&self, org_id: Uuid, code: &str)
    -> Result<Option<Currency>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Currency>, DomainError>;
    async fn save(&self, currency: &Currency) -> Result<(), DomainError>;
    async fn get_rate(
        &self,
        from_id: Uuid,
        to_id: Uuid,
    ) -> Result<Option<ExchangeRate>, DomainError>;
    async fn get_exchange_rate(
        &self,
        org_id: Uuid,
        from: &str,
        to: &str,
    ) -> Result<Option<ExchangeRate>, DomainError>;
    async fn save_rate(&self, rate: &ExchangeRate) -> Result<(), DomainError>;
}

// =========================================================================
// Progression & Overrides
// =========================================================================

#[async_trait]
pub trait PromotionRepository: Send + Sync {
    async fn find_job_by_id(&self, job_id: Uuid) -> Result<Option<PromotionJob>, DomainError>;
    async fn find_jobs_by_org(&self, org_id: Uuid) -> Result<Vec<PromotionJob>, DomainError>;
    async fn find_details_by_job(&self, job_id: Uuid) -> Result<Vec<PromotionDetail>, DomainError>;
    async fn save_job(&self, job: &PromotionJob) -> Result<(), DomainError>;
    async fn save_detail(&self, detail: &PromotionDetail) -> Result<(), DomainError>;
    async fn save_details(&self, details: &[PromotionDetail]) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OverrideRequestRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AcademicOverrideRequest>, DomainError>;
    async fn find_by_org(
        &self,
        org_id: Uuid,
        status: Option<&str>,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError>;
    async fn find_active_override(
        &self,
        org_id: Uuid,
        override_type: &str,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Option<AcademicOverrideRequest>, DomainError>;
    async fn find_pending_for_role(
        &self,
        org_id: Uuid,
        role: &str,
    ) -> Result<Vec<AcademicOverrideRequest>, DomainError>;
    async fn save(&self, req: &AcademicOverrideRequest) -> Result<(), DomainError>;
}

// =========================================================================
// Workflows, Documents & Integrations
// =========================================================================

#[async_trait]
pub trait WorkflowRepository: Send + Sync {
    async fn find_rules_by_trigger(
        &self,
        org_id: Uuid,
        trigger_event: &str,
    ) -> Result<Vec<WorkflowRule>, DomainError>;
    async fn find_actions_by_rule(&self, rule_id: Uuid)
    -> Result<Vec<WorkflowAction>, DomainError>;
    async fn save_execution(&self, execution: &WorkflowExecution) -> Result<(), DomainError>;
}

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Document>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError>;
    async fn find_by_entity(
        &self,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Vec<Document>, DomainError>;
    async fn find_by_folder(
        &self,
        org_id: Uuid,
        folder_id: Uuid,
    ) -> Result<Vec<Document>, DomainError>;
    async fn find_by_curriculum(
        &self,
        org_id: Uuid,
        academic_year_id: Option<Uuid>,
        term_id: Option<Uuid>,
    ) -> Result<Vec<Document>, DomainError>;
    async fn find_archived(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError>;
    async fn find_trash(&self, org_id: Uuid) -> Result<Vec<Document>, DomainError>;
    async fn save(&self, document: &Document) -> Result<(), DomainError>;
    async fn soft_delete(&self, id: Uuid, deleted_by: Option<Uuid>) -> Result<(), DomainError>;
    async fn restore(&self, id: Uuid) -> Result<(), DomainError>;
    async fn purge(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_entity(&self, entity_type: &str, entity_id: Uuid)
    -> Result<(), DomainError>;
}

#[async_trait]
pub trait FolderRepository: Send + Sync {
    async fn find_by_id(&self, org_id: Uuid, id: Uuid) -> Result<Option<Folder>, DomainError>;
    async fn find_by_path(&self, org_id: Uuid, path: &str) -> Result<Option<Folder>, DomainError>;
    async fn find_by_entity(
        &self,
        org_id: Uuid,
        entity_type: &str,
        entity_id: Uuid,
    ) -> Result<Option<Folder>, DomainError>;
    async fn list_children(
        &self,
        org_id: Uuid,
        parent_id: Option<Uuid>,
    ) -> Result<Vec<Folder>, DomainError>;
    async fn list_by_type(
        &self,
        org_id: Uuid,
        folder_type: &str,
    ) -> Result<Vec<Folder>, DomainError>;
    async fn list_all(&self, org_id: Uuid) -> Result<Vec<Folder>, DomainError>;
    async fn save(&self, folder: &Folder) -> Result<(), DomainError>;
    async fn delete(&self, org_id: Uuid, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait IntegrationRepository: Send + Sync {
    async fn find_configs_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<IntegrationConfig>, DomainError>;
    async fn get_secrets_by_config(
        &self,
        config_id: Uuid,
    ) -> Result<Vec<IntegrationSecret>, DomainError>;
    async fn save_config(&self, config: &IntegrationConfig) -> Result<(), DomainError>;
    async fn save_secret(&self, secret: &IntegrationSecret) -> Result<(), DomainError>;
    async fn find_config_by_provider(
        &self,
        org_id: Uuid,
        provider_name: &str,
    ) -> Result<Option<IntegrationConfig>, DomainError>;
    async fn delete_config(&self, config_id: Uuid) -> Result<(), DomainError>;
    async fn delete_by_provider(
        &self,
        org_id: Uuid,
        provider_name: &str,
    ) -> Result<(), DomainError>;
}

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError>;
    async fn save_settings(
        &self,
        org_id: Uuid,
        settings: &OrganizationSettings,
    ) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PolicyRepository: Send + Sync {
    async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardCounts {
    pub total_students: u64,
    pub total_teachers: u64,
    pub total_classes: u64,
    pub total_enrollments: u64,
    pub total_subjects: u64,
    pub active_academic_year: Option<String>,
    pub total_payments_amount: f64,
    pub pending_fees_amount: f64,
}

#[async_trait]
pub trait DashboardRepository: Send + Sync {
    async fn get_counts(&self, org_id: Uuid) -> Result<DashboardCounts, DomainError>;
}

// =========================================================================
// System (Health / Readiness / Bootstrap Status)
// =========================================================================

/// System-level queries for health checks, bootstrap status, and readiness.
#[async_trait]
pub trait SystemRepository: Send + Sync {
    /// Check if any organization exists in the database.
    async fn has_organization(&self) -> Result<bool, DomainError>;

    /// Check if any admin user exists (user with super_admin or admin role).
    async fn has_admin(&self) -> Result<bool, DomainError>;

    /// Check if any academic year exists for the given organization.
    async fn has_academic_year(&self, org_id: Uuid) -> Result<bool, DomainError>;

    /// Check if any classes exist for the given organization.
    async fn has_classes(&self, org_id: Uuid) -> Result<bool, DomainError>;

    /// Check if any subjects exist for the given organization.
    async fn has_subjects(&self, org_id: Uuid) -> Result<bool, DomainError>;

    /// Get the name of the first organization (for display in diagnostics).
    async fn get_org_name(&self) -> Result<Option<String>, DomainError>;

    /// Get the first organization's ID (for readiness checks).
    async fn get_first_org_id(&self) -> Result<Option<Uuid>, DomainError>;
}

// =========================================================================
// Autopilot Task Repository Port
// =========================================================================

#[async_trait]
pub trait AutopilotTaskRepository: Send + Sync {
    /// Retrieve all pending tasks due for execution across all orgs (or for a specific org)
    async fn find_pending_tasks(
        &self,
        org_id: Option<Uuid>,
    ) -> Result<Vec<AutopilotTask>, DomainError>;

    /// Find a specific task by its ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AutopilotTask>, DomainError>;

    /// Find tasks by org with status/task_type filters and pagination
    async fn find_by_org(
        &self,
        org_id: Uuid,
        status: Option<&str>,
        task_type: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<(Vec<AutopilotTask>, u64), DomainError>;

    /// Save (insert or update) an Autopilot task
    async fn save_task(&self, task: &AutopilotTask) -> Result<(), DomainError>;

    /// Delete a task by ID
    async fn delete_task(&self, id: Uuid) -> Result<(), DomainError>;

    /// Find stuck tasks in Executing status exceeding threshold seconds
    async fn find_stuck_tasks(
        &self,
        org_id: Uuid,
        threshold_seconds: i64,
    ) -> Result<Vec<AutopilotTask>, DomainError>;

    /// Find overdue tasks in Pending status where scheduled_at < now
    async fn find_overdue_tasks(&self, org_id: Uuid) -> Result<Vec<AutopilotTask>, DomainError>;
}

// =========================================================================
// Employee Repository Port
// =========================================================================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmployeeFilter {
    pub search: Option<String>,
    pub employee_type: Option<String>,
    pub department: Option<String>,
    pub org_unit_id: Option<Uuid>,
    pub status: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[async_trait]
pub trait EmployeeRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Employee>, DomainError>;
    async fn find_by_person_id(
        &self,
        org_id: Uuid,
        person_id: Uuid,
    ) -> Result<Option<Employee>, DomainError>;
    async fn find_by_user_id(
        &self,
        org_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<Employee>, DomainError>;
    async fn find_by_employee_number(
        &self,
        org_id: Uuid,
        employee_number: &str,
    ) -> Result<Option<Employee>, DomainError>;
    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<EmployeeDetail>, DomainError>;
    async fn list(
        &self,
        org_id: Uuid,
        filter: EmployeeFilter,
    ) -> Result<PaginatedList<EmployeeDetail>, DomainError>;
    async fn list_by_type(
        &self,
        org_id: Uuid,
        employee_type: &str,
    ) -> Result<Vec<Employee>, DomainError>;
    async fn count_by_status(
        &self,
        org_id: Uuid,
    ) -> Result<std::collections::HashMap<String, i64>, DomainError>;
    async fn save(&self, employee: &Employee) -> Result<(), DomainError>;
    async fn create_with_license_limit(
        &self,
        employee: &Employee,
        max_staff: Option<u64>,
    ) -> Result<(), DomainError> {
        if let Some(max_staff) = max_staff {
            let counts = self.count_by_status(employee.org_id).await?;
            let ended = counts.get("terminated").copied().unwrap_or(0)
                + counts.get("resigned").copied().unwrap_or(0);
            let current = counts
                .get("all")
                .copied()
                .unwrap_or(0)
                .saturating_sub(ended);
            if (current as u64).saturating_add(1) > max_staff {
                return Err(DomainError::LicenseRequired(format!(
                    "The staff quota of {max_staff} has been reached"
                )));
            }
        }
        self.save(employee).await
    }
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Group Repository Port
// =========================================================================

#[async_trait]
pub trait GroupRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DomainError>;
    async fn find_by_name(&self, org_id: Uuid, name: &str) -> Result<Option<Group>, DomainError>;
    async fn find_detail_by_id(&self, id: Uuid) -> Result<Option<GroupDetail>, DomainError>;
    async fn list_by_org(&self, org_id: Uuid) -> Result<Vec<GroupDetail>, DomainError>;
    async fn save(&self, group: &Group) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;

    // Membership
    async fn get_members(&self, group_id: Uuid) -> Result<Vec<GroupMemberInfo>, DomainError>;
    async fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<Group>, DomainError>;
    async fn add_member(&self, membership: &UserGroupMembership) -> Result<(), DomainError>;
    async fn remove_member(&self, user_id: Uuid, group_id: Uuid) -> Result<(), DomainError>;
    async fn bulk_add_members(
        &self,
        memberships: &[UserGroupMembership],
    ) -> Result<u64, DomainError>;
    async fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DomainError>;

    // Permissions
    async fn get_group_permissions(&self, group_id: Uuid) -> Result<Vec<String>, DomainError>;
    async fn set_group_permissions(
        &self,
        org_id: Uuid,
        group_id: Uuid,
        permissions: &[String],
    ) -> Result<(), DomainError>;
    async fn get_user_group_permissions(&self, user_id: Uuid) -> Result<Vec<String>, DomainError>;
}

// =========================================================================
// Document Requirement Repository Port
// =========================================================================

#[async_trait]
pub trait DocumentRequirementRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DocumentRequirement>, DomainError>;
    async fn find_by_org(&self, org_id: Uuid) -> Result<Vec<DocumentRequirement>, DomainError>;
    async fn find_by_target(
        &self,
        org_id: Uuid,
        target_type: &str,
        target_id: Option<Uuid>,
    ) -> Result<Vec<DocumentRequirement>, DomainError>;
    async fn save(&self, req: &DocumentRequirement) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}

// =========================================================================
// Sequence Counter Repository Port
// =========================================================================

#[async_trait]
pub trait SequenceCounterRepository: Send + Sync {
    async fn next_sequence(
        &self,
        org_id: Uuid,
        sequence_type: &str,
        academic_year: i32,
    ) -> Result<u64, DomainError>;
}

// =========================================================================
// Role & Permission Repository Port
// =========================================================================

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn find_by_id(&self, org_id: Uuid, role_id: &str) -> Result<Option<Role>, DomainError>;
    async fn list_by_org(&self, org_id: Uuid) -> Result<Vec<Role>, DomainError>;
    async fn count_by_org(&self, org_id: Uuid) -> Result<u64, DomainError>;
    async fn save_role(&self, role: &Role) -> Result<(), DomainError>;
    async fn save_role_permissions(
        &self,
        org_id: Uuid,
        role_id: &str,
        permissions: &[RolePermission],
    ) -> Result<(), DomainError>;
    async fn assign_role(&self, assignment: &UserRoleAssignment) -> Result<(), DomainError>;
    async fn find_assignment(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        role_id: &str,
    ) -> Result<Option<UserRoleAssignment>, DomainError>;
}
