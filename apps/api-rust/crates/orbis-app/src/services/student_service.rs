use orbis_domain::entities::student::{Student, StudentStatus, AcademicStanding, StudentDetail};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::enrollment::{Enrollment, StudyMode};
use orbis_domain::entities::student_guardian::StudentGuardian;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::{StudentFilter, EnrollmentFilter, PaginatedList};
use orbis_domain::iam::permission::Permission;
use crate::context::RequestContext;
use crate::ports::repositories::{StudentRepository, PersonRepository, EnrollmentRepository, StudentGuardianRepository};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

pub struct StudentService {
    pub student_repo: Arc<dyn StudentRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub guardian_repo: Arc<dyn StudentGuardianRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

#[derive(Debug, Clone)]
pub struct RegisterStudentInput {
    pub person_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    pub first_name: String,
    pub last_name: String,
    pub full_name: Option<String>,
    pub name_ar: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub initial_period_id: Option<Uuid>,
    pub initial_class_id: Option<Uuid>,
    pub study_mode: Option<StudyMode>,
}

impl StudentService {
    pub fn new(
        student_repo: Arc<dyn StudentRepository>,
        person_repo: Arc<dyn PersonRepository>,
        enrollment_repo: Arc<dyn EnrollmentRepository>,
        guardian_repo: Arc<dyn StudentGuardianRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            student_repo,
            person_repo,
            enrollment_repo,
            guardian_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Register a new student: creates/links Person, creates Student record, and optionally enrolls.
    pub async fn register_student(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: RegisterStudentInput,
    ) -> Result<StudentDetail, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentCreate).await?;

        // 1. Check if student number already exists in org
        if self.student_repo.find_by_student_number(org_id, &input.student_number).await?.is_some() {
            return Err(DomainError::Conflict(format!(
                "Student number '{}' is already assigned in this organization",
                input.student_number
            )));
        }

        let now = Utc::now();
        let full_name = input.full_name.unwrap_or_else(|| format!("{} {}", input.first_name, input.last_name));

        // 2. Resolve or create Person
        let person_id = if let Some(pid) = input.person_id {
            pid
        } else {
            let pid = Uuid::new_v4();
            let person = Person {
                id: pid,
                org_id: Some(org_id),
                first_name: input.first_name,
                last_name: input.last_name,
                full_name: Some(full_name.clone()),
                name_ar: input.name_ar,
                national_id: input.national_id,
                passport_number: input.passport_number,
                gender: input.gender,
                date_of_birth: input.date_of_birth,
                place_of_birth: input.place_of_birth,
                nationality: input.nationality,
                blood_type: input.blood_type,
                medical_notes: input.medical_notes,
                special_needs_notes: input.special_needs_notes,
                phone: input.phone,
                email: input.email,
                address: None,
                created_at: now,
                updated_at: now,
            };
            self.person_repo.save(&person).await?;
            pid
        };

        let person = self.person_repo.find_by_id(person_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Person {} not found", person_id)))?;

        // 3. Create Student profile
        let student_id = Uuid::new_v4();
        let student = Student {
            id: student_id,
            org_id,
            person_id,
            user_id: input.user_id,
            student_number: input.student_number.clone(),
            status: StudentStatus::Active,
            standing: AcademicStanding::GoodStanding,
            admitted_at: now,
            graduated_at: None,
            is_deleted: false,
            deleted_at: None,
            meta: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.student_repo.save(&student).await?;

        // 4. Optionally create initial enrollment
        if let Some(period_id) = input.initial_period_id {
            let mode = input.study_mode.unwrap_or(StudyMode::Regular);
            let enrollment = Enrollment::new(
                org_id,
                period_id,
                student_id,
                input.initial_class_id,
                input.student_number.clone(),
                mode,
            );
            self.enrollment_repo.save(&enrollment).await?;

            let _ = self.event_bus.publish(ctx, &DomainEvent::StudentEnrolled {
                enrollment_id: enrollment.id,
                student_id,
                class_id: input.initial_class_id,
                org_id,
                study_mode: mode.to_string(),
                timestamp: now,
            }).await;
        }

        // 5. Publish Domain Event
        let _ = self.event_bus.publish(ctx, &DomainEvent::StudentRegistered {
            student_id,
            org_id,
            student_number: input.student_number,
            timestamp: now,
        }).await;

        Ok(StudentDetail { student, person })
    }

    /// Enroll a student in an academic period/class
    pub async fn enroll_student(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        class_id: Option<Uuid>,
        student_number: String,
        study_mode: StudyMode,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let student = self.student_repo.find_by_id(student_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        if !student.can_enroll() {
            return Err(DomainError::Validation(format!(
                "Cannot enroll student with status {:?}",
                student.status
            )));
        }

        let now = Utc::now();
        let enrollment = Enrollment::new(
            org_id,
            period_id,
            student_id,
            class_id,
            student_number,
            study_mode,
        );

        self.enrollment_repo.save(&enrollment).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::StudentEnrolled {
            enrollment_id: enrollment.id,
            student_id,
            class_id,
            org_id,
            study_mode: study_mode.to_string(),
            timestamp: now,
        }).await;

        Ok(enrollment)
    }

    /// Update student overall status (Active, Suspended, Graduated, etc.)
    pub async fn update_student_status(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        new_status: StudentStatus,
        _reason: Option<String>,
    ) -> Result<Student, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let mut student = self.student_repo.find_by_id(student_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        let old_status = student.status;
        student.status = new_status;
        student.updated_at = Utc::now();

        if new_status == StudentStatus::Graduated {
            student.graduated_at = Some(Utc::now());
        }

        self.student_repo.save(&student).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::StudentStatusChanged {
            student_id,
            old_status: old_status.to_string(),
            new_status: new_status.to_string(),
            timestamp: Utc::now(),
        }).await;

        Ok(student)
    }

