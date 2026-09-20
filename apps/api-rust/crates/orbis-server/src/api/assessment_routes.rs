//! Assessment routes — Schemes, Grades, Attendance, Behavior.

use axum::{extract::{State, Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDate;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssessmentSchemeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSchemeRequest {
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_score: Option<f64>,
    pub formula: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GradeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub class_subject_id: Uuid,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Decimal,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct RecordGradeRequest {
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub class_subject_id: Uuid,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Option<Decimal>,
    pub is_excused: Option<bool>,
    pub is_exempt: Option<bool>,
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
    pub class_subject_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema)]
pub struct CalculationResultDto {
    pub final_score: Decimal,
    pub message: String,
}

#[derive(Deserialize)]
pub struct GradeQuery { pub student_id: Option<Uuid>, pub class_subject_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct AttendanceQuery { pub class_id: Uuid, pub date: NaiveDate }

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/assessment-schemes", request_body = CreateSchemeRequest,
    responses((status = 201, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn create_scheme(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CreateSchemeRequest>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state.assessment_service.create_assessment_scheme(
        &ctx,
        ctx.tenant_id,
        p.name,
        p.max_score,
        p.formula,
    ).await?;
    Ok(ApiResponse::ok(AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score, formula: s.formula,
    }))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes",
    responses((status = 200, body = Vec<AssessmentSchemeDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_schemes(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AssessmentSchemeDto>>>, ApiError> {
    let schemes = state.assessment_service.list_assessment_schemes(ctx.tenant_id).await?;
    let dtos = schemes.into_iter().map(|s| AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score, formula: s.formula,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes/{id}",
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_scheme(
    _ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state.assessment_scheme_repo.find_by_id(id).await?.ok_or_else(|| ApiError::not_found("Scheme not found"))?;
    Ok(ApiResponse::ok(AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score, formula: s.formula,
    }))
}

#[utoipa::path(put, path = "/api/v1/assessment-schemes/{id}", request_body = UpdateSchemeRequest,
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn update_scheme(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>, Json(p): Json<UpdateSchemeRequest>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state.assessment_service.update_assessment_scheme(
        &ctx,
        ctx.tenant_id,
        id,
        p.name,
        p.description,
        p.max_score,
        p.formula,
    ).await?;
    Ok(ApiResponse::ok(AssessmentSchemeDto {
        id: s.id, org_id: s.org_id, name: s.name, description: s.description, max_score: s.max_score, formula: s.formula,
    }))
}

#[utoipa::path(delete, path = "/api/v1/assessment-schemes/{id}",
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_scheme(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.assessment_service.delete_assessment_scheme(
        &ctx,
        ctx.tenant_id,
        id,
    ).await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(post, path = "/api/v1/grades", request_body = RecordGradeRequest,
    responses((status = 201, body = GradeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn record_grade(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<RecordGradeRequest>,
) -> Result<Json<ApiResponse<GradeDto>>, ApiError> {
    let g = state.assessment_service.record_grade(
        &ctx,
        ctx.tenant_id,
        p.period_id,
        p.student_id,
        p.class_subject_id,
        p.component_id,
        p.score,
        p.max_score.unwrap_or_else(|| Decimal::from(100)),
        false,
    ).await?;

    Ok(ApiResponse::ok(GradeDto {
        id: g.id,
        org_id: g.org_id,
        period_id: g.period_id,
        student_id: g.student_id,
        enrollment_id: g.enrollment_id,
        class_subject_id: g.class_subject_id,
        component_id: g.component_id,
        score: g.score,
        max_score: g.max_score,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        notes: g.notes,
    }))
}

#[utoipa::path(get, path = "/api/v1/grades",
    responses((status = 200, body = Vec<GradeDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_grades(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<GradeQuery>,
) -> Result<Json<ApiResponse<Vec<GradeDto>>>, ApiError> {
    let grades = if let Some(sid) = q.student_id {
        state.grade_repo.find_by_student(sid).await?
    } else if let Some(csid) = q.class_subject_id {
        state.grade_repo.find_by_class_subject(csid).await?
    } else { vec![] };
    let dtos = grades.into_iter().map(|g| GradeDto {
        id: g.id,
        org_id: g.org_id,
        period_id: g.period_id,
        student_id: g.student_id,
        enrollment_id: g.enrollment_id,
        class_subject_id: g.class_subject_id,
        component_id: g.component_id,
        score: g.score,
        max_score: g.max_score,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        notes: g.notes,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate", request_body = CalculateGradesRequest,
    responses((status = 200, body = CalculationResultDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn calculate_grades(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CalculateGradesRequest>,
) -> Result<Json<ApiResponse<CalculationResultDto>>, ApiError> {
    let class_subject_id = p.class_subject_id.unwrap_or_else(Uuid::nil);
    let score = state.assessment_service.calculate_scheme_grades(&ctx, p.scheme_id, p.student_id, class_subject_id).await?;
    Ok(ApiResponse::ok(CalculationResultDto {
        final_score: score,
        message: "Grade calculated successfully".into(),
    }))
}

#[derive(Deserialize, ToSchema)]
pub struct CalculateTermResultRequest {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub period_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct CalculateAnnualResultRequest {
    pub student_id: Uuid,
    pub class_id: Uuid,
    pub academic_year_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct CalculateRankingsRequest {
    pub items: Vec<ScoreItemRequest>,
    pub tie_policy: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ScoreItemRequest {
    pub id: Uuid,
    pub score: Decimal,
}

#[derive(Serialize, ToSchema)]
pub struct RankingItemDto {
    pub id: Uuid,
    pub rank: u32,
}

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate-rankings", request_body = CalculateRankingsRequest,
    responses((status = 200, body = Vec<RankingItemDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn calculate_rankings(
    _ctx: AuthContext, State(_state): State<AppState>, Json(p): Json<CalculateRankingsRequest>,
) -> Result<Json<ApiResponse<Vec<RankingItemDto>>>, ApiError> {
    use orbis_domain::entities::academic_policy::TiePolicy;
    use orbis_app::services::calculation_engine::AcademicCalculationEngine;

    let tie_policy = match p.tie_policy.as_deref() {
        Some("dense") => TiePolicy::Dense,
        Some("fractional") => TiePolicy::Fractional,
        _ => TiePolicy::StandardCompetition,
    };

    let items_tuples: Vec<(Uuid, Decimal)> = p.items.into_iter().map(|i| (i.id, i.score)).collect();
    let rankings = AcademicCalculationEngine::compute_rankings(&items_tuples, tie_policy);

    let dtos = rankings.into_iter().map(|(id, rank)| RankingItemDto { id, rank }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/attendance", request_body = MarkAttendanceRequest,
    responses((status = 201, body = AttendanceDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn mark_attendance(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<MarkAttendanceRequest>,
) -> Result<Json<ApiResponse<AttendanceDto>>, ApiError> {
    let a = state.assessment_service.mark_attendance(
        &ctx,
        ctx.tenant_id,
        Uuid::nil(),
        p.student_id,
        Some(p.class_id),
        None,
        p.date,
        p.status,
        p.notes,
    ).await?;
    Ok(ApiResponse::ok(AttendanceDto {
        id: a.id, student_id: a.student_id, class_id: a.class_id,
        class_subject_id: a.class_subject_id, scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.attendance_date, status: a.status, notes: a.notes,
    }))
}

#[utoipa::path(get, path = "/api/v1/attendance",
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_attendance(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<AttendanceQuery>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let list = state.attendance_repo.find_by_class_and_date(q.class_id, q.date).await?;
    let dtos = list.into_iter().map(|a| AttendanceDto {
        id: a.id, student_id: a.student_id, class_id: a.class_id,
        class_subject_id: a.class_subject_id, scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.attendance_date, status: a.status, notes: a.notes,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/assessment-schemes", post(create_scheme).get(list_schemes))
        .route("/api/v1/assessment-schemes/:id", get(get_scheme).put(update_scheme).delete(delete_scheme))
        .route("/api/v1/grades", post(record_grade).get(list_grades))
        .route("/api/v1/grades/actions/calculate", post(calculate_grades))
        .route("/api/v1/grades/actions/calculate-rankings", post(calculate_rankings))
        .route("/api/v1/attendance", post(mark_attendance).get(list_attendance))
}
