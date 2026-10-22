use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::license::LicenseRuntime;
use crate::ports::repositories::{
    AcademicCalendarRepository, AcademicPeriodRepository, EnrollmentRepository,
    FeeTemplateRepository, OrganizationRepository, OverrideRequestRepository, PersonRepository,
    SectionRepository, SequenceCounterRepository, SettingsRepository, StudentFeeRepository,
    StudentGuardianRepository, StudentRepository,
};
use chrono::{Datelike, NaiveDate, Utc};
use orbis_domain::entities::enrollment::{Enrollment, EnrollmentStatus, StudyMode};
use orbis_domain::entities::organization::InstitutionType;
use orbis_domain::entities::override_request::{
    AcademicOverrideRequest, OverrideStatus, OverrideType, RiskLevel,
};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::student::{AcademicStanding, Student, StudentDetail, StudentStatus};
use orbis_domain::entities::student_fee::{FeeStatus, StudentFee};
use orbis_domain::entities::student_guardian::StudentGuardian;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use orbis_domain::queries::filters::{EnrollmentFilter, PaginatedList, StudentFilter};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct StudentService {
    pub student_repo: Arc<dyn StudentRepository>,
    pub person_repo: Arc<dyn PersonRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub org_repo: Arc<dyn OrganizationRepository>,
    pub calendar_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub guardian_repo: Arc<dyn StudentGuardianRepository>,
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub sequence_repo: Arc<dyn SequenceCounterRepository>,
    pub section_repo: Arc<dyn SectionRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub override_repo: Arc<dyn OverrideRequestRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
    license_runtime: Option<Arc<dyn LicenseRuntime>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterStudentInput {
    pub person_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub student_number: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
    pub initial_period_id: Option<Uuid>,
    pub initial_section_id: Option<Uuid>,
    pub study_mode: Option<StudyMode>,
    pub bypass_capacity_check: Option<bool>,
    #[serde(default)]
    pub is_draft: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterStudentResult {
    pub student: StudentDetail,
    pub enrollment: Option<Enrollment>,
    pub override_request: Option<AcademicOverrideRequest>,
    pub auto_assigned_fees_count: usize,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct DuplicateCheckInput {
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub exclude_student_id: Option<Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateMatchItem {
    pub field: String, // "national_id", "phone", "email"
    pub value: String,
    pub matched_person_id: Uuid,
    pub matched_person_name: String,
    pub student_id: Option<Uuid>,
    pub student_number: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateCheckResult {
    pub has_duplicate: bool,
    pub matches: Vec<DuplicateMatchItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkRegisterItemInput {
    pub row_index: usize,
    pub registration: RegisterStudentInput,
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub guardian_relationship: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkRegisterItemResult {
    pub row_index: usize,
    pub success: bool,
    pub student_id: Option<Uuid>,
    pub student_number: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkRegisterStudentsInput {
    pub items: Vec<BulkRegisterItemInput>,
    pub atomic: Option<bool>,
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkRegisterStudentResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub is_atomic: bool,
    pub is_dry_run: bool,
    pub items: Vec<BulkRegisterItemResult>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct UpdateStudentProfileInput {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
}

impl StudentService {
    pub fn new(
        student_repo: Arc<dyn StudentRepository>,
        person_repo: Arc<dyn PersonRepository>,
        enrollment_repo: Arc<dyn EnrollmentRepository>,
        org_repo: Arc<dyn OrganizationRepository>,
        calendar_repo: Arc<dyn AcademicCalendarRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        guardian_repo: Arc<dyn StudentGuardianRepository>,
        settings_repo: Arc<dyn SettingsRepository>,
        sequence_repo: Arc<dyn SequenceCounterRepository>,
        section_repo: Arc<dyn SectionRepository>,
        fee_template_repo: Arc<dyn FeeTemplateRepository>,
        student_fee_repo: Arc<dyn StudentFeeRepository>,
        override_repo: Arc<dyn OverrideRequestRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            student_repo,
            person_repo,
            enrollment_repo,
            org_repo,
            calendar_repo,
            period_repo,
            guardian_repo,
            settings_repo,
            sequence_repo,
            section_repo,
            fee_template_repo,
            student_fee_repo,
            override_repo,
            event_bus,
            permission_checker,
            license_runtime: None,
        }
    }

    pub fn with_license_runtime(mut self, runtime: Arc<dyn LicenseRuntime>) -> Self {
        self.license_runtime = Some(runtime);
        self
    }

    /// Check for duplicate student/person by national_id, phone, or email
    pub async fn check_duplicate(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        input: DuplicateCheckInput,
    ) -> Result<DuplicateCheckResult, DomainError> {
        let mut matches = Vec::new();

        if let Some(nid) = input.national_id.as_deref() {
            let clean_nid = nid.trim();
            if !clean_nid.is_empty() {
                if let Some(person) = self
                    .person_repo
                    .find_by_national_id(Some(org_id), clean_nid)
                    .await?
                {
                    if input.exclude_student_id.is_none() {
                        let student = self.student_repo.find_by_person_id(person.id).await?;
                        matches.push(DuplicateMatchItem {
                            field: "national_id".to_string(),
                            value: clean_nid.to_string(),
                            matched_person_id: person.id,
                            matched_person_name: person.full_name(),
                            student_id: student.as_ref().map(|s| s.id),
                            student_number: student.as_ref().map(|s| s.student_number.clone()),
                        });
                    }
                }
            }
        }

        if let Some(phone) = input.phone.as_deref() {
            let clean_phone = phone.trim();
            if !clean_phone.is_empty() {
                if let Some(person) = self
                    .person_repo
                    .find_by_phone(Some(org_id), clean_phone)
                    .await?
                {
                    let student = self.student_repo.find_by_person_id(person.id).await?;
                    if student.as_ref().map(|s| s.id) != input.exclude_student_id {
                        matches.push(DuplicateMatchItem {
                            field: "phone".to_string(),
                            value: clean_phone.to_string(),
                            matched_person_id: person.id,
                            matched_person_name: person.full_name(),
                            student_id: student.as_ref().map(|s| s.id),
                            student_number: student.as_ref().map(|s| s.student_number.clone()),
                        });
                    }
                }
            }
        }

        if let Some(email) = input.email.as_deref() {
            let clean_email = email.trim();
            if !clean_email.is_empty() {
                if let Some(person) = self
                    .person_repo
                    .find_by_email(Some(org_id), clean_email)
                    .await?
                {
                    let student = self.student_repo.find_by_person_id(person.id).await?;
                    if student.as_ref().map(|s| s.id) != input.exclude_student_id {
                        matches.push(DuplicateMatchItem {
                            field: "email".to_string(),
                            value: clean_email.to_string(),
                            matched_person_id: person.id,
                            matched_person_name: person.full_name(),
                            student_id: student.as_ref().map(|s| s.id),
                            student_number: student.as_ref().map(|s| s.student_number.clone()),
                        });
                    }
                }
            }
        }

        let has_duplicate = !matches.is_empty();
        Ok(DuplicateCheckResult {
            has_duplicate,
            matches,
        })
    }

    /// Register a new student: creates/links Person, creates Student record, evaluates admission policies and enrolls.
    pub async fn register_student(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: RegisterStudentInput,
    ) -> Result<RegisterStudentResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentCreate)
            .await?;
        let max_students = if let Some(runtime) = self.license_runtime.as_ref() {
            runtime.authorize(org_id, Some("students"))?;
            let max = runtime.get_limit("students.max");
            if max.is_some() {
                let current = self.student_repo.find_by_org(org_id).await?.len() as u64;
                runtime.check_limit("students.max", current, 1)?;
            }
            max
        } else {
            None
        };

        let now = Utc::now();
        let is_draft = input.is_draft;
        let org_settings = self
            .settings_repo
            .get_settings(org_id)
            .await
            .unwrap_or_default();
        let admission_cfg = org_settings.admission;
        let numbering_schema = admission_cfg.numbering;
        let admission_policy = admission_cfg.policy;

        // 1. Enforce unique national ID policy if configured
        if !admission_policy.allow_duplicate_national_id {
            if let Some(nid) = input.national_id.as_deref() {
                let clean_nid = nid.trim();
                if !clean_nid.is_empty() {
                    if let Some(existing_person) = self
                        .person_repo
                        .find_by_national_id(Some(org_id), clean_nid)
                        .await?
                    {
                        return Err(DomainError::Conflict(format!(
                            "A person with national ID '{}' already exists (Name: '{}')",
                            clean_nid,
                            existing_person.full_name()
                        )));
                    }
                }
            }
        }

        // 2. Resolve or generate Student Number via Numbering Schema + Sequence Generator
        let student_number = if let Some(sn) = input
            .student_number
            .as_deref()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
        {
            sn.to_string()
        } else {
            let current_year = now.year();
            let seq = self
                .sequence_repo
                .next_sequence(org_id, "student", current_year)
                .await?;
            numbering_schema.format_number(current_year, seq)
        };

        if self
            .student_repo
            .find_by_student_number(org_id, &student_number)
            .await?
            .is_some()
        {
            return Err(DomainError::Conflict(format!(
                "Student number '{}' is already assigned in this organization",
                student_number
            )));
        }

        // 3. Resolve or create Person record
        let person_id = if let Some(pid) = input.person_id {
            pid
        } else {
            let pid = Uuid::new_v4();
            let person = Person {
                id: pid,
                org_id: Some(org_id),
                first_name: input.first_name,
                middle_name: input.middle_name,
                last_name: input.last_name,
                first_name_ar: input.first_name_ar,
                middle_name_ar: input.middle_name_ar,
                last_name_ar: input.last_name_ar,
                display_name: input.display_name,
                national_id: input.national_id,
                passport_number: input.passport_number,
                gender: input.gender,
                birth_date: input.birth_date,
                place_of_birth: input.place_of_birth,
                nationality: input.nationality,
                blood_type: input.blood_type,
                marital_status: None,
                medical_notes: input.medical_notes,
                special_needs_notes: input.special_needs_notes,
                phone: input.phone,
                email: input.email,
                address: input.address,
                avatar_url: input.avatar_url,
                created_at: now,
                updated_at: now,
            };
            self.person_repo.save(&person).await?;
            pid
        };

        let person = self
            .person_repo
            .find_by_id(person_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Person {} not found", person_id)))?;

        // 4. Create Student profile
        let student_id = Uuid::new_v4();
        let student = Student {
            id: student_id,
            org_id,
            person_id,
            user_id: input.user_id,
            student_number: student_number.clone(),
            status: if is_draft {
                StudentStatus::Inactive
            } else {
                StudentStatus::Active
            },
            standing: AcademicStanding::GoodStanding,
            admitted_at: now,
            graduated_at: None,
            is_deleted: false,
            deleted_at: None,
            meta: serde_json::json!({}),
            org_unit_id: None,
            created_by: ctx.user_id(),
            updated_by: ctx.user_id(),
            created_at: now,
            updated_at: now,
        };

        self.student_repo
            .create_with_license_limit(&student, max_students)
            .await?;

        // 5. Optionally create initial enrollment & evaluate policy / capacity
        let mut created_enrollment: Option<Enrollment> = None;
        let mut created_override_req: Option<AcademicOverrideRequest> = None;
        let mut auto_assigned_fees_count = 0usize;

        // 5. Resolve enrollment period
        // For schools: auto-resolve the Annual period from the active calendar
        // For universities/institutes: use the explicitly provided period_id
        let resolved_period_id = if let Some(pid) = input.initial_period_id {
            Some(pid)
        } else if !is_draft {
            // Auto-resolve for schools: find the Annual period in the active calendar
            let org = self.org_repo.find_by_id(org_id).await?;
            let institution_type = org
                .as_ref()
                .map(|o| InstitutionType::from_str_loose(&o.institution_type))
                .unwrap_or(InstitutionType::School);
            if institution_type == InstitutionType::School {
                if let Some(calendar) = self.calendar_repo.find_current(org_id).await? {
                    let periods = self.period_repo.find_by_year(calendar.id).await?;
                    periods
                        .into_iter()
                        .find(|p| p.period_type.is_annual())
                        .map(|p| p.id)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(period_id) = resolved_period_id.filter(|_| !is_draft) {
            let mode = input.study_mode.unwrap_or(StudyMode::Regular);
            let mut enrollment_status = EnrollmentStatus::Active;

            // Check capacity invariant
            if let Some(class_id) = input.initial_section_id {
                let current_enrollments = self.enrollment_repo.find_by_section(class_id).await?;
                let active_count = current_enrollments
                    .iter()
                    .filter(|e| e.status == EnrollmentStatus::Active)
                    .count() as i32;

                let max_cap = if let Ok(Some(cls)) = self.section_repo.find_by_id(class_id).await {
                    cls.capacity.unwrap_or(admission_policy.max_class_capacity)
                } else {
                    admission_policy.max_class_capacity
                };

                if active_count >= max_cap && input.bypass_capacity_check != Some(true) {
                    // Capacity breach: Create CapacityOverride request and hold enrollment
                    let req_id = Uuid::new_v4();
                    let override_req = AcademicOverrideRequest {
                        id: req_id,
                        org_id,
                        override_type: OverrideType::CapacityOverride,
                        risk_level: RiskLevel::Medium,
                        entity_type: "enrollment".to_string(),
                        entity_id: student_id,
                        field_name: Some("class_id".to_string()),
                        old_value: None,
                        new_value: Some(class_id.to_string()),
                        reason: format!(
                            "Class capacity reached ({}/{}). Override required to add student.",
                            active_count, max_cap
                        ),
                        requested_by: ctx.user_id().unwrap_or(student_id),
                        requested_by_name: None,
                        status: OverrideStatus::Pending,
                        current_approver_role: Some("admin".to_string()),
                        approval_chain: vec![],
                        resolved_by: None,
                        resolved_by_name: None,
                        resolved_at: None,
                        resolution_notes: None,
                        expires_at: None,
                        metadata: serde_json::json!({
                            "class_id": class_id,
                            "active_count": active_count,
                            "max_capacity": max_cap
                        }),
                        created_at: now,
                        updated_at: now,
                    };
                    self.override_repo.save(&override_req).await?;
                    created_override_req = Some(override_req);
                    enrollment_status = EnrollmentStatus::PendingApproval;
                }
            }

            // Check approval policy
            if admission_policy.requires_approval && enrollment_status == EnrollmentStatus::Active {
                enrollment_status = EnrollmentStatus::PendingApproval;
                let req_id = Uuid::new_v4();
                let override_req = AcademicOverrideRequest {
                    id: req_id,
                    org_id,
                    override_type: OverrideType::AdmissionApproval,
                    risk_level: RiskLevel::Low,
                    entity_type: "enrollment".to_string(),
                    entity_id: student_id,
                    field_name: None,
                    old_value: None,
                    new_value: None,
                    reason: "New applicant registration requires administrative review and verification.".to_string(),
                    requested_by: ctx.user_id().unwrap_or(student_id),
                    requested_by_name: None,
                    status: OverrideStatus::Pending,
                    current_approver_role: Some("registrar".to_string()),
                    approval_chain: vec![],
                    resolved_by: None,
                    resolved_by_name: None,
                    resolved_at: None,
                    resolution_notes: None,
                    expires_at: None,
                    metadata: serde_json::json!({
                        "period_id": period_id,
                        "class_id": input.initial_section_id
                    }),
                    created_at: now,
                    updated_at: now,
                };
                self.override_repo.save(&override_req).await?;
                created_override_req = Some(override_req);
            }

            let mut enrollment = Enrollment::new(
                org_id,
                period_id,
                student_id,
                input.initial_section_id,
                student_number.clone(),
                mode,
            );
            enrollment.status = enrollment_status;
            self.enrollment_repo.save(&enrollment).await?;

            // 6. Auto-assign tuition fee templates if enabled and enrollment is active
            if admission_policy.auto_assign_fees && enrollment_status == EnrollmentStatus::Active {
                if let Ok(templates) = self.fee_template_repo.find_by_org(org_id).await {
                    for tpl in templates
                        .into_iter()
                        .filter(|t| t.is_mandatory && t.is_active)
                    {
                        let student_fee = StudentFee {
                            id: Uuid::new_v4(),
                            org_id,
                            period_id,
                            student_id,
                            enrollment_id: Some(enrollment.id),
                            fee_template_id: tpl.id,
                            original_amount: tpl.default_amount,
                            discount_amount: rust_decimal::Decimal::ZERO,
                            net_amount: tpl.default_amount,
                            paid_amount: rust_decimal::Decimal::ZERO,
                            balance_due: tpl.default_amount,
                            status: FeeStatus::Pending,
                            due_date: now.date_naive(),
                            discount_details: None,
                            installment_number: Some(1),
                            notes: Some(format!("Auto-assigned mandatory fee: {}", tpl.name)),
                            created_by: ctx.user_id(),
                            created_at: now,
                            updated_at: now,
                        };
                        let _ = self.student_fee_repo.save(&student_fee).await;
                        auto_assigned_fees_count += 1;
                    }
                }
            }

            let _ = self
                .event_bus
                .publish(
                    ctx,
                    &DomainEvent::StudentEnrolled {
                        enrollment_id: enrollment.id,
                        student_id,
                        section_id: input.initial_section_id,
                        org_id,
                        study_mode: mode.to_string(),
                        timestamp: now,
                    },
                )
                .await;

            created_enrollment = Some(enrollment);
        }

        // 7. Publish Domain Event
        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::StudentRegistered {
                    student_id,
                    org_id,
                    student_number,
                    timestamp: now,
                },
            )
            .await;

        Ok(RegisterStudentResult {
            student: StudentDetail { student, person },
            enrollment: created_enrollment,
            override_request: created_override_req,
            auto_assigned_fees_count,
        })
    }

    /// Enroll an existing student in an academic period/class
    pub async fn enroll_student(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        section_id: Option<Uuid>,
        student_number: String,
        study_mode: StudyMode,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        let completing_draft = student.status == StudentStatus::Inactive;
        if !student.can_enroll() && !completing_draft {
            return Err(DomainError::Validation(format!(
                "Cannot enroll student with status {:?}",
                student.status
            )));
        }

        let existing_student_enrollments = self.enrollment_repo.find_by_student(student_id).await?;
        if let Some(existing) = existing_student_enrollments.iter().find(|enrollment| {
            enrollment.period_id == period_id
                && matches!(
                    enrollment.status,
                    EnrollmentStatus::Active | EnrollmentStatus::PendingApproval
                )
        }) {
            if completing_draft {
                return Ok(existing.clone());
            }
            return Err(DomainError::Conflict(
                "Student is already actively enrolled in this period".into(),
            ));
        }

        let now = Utc::now();
        let org_settings = self
            .settings_repo
            .get_settings(org_id)
            .await
            .unwrap_or_default();
        let admission_policy = org_settings.admission.policy;
        let allow_waitlist =
            org_settings.enrollment.allow_waitlist || admission_policy.allow_waitlist;
        let requires_approval =
            org_settings.enrollment.require_admin_approval || admission_policy.requires_approval;
        let max_class_capacity = org_settings.enrollment.max_class_size_default.max(0);
        let mut enrollment_status = EnrollmentStatus::Active;

        if completing_draft {
            if !org_settings.enrollment.registration_enabled {
                return Err(DomainError::Validation(
                    "Student registration is disabled by organization policy".to_string(),
                ));
            }

            if let Some(class_id) = section_id {
                let enrollments = self.enrollment_repo.find_by_section(class_id).await?;
                let active_count = enrollments
                    .iter()
                    .filter(|enrollment| enrollment.status == EnrollmentStatus::Active)
                    .count() as i32;
                let capacity = self
                    .section_repo
                    .find_by_id(class_id)
                    .await?
                    .and_then(|section| section.capacity)
                    .unwrap_or(max_class_capacity);

                if active_count >= capacity {
                    if !allow_waitlist {
                        return Err(DomainError::Validation(format!(
                            "Class capacity exceeded ({active_count}/{capacity})"
                        )));
                    }

                    let request = AcademicOverrideRequest {
                        id: Uuid::new_v4(),
                        org_id,
                        override_type: OverrideType::CapacityOverride,
                        risk_level: RiskLevel::Medium,
                        entity_type: "enrollment".to_string(),
                        entity_id: student_id,
                        field_name: Some("class_id".to_string()),
                        old_value: None,
                        new_value: Some(class_id.to_string()),
                        reason: format!(
                            "Class capacity reached ({active_count}/{capacity}); review required."
                        ),
                        requested_by: ctx.user_id().unwrap_or(student_id),
                        requested_by_name: None,
                        status: OverrideStatus::Pending,
                        current_approver_role: Some("admin".to_string()),
                        approval_chain: vec![],
                        resolved_by: None,
                        resolved_by_name: None,
                        resolved_at: None,
                        resolution_notes: None,
                        expires_at: None,
                        metadata: serde_json::json!({
                            "class_id": class_id,
                            "active_count": active_count,
                            "max_capacity": capacity
                        }),
                        created_at: now,
                        updated_at: now,
                    };
                    self.override_repo.save(&request).await?;
                    enrollment_status = EnrollmentStatus::PendingApproval;
                }
            }

            if requires_approval {
                enrollment_status = EnrollmentStatus::PendingApproval;
                let request = AcademicOverrideRequest {
                    id: Uuid::new_v4(),
                    org_id,
                    override_type: OverrideType::AdmissionApproval,
                    risk_level: RiskLevel::Low,
                    entity_type: "enrollment".to_string(),
                    entity_id: student_id,
                    field_name: None,
                    old_value: None,
                    new_value: None,
                    reason: "Student completed registration and requires administrative review."
                        .to_string(),
                    requested_by: ctx.user_id().unwrap_or(student_id),
                    requested_by_name: None,
                    status: OverrideStatus::Pending,
                    current_approver_role: Some("registrar".to_string()),
                    approval_chain: vec![],
                    resolved_by: None,
                    resolved_by_name: None,
                    resolved_at: None,
                    resolution_notes: None,
                    expires_at: None,
                    metadata: serde_json::json!({
                        "period_id": period_id,
                        "class_id": section_id
                    }),
                    created_at: now,
                    updated_at: now,
                };
                self.override_repo.save(&request).await?;
            }
        }

        let mut enrollment = Enrollment::new(
            org_id,
            period_id,
            student_id,
            section_id,
            student_number,
            study_mode,
        );
        enrollment.status = enrollment_status;

        self.enrollment_repo.save(&enrollment).await?;

        if completing_draft
            && admission_policy.auto_assign_fees
            && enrollment_status == EnrollmentStatus::Active
        {
            if let Ok(templates) = self.fee_template_repo.find_by_org(org_id).await {
                for template in templates
                    .into_iter()
                    .filter(|template| template.is_mandatory && template.is_active)
                {
                    let fee = StudentFee {
                        id: Uuid::new_v4(),
                        org_id,
                        period_id,
                        student_id,
                        enrollment_id: Some(enrollment.id),
                        fee_template_id: template.id,
                        original_amount: template.default_amount,
                        discount_amount: rust_decimal::Decimal::ZERO,
                        net_amount: template.default_amount,
                        paid_amount: rust_decimal::Decimal::ZERO,
                        balance_due: template.default_amount,
                        status: FeeStatus::Pending,
                        due_date: now.date_naive(),
                        discount_details: None,
                        installment_number: Some(1),
                        notes: Some(format!("Auto-assigned mandatory fee: {}", template.name)),
                        created_by: ctx.user_id(),
                        created_at: now,
                        updated_at: now,
                    };
                    let _ = self.student_fee_repo.save(&fee).await;
                }
            }
        }

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::StudentEnrolled {
                    enrollment_id: enrollment.id,
                    student_id,
                    section_id,
                    org_id,
                    study_mode: study_mode.to_string(),
                    timestamp: now,
                },
            )
            .await;

        Ok(enrollment)
    }

    /// Update student overall status
    pub async fn update_student_status(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        new_status: StudentStatus,
        _reason: Option<String>,
    ) -> Result<Student, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let mut student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        let old_status = student.status;
        student.status = new_status;
        student.updated_at = Utc::now();

        if new_status == StudentStatus::Graduated {
            student.graduated_at = Some(Utc::now());
        }

        self.student_repo.save(&student).await?;

        let _ = self
            .event_bus
            .publish(
                ctx,
                &DomainEvent::StudentStatusChanged {
                    student_id,
                    old_status: old_status.to_string(),
                    new_status: new_status.to_string(),
                    timestamp: Utc::now(),
                },
            )
            .await;

        Ok(student)
    }

    /// Get student detailed profile by ID
    pub async fn get_student(
        &self,
        student_id: Uuid,
    ) -> Result<Option<StudentDetail>, DomainError> {
        let student = match self.student_repo.find_by_id(student_id).await? {
            Some(s) => s,
            None => return Ok(None),
        };

        let person = self
            .person_repo
            .find_by_id(student.person_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Person {} not found for student {}",
                    student.person_id, student_id
                ))
            })?;

        Ok(Some(StudentDetail { student, person }))
    }

    /// Get student detailed profile with context (for auth/permission checks if needed)
    pub async fn get_student_detail(
        &self,
        _ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Option<StudentDetail>, DomainError> {
        self.get_student(student_id).await
    }

    /// List students with pagination & filters returning StudentDetail items
    pub async fn list_students(
        &self,
        org_id: Uuid,
        filter: StudentFilter,
    ) -> Result<PaginatedList<StudentDetail>, DomainError> {
        self.student_repo.list(org_id, filter).await
    }

    /// List enrollments with filters
    pub async fn list_enrollments(
        &self,
        org_id: Uuid,
        filter: EnrollmentFilter,
    ) -> Result<PaginatedList<Enrollment>, DomainError> {
        self.enrollment_repo.list(org_id, filter).await
    }

    /// Transfer student enrollment to another class
    pub async fn transfer_class(
        &self,
        ctx: &RequestContext,
        enrollment_id: Uuid,
        target_section_id: Uuid,
        _reason: Option<String>,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let mut enrollment = self
            .enrollment_repo
            .find_by_id(enrollment_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Enrollment {} not found", enrollment_id))
            })?;

        enrollment.section_id = Some(target_section_id);
        enrollment.updated_at = Utc::now();

        self.enrollment_repo.save(&enrollment).await?;
        Ok(enrollment)
    }

    /// Change study mode for an enrollment
    pub async fn change_study_mode(
        &self,
        ctx: &RequestContext,
        enrollment_id: Uuid,
        new_mode: StudyMode,
        _reason: Option<String>,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let mut enrollment = self
            .enrollment_repo
            .find_by_id(enrollment_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Enrollment {} not found", enrollment_id))
            })?;

        enrollment.study_mode = new_mode;
        enrollment.updated_at = Utc::now();

        self.enrollment_repo.save(&enrollment).await?;
        Ok(enrollment)
    }

    /// Assign guardian
    pub async fn assign_guardian(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        guardian_id: Uuid,
        relationship: String,
        is_primary: bool,
    ) -> Result<StudentGuardian, DomainError> {
        self.assign_guardian_full(
            ctx,
            student_id,
            guardian_id,
            relationship,
            None,
            is_primary,
            Some(true),
            Some(false),
            Some(is_primary),
            None,
            None,
            None,
            None,
            None,
        )
        .await
    }

    /// Assign guardian with full options
    pub async fn assign_guardian_full(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        guardian_id: Uuid,
        relationship: String,
        relationship_ar: Option<String>,
        is_primary: bool,
        is_emergency_contact: Option<bool>,
        is_authorized_pickup: Option<bool>,
        is_payer: Option<bool>,
        payment_share_percent: Option<f64>,
        occupation: Option<String>,
        employer: Option<String>,
        work_phone: Option<String>,
        notes: Option<String>,
    ) -> Result<StudentGuardian, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let now = Utc::now();
        let sg = StudentGuardian {
            id: Uuid::new_v4(),
            student_id,
            guardian_id,
            relationship,
            relationship_ar,
            is_primary,
            is_emergency_contact: is_emergency_contact.unwrap_or(true),
            is_authorized_pickup: is_authorized_pickup.unwrap_or(false),
            is_payer: is_payer.unwrap_or(is_primary),
            payment_share_percent,
            occupation,
            employer,
            work_phone,
            notes,
            created_at: now,
            updated_at: now,
        };

        self.guardian_repo.save(&sg).await?;
        Ok(sg)
    }

    /// Get student guardians
    pub async fn get_guardians(
        &self,
        _ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<StudentGuardian>, DomainError> {
        self.guardian_repo.find_by_student(student_id).await
    }

    /// Get student guardians alias
    pub async fn get_student_guardians(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<StudentGuardian>, DomainError> {
        self.get_guardians(ctx, student_id).await
    }

    /// Remove guardian linkage
    pub async fn remove_guardian(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        guardian_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;
        self.guardian_repo.delete(student_id, guardian_id).await
    }

    /// Update profile
    pub async fn update_student_profile(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        input: UpdateStudentProfileInput,
    ) -> Result<StudentDetail, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;

        let student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        let mut person = self
            .person_repo
            .find_by_id(student.person_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Person {} not found", student.person_id))
            })?;

        if let Some(v) = input.first_name {
            person.first_name = v;
        }
        if let Some(v) = input.middle_name {
            person.middle_name = Some(v);
        }
        if let Some(v) = input.last_name {
            person.last_name = v;
        }
        if let Some(v) = input.first_name_ar {
            person.first_name_ar = Some(v);
        }
        if let Some(v) = input.middle_name_ar {
            person.middle_name_ar = Some(v);
        }
        if let Some(v) = input.last_name_ar {
            person.last_name_ar = Some(v);
        }
        if let Some(v) = input.display_name {
            person.display_name = Some(v);
        }
        if let Some(v) = input.national_id {
            person.national_id = Some(v);
        }
        if let Some(v) = input.passport_number {
            person.passport_number = Some(v);
        }
        if let Some(v) = input.gender {
            person.gender = Some(v);
        }
        if let Some(v) = input.birth_date {
            person.birth_date = Some(v);
        }
        if let Some(v) = input.place_of_birth {
            person.place_of_birth = Some(v);
        }
        if let Some(v) = input.nationality {
            person.nationality = Some(v);
        }
        if let Some(v) = input.blood_type {
            person.blood_type = Some(v);
        }
        if let Some(v) = input.medical_notes {
            person.medical_notes = Some(v);
        }
        if let Some(v) = input.special_needs_notes {
            person.special_needs_notes = Some(v);
        }
        if let Some(v) = input.phone {
            person.phone = Some(v);
        }
        if let Some(v) = input.email {
            person.email = Some(v);
        }
        if let Some(v) = input.address {
            person.address = Some(v);
        }
        if let Some(v) = input.avatar_url {
            person.avatar_url = Some(v);
        }
        person.updated_at = Utc::now();

        self.person_repo.save(&person).await?;
        Ok(StudentDetail { student, person })
    }

    /// Soft delete student
    pub async fn soft_delete_student(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentDelete)
            .await?;

        let mut student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        student.is_deleted = true;
        student.deleted_at = Some(Utc::now());
        student.updated_at = Utc::now();

        self.student_repo.save(&student).await
    }

    /// Restore student
    pub async fn restore_student(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Student, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentDelete)
            .await?;

        let mut student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        student.is_deleted = false;
        student.deleted_at = None;
        student.updated_at = Utc::now();

        self.student_repo.save(&student).await?;
        Ok(student)
    }

    /// Bulk register students with full multi-stage validation, capacity simulation,
    /// settings enforcement, sequential numbering generation, guardian linkage, and atomic all-or-nothing execution.
    pub async fn bulk_register_students(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: BulkRegisterStudentsInput,
    ) -> Result<BulkRegisterStudentResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentCreate)
            .await?;

        let is_atomic = input.atomic.unwrap_or(true);
        let is_dry_run = input.dry_run.unwrap_or(false);
        let items = input.items;
        let total = items.len();

        let org_settings = self
            .settings_repo
            .get_settings(org_id)
            .await
            .unwrap_or_default();
        let admission_policy = org_settings.admission.policy;

        // ─────────────────────────────────────────────────────────────────
        // STAGE 1: INTRA-BATCH INTEGRITY & CONFLICT PRE-VALIDATION
        // ─────────────────────────────────────────────────────────────────
        let mut pre_errors: HashMap<usize, String> = HashMap::new();
        let mut seen_nids: HashMap<String, usize> = HashMap::new();
        let mut seen_passports: HashMap<String, usize> = HashMap::new();
        let mut seen_student_nums: HashMap<String, usize> = HashMap::new();
        let mut class_allocated_count: HashMap<Uuid, i32> = HashMap::new();
        let mut class_capacity_cache: HashMap<Uuid, (i32, i32)> = HashMap::new(); // class_id -> (current_active, max_capacity)

        for item in &items {
            let row = item.row_index;

            // 1. Required Demographics Check
            if item.registration.first_name.trim().is_empty() {
                pre_errors.insert(row, "First name is required".to_string());
                continue;
            }
            if item.registration.last_name.trim().is_empty() {
                pre_errors.insert(row, "Last name is required".to_string());
                continue;
            }

            // 2. Intra-Batch Duplicate National ID Check
            if let Some(nid) = item
                .registration
                .national_id
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(prev_row) = seen_nids.get(nid) {
                    pre_errors.insert(
                        row,
                        format!(
                            "Duplicate National ID '{}' within batch (conflicts with row {})",
                            nid, prev_row
                        ),
                    );
                    continue;
                }
                seen_nids.insert(nid.to_string(), row);

                // DB Conflict Check if duplicates not allowed
                if !admission_policy.allow_duplicate_national_id {
                    if let Ok(Some(existing_person)) = self
                        .person_repo
                        .find_by_national_id(Some(org_id), nid)
                        .await
                    {
                        pre_errors.insert(row, format!("A person with National ID '{}' already exists in organization (Name: '{}')", nid, existing_person.full_name()));
                        continue;
                    }
                }
            }

            // 3. Intra-Batch Duplicate Passport Check
            if let Some(pp) = item
                .registration
                .passport_number
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(prev_row) = seen_passports.get(pp) {
                    pre_errors.insert(
                        row,
                        format!(
                            "Duplicate Passport Number '{}' within batch (conflicts with row {})",
                            pp, prev_row
                        ),
                    );
                    continue;
                }
                seen_passports.insert(pp.to_string(), row);
            }

            // 4. Intra-Batch Explicit Student Number Check
            if let Some(sn) = item
                .registration
                .student_number
                .as_deref()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                if let Some(prev_row) = seen_student_nums.get(sn) {
                    pre_errors.insert(
                        row,
                        format!(
                            "Duplicate Student Number '{}' within batch (conflicts with row {})",
                            sn, prev_row
                        ),
                    );
                    continue;
                }
                seen_student_nums.insert(sn.to_string(), row);

                if let Ok(Some(_)) = self.student_repo.find_by_student_number(org_id, sn).await {
                    pre_errors.insert(
                        row,
                        format!("Student number '{}' already exists in organization", sn),
                    );
                    continue;
                }
            }

            // 5. Guardian Policy Check
            if !item.registration.is_draft && !admission_policy.allow_no_guardian {
                let has_guardian = item
                    .guardian_name
                    .as_deref()
                    .map(|s| !s.trim().is_empty())
                    .unwrap_or(false);
                if !has_guardian {
                    pre_errors.insert(
                        row,
                        "Guardian name is required by organization admission policy".to_string(),
                    );
                    continue;
                }
            }

            // 6. Cumulative Class Capacity & Multi-Enrollment Simulation
            if !item.registration.is_draft {
                if let Some(class_id) = item.registration.initial_section_id {
                    let (current_active, max_cap) = if let Some(cached) =
                        class_capacity_cache.get(&class_id)
                    {
                        *cached
                    } else {
                        let active = self
                            .enrollment_repo
                            .find_by_section(class_id)
                            .await
                            .map(|list| {
                                list.into_iter()
                                    .filter(|e| e.status == EnrollmentStatus::Active)
                                    .count() as i32
                            })
                            .unwrap_or(0);
                        let max_c =
                            if let Ok(Some(cls)) = self.section_repo.find_by_id(class_id).await {
                                cls.capacity.unwrap_or(admission_policy.max_class_capacity)
                            } else {
                                admission_policy.max_class_capacity
                            };
                        class_capacity_cache.insert(class_id, (active, max_c));
                        (active, max_c)
                    };

                    let allocated = class_allocated_count.entry(class_id).or_insert(0);
                    *allocated += 1;

                    if (current_active + *allocated) > max_cap
                        && item.registration.bypass_capacity_check != Some(true)
                    {
                        if is_atomic && !admission_policy.allow_waitlist {
                            pre_errors.insert(row, format!(
                            "Class capacity exceeded ({}/{} max). Total active: {}, requested in batch: {}",
                            current_active + *allocated, max_cap, current_active, *allocated
                        ));
                        }
                    }
                }
            }
        }

        // ─────────────────────────────────────────────────────────────────
        // STAGE 2: ATOMIC ALL-OR-NOTHING / DRY-RUN DECISION GATE
        // ─────────────────────────────────────────────────────────────────
        if !pre_errors.is_empty() && is_atomic {
            let mut results = Vec::with_capacity(total);
            for item in &items {
                let row = item.row_index;
                if let Some(err) = pre_errors.get(&row) {
                    results.push(BulkRegisterItemResult {
                        row_index: row,
                        success: false,
                        student_id: None,
                        student_number: None,
                        error: Some(err.clone()),
                    });
                } else {
                    results.push(BulkRegisterItemResult {
                        row_index: row,
                        success: false,
                        student_id: None,
                        student_number: None,
                        error: Some("Batch registration aborted: another record in the batch failed validation (All-or-Nothing policy).".to_string()),
                    });
                }
            }

            return Ok(BulkRegisterStudentResult {
                total,
                succeeded: 0,
                failed: total,
                is_atomic: true,
                is_dry_run,
                items: results,
            });
        }

        if is_dry_run {
            let mut succeeded = 0;
            let mut failed = 0;
            let mut results = Vec::with_capacity(total);
            for item in &items {
                let row = item.row_index;
                if let Some(err) = pre_errors.get(&row) {
                    failed += 1;
                    results.push(BulkRegisterItemResult {
                        row_index: row,
                        success: false,
                        student_id: None,
                        student_number: None,
                        error: Some(err.clone()),
                    });
                } else {
                    succeeded += 1;
                    results.push(BulkRegisterItemResult {
                        row_index: row,
                        success: true,
                        student_id: None,
                        student_number: item
                            .registration
                            .student_number
                            .clone()
                            .or_else(|| Some("[Auto-Generated Preview]".to_string())),
                        error: None,
                    });
                }
            }

            return Ok(BulkRegisterStudentResult {
                total,
                succeeded,
                failed,
                is_atomic,
                is_dry_run: true,
                items: results,
            });
        }

        // ─────────────────────────────────────────────────────────────────
        // STAGE 3: SEQUENTIAL EXECUTION & PERSISTENCE
        // ─────────────────────────────────────────────────────────────────
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut results = Vec::with_capacity(total);

        for item in items {
            let row_idx = item.row_index;

            if let Some(err) = pre_errors.get(&row_idx) {
                failed += 1;
                results.push(BulkRegisterItemResult {
                    row_index: row_idx,
                    success: false,
                    student_id: None,
                    student_number: None,
                    error: Some(err.clone()),
                });
                continue;
            }

            match self.register_student(ctx, org_id, item.registration).await {
                Ok(reg_res) => {
                    let stu_id = reg_res.student.student.id;
                    let stu_num = reg_res.student.student.student_number.clone();

                    // Optional guardian linkage if provided
                    if let Some(ref g_name) = item.guardian_name {
                        if !g_name.trim().is_empty() {
                            let g_phone = item.guardian_phone.as_deref().unwrap_or("").trim();
                            let mut guardian_person_id = None;
                            if !g_phone.is_empty() {
                                if let Ok(Some(existing_p)) =
                                    self.person_repo.find_by_phone(Some(org_id), g_phone).await
                                {
                                    guardian_person_id = Some(existing_p.id);
                                }
                            }
                            let g_pid = if let Some(pid) = guardian_person_id {
                                pid
                            } else {
                                let new_pid = Uuid::new_v4();
                                let now = Utc::now();
                                let g_person = Person {
                                    id: new_pid,
                                    org_id: Some(org_id),
                                    first_name: g_name.clone(),
                                    middle_name: None,
                                    last_name: "".to_string(),
                                    first_name_ar: Some(g_name.clone()),
                                    middle_name_ar: None,
                                    last_name_ar: None,
                                    display_name: Some(g_name.clone()),
                                    national_id: None,
                                    passport_number: None,
                                    gender: None,
                                    birth_date: None,
                                    place_of_birth: None,
                                    nationality: None,
                                    blood_type: None,
                                    marital_status: None,
                                    medical_notes: None,
                                    special_needs_notes: None,
                                    phone: if g_phone.is_empty() {
                                        None
                                    } else {
                                        Some(g_phone.to_string())
                                    },
                                    email: None,
                                    address: None,
                                    avatar_url: None,
                                    created_at: now,
                                    updated_at: now,
                                };
                                let _ = self.person_repo.save(&g_person).await;
                                new_pid
                            };

                            let sg = StudentGuardian {
                                id: Uuid::new_v4(),
                                student_id: stu_id,
                                guardian_id: g_pid,
                                relationship: item
                                    .guardian_relationship
                                    .unwrap_or_else(|| "guardian".to_string()),
                                relationship_ar: None,
                                is_primary: true,
                                is_emergency_contact: true,
                                is_authorized_pickup: true,
                                is_payer: true,
                                payment_share_percent: None,
                                occupation: None,
                                employer: None,
                                work_phone: None,
                                notes: None,
                                created_at: Utc::now(),
                                updated_at: Utc::now(),
                            };
                            let _ = self.guardian_repo.save(&sg).await;
                        }
                    }

                    succeeded += 1;
                    results.push(BulkRegisterItemResult {
                        row_index: row_idx,
                        success: true,
                        student_id: Some(stu_id),
                        student_number: Some(stu_num),
                        error: None,
                    });
                }
                Err(err) => {
                    failed += 1;
                    results.push(BulkRegisterItemResult {
                        row_index: row_idx,
                        success: false,
                        student_id: None,
                        student_number: None,
                        error: Some(err.to_string()),
                    });
                }
            }
        }

        Ok(BulkRegisterStudentResult {
            total,
            succeeded,
            failed,
            is_atomic,
            is_dry_run: false,
            items: results,
        })
    }
}
