//! Student routes — Enrollments, Guardians.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::student::{
    enrollment::EnrollStudentUseCase,
    student_guardian::AssignGuardianUseCase,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EnrollmentDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub period_id: Uuid,
    pub org_id: Uuid,
    pub student_number: String,
    pub enrollment_type: String,
    pub status: String,
    pub enrollment_date: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct EnrollStudentRequest {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub student_number: String,
    pub enrollment_type: String,
}

#[derive(Serialize, ToSchema)]
pub struct GuardianDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub guardian_id: Uuid,
    pub relationship: String,
    pub is_primary: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignGuardianRequest {
    pub student_id: Uuid,
    pub guardian_id: Uuid,
    pub relationship: String,
    pub is_primary: bool,
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

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/enrollments", request_body = EnrollStudentRequest,
    responses((status = 201, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
async fn enroll_student(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<EnrollStudentRequest>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let uc = EnrollStudentUseCase {
        enrollment_repo: state.enrollment_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let e = uc.execute(&ctx, ctx.tenant_id, p.period_id, p.student_id, p.class_id, p.student_number, p.enrollment_type).await?;
    Ok(ApiResponse::ok(EnrollmentDto {
        id: e.id, student_id: e.student_id, class_id: e.class_id,
        period_id: e.period_id, org_id: e.org_id, student_number: e.student_number,
        enrollment_type: e.enrollment_type, status: e.status,
        enrollment_date: e.enrollment_date,
    }))
}

#[utoipa::path(get, path = "/api/v1/enrollments",
    responses((status = 200, body = Vec<EnrollmentDto>)),
    security(("bearer" = [])), tag = "Student")]
async fn list_enrollments(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<EnrollmentQuery>,
) -> Result<Json<ApiResponse<Vec<EnrollmentDto>>>, ApiError> {
    let enrollments = if let Some(class_id) = q.class_id {
        state.enrollment_repo.find_by_class(class_id).await?
    } else if let Some(student_id) = q.student_id {
        state.enrollment_repo.find_by_student(student_id).await?
    } else {
        state.enrollment_repo.find_by_org(ctx.tenant_id).await?
    };
    let dtos = enrollments.into_iter().map(|e| EnrollmentDto {
        id: e.id, student_id: e.student_id, class_id: e.class_id,
        period_id: e.period_id, org_id: e.org_id, student_number: e.student_number,
        enrollment_type: e.enrollment_type, status: e.status,
        enrollment_date: e.enrollment_date,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/enrollments/{id}",
    params(("id" = Uuid, Path, description = "Enrollment ID")),
    responses((status = 200, body = EnrollmentDto)),
    security(("bearer" = [])), tag = "Student")]
async fn get_enrollment(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<EnrollmentDto>>, ApiError> {
    let e = state.enrollment_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Enrollment {} not found", id)))?;
    Ok(ApiResponse::ok(EnrollmentDto {
        id: e.id, student_id: e.student_id, class_id: e.class_id,
        period_id: e.period_id, org_id: e.org_id, student_number: e.student_number,
        enrollment_type: e.enrollment_type, status: e.status,
        enrollment_date: e.enrollment_date,
    }))
}

#[utoipa::path(post, path = "/api/v1/student-guardians", request_body = AssignGuardianRequest,
    responses((status = 201, body = GuardianDto)),
    security(("bearer" = [])), tag = "Student")]
async fn assign_guardian(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignGuardianRequest>,
) -> Result<Json<ApiResponse<GuardianDto>>, ApiError> {
    let uc = AssignGuardianUseCase {
        student_guardian_repo: state.student_guardian_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let g = uc.execute(&ctx, p.student_id, p.guardian_id, p.relationship, p.is_primary).await?;
    Ok(ApiResponse::ok(GuardianDto {
        id: g.id, student_id: g.student_id, guardian_id: g.guardian_id,
        relationship: g.relationship, is_primary: g.is_primary,
    }))
}

#[utoipa::path(get, path = "/api/v1/student-guardians",
    params(("student_id" = Uuid, Query, description = "Student ID")),
    responses((status = 200, body = Vec<GuardianDto>)),
    security(("bearer" = [])), tag = "Student")]
async fn list_guardians(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<GuardianQuery>,
) -> Result<Json<ApiResponse<Vec<GuardianDto>>>, ApiError> {
    let guardians = state.student_guardian_repo.find_by_student(q.student_id).await?;
    let dtos = guardians.into_iter().map(|g| GuardianDto {
        id: g.id, student_id: g.student_id, guardian_id: g.guardian_id,
        relationship: g.relationship, is_primary: g.is_primary,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/enrollments", post(enroll_student).get(list_enrollments))
        .route("/api/v1/enrollments/:id", get(get_enrollment))
        .route("/api/v1/student-guardians", post(assign_guardian).get(list_guardians))
}
