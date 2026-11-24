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
use orbis_domain::entities::settings::OrganizationSettings;
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
    enrollment_lock: Arc<tokio::sync::Mutex<()>>,
}

fn capacity_override_request(
    org_id: Uuid,
    student_id: Uuid,
    section_id: Uuid,
    requested_by: Uuid,
    now: chrono::DateTime<Utc>,
) -> AcademicOverrideRequest {
    AcademicOverrideRequest {
        id: Uuid::new_v4(),
        org_id,
        override_type: OverrideType::CapacityOverride,
        risk_level: RiskLevel::Medium,
        entity_type: "enrollment".to_string(),
        entity_id: student_id,
        field_name: Some("class_id".to_string()),
        old_value: None,
        new_value: Some(section_id.to_string()),
        reason: "Class capacity reached; review required.".to_string(),
        requested_by,
        requested_by_name: None,
        status: OverrideStatus::Pending,
        current_approver_role: Some("admin".to_string()),
        approval_chain: vec![],
        resolved_by: None,
        resolved_by_name: None,
        resolved_at: None,
        resolution_notes: None,
        expires_at: None,
        metadata: serde_json::json!({ "class_id": section_id }),
        created_at: now,
        updated_at: now,
    }
}

fn admission_override_request(
    org_id: Uuid,
    student_id: Uuid,
    period_id: Uuid,
    section_id: Option<Uuid>,
    requested_by: Uuid,
    reason: &str,
    now: chrono::DateTime<Utc>,
) -> AcademicOverrideRequest {
    AcademicOverrideRequest {
        id: Uuid::new_v4(),
        org_id,
        override_type: OverrideType::AdmissionApproval,
        risk_level: RiskLevel::Low,
        entity_type: "enrollment".to_string(),
        entity_id: student_id,
        field_name: None,
        old_value: None,
        new_value: None,
        reason: reason.to_string(),
        requested_by,
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
    }
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
    pub full_name: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub exclude_student_id: Option<Uuid>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateMatchItem {
    pub field: String, // "national_id", "phone", "email", "full_name"
    pub value: String,
    pub matched_person_id: Uuid,
    pub matched_person_name: String,
    pub student_id: Option<Uuid>,
    pub student_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkTransferStudentItem {
    pub enrollment_id: Uuid,
    pub target_section_id: Uuid,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkTransferStudentItemResult {
    pub enrollment_id: Uuid,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkTransferStudentResult {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub items: Vec<BulkTransferStudentItemResult>,
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

fn normalize_name_for_match(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_space = false;
    for ch in input.chars() {
        // remove tatweel and Arabic harakat
        if matches!(ch, '\u{0640}' | '\u{064B}'..='\u{065F}' | '\u{0670}') {
            continue;
        }
        let mapped = match ch {
            'أ' | 'إ' | 'آ' | 'ٱ' => 'ا',
            'ى' => 'ي',
            'ؤ' => 'و',
            'ئ' => 'ي',
            'ة' => 'ه',
            c => c,
        };
        for lc in mapped.to_lowercase() {
            if lc.is_whitespace() {
                if !prev_space {
                    out.push(' ');
                    prev_space = true;
                }
            } else {
                out.push(lc);
                prev_space = false;
            }
        }
    }
    out.trim().to_string()
}

fn compose_full_name(
    first: &str,
    middle: Option<&str>,
    last: &str,
) -> Option<String> {
    let f = first.trim();
    let l = last.trim();
    if f.is_empty() && l.is_empty() {
        return None;
    }
    let m = middle.map(str::trim).filter(|v| !v.is_empty());
    Some(match m {
        Some(mid) if !f.is_empty() || !l.is_empty() => format!("{} {} {}", f, mid, l).trim().to_string(),
        _ => format!("{} {}", f, l).trim().to_string(),
    })
}

fn candidate_name_keys(
    full_name: Option<&str>,
    first: Option<&str>,
    middle: Option<&str>,
    last: Option<&str>,
    first_ar: Option<&str>,
    middle_ar: Option<&str>,
    last_ar: Option<&str>,
) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(v) = full_name.and_then(|s| {
        let t = s.trim();
        if t.is_empty() { None } else { Some(t) }
    }) {
        let k = normalize_name_for_match(v);
        if !k.is_empty() {
            keys.push(k);
        }
    }
    if let (Some(f), Some(l)) = (first, last) {
        if let Some(name) = compose_full_name(f, middle, l) {
            let k = normalize_name_for_match(&name);
            if !k.is_empty() {
                keys.push(k);
            }
        }
    }
    if let (Some(f), Some(l)) = (first_ar, last_ar) {
        if let Some(name) = compose_full_name(f, middle_ar, l) {
            let k = normalize_name_for_match(&name);
            if !k.is_empty() {
                keys.push(k);
            }
        }
    }
    keys.sort();
    keys.dedup();
    keys
}

fn student_name_keys(detail: &StudentDetail) -> Vec<String> {
    let p = &detail.person;
    let mut keys = Vec::new();
    let full = p.full_name();
    let k = normalize_name_for_match(&full);
    if !k.is_empty() {
        keys.push(k);
    }
    if let Some(ar) = p.full_name_ar() {
        let k = normalize_name_for_match(&ar);
        if !k.is_empty() {
            keys.push(k);
        }
    }
    if let Some(name) = compose_full_name(&p.first_name, p.middle_name.as_deref(), &p.last_name) {
        let k = normalize_name_for_match(&name);
        if !k.is_empty() {
            keys.push(k);
        }
    }
    if let (Some(f), Some(l)) = (p.first_name_ar.as_deref(), p.last_name_ar.as_deref()) {
        if let Some(name) = compose_full_name(f, p.middle_name_ar.as_deref(), l) {
            let k = normalize_name_for_match(&name);
            if !k.is_empty() {
                keys.push(k);
            }
        }
    }
    keys.sort();
    keys.dedup();
    keys
}

fn collect_name_duplicate_matches(
    candidates: &[StudentDetail],
    input: &DuplicateCheckInput,
    exclude_student_id: Option<Uuid>,
) -> Vec<DuplicateMatchItem> {
    let keys = candidate_name_keys(
        input.full_name.as_deref(),
        input.first_name.as_deref(),
        input.middle_name.as_deref(),
        input.last_name.as_deref(),
        input.first_name_ar.as_deref(),
        input.middle_name_ar.as_deref(),
        input.last_name_ar.as_deref(),
    );
    if keys.is_empty() {
        return Vec::new();
    }
    let mut matches = Vec::new();
    for existing in candidates {
        if exclude_student_id == Some(existing.student.id) {
            continue;
        }
        let existing_keys = student_name_keys(existing);
        let matched_key = keys.iter().find(|k| existing_keys.contains(k));
        if let Some(key) = matched_key {
            let same_birth_date = match (input.birth_date, existing.person.birth_date) {
                (Some(a), Some(b)) => a == b,
                _ => false,
            };
            let (confidence, reason) = if same_birth_date {
                (
                    "high".to_string(),
                    "Existing student has the same full name and date of birth in this organization".to_string(),
                )
            } else {
                (
                    "medium".to_string(),
                    "Existing student has the same full name in this organization. Verify identity before creating another record".to_string(),
                )
            };
            let matched_person_name = existing
                .person
                .full_name_ar()
                .unwrap_or_else(|| existing.person.full_name());
            matches.push(DuplicateMatchItem {
                field: "full_name".to_string(),
                value: key.clone(),
                matched_person_id: existing.person.id,
                matched_person_name,
                student_id: Some(existing.student.id),
                student_number: Some(existing.student.student_number.clone()),
                reason: Some(reason),
                confidence: Some(confidence),
            });
        }
    }
    matches
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
            enrollment_lock: Arc::new(tokio::sync::Mutex::new(())),
        }
    }

    pub fn with_license_runtime(mut self, runtime: Arc<dyn LicenseRuntime>) -> Self {
        self.license_runtime = Some(runtime);
        self
    }

    async fn find_student_in_org(
        &self,
        student_id: Uuid,
        org_id: Uuid,
    ) -> Result<Student, DomainError> {
        let student = self
            .student_repo
            .find_by_id(student_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;
        if student.org_id != org_id {
            return Err(DomainError::NotFound("Student not found".into()));
        }
        Ok(student)
    }

    async fn find_person_in_org(
        &self,
        person_id: Uuid,
        org_id: Uuid,
    ) -> Result<Person, DomainError> {
        let person = self
            .person_repo
            .find_by_id(person_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Person {} not found", person_id)))?;
        if person.org_id != Some(org_id) {
            return Err(DomainError::NotFound("Person not found".into()));
        }
        Ok(person)
    }

    /// Check for duplicate student/person by national_id, phone, or email
    pub async fn check_duplicate(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: DuplicateCheckInput,
    ) -> Result<DuplicateCheckResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;
        if ctx.tenant_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

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
                            reason: None,
                            confidence: None,
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
                            reason: None,
                            confidence: None,
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
                            reason: None,
                            confidence: None,
                        });
                    }
                }
            }
        }

        // Full-name duplicate/suspected-duplicate detection (institutional safety)
        let org_settings = self
            .settings_repo
            .get_settings(org_id)
            .await
            .unwrap_or_default();
        if org_settings.admission.policy.prevent_duplicate_full_name {
            let existing_details = self.student_repo.find_active_details_by_org(org_id).await?;
            let name_matches =
                collect_name_duplicate_matches(&existing_details, &input, input.exclude_student_id);
            matches.extend(name_matches);
        }

        // de-duplicate identical matches (same field/person/student)
        matches.sort_by(|a, b| {
            (
                &a.field,
                &a.matched_person_id,
                &a.student_id,
                &a.value,
            )
                .cmp(&(
                    &b.field,
                    &b.matched_person_id,
                    &b.student_id,
                    &b.value,
                ))
        });
        matches.dedup_by(|a, b| {
            a.field == b.field
                && a.matched_person_id == b.matched_person_id
                && a.student_id == b.student_id
                && a.value == b.value
        });

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
        if ctx.tenant_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }
        let _enrollment_guard = self.enrollment_lock.lock().await;
        self.register_student_unlocked(ctx, org_id, input, None)
            .await
    }

    async fn register_student_unlocked(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        input: RegisterStudentInput,
        known_settings: Option<&OrganizationSettings>,
    ) -> Result<RegisterStudentResult, DomainError> {
        let max_students = if let Some(runtime) = self.license_runtime.as_ref() {
            runtime.authorize(org_id, Some("students"))?;
            // The repository checks the quota under a database lock when inserting.
            runtime.get_limit("students.max")
        } else {
            None
        };

        let now = Utc::now();
        let is_draft = input.is_draft;
        let org_settings = match known_settings {
            Some(settings) => settings.clone(),
            None => self
                .settings_repo
                .get_settings(org_id)
                .await
                .unwrap_or_default(),
        };
        let admission_cfg = org_settings.admission;
        let numbering_schema = admission_cfg.numbering;
        let admission_policy = admission_cfg.policy;

        // 1. Enforce duplicate-identity policies
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

        if admission_policy.prevent_duplicate_full_name {
            let existing_details = self.student_repo.find_active_details_by_org(org_id).await?;
            let dup_input = DuplicateCheckInput {
                full_name: input.display_name.clone(),
                first_name: Some(input.first_name.clone()),
                middle_name: input.middle_name.clone(),
                last_name: Some(input.last_name.clone()),
                first_name_ar: input.first_name_ar.clone(),
                middle_name_ar: input.middle_name_ar.clone(),
                last_name_ar: input.last_name_ar.clone(),
                birth_date: input.birth_date,
                exclude_student_id: None,
                ..Default::default()
            };
            if let Some(matched) = collect_name_duplicate_matches(&existing_details, &dup_input, None).into_iter().next() {
                return Err(DomainError::Conflict(format!(
                    "A student with the same full name already exists in this organization ({} - {}). Please verify the student identity before creating a new record.",
                    matched.student_number.unwrap_or_else(|| "existing record".to_string()),
                    matched.matched_person_name
                )));
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
        if person.org_id != Some(org_id) {
            return Err(DomainError::NotFound("Person not found".into()));
        }

        if let Some(existing_student) = self.student_repo.find_by_person_id(person_id).await? {
            if existing_student.org_id == org_id {
                return Err(DomainError::Conflict(format!(
                    "A student profile already exists for this person (Student number '{}')",
                    existing_student.student_number
                )));
            }
        }

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
            if let Ok(Some(period)) = self.period_repo.find_by_id(period_id).await {
                let now = Utc::now();
                let window_open = period.is_enrollment_open
                    && period
                        .enrollment_starts_at
                        .is_none_or(|started| started <= now)
                    && period.enrollment_ends_at.is_none_or(|ends| ends >= now);
                let late_allowed = org_settings.academic.allow_late_enrollment
                    || admission_policy.allow_late_enrollment;
                if !window_open && !late_allowed {
                    return Err(DomainError::Validation(
                        "Enrollment is outside the configured period window".into(),
                    ));
                }
            }
            let mode = input.study_mode.unwrap_or(StudyMode::Regular);
            let mut enrollment_status = EnrollmentStatus::Active;

            if admission_policy.requires_approval {
                enrollment_status = EnrollmentStatus::PendingApproval;
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
            let capacity_override = input.initial_section_id.map(|section_id| {
                capacity_override_request(
                    org_id,
                    student_id,
                    section_id,
                    ctx.user_id().unwrap_or(student_id),
                    now,
                )
            });
            let admission_override = admission_policy.requires_approval.then(|| {
                admission_override_request(
                    org_id,
                    student_id,
                    period_id,
                    input.initial_section_id,
                    ctx.user_id().unwrap_or(student_id),
                    "New applicant registration requires administrative review and verification.",
                    now,
                )
            });
            let save_result = self
                .enrollment_repo
                .save_with_capacity(
                    &mut enrollment,
                    org_settings.enrollment.max_class_size_default.max(0),
                    org_settings.enrollment.allow_waitlist || admission_policy.allow_waitlist,
                    capacity_override,
                    admission_override,
                )
                .await?;
            enrollment_status = enrollment.status;
            created_override_req = save_result.override_request;

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
            if requires_approval {
                enrollment_status = EnrollmentStatus::PendingApproval;
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

        let capacity_override = section_id.map(|section_id| {
            capacity_override_request(
                org_id,
                student_id,
                section_id,
                ctx.user_id().unwrap_or(student_id),
                now,
            )
        });
        let admission_override = (completing_draft && requires_approval).then(|| {
            admission_override_request(
                org_id,
                student_id,
                period_id,
                section_id,
                ctx.user_id().unwrap_or(student_id),
                "Student completed registration and requires administrative review.",
                now,
            )
        });
        self.enrollment_repo
            .save_with_capacity(
                &mut enrollment,
                max_class_capacity,
                allow_waitlist,
                capacity_override,
                admission_override,
            )
            .await?;
        enrollment_status = enrollment.status;

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

        let mut student = self.find_student_in_org(student_id, ctx.tenant_id).await?;

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

    /// Get student detailed profile within the caller's organization.
    pub async fn get_student_detail(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Option<StudentDetail>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentRead)
            .await?;

        let student = match self.student_repo.find_by_id(student_id).await? {
            Some(student) if student.org_id == ctx.tenant_id => student,
            _ => return Ok(None),
        };
        let person = match self.person_repo.find_by_id(student.person_id).await? {
            Some(person) if person.org_id == Some(ctx.tenant_id) => person,
            _ => return Ok(None),
        };

        Ok(Some(StudentDetail { student, person }))
    }

    pub async fn get_enrollment(
        &self,
        ctx: &RequestContext,
        enrollment_id: Uuid,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker
            .check(ctx, Permission::EnrollmentRead)
            .await?;
        let enrollment = self
            .enrollment_repo
            .find_by_id(enrollment_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Enrollment {} not found", enrollment_id))
            })?;
        if enrollment.org_id != ctx.tenant_id {
            return Err(DomainError::NotFound("Enrollment not found".into()));
        }
        Ok(enrollment)
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
        if enrollment.org_id != ctx.tenant_id {
            return Err(DomainError::NotFound("Enrollment not found".into()));
        }

        let settings = self.settings_repo.get_settings(enrollment.org_id).await?;
        let default_capacity = settings.enrollment.max_class_size_default.max(0);
        let allow_waitlist =
            settings.enrollment.allow_waitlist || settings.admission.policy.allow_waitlist;
        let target_section = self
            .section_repo
            .find_by_id(target_section_id)
            .await?
            .ok_or_else(|| DomainError::NotFound("Target class not found".into()))?;
        if target_section.org_id != ctx.tenant_id
            || target_section.is_deleted
            || !target_section.is_active
        {
            return Err(DomainError::Validation(
                "Target class is not active in this organization".into(),
            ));
        }
        enrollment.section_id = Some(target_section_id);
        enrollment.updated_at = Utc::now();
        let capacity_override = Some(capacity_override_request(
            enrollment.org_id,
            enrollment.student_id,
            target_section_id,
            ctx.user_id().unwrap_or(enrollment.student_id),
            Utc::now(),
        ));

        self.enrollment_repo
            .save_with_capacity(
                &mut enrollment,
                target_section.capacity.unwrap_or(default_capacity).max(0),
                allow_waitlist,
                capacity_override,
                None,
            )
            .await?;
        Ok(enrollment)
    }

    pub async fn bulk_transfer_classes(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        items: Vec<BulkTransferStudentItem>,
        atomic: bool,
    ) -> Result<BulkTransferStudentResult, DomainError> {
        self.permission_checker
            .check(ctx, Permission::StudentUpdate)
            .await?;
        if ctx.tenant_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }

        let mut pre_errors: HashMap<Uuid, String> = HashMap::new();
        let mut target_counts: HashMap<Uuid, i32> = HashMap::new();

        for item in &items {
            let _enrollment = match self.enrollment_repo.find_by_id(item.enrollment_id).await? {
                Some(enrollment) if enrollment.org_id == org_id => enrollment,
                _ => {
                    pre_errors.insert(
                        item.enrollment_id,
                        "Enrollment not found in this organization".to_string(),
                    );
                    continue;
                }
            };
            let target = match self.section_repo.find_by_id(item.target_section_id).await? {
                Some(section)
                    if section.org_id == org_id && section.is_active && !section.is_deleted =>
                {
                    section
                }
                _ => {
                    pre_errors.insert(
                        item.enrollment_id,
                        "Target class is invalid for this organization and period".to_string(),
                    );
                    continue;
                }
            };

            let settings = self
                .settings_repo
                .get_settings(org_id)
                .await
                .unwrap_or_default();
            let capacity = target
                .capacity
                .unwrap_or(settings.enrollment.max_class_size_default)
                .max(0);
            let allow_waitlist =
                settings.enrollment.allow_waitlist || settings.admission.policy.allow_waitlist;
            let active_count = self
                .enrollment_repo
                .find_by_section(item.target_section_id)
                .await?
                .into_iter()
                .filter(|record| record.status == EnrollmentStatus::Active)
                .count() as i32;
            let allocated = target_counts.entry(item.target_section_id).or_insert(0);
            *allocated += 1;
            if active_count + *allocated > capacity && !allow_waitlist {
                pre_errors.insert(
                    item.enrollment_id,
                    format!(
                        "Target class capacity exceeded ({}/{})",
                        active_count + *allocated,
                        capacity
                    ),
                );
            }
        }

        if atomic && !pre_errors.is_empty() {
            let items = items
                .iter()
                .map(|item| BulkTransferStudentItemResult {
                    enrollment_id: item.enrollment_id,
                    success: false,
                    error: pre_errors.get(&item.enrollment_id).cloned().or_else(|| {
                        Some("Batch transfer aborted by another row's validation".to_string())
                    }),
                })
                .collect::<Vec<_>>();
            return Ok(BulkTransferStudentResult {
                total: items.len(),
                succeeded: 0,
                failed: items.len(),
                items,
            });
        }

        let mut results = Vec::with_capacity(items.len());
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        for item in items {
            if let Some(error) = pre_errors.get(&item.enrollment_id) {
                failed += 1;
                results.push(BulkTransferStudentItemResult {
                    enrollment_id: item.enrollment_id,
                    success: false,
                    error: Some(error.clone()),
                });
                continue;
            }
            match self
                .transfer_class(ctx, item.enrollment_id, item.target_section_id, item.reason)
                .await
            {
                Ok(_) => {
                    succeeded += 1;
                    results.push(BulkTransferStudentItemResult {
                        enrollment_id: item.enrollment_id,
                        success: true,
                        error: None,
                    });
                }
                Err(error) => {
                    failed += 1;
                    results.push(BulkTransferStudentItemResult {
                        enrollment_id: item.enrollment_id,
                        success: false,
                        error: Some(error.to_string()),
                    });
                }
            }
        }

        Ok(BulkTransferStudentResult {
            total: succeeded + failed,
            succeeded,
            failed,
            items: results,
        })
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
        if enrollment.org_id != ctx.tenant_id {
            return Err(DomainError::NotFound("Enrollment not found".into()));
        }

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
        self.find_student_in_org(student_id, ctx.tenant_id).await?;
        self.find_person_in_org(guardian_id, ctx.tenant_id).await?;

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
        ctx: &RequestContext,
        student_id: Uuid,
    ) -> Result<Vec<StudentGuardian>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::GuardianRead)
            .await?;
        self.find_student_in_org(student_id, ctx.tenant_id).await?;
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
        self.find_student_in_org(student_id, ctx.tenant_id).await?;
        self.find_person_in_org(guardian_id, ctx.tenant_id).await?;
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

        let student = self.find_student_in_org(student_id, ctx.tenant_id).await?;
        let mut person = self
            .find_person_in_org(student.person_id, ctx.tenant_id)
            .await?;

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

        let mut student = self.find_student_in_org(student_id, ctx.tenant_id).await?;

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

        let mut student = self.find_student_in_org(student_id, ctx.tenant_id).await?;
        self.find_person_in_org(student.person_id, ctx.tenant_id)
            .await?;

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
        if ctx.tenant_id != org_id {
            return Err(DomainError::Unauthorized("Organization mismatch".into()));
        }
        let _enrollment_guard = self.enrollment_lock.lock().await;

        let is_atomic = input.atomic.unwrap_or(true);
        let is_dry_run = input.dry_run.unwrap_or(false);
        let items = input.items;
        let total = items.len();

        let org_settings = self
            .settings_repo
            .get_settings(org_id)
            .await
            .unwrap_or_default();
        let admission_policy = org_settings.admission.policy.clone();
        let allow_waitlist =
            org_settings.enrollment.allow_waitlist || admission_policy.allow_waitlist;

        // ─────────────────────────────────────────────────────────────────
        // STAGE 1: INTRA-BATCH INTEGRITY & CONFLICT PRE-VALIDATION
        // ─────────────────────────────────────────────────────────────────
        let mut pre_errors: HashMap<usize, String> = HashMap::new();
        let mut seen_nids: HashMap<String, usize> = HashMap::new();
        let mut seen_passports: HashMap<String, usize> = HashMap::new();
        let mut seen_student_nums: HashMap<String, usize> = HashMap::new();
        let mut seen_full_names: HashMap<String, usize> = HashMap::new();
        let mut class_allocated_count: HashMap<Uuid, i32> = HashMap::new();
        let mut class_capacity_cache: HashMap<Uuid, (i32, i32)> = HashMap::new(); // class_id -> (current_active, max_capacity)
        let existing_student_details = if admission_policy.prevent_duplicate_full_name {
            self.student_repo.find_active_details_by_org(org_id).await?
        } else {
            Vec::new()
        };

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

            // 1b. Full-name duplicate detection (existing students + intra-batch)
            if admission_policy.prevent_duplicate_full_name {
                let dup_input = DuplicateCheckInput {
                    full_name: item.registration.display_name.clone(),
                    first_name: Some(item.registration.first_name.clone()),
                    middle_name: item.registration.middle_name.clone(),
                    last_name: Some(item.registration.last_name.clone()),
                    first_name_ar: item.registration.first_name_ar.clone(),
                    middle_name_ar: item.registration.middle_name_ar.clone(),
                    last_name_ar: item.registration.last_name_ar.clone(),
                    birth_date: item.registration.birth_date,
                    exclude_student_id: None,
                    ..Default::default()
                };
                let keys = candidate_name_keys(
                    dup_input.full_name.as_deref(),
                    dup_input.first_name.as_deref(),
                    dup_input.middle_name.as_deref(),
                    dup_input.last_name.as_deref(),
                    dup_input.first_name_ar.as_deref(),
                    dup_input.middle_name_ar.as_deref(),
                    dup_input.last_name_ar.as_deref(),
                );
                if !keys.is_empty() {
                    if let Some(matched) =
                        collect_name_duplicate_matches(&existing_student_details, &dup_input, None)
                            .into_iter()
                            .next()
                    {
                        pre_errors.insert(
                            row,
                            format!(
                                "A student with the same full name already exists in this organization ({} - {}).",
                                matched
                                    .student_number
                                    .unwrap_or_else(|| "existing record".to_string()),
                                matched.matched_person_name
                            ),
                        );
                        continue;
                    }
                    if let Some(prev_row) = keys.iter().find_map(|k| seen_full_names.get(k)) {
                        pre_errors.insert(
                            row,
                            format!(
                                "Duplicate full name within batch (conflicts with row {})",
                                prev_row
                            ),
                        );
                        continue;
                    }
                    for k in keys {
                        seen_full_names.insert(k, row);
                    }
                }
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

                if let Ok(Some(existing_person)) = self
                    .person_repo
                    .find_by_national_id(Some(org_id), nid)
                    .await
                {
                    if let Ok(Some(existing_student)) = self
                        .student_repo
                        .find_by_person_id(existing_person.id)
                        .await
                    {
                        if existing_student.org_id == org_id {
                            pre_errors.insert(
                                row,
                                format!(
                                    "A student profile already exists for National ID '{}' (Student number '{}')",
                                    nid, existing_student.student_number
                                ),
                            );
                            continue;
                        }
                    }
                }

                // Preserve the configured policy for duplicate person identities.
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
                if let Some(period_id) = item.registration.initial_period_id {
                    if let Ok(Some(period)) = self.period_repo.find_by_id(period_id).await {
                        let now = Utc::now();
                        let window_open = period.is_enrollment_open
                            && period
                                .enrollment_starts_at
                                .is_none_or(|started| started <= now)
                            && period.enrollment_ends_at.is_none_or(|ends| ends >= now);
                        let late_allowed = org_settings.enrollment.allow_late_enrollment
                            || org_settings.academic.allow_late_enrollment
                            || admission_policy.allow_late_enrollment;
                        if !window_open && !late_allowed {
                            pre_errors.insert(
                                row,
                                "Enrollment is outside the configured period window".to_string(),
                            );
                            continue;
                        }
                    }
                }
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

                    if (current_active + *allocated) > max_cap {
                        if !allow_waitlist {
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

            match self
                .register_student_unlocked(ctx, org_id, item.registration, Some(&org_settings))
                .await
            {
                Ok(reg_res) => {
                    let stu_id = reg_res.student.student.id;
                    let stu_num = reg_res.student.student.student_number.clone();
                    let mut guardian_error: Option<String> = None;

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
                                match self.person_repo.save(&g_person).await {
                                    Ok(()) => new_pid,
                                    Err(error) => {
                                        guardian_error =
                                            Some(format!("Guardian provisioning failed: {error}"));
                                        Uuid::nil()
                                    }
                                }
                            };

                            if guardian_error.is_none() {
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
                                if let Err(error) = self.guardian_repo.save(&sg).await {
                                    guardian_error =
                                        Some(format!("Guardian linkage failed: {error}"));
                                }
                            }
                        }
                    }

                    if let Some(reason) = guardian_error {
                        failed += 1;
                        results.push(BulkRegisterItemResult {
                            row_index: row_idx,
                            success: false,
                            student_id: Some(stu_id),
                            student_number: Some(stu_num),
                            error: Some(reason),
                        });
                        continue;
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
