//! Assessment routes — Schemes, Grades, Attendance, Behavior.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDate;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::assessment::{
    assessment_scheme::CreateAssessmentSchemeUseCase,
    grade::RecordGradeUseCase,
    attendance::MarkAttendanceUseCase,
    calculate_grades::CalculateGradesUseCase,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssessmentSchemeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSchemeRequest {
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GradeDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_subject_id: Uuid,
    pub scheme_id: Uuid,
    pub score: f64,
    pub is_excused: bool,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RecordGradeRequest {
    pub student_id: Uuid,
    pub class_subject_id: Uuid,
    pub scheme_id: Uuid,
    pub score: f64,
    pub is_excused: bool,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AttendanceDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub class_id: Option<Uuid>,
    pub class_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
    pub date: NaiveDate,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct MarkAttendanceRequest {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub date: NaiveDate,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CalculateGradesRequest {
    pub scheme_id: Uuid,
    pub student_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct CalculationResultDto {
    pub final_score: f64,
    pub message: String,
}

#[derive(Deserialize)]
pub struct GradeQuery { pub student_id: Option<Uuid>, pub class_subject_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct AttendanceQuery { pub class_id: Uuid, pub date: NaiveDate }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/assessment-schemes", request_body = CreateSchemeRequest,
    responses((status = 201, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
async fn create_scheme(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateSchemeRequest>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let uc = CreateAssessmentSchemeUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
        permission_checker: state.permission_checker.clone(),
    };
    let s = uc.execute(&ctx, ctx.tenant_id, p.name, p.description, p.max_score, p.formula).await?;
    Ok(ApiResponse::ok(AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score,
    }))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes",
    responses((status = 200, body = Vec<AssessmentSchemeDto>)), security(("bearer" = [])), tag = "Assessment")]
async fn list_schemes(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AssessmentSchemeDto>>>, ApiError> {
    let schemes = state.assessment_scheme_repo.find_by_org(ctx.tenant_id).await?;
    let dtos = schemes.into_iter().map(|s| AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes/{id}",
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
async fn get_scheme(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state.assessment_scheme_repo.find_by_id(id).await?.ok_or_else(|| ApiError::not_found("Scheme not found"))?;
    Ok(ApiResponse::ok(AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score,
    }))
}

#[utoipa::path(post, path = "/api/v1/grades", request_body = RecordGradeRequest,
    responses((status = 201, body = GradeDto)), security(("bearer" = [])), tag = "Assessment")]
async fn record_grade(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<RecordGradeRequest>,
) -> Result<Json<ApiResponse<GradeDto>>, ApiError> {
    let uc = RecordGradeUseCase {
        grade_repo: state.grade_repo.clone(),
        event_bus: state.event_bus.clone(),
        permission_checker: state.permission_checker.clone(),
    };
    let g = uc.execute(&ctx, p.student_id, p.class_subject_id, p.scheme_id, p.score, p.is_excused, p.notes).await?;
    Ok(ApiResponse::ok(GradeDto {
        id: g.id, student_id: g.student_id, class_subject_id: g.class_subject_id,
        scheme_id: g.scheme_id, score: g.score, is_excused: g.is_excused, notes: g.notes,
    }))
}

#[utoipa::path(get, path = "/api/v1/grades",
    responses((status = 200, body = Vec<GradeDto>)), security(("bearer" = [])), tag = "Assessment")]
async fn list_grades(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<GradeQuery>,
) -> Result<Json<ApiResponse<Vec<GradeDto>>>, ApiError> {
    let grades = if let Some(sid) = q.student_id {
        state.grade_repo.find_by_student(sid).await?
    } else if let Some(csid) = q.class_subject_id {
        state.grade_repo.find_by_class_subject(csid).await?
    } else { vec![] };
    let dtos = grades.into_iter().map(|g| GradeDto {
        id: g.id, student_id: g.student_id, class_subject_id: g.class_subject_id,
        scheme_id: g.scheme_id, score: g.score, is_excused: g.is_excused, notes: g.notes,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate", request_body = CalculateGradesRequest,
    responses((status = 200, body = CalculationResultDto)), security(("bearer" = [])), tag = "Assessment")]
async fn calculate_grades(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CalculateGradesRequest>,
) -> Result<Json<ApiResponse<CalculationResultDto>>, ApiError> {
    let uc = CalculateGradesUseCase {
        scheme_repo: state.assessment_scheme_repo.clone(),
        grade_repo: state.grade_repo.clone(),
    };
    let score = uc.execute(&ctx, p.scheme_id, p.student_id).await?;
    Ok(ApiResponse::ok(CalculationResultDto {
        final_score: score,
        message: "Grade calculated successfully".into(),
    }))
}

#[utoipa::path(post, path = "/api/v1/attendance", request_body = MarkAttendanceRequest,
    responses((status = 201, body = AttendanceDto)), security(("bearer" = [])), tag = "Assessment")]
async fn mark_attendance(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<MarkAttendanceRequest>,
) -> Result<Json<ApiResponse<AttendanceDto>>, ApiError> {
    let uc = MarkAttendanceUseCase {
        attendance_repo: state.attendance_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let a = uc.execute(&ctx, p.student_id, p.class_id, p.date, p.status, p.notes).await?;
    Ok(ApiResponse::ok(AttendanceDto {
        id: a.id, student_id: a.student_id, class_id: a.class_id,
        class_subject_id: a.class_subject_id, scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.date, status: a.status, notes: a.notes,
    }))
}

#[utoipa::path(get, path = "/api/v1/attendance",
    params(("class_id" = Uuid, Query, description = "Class ID"), ("date" = NaiveDate, Query, description = "Attendance Date")),
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
async fn list_attendance(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<AttendanceQuery>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let records = state.attendance_repo.find_by_class_and_date(q.class_id, q.date).await?;
    let dtos = records.into_iter().map(|a| AttendanceDto {
        id: a.id, student_id: a.student_id, class_id: a.class_id,
        class_subject_id: a.class_subject_id, scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.date, status: a.status, notes: a.notes,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/assessment-schemes", post(create_scheme).get(list_schemes))
        .route("/api/v1/assessment-schemes/{id}", get(get_scheme))
        .route("/api/v1/grades", post(record_grade).get(list_grades))
        .route("/api/v1/grades/actions/calculate", post(calculate_grades))
        .route("/api/v1/attendance", post(mark_attendance).get(list_attendance))
}

