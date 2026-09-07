use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    OrganizationCreated { org_id: Uuid, name: String, timestamp: DateTime<Utc> },
    OrganizationUpdated { org_id: Uuid, timestamp: DateTime<Utc> },
    OrgUnitCreated { unit_id: Uuid, org_id: Uuid, parent_id: Option<Uuid>, unit_type: String, timestamp: DateTime<Utc> },
    OrgUnitUpdated { unit_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    
    UserCreated { user_id: Uuid, person_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    UserLoggedIn { user_id: Uuid, org_id: Uuid, session_id: Uuid, timestamp: DateTime<Utc> },
    UserRegistered { user_id: Uuid, email: String, org_id: Option<Uuid> },
    UserEmailVerified { user_id: Uuid },

    AcademicYearCreated { year_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    AcademicPeriodCreated { period_id: Uuid, year_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    AcademicPeriodStateChanged { period_id: Uuid, org_id: Uuid, new_state: String, timestamp: DateTime<Utc> },
    
    SubjectCreated { subject_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    ClassCreated { class_id: Uuid, org_id: Uuid, period_id: Uuid, timestamp: DateTime<Utc> },
    ClassSubjectAssigned { class_subject_id: Uuid, class_id: Uuid, subject_id: Uuid, timestamp: DateTime<Utc> },

    StudentRegistered { student_id: Uuid, org_id: Uuid, student_number: String, timestamp: DateTime<Utc> },
    StudentEnrolled { enrollment_id: Uuid, student_id: Uuid, class_id: Option<Uuid>, org_id: Uuid, study_mode: String, timestamp: DateTime<Utc> },
    StudentStudyModeChanged { student_id: Uuid, old_mode: String, new_mode: String, timestamp: DateTime<Utc> },
    StudentStatusChanged { student_id: Uuid, old_status: String, new_status: String, timestamp: DateTime<Utc> },
    StudentWithdrawn { enrollment_id: Uuid, student_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    GuardianAssigned { student_guardian_id: Uuid, student_id: Uuid, guardian_id: Uuid, timestamp: DateTime<Utc> },
    CohortPromoted { job_id: Uuid, org_id: Uuid, promoted_count: u32, timestamp: DateTime<Utc> },


    RoomCreated { room_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    TimeSlotCreated { time_slot_id: Uuid, period_id: Uuid, timestamp: DateTime<Utc> },
    LessonScheduled { lesson_id: Uuid, class_subject_id: Uuid, time_slot_id: Uuid, timestamp: DateTime<Utc> },
    TimetableSolved { job_id: Uuid, period_id: Uuid, lessons_count: usize, timestamp: DateTime<Utc> },

    GradeRecorded { grade_id: Uuid, student_id: Uuid, class_subject_id: Uuid, score: Decimal, timestamp: DateTime<Utc> },
    GpaCalculated { student_id: Uuid, period_id: Uuid, gpa: Decimal, cgpa: Decimal, timestamp: DateTime<Utc> },
    AttendanceMarked { attendance_id: Uuid, student_id: Uuid, class_id: Option<Uuid>, class_subject_id: Option<Uuid>, scheduled_lesson_id: Option<Uuid>, status: String, timestamp: DateTime<Utc> },
    BehaviorLogged { record_id: Uuid, student_id: Uuid, record_type: String, points: i32, timestamp: DateTime<Utc> },

    ExamSessionCreated { session_id: Uuid, org_id: Uuid, timestamp: DateTime<Utc> },
    ExamScheduled { schedule_id: Uuid, session_id: Uuid, timestamp: DateTime<Utc> },
    SeatingNumberAssigned { student_id: Uuid, session_id: Uuid, seating_number: String, timestamp: DateTime<Utc> },

    StudentFeeGenerated { fee_id: Uuid, student_id: Uuid, enrollment_id: Option<Uuid>, net_amount: Decimal, timestamp: DateTime<Utc> },
    PaymentReceived { payment_id: Uuid, fee_id: Option<Uuid>, amount: Decimal, timestamp: DateTime<Utc> },
    JournalEntryPosted { entry_id: Uuid, amount: Decimal, timestamp: DateTime<Utc> },

    OverrideSubmitted { request_id: Uuid, org_id: Uuid, override_type: String, requested_by: Uuid, timestamp: DateTime<Utc> },
    OverrideApproved { request_id: Uuid, org_id: Uuid, approver_id: Uuid, timestamp: DateTime<Utc> },
    OverrideRejected { request_id: Uuid, org_id: Uuid, rejecter_id: Uuid, reason: String, timestamp: DateTime<Utc> },

    DocumentUploaded { document_id: Uuid, entity_type: Option<String>, entity_id: Option<Uuid>, timestamp: DateTime<Utc> },
    DocumentDeleted { document_id: Uuid, timestamp: DateTime<Utc> },
}