    /// Change study mode for an active enrollment
    pub async fn change_study_mode(
        &self,
        ctx: &RequestContext,
        enrollment_id: Uuid,
        new_mode: StudyMode,
        _reason: Option<String>,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let mut enrollment = self.enrollment_repo.find_by_id(enrollment_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Enrollment {} not found", enrollment_id)))?;

        let old_mode = enrollment.study_mode;
        enrollment.study_mode = new_mode;
        enrollment.is_attendance_exempt = new_mode.default_attendance_exempt();
        enrollment.is_grade_exempt = new_mode.default_grade_exempt();
        enrollment.updated_at = Utc::now();

        self.enrollment_repo.save(&enrollment).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::StudentStudyModeChanged {
            student_id: enrollment.student_id,
            old_mode: old_mode.to_string(),
            new_mode: new_mode.to_string(),
            timestamp: Utc::now(),
        }).await;

        Ok(enrollment)
    }

    /// Assign guardian to student
    pub async fn assign_guardian(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        guardian_id: Uuid,
        relationship: String,
        is_primary: bool,
    ) -> Result<StudentGuardian, DomainError> {
        self.permission_checker.check(ctx, Permission::GuardianWrite).await?;

        let now = Utc::now();
        let sg_id = Uuid::new_v4();

        let guardian = StudentGuardian {
            id: sg_id,
            student_id,
            guardian_id,
            relationship,
            relationship_ar: None,
            is_primary,
            is_emergency_contact: false,
            is_authorized_pickup: false,
            can_view_grades: true,
            can_view_attendance: true,
            can_view_behavior: true,
            can_view_finance: true,
            can_view_schedule: true,
            can_receive_notifications: true,
            is_payer: false,
            payment_share_percent: None,
            occupation: None,
            employer: None,
            work_phone: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.guardian_repo.save(&guardian).await?;

        let _ = self.event_bus.publish(ctx, &DomainEvent::GuardianAssigned {
            student_guardian_id: sg_id,
            student_id,
            guardian_id,
            timestamp: now,
        }).await;

        Ok(guardian)
    }

    /// Get student guardians
    pub async fn get_student_guardians(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<StudentGuardian>, DomainError> {
        self.permission_checker.check(ctx, Permission::GuardianRead).await?;
        self.guardian_repo.find_by_student(student_id).await
    }

    /// List students with pagination and filtering
    pub async fn list_students(
        &self,
        org_id: Uuid,
        filter: StudentFilter,
    ) -> Result<PaginatedList<StudentDetail>, DomainError> {
        self.student_repo.list(org_id, filter).await
    }

    /// Get student detail by ID
    pub async fn get_student(&self, id: Uuid) -> Result<Option<StudentDetail>, DomainError> {
        self.student_repo.find_detail_by_id(id).await
    }

    /// List enrollments with pagination and filtering
    pub async fn list_enrollments(
        &self,
        org_id: Uuid,
        filter: EnrollmentFilter,
    ) -> Result<PaginatedList<Enrollment>, DomainError> {
        self.enrollment_repo.list(org_id, filter).await
    }
}
