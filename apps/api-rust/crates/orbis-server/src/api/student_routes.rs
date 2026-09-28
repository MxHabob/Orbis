//! Student routes — Registrations, Enrollments, Study Mode changes, Status updates, Guardians.

use axum::{extract::{State, Path, Query}, routing::{get, post, patch}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc, NaiveDate};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::student::{StudentStatus, StudentDetail};
use orbis_domain::entities::enrollment::{Enrollment, StudyMode};
use orbis_domain::queries::filters::StudentFilter;

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
    pub last_name: String,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub gender: String,
    pub date_of_birth: Option<NaiveDate>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub standing: String,
    pub admitted_at: DateTime<Utc>,
    pub graduated_at: Option<DateTime<Utc>>,
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
            last_name: p.last_name,
            full_name,
            name_ar: full_name_ar,
            gender: p.gender.unwrap_or_else(|| "male".into()),
            date_of_birth: p.birth_date,
            nationality: p.nationality,
            blood_type: p.blood_type,
            medical_notes: p.medical_notes,
            special_needs_notes: p.special_needs_notes,
            avatar_url: p.avatar_url,
            status: s.status.to_string(),
            standing: s.standing.to_string(),
            admitted_at: s.admitted_at,
            graduated_at: s.graduated_at,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterStudentRequest {
    pub person_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub student_number: String,
    pub national_id: Option<String>,
    pub passport_number: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub name_ar: Option<String>,
    pub gender: String,
    pub date_of_birth: Option<NaiveDate>,
    pub place_of_birth: Option<String>,
    pub nationality: Option<String>,
    pub blood_type: Option<String>,
    pub medical_notes: Option<String>,
    pub special_needs_notes: Option<String>,
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
}

#[derive(Deserialize)]
pub struct EnrollmentQuery {
    pub class_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
}

#[derive(Deserialize)]
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

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/students", request_body = RegisterStudentRequest,
    responses((status = 201, body = StudentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn register_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<RegisterStudentRequest>,
) -> Result<Json<ApiResponse<StudentDto>>, ApiError> {
    let input = orbis_app::services::RegisterStudentInput {
        person_id: p.person_id,
        user_id: p.user_id,
        student_number: if p.student_number.trim().is_empty() { None } else { Some(p.student_number) },
        first_name: p.first_name,
        middle_name: None,
        last_name: p.last_name,
        first_name_ar: p.name_ar,
        middle_name_ar: None,
        last_name_ar: None,
        display_name: None,
        national_id: p.national_id,
        passport_number: p.passport_number,
        gender: Some(p.gender),
        birth_date: p.date_of_birth,
        place_of_birth: p.place_of_birth,
        nationality: p.nationality,
        blood_type: p.blood_type,
        medical_notes: p.medical_notes,
        special_needs_notes: p.special_needs_notes,
        phone: None,
        email: None,
        avatar_url: p.avatar_url,
        initial_period_id: None,
        initial_class_id: None,
        study_mode: None,
        bypass_capacity_check: None,
    };

    let result = state.student_service.register_student(&ctx, ctx.tenant_id, input).await?;
    Ok(ApiResponse::ok(result.student.into()))
}

#[utoipa::path(get, path = "/api/v1/students",
    responses((status = 200, body = Vec<StudentDto>)),
    security(("bearer" = [])), tag = "Student")]
pub async fn list_students(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<StudentDto>>>, ApiError> {
    let paginated = state.student_service.list_students(ctx.tenant_id, StudentFilter::default()).await?;
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

#[utoipa::path(post, path = "/api/v1/enrollments", request_body = EnrollStudentRequest,
    responses((status = 201, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
pub async fn enroll_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<EnrollStudentRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let study_mode = match p.study_mode.as_deref() {
        Some("homeschooled") => StudyMode::Homeschooled,
        Some("auditor") => StudyMode::Auditor,
        Some("visiting") => StudyMode::Visiting,
        Some("distance_learning") => StudyMode::DistanceLearning,
        Some("part_time") => StudyMode::PartTime,
        Some("exchange") => StudyMode::Exchange,
        Some("special_needs") => StudyMode::SpecialNeeds,
        _ => StudyMode::Regular,
    };

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
    let new_mode = match p.new_mode.as_str() {
        "homeschooled" => StudyMode::Homeschooled,
        "auditor" => StudyMode::Auditor,
        "visiting" => StudyMode::Visiting,
        "distance_learning" => StudyMode::DistanceLearning,
        "part_time" => StudyMode::PartTime,
        "exchange" => StudyMode::Exchange,
        "special_needs" => StudyMode::SpecialNeeds,
        _ => StudyMode::Regular,
    };

    let e = state.student_service.change_study_mode(&ctx, id, new_mode, p.reason).await?;
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
    let g = state.student_service.assign_guardian(
        &ctx,
        p.student_id,
        p.guardian_id,
        p.relationship,
        p.is_primary.unwrap_or(false),
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
    }))
}

#[utoipa::path(get, path = "/api/v1/student-guardians",
    params(("student_id" = Uuid, Query, description = "Student ID")),
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
    }).collect();
    Ok(ApiResponse::ok(dtos))
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
        .route("/api/v1/students/check-duplicate", post(check_duplicate))
        .route("/api/v1/students/:id", get(get_student))
        .route("/api/v1/students/:id/status", patch(update_student_status))
        .route("/api/v1/enrollments", post(enroll_student).get(list_enrollments))
        .route("/api/v1/enrollments/:id", get(get_enrollment))
        .route("/api/v1/enrollments/:id/study-mode", patch(change_study_mode))
        .route("/api/v1/student-guardians", post(assign_guardian).get(list_guardians))
}
