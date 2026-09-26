//! Tauri IPC commands — Student (Registration, Profiles, Enrollments, Guardians).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveDate;
use orbis_domain::entities::student::{Student, StudentStatus, AcademicStanding, StudentDetail};
use orbis_domain::entities::person::Person;
use orbis_domain::entities::enrollment::{Enrollment, StudyMode, EnrollmentStatus};
use orbis_domain::entities::student_guardian::StudentGuardian;
use orbis_domain::queries::filters::{StudentFilter, EnrollmentFilter, PaginationParams, PaginatedList};
use orbis_app::services::student_service::RegisterStudentInput as ServiceRegisterStudentInput;
use orbis_app::services::student_service::UpdateStudentProfileInput as ServiceUpdateProfileInput;
use super::make_ctx;
use super::org_cmds::ApiResponse;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

// ── DTOs ─────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersonDto {
    pub id: String,
    pub org_id: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn person_to_dto(p: Person) -> PersonDto {
    let full_name = p.full_name();
    let full_name_ar = p.full_name_ar();
    PersonDto {
        id: p.id.to_string(),
        org_id: p.org_id.map(|o| o.to_string()),
        first_name: p.first_name,
        middle_name: p.middle_name,
        last_name: p.last_name,
        first_name_ar: p.first_name_ar,
        middle_name_ar: p.middle_name_ar,
        last_name_ar: p.last_name_ar,
        display_name: p.display_name,
        full_name,
        full_name_ar,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: p.gender,
        birth_date: p.birth_date.map(|d| d.to_string()),
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: p.phone,
        email: p.email,
        address: p.address,
        avatar_url: p.avatar_url,
        created_at: p.created_at.to_rfc3339(),
        updated_at: p.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StudentDto {
    pub id: String,
    pub org_id: String,
    pub person_id: String,
    pub user_id: Option<String>,
    pub student_number: String,
    pub status: String,
    pub standing: String,
    pub admitted_at: String,
    pub graduated_at: Option<String>,
    pub meta: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
}

pub fn student_to_dto(s: Student) -> StudentDto {
    StudentDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        person_id: s.person_id.to_string(),
        user_id: s.user_id.map(|u| u.to_string()),
        student_number: s.student_number,
        status: s.status.to_string(),
        standing: s.standing.to_string(),
        admitted_at: s.admitted_at.to_rfc3339(),
        graduated_at: s.graduated_at.map(|d| d.to_rfc3339()),
        meta: s.meta,
        created_at: s.created_at.to_rfc3339(),
        updated_at: s.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StudentDetailDto {
    pub student: StudentDto,
    pub person: PersonDto,
}

pub fn student_detail_to_dto(d: StudentDetail) -> StudentDetailDto {
    StudentDetailDto {
        student: student_to_dto(d.student),
        person: person_to_dto(d.person),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EnrollmentDto {
    pub id: String,
    pub student_id: String,
    pub class_id: Option<String>,
    pub period_id: String,
    pub org_id: String,
    pub student_number: String,
    pub study_mode: String,
    pub status: String,
    pub is_attendance_exempt: bool,
    pub is_grade_exempt: bool,
    pub enrollment_date: String,
    pub withdrawal_date: Option<String>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn enrollment_to_dto(e: Enrollment) -> EnrollmentDto {
    EnrollmentDto {
        id: e.id.to_string(),
        student_id: e.student_id.to_string(),
        class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(),
        org_id: e.org_id.to_string(),
        student_number: e.student_number,
        study_mode: e.study_mode.to_string(),
        status: e.status.to_string(),
        is_attendance_exempt: e.is_attendance_exempt,
        is_grade_exempt: e.is_grade_exempt,
        enrollment_date: e.enrolled_at.to_rfc3339(),
        withdrawal_date: e.withdrawn_at.map(|d| d.to_rfc3339()),
        withdrawal_reason: e.withdrawal_reason,
        meta: Some(e.meta),
        created_at: e.created_at.to_rfc3339(),
        updated_at: e.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuardianDto {
    pub id: String,
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: bool,
    pub is_emergency_contact: bool,
    pub is_authorized_pickup: bool,
    pub is_payer: bool,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn guardian_to_dto(g: StudentGuardian) -> GuardianDto {
    GuardianDto {
        id: g.id.to_string(),
        student_id: g.student_id.to_string(),
        guardian_id: g.guardian_id.to_string(),
        relationship: g.relationship,
        relationship_ar: g.relationship_ar,
        is_primary: g.is_primary,
        is_emergency_contact: g.is_emergency_contact,
        is_authorized_pickup: g.is_authorized_pickup,
        is_payer: g.is_payer,
        payment_share_percent: g.payment_share_percent,
        occupation: g.occupation,
        employer: g.employer,
        work_phone: g.work_phone,
        notes: g.notes,
        created_at: g.created_at.to_rfc3339(),
        updated_at: g.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResponseDto<T: Serialize> {
    pub items: Vec<T>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

fn paginated_to_dto<T: Serialize, U: Serialize, F: Fn(T) -> U>(p: PaginatedList<T>, f: F) -> PaginatedResponseDto<U> {
    PaginatedResponseDto {
        items: p.items.into_iter().map(f).collect(),
        total_count: p.total_count,
        page: p.page,
        page_size: p.page_size,
        total_pages: p.total_pages,
    }
}

// ── Request Inputs ───────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct RegisterStudentRequest {
    pub person_id: Option<String>,
    pub user_id: Option<String>,
    pub student_number: String,
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
    pub birth_date: Option<String>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub initial_period_id: Option<String>,
    pub initial_class_id: Option<String>,
    pub study_mode: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct StudentFilterInput {
    pub period_id: Option<String>,
    pub class_id: Option<String>,
    pub status: Option<String>,
    pub standing: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateStudentStatusInput {
    pub student_id: String,
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ChangeStudyModeInput {
    pub enrollment_id: String,
    pub study_mode: String,
    pub reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateStudentProfileRequest {
    pub student_id: String,
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
    pub birth_date: Option<String>,
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

#[derive(Deserialize, Debug)]
pub struct EnrollStudentInput {
    pub period_id: String,
    pub student_id: String,
    pub class_id: Option<String>,
    pub student_number: String,
    pub study_mode: Option<String>,
}

#[derive(Deserialize, Default, Debug)]
pub struct EnrollmentFilterInput {
    pub period_id: Option<String>,
    pub class_id: Option<String>,
    pub student_id: Option<String>,
    pub study_mode: Option<String>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AssignGuardianInput {
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default)]
    pub is_emergency_contact: bool,
    #[serde(default)]
    pub is_authorized_pickup: bool,
    #[serde(default)]
    pub is_payer: bool,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

// ── Helpers ──────────────────────────────────────────────────────

fn parse_study_mode(mode_str: Option<&str>) -> StudyMode {
    match mode_str {
        Some("homeschooled") | Some("homeschool") => StudyMode::Homeschooled,
        Some("auditor") => StudyMode::Auditor,
        Some("visiting") => StudyMode::Visiting,
        Some("distance_learning") => StudyMode::DistanceLearning,
        Some("part_time") => StudyMode::PartTime,
        Some("exchange") => StudyMode::Exchange,
        Some("special_needs") => StudyMode::SpecialNeeds,
        _ => StudyMode::Regular,
    }
}

fn parse_student_status(status_str: &str) -> Option<StudentStatus> {
    match status_str {
        "active" => Some(StudentStatus::Active),
        "on_leave" => Some(StudentStatus::OnLeave),
        "suspended" => Some(StudentStatus::Suspended),
        "withdrawn" => Some(StudentStatus::Withdrawn),
        "transferred_out" => Some(StudentStatus::TransferredOut),
        "graduated" => Some(StudentStatus::Graduated),
        "expelled" => Some(StudentStatus::Expelled),
        _ => None,
    }
}

fn parse_enrollment_status(status_str: &str) -> Option<EnrollmentStatus> {
    match status_str {
        "active" => Some(EnrollmentStatus::Active),
        "pending_approval" => Some(EnrollmentStatus::PendingApproval),
        "on_hold" => Some(EnrollmentStatus::OnHold),
        "withdrawn" => Some(EnrollmentStatus::Withdrawn),
        "transferred" => Some(EnrollmentStatus::Transferred),
        "graduated" => Some(EnrollmentStatus::Graduated),
        "suspended" => Some(EnrollmentStatus::Suspended),
        _ => None,
    }
}

fn parse_academic_standing(standing_str: &str) -> Option<AcademicStanding> {
    match standing_str {
        "good_standing" => Some(AcademicStanding::GoodStanding),
        "honor_list" => Some(AcademicStanding::HonorList),
        "warning_1" => Some(AcademicStanding::AcademicWarning1),
        "warning_2" => Some(AcademicStanding::AcademicWarning2),
        "warning_3" => Some(AcademicStanding::AcademicWarning3),
        "probation" => Some(AcademicStanding::Probation),
        "dismissed" => Some(AcademicStanding::Dismissed),
        _ => None,
    }
}

// ── Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn student_register_student(
    request: RegisterStudentRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StudentDetailDto>, String> {
    let ctx = make_ctx(&state);
    let person_id = request.person_id.and_then(|id| Uuid::parse_str(&id).ok());
    let user_id = request.user_id.and_then(|id| Uuid::parse_str(&id).ok());
    let initial_period_id = request.initial_period_id.and_then(|id| Uuid::parse_str(&id).ok());
    let initial_class_id = request.initial_class_id.and_then(|id| Uuid::parse_str(&id).ok());
    let birth_date = request.birth_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let study_mode = request.study_mode.as_deref().map(|s| parse_study_mode(Some(s)));

    let input = ServiceRegisterStudentInput {
        person_id,
        user_id,
        student_number: if request.student_number.trim().is_empty() { None } else { Some(request.student_number) },
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        first_name_ar: request.first_name_ar,
        middle_name_ar: request.middle_name_ar,
        last_name_ar: request.last_name_ar,
        display_name: request.display_name,
        national_id: request.national_id,
        passport_number: request.passport_number,
        gender: request.gender,
        birth_date,
        place_of_birth: request.place_of_birth,
        nationality: request.nationality,
        blood_type: request.blood_type,
        medical_notes: request.medical_notes,
        special_needs_notes: request.special_needs_notes,
        phone: request.phone,
        email: request.email,
        avatar_url: request.avatar_url,
        initial_period_id,
        initial_class_id,
        study_mode,
        bypass_capacity_check: None,
    };

    let result = state.student_service
        .register_student(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(student_detail_to_dto(result.student)))
}

#[tauri::command]
pub async fn student_list_students(
    filter: Option<StudentFilterInput>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<PaginatedResponseDto<StudentDetailDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let domain_filter = StudentFilter {
        period_id: f.period_id.and_then(|s| Uuid::parse_str(&s).ok()),
        class_id: f.class_id.and_then(|s| Uuid::parse_str(&s).ok()),
        status: f.status.as_deref().and_then(parse_student_status),
        standing: f.standing.as_deref().and_then(parse_academic_standing),
        search: f.search.filter(|s| !s.trim().is_empty()),
        pagination: Some(PaginationParams {
            page: f.page,
            page_size: f.page_size,
            sort_by: f.sort_by,
            sort_dir: f.sort_dir,
        }),
    };

    let result = state.student_service
        .list_students(ctx.tenant_id, domain_filter)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(paginated_to_dto(result, student_detail_to_dto)))
}

#[tauri::command]
pub async fn student_get_student(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StudentDetailDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid student ID")?;
    let detail = state.student_service
        .get_student(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Student not found".to_string())?;

    Ok(ok(student_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn student_update_student_status(
    request: UpdateStudentStatusInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StudentDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student ID")?;
    let status = parse_student_status(&request.status).ok_or_else(|| "Invalid status".to_string())?;

    let student = state.student_service
        .update_student_status(&ctx, student_id, status, request.reason)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(student_to_dto(student)))
}

#[tauri::command]
pub async fn student_update_profile(
    request: UpdateStudentProfileRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StudentDetailDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid student ID")?;
    let birth_date = request.birth_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());

    let input = ServiceUpdateProfileInput {
        first_name: request.first_name,
        middle_name: request.middle_name,
        last_name: request.last_name,
        first_name_ar: request.first_name_ar,
        middle_name_ar: request.middle_name_ar,
        last_name_ar: request.last_name_ar,
        display_name: request.display_name,
        national_id: request.national_id,
        passport_number: request.passport_number,
        gender: request.gender,
        birth_date,
        place_of_birth: request.place_of_birth,
        nationality: request.nationality,
        blood_type: request.blood_type,
        medical_notes: request.medical_notes,
        special_needs_notes: request.special_needs_notes,
        phone: request.phone,
        email: request.email,
        address: request.address,
        avatar_url: request.avatar_url,
    };

    let detail = state.student_service
        .update_student_profile(&ctx, student_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(student_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn student_change_study_mode(
    request: ChangeStudyModeInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EnrollmentDto>, String> {
    let ctx = make_ctx(&state);
    let enrollment_id = Uuid::parse_str(&request.enrollment_id).map_err(|_| "Invalid enrollment ID")?;
    let study_mode = parse_study_mode(Some(&request.study_mode));

    let enrollment = state.student_service
        .change_study_mode(&ctx, enrollment_id, study_mode, request.reason)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(enrollment_to_dto(enrollment)))
}

#[derive(Deserialize, Debug)]
pub struct TransferClassInput {
    pub enrollment_id: String,
    pub target_class_id: String,
    pub reason: Option<String>,
}

#[tauri::command]
pub async fn student_transfer_class(
    request: TransferClassInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EnrollmentDto>, String> {
    let ctx = make_ctx(&state);
    let enrollment_id = Uuid::parse_str(&request.enrollment_id).map_err(|_| "Invalid enrollment ID")?;
    let target_class_id = Uuid::parse_str(&request.target_class_id).map_err(|_| "Invalid target class ID")?;

    let enrollment = state.student_service
        .transfer_class(&ctx, enrollment_id, target_class_id, request.reason)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(enrollment_to_dto(enrollment)))
}

#[tauri::command]
pub async fn student_enroll_student(
    request: EnrollStudentInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EnrollmentDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_id = request.class_id.and_then(|c| Uuid::parse_str(&c).ok());
    let study_mode = parse_study_mode(request.study_mode.as_deref());

    let e = state.student_service
        .enroll_student(&ctx, ctx.tenant_id, period_id, student_id, class_id, request.student_number, study_mode)
        .await
        .map_err(|err| err.to_string())?;

    Ok(ok(enrollment_to_dto(e)))
}

#[tauri::command]
pub async fn student_list_enrollments(
    filter: Option<EnrollmentFilterInput>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<EnrollmentDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let domain_filter = EnrollmentFilter {
        period_id: f.period_id.and_then(|s| Uuid::parse_str(&s).ok()),
        class_id: f.class_id.and_then(|s| Uuid::parse_str(&s).ok()),
        student_id: f.student_id.and_then(|s| Uuid::parse_str(&s).ok()),
        study_mode: f.study_mode.as_deref().map(|s| parse_study_mode(Some(s))),
        status: f.status.as_deref().and_then(parse_enrollment_status),
        pagination: Some(PaginationParams {
            page: f.page.or(Some(1)),
            page_size: f.page_size.or(Some(1000)),
            sort_by: f.sort_by,
            sort_dir: f.sort_dir,
        }),
    };

    let result = state.student_service
        .list_enrollments(ctx.tenant_id, domain_filter)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(result.items.into_iter().map(enrollment_to_dto).collect()))
}

#[tauri::command]
pub async fn student_get_enrollment(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<EnrollmentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let e = state.enrollment_repo
        .find_by_id(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Enrollment not found".to_string())?;
    Ok(ok(enrollment_to_dto(e)))
}

#[tauri::command]
pub async fn student_assign_guardian(
    request: AssignGuardianInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<GuardianDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let guardian_id = Uuid::parse_str(&request.guardian_id).map_err(|_| "Invalid guardianId")?;

    let g = state.student_service
        .assign_guardian(&ctx, student_id, guardian_id, request.relationship, request.is_primary)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(guardian_to_dto(g)))
}

#[tauri::command]
pub async fn student_list_guardians(
    student_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<GuardianDto>>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let items = state.student_service
        .get_student_guardians(&ctx, sid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(guardian_to_dto).collect()))
}

#[derive(Deserialize, Debug)]
pub struct CheckDuplicateInput {
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub exclude_student_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DuplicateMatchDto {
    pub field: String,
    pub value: String,
    pub matched_person_id: String,
    pub matched_person_name: String,
    pub student_id: Option<String>,
    pub student_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DuplicateCheckResultDto {
    pub has_duplicate: bool,
    pub matches: Vec<DuplicateMatchDto>,
}

#[tauri::command]
pub async fn student_check_duplicate(
    request: CheckDuplicateInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DuplicateCheckResultDto>, String> {
    let ctx = make_ctx(&state);
    let exclude_student_id = request.exclude_student_id.and_then(|id| Uuid::parse_str(&id).ok());
    let input = orbis_app::services::student_service::DuplicateCheckInput {
        national_id: request.national_id,
        phone: request.phone,
        email: request.email,
        exclude_student_id,
    };

    let res = state.student_service
        .check_duplicate(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    let matches = res.matches.into_iter().map(|m| DuplicateMatchDto {
        field: m.field,
        value: m.value,
        matched_person_id: m.matched_person_id.to_string(),
        matched_person_name: m.matched_person_name,
        student_id: m.student_id.map(|id| id.to_string()),
        student_number: m.student_number,
    }).collect();

    Ok(ok(DuplicateCheckResultDto {
        has_duplicate: res.has_duplicate,
        matches,
    }))
}

