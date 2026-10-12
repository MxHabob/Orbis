use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    OrganizationCreated {
        org_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    OrganizationUpdated {
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    OrgUnitCreated {
        unit_id: Uuid,
        org_id: Uuid,
        parent_id: Option<Uuid>,
        unit_type: String,
        timestamp: DateTime<Utc>,
    },
    OrgUnitUpdated {
        unit_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    OrgUnitDeleted {
        unit_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    UserCreated {
        user_id: Uuid,
        person_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserLoggedIn {
        user_id: Uuid,
        org_id: Uuid,
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserRegistered {
        user_id: Uuid,
        email: String,
        org_id: Option<Uuid>,
    },
    UserEmailVerified {
        user_id: Uuid,
    },

    AcademicCalendarCreated {
        year_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    AcademicPeriodCreated {
        period_id: Uuid,
        year_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    AcademicPeriodStateChanged {
        period_id: Uuid,
        org_id: Uuid,
        new_state: String,
        timestamp: DateTime<Utc>,
    },

    SubjectCreated {
        subject_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ClassCreated {
        class_id: Uuid,
        org_id: Uuid,
        period_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    SectionSubjectAssigned {
        section_subject_id: Uuid,
        class_id: Uuid,
        subject_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    StudentRegistered {
        student_id: Uuid,
        org_id: Uuid,
        student_number: String,
        timestamp: DateTime<Utc>,
    },
    StudentEnrolled {
        enrollment_id: Uuid,
        student_id: Uuid,
        class_id: Option<Uuid>,
        org_id: Uuid,
        study_mode: String,
        timestamp: DateTime<Utc>,
    },
    StudentStudyModeChanged {
        student_id: Uuid,
        old_mode: String,
        new_mode: String,
        timestamp: DateTime<Utc>,
    },
    StudentStatusChanged {
        student_id: Uuid,
        old_status: String,
        new_status: String,
        timestamp: DateTime<Utc>,
    },
    StudentProfileUpdated {
        student_id: Uuid,
        person_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    StudentClassTransferred {
        student_id: Uuid,
        enrollment_id: Uuid,
        old_class_id: Option<Uuid>,
        new_class_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },
    StudentWithdrawn {
        enrollment_id: Uuid,
        student_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    GuardianAssigned {
        student_guardian_id: Uuid,
        student_id: Uuid,
        guardian_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    CohortPromoted {
        job_id: Uuid,
        org_id: Uuid,
        promoted_count: u32,
        timestamp: DateTime<Utc>,
    },

    RoomCreated {
        room_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    RoomUpdated {
        room_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    RoomDeleted {
        room_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TimeSlotCreated {
        time_slot_id: Uuid,
        period_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TimeSlotUpdated {
        time_slot_id: Uuid,
        period_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TimeSlotDeleted {
        time_slot_id: Uuid,
        period_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    LessonScheduled {
        lesson_id: Uuid,
        section_subject_id: Uuid,
        time_slot_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    LessonDeleted {
        lesson_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TimetableSolved {
        job_id: Uuid,
        period_id: Uuid,
        lessons_count: usize,
        timestamp: DateTime<Utc>,
    },

    AssessmentSchemeCreated {
        scheme_id: Uuid,
        org_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    AssessmentSchemeUpdated {
        scheme_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    AssessmentSchemeDeleted {
        scheme_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    GradeRecorded {
        grade_id: Uuid,
        student_id: Uuid,
        section_subject_id: Uuid,
        score: Decimal,
        timestamp: DateTime<Utc>,
    },
    GradeDeleted {
        grade_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    GpaCalculated {
        student_id: Uuid,
        period_id: Uuid,
        gpa: Decimal,
        cgpa: Decimal,
        timestamp: DateTime<Utc>,
    },
    AttendanceMarked {
        attendance_id: Uuid,
        student_id: Uuid,
        class_id: Option<Uuid>,
        section_subject_id: Option<Uuid>,
        scheduled_lesson_id: Option<Uuid>,
        status: String,
        timestamp: DateTime<Utc>,
    },
    AttendanceDeleted {
        attendance_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    BehaviorLogged {
        record_id: Uuid,
        student_id: Uuid,
        record_type: String,
        points: i32,
        timestamp: DateTime<Utc>,
    },
    BehaviorDeleted {
        record_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    ExamSessionCreated {
        session_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ExamSessionUpdated {
        session_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ExamSessionDeleted {
        session_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ExamScheduled {
        schedule_id: Uuid,
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ExamScheduleDeleted {
        schedule_id: Uuid,
        session_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    SeatingNumberAssigned {
        student_id: Uuid,
        session_id: Uuid,
        seating_number: String,
        timestamp: DateTime<Utc>,
    },
    SeatingNumberDeleted {
        seating_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    SecretNumberAssigned {
        secret_id: Uuid,
        schedule_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    SecretNumberDeleted {
        secret_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ProctorAssigned {
        proctor_id: Uuid,
        schedule_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ProctorDeleted {
        proctor_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    // Finance
    AccountCreated {
        account_id: Uuid,
        org_id: Uuid,
        code: String,
        timestamp: DateTime<Utc>,
    },
    AccountUpdated {
        account_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    AccountDeleted {
        account_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    FeeTemplateCreated {
        template_id: Uuid,
        org_id: Uuid,
        code: String,
        timestamp: DateTime<Utc>,
    },
    FeeTemplateUpdated {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    FeeTemplateDeleted {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DiscountTemplateCreated {
        template_id: Uuid,
        org_id: Uuid,
        code: String,
        timestamp: DateTime<Utc>,
    },
    DiscountTemplateUpdated {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DiscountTemplateDeleted {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    StudentFeeGenerated {
        fee_id: Uuid,
        student_id: Uuid,
        enrollment_id: Option<Uuid>,
        net_amount: Decimal,
        timestamp: DateTime<Utc>,
    },
    StudentFeeUpdated {
        fee_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    StudentFeeDeleted {
        fee_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    PaymentReceived {
        payment_id: Uuid,
        fee_id: Option<Uuid>,
        amount: Decimal,
        timestamp: DateTime<Utc>,
    },
    PaymentDeleted {
        payment_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    JournalEntryPosted {
        entry_id: Uuid,
        amount: Decimal,
        timestamp: DateTime<Utc>,
    },
    JournalEntryReversed {
        entry_id: Uuid,
        reversal_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    JournalEntryDeleted {
        entry_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    OverrideSubmitted {
        request_id: Uuid,
        org_id: Uuid,
        override_type: String,
        requested_by: Uuid,
        timestamp: DateTime<Utc>,
    },
    OverrideApproved {
        request_id: Uuid,
        org_id: Uuid,
        approver_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    OverrideRejected {
        request_id: Uuid,
        org_id: Uuid,
        rejecter_id: Uuid,
        reason: String,
        timestamp: DateTime<Utc>,
    },

    DocumentUploaded {
        document_id: Uuid,
        entity_type: Option<String>,
        entity_id: Option<Uuid>,
        timestamp: DateTime<Utc>,
    },
    DocumentUpdated {
        document_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DocumentDeleted {
        document_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DocumentArchived {
        document_id: Uuid,
        org_id: Uuid,
        reason: Option<String>,
        timestamp: DateTime<Utc>,
    },
    DocumentRestored {
        document_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DocumentPurged {
        document_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DocumentRequirementCreated {
        requirement_id: Uuid,
        org_id: Uuid,
        code: String,
        timestamp: DateTime<Utc>,
    },
    DocumentRequirementUpdated {
        requirement_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    DocumentRequirementDeleted {
        requirement_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },

    // --- Employee & Groups & Identity Provisioning ---
    EmployeeCreated {
        employee_id: Uuid,
        person_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    EmployeeUpdated {
        employee_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    EmployeeStatusChanged {
        employee_id: Uuid,
        old_status: String,
        new_status: String,
        timestamp: DateTime<Utc>,
    },
    EmployeeDeleted {
        employee_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    EmployeeRestored {
        employee_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    GroupCreated {
        group_id: Uuid,
        org_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    GroupUpdated {
        group_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    GroupDeleted {
        group_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserGroupAssigned {
        user_id: Uuid,
        group_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserGroupRemoved {
        user_id: Uuid,
        group_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    UserAccountProvisioned {
        user_id: Uuid,
        person_id: Uuid,
        org_id: Uuid,
        actor_type: String,
        timestamp: DateTime<Utc>,
    },
    UserAccountStatusChanged {
        user_id: Uuid,
        org_id: Uuid,
        old_status: String,
        new_status: String,
        timestamp: DateTime<Utc>,
    },
    UserPasswordReset {
        user_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    CredentialDeliveryAttempted {
        user_id: Uuid,
        channel: String,
        success: bool,
        timestamp: DateTime<Utc>,
    },

    // --- Integration ---
    IntegrationConnected {
        config_id: Uuid,
        org_id: Uuid,
        provider_name: String,
        timestamp: DateTime<Utc>,
    },
    IntegrationDisconnected {
        config_id: Uuid,
        org_id: Uuid,
        provider_name: String,
        timestamp: DateTime<Utc>,
    },
    IntegrationToggled {
        config_id: Uuid,
        org_id: Uuid,
        is_enabled: bool,
        timestamp: DateTime<Utc>,
    },

    // --- Reporting & Templates ---
    TemplateCreated {
        template_id: Uuid,
        org_id: Uuid,
        name: String,
        timestamp: DateTime<Utc>,
    },
    TemplateUpdated {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TemplateDeleted {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TemplatePublished {
        template_id: Uuid,
        org_id: Uuid,
        version: i32,
        timestamp: DateTime<Utc>,
    },
    TemplateArchived {
        template_id: Uuid,
        org_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    ReportGenerated {
        job_id: Uuid,
        org_id: Uuid,
        report_code: String,
        timestamp: DateTime<Utc>,
    },
}

impl DomainEvent {
    /// Maps a domain event to audit trail fields: `(action, entity_type, entity_id)`.
    ///
    /// Returns `None` for events that should not generate audit entries (e.g.,
    /// derived calculations, credential delivery attempts).
    pub fn audit_fields(&self) -> Option<(&'static str, &'static str, Uuid)> {
        match self {
            // Organization
            DomainEvent::OrganizationCreated { org_id, .. } => {
                Some(("organization.created", "organization", *org_id))
            }
            DomainEvent::OrganizationUpdated { org_id, .. } => {
                Some(("organization.updated", "organization", *org_id))
            }
            DomainEvent::OrgUnitCreated { unit_id, .. } => {
                Some(("org_unit.created", "org_unit", *unit_id))
            }
            DomainEvent::OrgUnitUpdated { unit_id, .. } => {
                Some(("org_unit.updated", "org_unit", *unit_id))
            }
            DomainEvent::OrgUnitDeleted { unit_id, .. } => {
                Some(("org_unit.deleted", "org_unit", *unit_id))
            }

            // User / Auth
            DomainEvent::UserCreated { user_id, .. } => Some(("user.created", "user", *user_id)),
            DomainEvent::UserLoggedIn { .. } => None, // Tracked via Session; not audit-worthy
            DomainEvent::UserRegistered { user_id, .. } => {
                Some(("user.registered", "user", *user_id))
            }
            DomainEvent::UserEmailVerified { user_id } => {
                Some(("user.email_verified", "user", *user_id))
            }

            // Academic
            DomainEvent::AcademicCalendarCreated { year_id, .. } => {
                Some(("academic_calendar.created", "academic_calendar", *year_id))
            }
            DomainEvent::AcademicPeriodCreated { period_id, .. } => {
                Some(("academic_period.created", "academic_period", *period_id))
            }
            DomainEvent::AcademicPeriodStateChanged { period_id, .. } => Some((
                "academic_period.state_changed",
                "academic_period",
                *period_id,
            )),
            DomainEvent::SubjectCreated { subject_id, .. } => {
                Some(("subject.created", "subject", *subject_id))
            }
            DomainEvent::ClassCreated { class_id, .. } => {
                Some(("class.created", "class", *class_id))
            }
            DomainEvent::SectionSubjectAssigned {
                section_subject_id, ..
            } => Some(("section_subject.assigned", "section_subject", *section_subject_id)),

            // Student
            DomainEvent::StudentRegistered { student_id, .. } => {
                Some(("student.created", "student", *student_id))
            }
            DomainEvent::StudentEnrolled { enrollment_id, .. } => {
                Some(("enrollment.created", "enrollment", *enrollment_id))
            }
            DomainEvent::StudentStudyModeChanged { student_id, .. } => {
                Some(("student.study_mode_changed", "student", *student_id))
            }
            DomainEvent::StudentStatusChanged { student_id, .. } => {
                Some(("student.status_changed", "student", *student_id))
            }
            DomainEvent::StudentProfileUpdated { student_id, .. } => {
                Some(("student.updated", "student", *student_id))
            }
            DomainEvent::StudentClassTransferred { enrollment_id, .. } => {
                Some(("enrollment.transferred", "enrollment", *enrollment_id))
            }
            DomainEvent::StudentWithdrawn { enrollment_id, .. } => {
                Some(("enrollment.withdrawn", "enrollment", *enrollment_id))
            }
            DomainEvent::GuardianAssigned {
                student_guardian_id,
                ..
            } => Some((
                "guardian.assigned",
                "student_guardian",
                *student_guardian_id,
            )),
            DomainEvent::CohortPromoted { job_id, .. } => {
                Some(("promotion.completed", "progression_job", *job_id))
            }

            // Scheduling
            DomainEvent::RoomCreated { room_id, .. } => Some(("room.created", "room", *room_id)),
            DomainEvent::RoomUpdated { room_id, .. } => Some(("room.updated", "room", *room_id)),
            DomainEvent::RoomDeleted { room_id, .. } => Some(("room.deleted", "room", *room_id)),
            DomainEvent::TimeSlotCreated { time_slot_id, .. } => {
                Some(("time_slot.created", "time_slot", *time_slot_id))
            }
            DomainEvent::TimeSlotUpdated { time_slot_id, .. } => {
                Some(("time_slot.updated", "time_slot", *time_slot_id))
            }
            DomainEvent::TimeSlotDeleted { time_slot_id, .. } => {
                Some(("time_slot.deleted", "time_slot", *time_slot_id))
            }
            DomainEvent::LessonScheduled { .. } => None, // Bulk operation — audit the solver job instead
            DomainEvent::LessonDeleted { lesson_id, .. } => {
                Some(("scheduled_lesson.deleted", "scheduled_lesson", *lesson_id))
            }
            DomainEvent::TimetableSolved { job_id, .. } => {
                Some(("timetable.solved", "solver_job", *job_id))
            }

            // Assessment
            DomainEvent::AssessmentSchemeCreated { scheme_id, .. } => {
                Some(("assessment_scheme.created", "assessment_scheme", *scheme_id))
            }
            DomainEvent::AssessmentSchemeUpdated { scheme_id, .. } => {
                Some(("assessment_scheme.updated", "assessment_scheme", *scheme_id))
            }
            DomainEvent::AssessmentSchemeDeleted { scheme_id, .. } => {
                Some(("assessment_scheme.deleted", "assessment_scheme", *scheme_id))
            }
            DomainEvent::GradeRecorded { grade_id, .. } => {
                Some(("grade.recorded", "grade", *grade_id))
            }
            DomainEvent::GradeDeleted { grade_id, .. } => {
                Some(("grade.deleted", "grade", *grade_id))
            }
            DomainEvent::GpaCalculated { .. } => None, // Derived calculation — no audit
            DomainEvent::AttendanceMarked { attendance_id, .. } => {
                Some(("attendance.marked", "attendance", *attendance_id))
            }
            DomainEvent::AttendanceDeleted { attendance_id, .. } => {
                Some(("attendance.deleted", "attendance", *attendance_id))
            }
            DomainEvent::BehaviorLogged { record_id, .. } => {
                Some(("behavior.logged", "behavior_record", *record_id))
            }
            DomainEvent::BehaviorDeleted { record_id, .. } => {
                Some(("behavior.deleted", "behavior_record", *record_id))
            }

            // Exams
            DomainEvent::ExamSessionCreated { session_id, .. } => {
                Some(("exam_session.created", "exam_session", *session_id))
            }
            DomainEvent::ExamSessionUpdated { session_id, .. } => {
                Some(("exam_session.updated", "exam_session", *session_id))
            }
            DomainEvent::ExamSessionDeleted { session_id, .. } => {
                Some(("exam_session.deleted", "exam_session", *session_id))
            }
            DomainEvent::ExamScheduled { schedule_id, .. } => {
                Some(("exam.scheduled", "exam_schedule", *schedule_id))
            }
            DomainEvent::ExamScheduleDeleted { schedule_id, .. } => {
                Some(("exam.schedule_deleted", "exam_schedule", *schedule_id))
            }
            DomainEvent::SeatingNumberAssigned { .. } => None, // Bulk operation
            DomainEvent::SeatingNumberDeleted { seating_id, .. } => {
                Some(("exam.seating_deleted", "seating_number", *seating_id))
            }
            DomainEvent::SecretNumberAssigned { secret_id, .. } => {
                Some(("exam.secret_assigned", "secret_number", *secret_id))
            }
            DomainEvent::SecretNumberDeleted { secret_id, .. } => {
                Some(("exam.secret_deleted", "secret_number", *secret_id))
            }
            DomainEvent::ProctorAssigned { proctor_id, .. } => {
                Some(("exam.proctor_assigned", "exam_proctor", *proctor_id))
            }
            DomainEvent::ProctorDeleted { proctor_id, .. } => {
                Some(("exam.proctor_deleted", "exam_proctor", *proctor_id))
            }

            // Finance
            DomainEvent::AccountCreated { account_id, .. } => {
                Some(("account.created", "account", *account_id))
            }
            DomainEvent::AccountUpdated { account_id, .. } => {
                Some(("account.updated", "account", *account_id))
            }
            DomainEvent::AccountDeleted { account_id, .. } => {
                Some(("account.deleted", "account", *account_id))
            }
            DomainEvent::FeeTemplateCreated { template_id, .. } => {
                Some(("fee_template.created", "fee_template", *template_id))
            }
            DomainEvent::FeeTemplateUpdated { template_id, .. } => {
                Some(("fee_template.updated", "fee_template", *template_id))
            }
            DomainEvent::FeeTemplateDeleted { template_id, .. } => {
                Some(("fee_template.deleted", "fee_template", *template_id))
            }
            DomainEvent::DiscountTemplateCreated { template_id, .. } => Some((
                "discount_template.created",
                "discount_template",
                *template_id,
            )),
            DomainEvent::DiscountTemplateUpdated { template_id, .. } => Some((
                "discount_template.updated",
                "discount_template",
                *template_id,
            )),
            DomainEvent::DiscountTemplateDeleted { template_id, .. } => Some((
                "discount_template.deleted",
                "discount_template",
                *template_id,
            )),
            DomainEvent::StudentFeeGenerated { fee_id, .. } => {
                Some(("student_fee.generated", "student_fee", *fee_id))
            }
            DomainEvent::StudentFeeUpdated { fee_id, .. } => {
                Some(("student_fee.updated", "student_fee", *fee_id))
            }
            DomainEvent::StudentFeeDeleted { fee_id, .. } => {
                Some(("student_fee.deleted", "student_fee", *fee_id))
            }
            DomainEvent::PaymentReceived { payment_id, .. } => {
                Some(("payment.received", "payment", *payment_id))
            }
            DomainEvent::PaymentDeleted { payment_id, .. } => {
                Some(("payment.deleted", "payment", *payment_id))
            }
            DomainEvent::JournalEntryPosted { entry_id, .. } => {
                Some(("journal_entry.posted", "journal_entry", *entry_id))
            }
            DomainEvent::JournalEntryReversed { entry_id, .. } => {
                Some(("journal_entry.reversed", "journal_entry", *entry_id))
            }
            DomainEvent::JournalEntryDeleted { entry_id, .. } => {
                Some(("journal_entry.deleted", "journal_entry", *entry_id))
            }

            // Overrides
            DomainEvent::OverrideSubmitted { request_id, .. } => {
                Some(("override.submitted", "override_request", *request_id))
            }
            DomainEvent::OverrideApproved { request_id, .. } => {
                Some(("override.approved", "override_request", *request_id))
            }
            DomainEvent::OverrideRejected { request_id, .. } => {
                Some(("override.rejected", "override_request", *request_id))
            }

            // Documents
            DomainEvent::DocumentUploaded { document_id, .. } => {
                Some(("document.uploaded", "document", *document_id))
            }
            DomainEvent::DocumentUpdated { document_id, .. } => {
                Some(("document.updated", "document", *document_id))
            }
            DomainEvent::DocumentDeleted { document_id, .. } => {
                Some(("document.deleted", "document", *document_id))
            }
            DomainEvent::DocumentArchived { document_id, .. } => {
                Some(("document.archived", "document", *document_id))
            }
            DomainEvent::DocumentRestored { document_id, .. } => {
                Some(("document.restored", "document", *document_id))
            }
            DomainEvent::DocumentPurged { document_id, .. } => {
                Some(("document.purged", "document", *document_id))
            }
            DomainEvent::DocumentRequirementCreated { requirement_id, .. } => Some((
                "document_requirement.created",
                "document_requirement",
                *requirement_id,
            )),
            DomainEvent::DocumentRequirementUpdated { requirement_id, .. } => Some((
                "document_requirement.updated",
                "document_requirement",
                *requirement_id,
            )),
            DomainEvent::DocumentRequirementDeleted { requirement_id, .. } => Some((
                "document_requirement.deleted",
                "document_requirement",
                *requirement_id,
            )),

            // Employee & Groups
            DomainEvent::EmployeeCreated { employee_id, .. } => {
                Some(("employee.created", "employee", *employee_id))
            }
            DomainEvent::EmployeeUpdated { employee_id, .. } => {
                Some(("employee.updated", "employee", *employee_id))
            }
            DomainEvent::EmployeeStatusChanged { employee_id, .. } => {
                Some(("employee.status_changed", "employee", *employee_id))
            }
            DomainEvent::EmployeeDeleted { employee_id, .. } => {
                Some(("employee.deleted", "employee", *employee_id))
            }
            DomainEvent::EmployeeRestored { employee_id, .. } => {
                Some(("employee.restored", "employee", *employee_id))
            }
            DomainEvent::GroupCreated { group_id, .. } => {
                Some(("group.created", "group", *group_id))
            }
            DomainEvent::GroupUpdated { group_id, .. } => {
                Some(("group.updated", "group", *group_id))
            }
            DomainEvent::GroupDeleted { group_id, .. } => {
                Some(("group.deleted", "group", *group_id))
            }
            DomainEvent::UserGroupAssigned { group_id, .. } => {
                Some(("user_group.assigned", "group", *group_id))
            }
            DomainEvent::UserGroupRemoved { group_id, .. } => {
                Some(("user_group.removed", "group", *group_id))
            }
            DomainEvent::UserAccountProvisioned { user_id, .. } => {
                Some(("user.provisioned", "user", *user_id))
            }
            DomainEvent::UserAccountStatusChanged { user_id, .. } => {
                Some(("user.status_changed", "user", *user_id))
            }
            DomainEvent::UserPasswordReset { user_id, .. } => {
                Some(("user.password_reset", "user", *user_id))
            }
            DomainEvent::CredentialDeliveryAttempted { .. } => None, // Operational — not an auditable entity change

            // Integration
            DomainEvent::IntegrationConnected { config_id, .. } => {
                Some(("integration.connected", "integration_config", *config_id))
            }
            DomainEvent::IntegrationDisconnected { config_id, .. } => {
                Some(("integration.disconnected", "integration_config", *config_id))
            }
            DomainEvent::IntegrationToggled { config_id, .. } => {
                Some(("integration.toggled", "integration_config", *config_id))
            }

            // Reporting & Templates
            DomainEvent::TemplateCreated { template_id, .. } => {
                Some(("template.created", "report_template", *template_id))
            }
            DomainEvent::TemplateUpdated { template_id, .. } => {
                Some(("template.updated", "report_template", *template_id))
            }
            DomainEvent::TemplateDeleted { template_id, .. } => {
                Some(("template.deleted", "report_template", *template_id))
            }
            DomainEvent::TemplatePublished { template_id, .. } => {
                Some(("template.published", "report_template", *template_id))
            }
            DomainEvent::TemplateArchived { template_id, .. } => {
                Some(("template.archived", "report_template", *template_id))
            }
            DomainEvent::ReportGenerated { job_id, .. } => {
                Some(("report.generated", "report_job", *job_id))
            }
        }
    }
}
