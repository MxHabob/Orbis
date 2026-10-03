//! Student routes — Registrations, Enrollments, Study Mode changes, Status updates, Guardians, Bulk registration.

use axum::{extract::{State, Path, Query}, routing::{get, post, patch, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::student::{StudentStatus, AcademicStanding, StudentDetail};
use orbis_domain::entities::enrollment::{Enrollment, StudyMode};
use orbis_domain::queries::filters::{StudentFilter, PaginationParams};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct StudentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub person_id: Uuid,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub display_name: Option<String>,
    pub gender: String,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub standing: String,
    pub admitted_at: DateTime<Utc>,
    pub graduated_at: Option<DateTime<Utc>>,
    pub org_unit_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<StudentDetail> for StudentDto {
    fn from(d: StudentDetail) -> Self {
        let s = d.student;
        let p = d.person;
        let full_name = p.full_name();
        let full_name_ar = p.full_name_ar();
        Self {
            id: s.id,
            org_id: s.org_id,
            person_id: s.person_id,
            user_id: s.user_id,
            student_number: s.student_number,
            national_id: p.national_id,
            passport_number: p.passport_number,
            first_name: p.first_name,
            middle_name: p.middle_name,
            last_name: p.last_name,
            full_name,
            name_ar: full_name_ar,
            first_name_ar: p.first_name_ar,
            middle_name_ar: p.middle_name_ar,
            last_name_ar: p.last_name_ar,
            display_name: p.display_name,
            gender: p.gender.unwrap_or_else(|| "male".into()),
            date_of_birth: p.birth_date,
            place_of_birth: p.place_of_birth,
            nationality: p.nationality,
            blood_type: p.blood_type,
            medical_notes: p.medical_notes,
            special_needs_notes: p.special_needs_notes,
            phone: p.phone,
            email: p.email,
            address: p.address,
            avatar_url: p.avatar_url,
            status: s.status.to_string(),
            standing: s.standing.to_string(),
            admitted_at: s.admitted_at,
            graduated_at: s.graduated_at,
            org_unit_id: s.org_unit_id,
            created_by: s.created_by,
            updated_by: s.updated_by,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterStudentRequest {
    pub person_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub student_number: Option<String>,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub first_name_ar: Option<String>,
    pub middle_name_ar: Option<String>,
    pub last_name_ar: Option<String>,
    pub name_ar: Option<String>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub birth_date: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub initial_period_id: Option<Uuid>,
    pub initial_class_id: Option<Uuid>,
    pub study_mode: Option<String>,
    pub bypass_capacity_check: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateStudentProfileRequest {
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EnrollmentDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub student_number: String,
    pub study_mode: String,
    pub status: String,
    pub is_attendance_exempt: bool,
    pub is_grade_exempt: bool,
    pub enrollment_date: DateTime<Utc>,
    pub withdrawal_date: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Enrollment> for EnrollmentDto {
    fn from(e: Enrollment) -> Self {
        Self {
            id: e.id,
            org_id: e.org_id,
            period_id: e.period_id,
            student_id: e.student_id,
            class_id: e.class_id,
            student_number: e.student_number,
            study_mode: e.study_mode.to_string(),
            status: e.status.to_string(),
            is_attendance_exempt: e.is_attendance_exempt,
            is_grade_exempt: e.is_grade_exempt,
            enrollment_date: e.enrolled_at,
            withdrawal_date: e.withdrawn_at,
            withdrawal_reason: e.withdrawal_reason,
            created_by: e.created_by,
            updated_by: e.updated_by,
            created_at: e.created_at,
            updated_at: e.updated_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct EnrollStudentRequest {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub student_number: String,
    pub study_mode: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ChangeStudyModeRequest {
    pub new_mode: String,
    pub reason: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct TransferClassRequest {
    pub target_class_id: Uuid,
    pub reason: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateStudentStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignGuardianRequest {
    pub student_id: Uuid,
    pub guardian_id: Uuid,
    pub relationship: String,
    pub relationship_ar: Option<String>,
    pub is_primary: Option<bool>,
    pub is_emergency_contact: Option<bool>,
    pub is_authorized_pickup: Option<bool>,
    pub is_payer: Option<bool>,
    pub payment_share_percent: Option<f64>,
    pub occupation: Option<String>,
    pub employer: Option<String>,
    pub work_phone: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GuardianDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub guardian_id: Uuid,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct StudentQuery {
    pub period_id: Option<Uuid>,
    pub class_id: Option<Uuid>,
    pub status: Option<String>,
    pub standing: Option<String>,
    pub search: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct EnrollmentQuery {
    pub class_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct GuardianQuery {
    pub student_id: Uuid,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DuplicateCheckRequest {
    pub national_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub exclude_student_id: Option<Uuid>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DuplicateMatchDto {
    pub field: String,
    pub value: String,
    pub matched_person_id: Uuid,
    pub matched_person_name: String,
    pub student_id: Option<Uuid>,
    pub student_number: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DuplicateCheckResponse {
    pub has_duplicate: bool,
    pub matches: Vec<DuplicateMatchDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct BulkRegisterStudentItem {
    pub row_index: usize,
    pub registration: RegisterStudentRequest,
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub guardian_relationship: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct BulkRegisterStudentsRequest {
    pub items: Vec<BulkRegisterStudentItem>,
    pub atomic: Option<bool>,
    pub dry_run: Option<bool>,
}

#[derive(Serialize, ToSchema)]
pub struct BulkRegisterItemResultDto {
    pub row_index: usize,
    pub success: bool,
    pub student_id: Option<Uuid>,
    pub student_number: Option<String>,
    pub error: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BulkRegisterStudentsResponse {
    pub total: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub is_atomic: bool,
    pub is_dry_run: bool,
    pub items: Vec<BulkRegisterItemResultDto>,
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

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/students", request_body = RegisterStudentRequest,
    responses((status = 201, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn register_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<RegisterStudentRequest>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let student_num = p.student_number.and_then(|sn| {
        let trimmed = sn.trim();
        if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
    });

    let birth_date = p.birth_date.or(p.date_of_birth);
    let first_name_ar = p.first_name_ar.or(p.name_ar);
    let study_mode = p.study_mode.as_deref().map(|s| parse_study_mode(Some(s)));

    let input = orbis_app::services::RegisterStudentInput {
        person_id: p.person_id,
        user_id: p.user_id,
        student_number: student_num,
        first_name: p.first_name,
        middle_name: p.middle_name,
        last_name: p.last_name,
        first_name_ar,
        middle_name_ar: p.middle_name_ar,
        last_name_ar: p.last_name_ar,
        display_name: p.display_name,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: p.gender,
        birth_date,
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: p.phone,
        email: p.email,
        avatar_url: p.avatar_url,
        initial_period_id: p.initial_period_id,
        initial_class_id: p.initial_class_id,
        study_mode,
        bypass_capacity_check: p.bypass_capacity_check,
    };

    let result = state.student_service.register_student(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(result.student.into()))
}

#[utoipa::path(get, path = "/api/v1/students",
    params(StudentQuery),
    responses((status = 200, body = Vec<StudentDto>)),
    security(("bearer" = [])), tag = "Student")]
pub async fn list_students(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<StudentQuery>,
) -> Result<Json<ApiResponse<Vec<StudentDto>>>, ApiError> {
    let domain_filter = StudentFilter {
        period_id: q.period_id,
        class_id: q.class_id,
        status: q.status.as_deref().and_then(|s| match s {
            "active" => Some(StudentStatus::Active),
            "on_leave" => Some(StudentStatus::OnLeave),
            "suspended" => Some(StudentStatus::Suspended),
            "withdrawn" => Some(StudentStatus::Withdrawn),
            "transferred_out" => Some(StudentStatus::TransferredOut),
            "graduated" => Some(StudentStatus::Graduated),
            "expelled" => Some(StudentStatus::Expelled),
            _ => None,
        }),
        standing: q.standing.as_deref().and_then(|s| match s {
            "good_standing" => Some(AcademicStanding::GoodStanding),
            "honor_list" => Some(AcademicStanding::HonorList),
            "warning_1" => Some(AcademicStanding::AcademicWarning1),
            "warning_2" => Some(AcademicStanding::AcademicWarning2),
            "warning_3" => Some(AcademicStanding::AcademicWarning3),
            "probation" => Some(AcademicStanding::Probation),
            "dismissed" => Some(AcademicStanding::Dismissed),
            _ => None,
        }),
        search: q.search.filter(|s| !s.trim().is_empty()),
        pagination: Some(PaginationParams {
            page: q.page,
            page_size: q.page_size,
            sort_by: q.sort_by,
            sort_dir: q.sort_dir,
        }),
    };
    let paginated = state.student_service.list_students(ctx.tenant_id, domain_filter).await?;
    let dtos = paginated.items.into_iter().map(Into::into).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/students/{id}",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn get_student(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let detail = state.student_service.get_student(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Student {} not found", id)))?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(patch, path = "/api/v1/students/{id}/status", request_body = UpdateStudentStatusRequest,
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn update_student_status(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateStudentStatusRequest>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let new_status = match p.status.as_str() {
        "on_leave" => StudentStatus::OnLeave,
        "suspended" => StudentStatus::Suspended,
        "withdrawn" => StudentStatus::Withdrawn,
        "transferred_out" => StudentStatus::TransferredOut,
        "graduated" => StudentStatus::Graduated,
        "expelled" => StudentStatus::Expelled,
        _ => StudentStatus::Active,
    };

    let student = state.student_service.update_student_status(&ctx, id, new_status, p.reason).await?;
    let person = state.person_repo.find_by_id(student.person_id).await?
        .ok_or_else(|| ApiError::not_found(format!("Person {} not found", student.person_id)))?;
    let detail = StudentDetail { student, person };
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(patch, path = "/api/v1/students/{id}/profile", request_body = UpdateStudentProfileRequest,
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn update_student_profile(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateStudentProfileRequest>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let input = orbis_app::services::student_service::UpdateStudentProfileInput {
        first_name: p.first_name,
        middle_name: p.middle_name,
        last_name: p.last_name,
        first_name_ar: p.first_name_ar,
        middle_name_ar: p.middle_name_ar,
        last_name_ar: p.last_name_ar,
        display_name: p.display_name,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: p.gender,
        birth_date: p.birth_date,
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: p.phone,
        email: p.email,
        address: p.address,
        avatar_url: p.avatar_url,
    };

    let detail = state.student_service.update_student_profile(&ctx, id, input).await?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(delete, path = "/api/v1/students/{id}",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "Student")]
pub async fn delete_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.student_service.soft_delete_student(&ctx, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(post, path = "/api/v1/students/{id}/restore",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn restore_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let student = state.student_service.restore_student(&ctx, id).await?;
    let person = state.person_repo.find_by_id(student.person_id).await?
        .ok_or_else(|| ApiError::not_found(format!("Person {} not found", student.person_id)))?;
    let detail = StudentDetail { student, person };
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(post, path = "/api/v1/students/bulk-register", request_body = BulkRegisterStudentsRequest,
    responses((status = 200, body = BulkRegisterStudentsResponse)),
    security(("bearer" = [])), tag = "Student")]
pub async fn bulk_register_students(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BulkRegisterStudentsRequest>,
) -> Result<Json<ApiResponse<BulkRegisterStudentsResponse>>, ApiError> {
    let mut domain_items = Vec::with_capacity(p.items.len());

    for item in p.items {
        let student_num = item.registration.student_number.and_then(|sn| {
            let trimmed = sn.trim();
            if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
        });

        let birth_date = item.registration.birth_date.or(item.registration.date_of_birth);
        let first_name_ar = item.registration.first_name_ar.or(item.registration.name_ar);
        let study_mode = item.registration.study_mode.as_deref().map(|s| parse_study_mode(Some(s)));

        let input = orbis_app::services::student_service::RegisterStudentInput {
            person_id: item.registration.person_id,
            user_id: item.registration.user_id,
            student_number: student_num,
            first_name: item.registration.first_name,
            middle_name: item.registration.middle_name,
            last_name: item.registration.last_name,
            first_name_ar,
            middle_name_ar: item.registration.middle_name_ar,
            last_name_ar: item.registration.last_name_ar,
            display_name: item.registration.display_name,
            national_id: item.registration.national_id,
            passport_number: item.registration.passport_number,
            gender: item.registration.gender,
            birth_date,
            place_of_birth: item.registration.place_of_birth,
            nationality: item.registration.nationality,
            blood_type: item.registration.blood_type,
            medical_notes: item.registration.medical_notes,
            special_needs_notes: item.registration.special_needs_notes,
            phone: item.registration.phone,
            email: item.registration.email,
            avatar_url: item.registration.avatar_url,
            initial_period_id: item.registration.initial_period_id,
            initial_class_id: item.registration.initial_class_id,
            study_mode,
            bypass_capacity_check: item.registration.bypass_capacity_check,
        };

        domain_items.push(orbis_app::services::student_service::BulkRegisterItemInput {
            row_index: item.row_index,
            registration: input,
            guardian_name: item.guardian_name,
            guardian_phone: item.guardian_phone,
            guardian_relationship: item.guardian_relationship,
        });
    }

    let res = state.student_service.bulk_register_students(&ctx, ctx.tenant_id, orbis_app::services::student_service::BulkRegisterStudentsInput {
        items: domain_items,
        atomic: p.atomic,
        dry_run: p.dry_run,
    }).await?;

    let items_dto = res.items.into_iter().map(|it| BulkRegisterItemResultDto {
        row_index: it.row_index,
        success: it.success,
        student_id: it.student_id,
        student_number: it.student_number,
        error: it.error,
    }).collect();

    Ok(ApiResponse::ok(BulkRegisterStudentsResponse {
        total: res.total,
        succeeded: res.succeeded,
        failed: res.failed,
        is_atomic: res.is_atomic,
        is_dry_run: res.is_dry_run,
        items: items_dto,
    }))
}

#[utoipa::path(post, path = "/api/v1/enrollments", request_body = EnrollStudentRequest,
    responses((status = 201, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn enroll_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<EnrollStudentRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let study_mode = parse_study_mode(p.study_mode.as_deref());

    let e = state.student_service.enroll_student(
        &ctx,
        ctx.tenant_id,
        p.period_id,
        p.student_id,
        p.class_id,
        p.student_number,
        study_mode,
    ).await?;

    Ok(ApiResponse::ok(e.into()))
}

#[utoipa::path(get, path = "/api/v1/enrollments",
    params(EnrollmentQuery),
    responses((status = 200, body = Vec<EnrollmentDto>)),
    security(("bearer" = [])), tag = "Student")]
pub async fn list_enrollments(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<EnrollmentQuery>,
) -> Result<Json<ApiResponse<Vec<EnrollmentDto>>>, ApiError> {
    let enrollments = if let Some(class_id) = q.class_id {
        state.enrollment_repo.find_by_class(class_id).await?
    } else if let Some(student_id) = q.student_id {
        state.enrollment_repo.find_by_student(student_id).await?
    } else {
        vec![]
    };
    let dtos = enrollments.into_iter().map(Into::into).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/enrollments/{id}",
    params(("id" = Uuid, Path, description = "Enrollment ID")),
    responses((status = 200, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn get_enrollment(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let e = state.enrollment_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Enrollment {} not found", id)))?;
    Ok(ApiResponse::ok(e.into()))
}

#[utoipa::path(patch, path = "/api/v1/enrollments/{id}/study-mode", request_body = ChangeStudyModeRequest,
    params(("id" = Uuid, Path, description = "Enrollment ID")),
    responses((status = 200, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn change_study_mode(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<ChangeStudyModeRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let new_mode = parse_study_mode(Some(&p.new_mode));
    let e = state.student_service.change_study_mode(&ctx, id, new_mode, p.reason).await?;
    Ok(ApiResponse::ok(e.into()))
}

#[utoipa::path(post, path = "/api/v1/enrollments/{id}/transfer-class", request_body = TransferClassRequest,
    params(("id" = Uuid, Path, description = "Enrollment ID")),
    responses((status = 200, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn transfer_class(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<TransferClassRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let e = state.student_service.transfer_class(&ctx, id, p.target_class_id, p.reason).await?;
    Ok(ApiResponse::ok(e.into()))
}

#[utoipa::path(post, path = "/api/v1/student-guardians", request_body = AssignGuardianRequest,
    responses((status = 201, body = GuardianDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn assign_guardian(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignGuardianRequest>,
) -> Result<Json<ApiResponse<GuardianDto>>, ApiError> {
    let g = state.student_service.assign_guardian_full(
        &ctx,
        p.student_id,
        p.guardian_id,
        p.relationship,
        p.relationship_ar,
        p.is_primary.unwrap_or(false),
        p.is_emergency_contact,
        p.is_authorized_pickup,
        p.is_payer,
        p.payment_share_percent,
        p.occupation,
        p.employer,
        p.work_phone,
        p.notes,
    ).await?;

    Ok(ApiResponse::ok(GuardianDto {
        id: g.id,
        student_id: g.student_id,
        guardian_id: g.guardian_id,
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
        created_at: g.created_at,
        updated_at: g.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/student-guardians",
    params(GuardianQuery),
    responses((status = 200, body = Vec<GuardianDto>)),
    security(("bearer" = [])), tag = "Student")]
pub async fn list_guardians(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<GuardianQuery>,
) -> Result<Json<ApiResponse<Vec<GuardianDto>>>, ApiError> {
    let guardians = state.student_service.get_student_guardians(&ctx, q.student_id).await?;
    let dtos = guardians.into_iter().map(|g| GuardianDto {
        id: g.id,
        student_id: g.student_id,
        guardian_id: g.guardian_id,
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
        created_at: g.created_at,
        updated_at: g.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(delete, path = "/api/v1/students/{student_id}/guardians/{guardian_id}",
    params(
        ("student_id" = Uuid, Path, description = "Student ID"),
        ("guardian_id" = Uuid, Path, description = "Guardian ID")
    ),
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "Student")]
pub async fn remove_guardian(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path((student_id, guardian_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.student_service.remove_guardian(&ctx, student_id, guardian_id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(post, path = "/api/v1/students/check-duplicate", request_body = DuplicateCheckRequest,
    responses((status = 200, body = DuplicateCheckResponse)),
    security(("bearer" = [])), tag = "Student")]
pub async fn check_duplicate(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<DuplicateCheckRequest>,
) -> Result<Json<ApiResponse<DuplicateCheckResponse>>, ApiError> {
    let input = orbis_app::services::student_service::DuplicateCheckInput {
        national_id: p.national_id,
        phone: p.phone,
        email: p.email,
        exclude_student_id: p.exclude_student_id,
    };

    let result = state.student_service.check_duplicate(&ctx, ctx.tenant_id, input).await?;
    let matches = result.matches.into_iter().map(|m| DuplicateMatchDto {
        field: m.field,
        value: m.value,
        matched_person_id: m.matched_person_id,
        matched_person_name: m.matched_person_name,
        student_id: m.student_id,
        student_number: m.student_number,
    }).collect();

    Ok(ApiResponse::ok(DuplicateCheckResponse {
        has_duplicate: result.has_duplicate,
        matches,
    }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/students", post(register_student).get(list_students))
        .route("/api/v1/students/bulk-register", post(bulk_register_students))
        .route("/api/v1/students/check-duplicate", post(check_duplicate))
        .route("/api/v1/students/:id", get(get_student).delete(delete_student))
        .route("/api/v1/students/:id/status", patch(update_student_status))
        .route("/api/v1/students/:id/profile", patch(update_student_profile))
        .route("/api/v1/students/:id/restore", post(restore_student))
        .route("/api/v1/students/:student_id/guardians/:guardian_id", delete(remove_guardian))
        .route("/api/v1/enrollments", post(enroll_student).get(list_enrollments))
        .route("/api/v1/enrollments/:id", get(get_enrollment))
        .route("/api/v1/enrollments/:id/study-mode", patch(change_study_mode))
        .route("/api/v1/enrollments/:id/transfer-class", post(transfer_class))
        .route("/api/v1/student-guardians", post(assign_guardian).get(list_guardians))
}
