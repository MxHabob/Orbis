//! Assessment routes — Schemes, Grades, Attendance, Behavior.

use axum::{extract::{State, Path, Query}, routing::{get, post, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDate;
use uuid::Uuid;
use rust_decimal::Decimal;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::services::assessment_service::{BatchGradeItem, BatchAttendanceItem};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssessmentSchemeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub weight: Option<Decimal>,
    pub letter_grade: Option<String>,
    pub is_final: bool,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub change_reason: Option<String>,
    pub recorded_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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

#[derive(Deserialize, ToSchema)]
pub struct RecordGradeItemRequest {
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

#[derive(Deserialize, ToSchema)]
pub struct BatchRecordGradesRequest {
    pub items: Vec<RecordGradeItemRequest>,
}

#[derive(Serialize, ToSchema)]
pub struct BatchRecordGradesResponseDto {
    pub grades: Vec<GradeDto>,
    pub count: usize,
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
    pub marked_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
pub struct MarkAttendanceItemRequest {
    pub student_id: Uuid,
    pub status: String,
    pub notes: Option<String>,
    pub class_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct BatchMarkAttendanceRequest {
    pub class_id: Uuid,
    pub date: NaiveDate,
    pub items: Vec<MarkAttendanceItemRequest>,
}

#[derive(Serialize, ToSchema)]
pub struct BatchMarkAttendanceResponseDto {
    pub attendances: Vec<AttendanceDto>,
    pub count: usize,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct BehaviorRecordDto {
    pub id: Uuid,
    pub student_id: Uuid,
    pub org_id: Uuid,
    pub reporter_id: Uuid,
    pub record_type: String,
    pub description: String,
    pub points: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct RecordBehaviorRequest {
    pub student_id: Uuid,
    pub record_type: String,
    pub description: String,
    pub points: i32,
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

#[derive(Deserialize, ToSchema)]
pub struct CalculateGpaRequest {
    pub student_id: Uuid,
    pub period_id: Uuid,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CumulativeGpaRecordDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub student_id: Uuid,
    pub period_id: Uuid,
    pub term_attempted_credits: Decimal,
    pub term_earned_credits: Decimal,
    pub term_points: Decimal,
    pub term_gpa: Decimal,
    pub cumulative_attempted_credits: Decimal,
    pub cumulative_earned_credits: Decimal,
    pub cumulative_points: Decimal,
    pub cumulative_gpa: Decimal,
    pub standing: String,
    pub calculated_at: chrono::DateTime<chrono::Utc>,
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

#[derive(Deserialize)]
pub struct GradeQuery { pub student_id: Option<Uuid>, pub class_subject_id: Option<Uuid> }

#[derive(Deserialize)]
pub struct AttendanceQuery { pub class_id: Uuid, pub date: NaiveDate }

fn grade_to_dto(g: orbis_domain::entities::grade::Grade) -> GradeDto {
    GradeDto {
        id: g.id,
        org_id: g.org_id,
        period_id: g.period_id,
        student_id: g.student_id,
        enrollment_id: g.enrollment_id,
        class_subject_id: g.class_subject_id,
        component_id: g.component_id,
        score: g.score,
        max_score: g.max_score,
        weight: g.weight,
        letter_grade: g.letter_grade,
        is_final: g.is_final,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        change_reason: g.change_reason,
        recorded_by: g.recorded_by,
        updated_by: g.updated_by,
        notes: g.notes,
        created_at: g.created_at,
        updated_at: g.updated_at,
    }
}

fn attendance_to_dto(a: orbis_domain::entities::attendance::Attendance) -> AttendanceDto {
    AttendanceDto {
        id: a.id,
        student_id: a.student_id,
        class_id: a.class_id,
        class_subject_id: a.class_subject_id,
        scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.attendance_date,
        status: a.status,
        notes: a.notes,
        marked_by: a.marked_by,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }
}

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
        created_at: s.created_at, updated_at: s.updated_at,
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
        created_at: s.created_at, updated_at: s.updated_at,
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
        created_at: s.created_at, updated_at: s.updated_at,
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
        created_at: s.created_at, updated_at: s.updated_at,
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

    Ok(ApiResponse::ok(grade_to_dto(g)))
}

#[utoipa::path(post, path = "/api/v1/grades/batch", request_body = BatchRecordGradesRequest,
    responses((status = 200, body = BatchRecordGradesResponseDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn batch_record_grades(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<BatchRecordGradesRequest>,
) -> Result<Json<ApiResponse<BatchRecordGradesResponseDto>>, ApiError> {
    let items = p.items.into_iter().map(|i| BatchGradeItem {
        period_id: i.period_id,
        student_id: i.student_id,
        enrollment_id: i.enrollment_id,
        class_subject_id: i.class_subject_id,
        component_id: i.component_id,
        score: i.score,
        max_score: i.max_score,
        is_excused: i.is_excused.unwrap_or(false),
        is_exempt: i.is_exempt.unwrap_or(false),
        notes: i.notes,
    }).collect();

    let grades = state.assessment_service.batch_record_grades(&ctx, ctx.tenant_id, items).await?;
    let dtos: Vec<GradeDto> = grades.into_iter().map(grade_to_dto).collect();
    let count = dtos.len();

    Ok(ApiResponse::ok(BatchRecordGradesResponseDto {
        grades: dtos,
        count,
    }))
}

#[utoipa::path(delete, path = "/api/v1/grades/{id}",
    params(("id" = Uuid, Path, description = "Grade ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_grade(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.assessment_service.delete_grade(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(()))
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
    let dtos = grades.into_iter().map(grade_to_dto).collect();
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

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate-gpa", request_body = CalculateGpaRequest,
    responses((status = 200, body = CumulativeGpaRecordDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn calculate_gpa(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<CalculateGpaRequest>,
) -> Result<Json<ApiResponse<CumulativeGpaRecordDto>>, ApiError> {
    let record = state.assessment_service.calculate_gpa(&ctx, ctx.tenant_id, p.period_id, p.student_id).await?;
    Ok(ApiResponse::ok(CumulativeGpaRecordDto {
        id: record.id,
        org_id: record.org_id,
        student_id: record.student_id,
        period_id: record.period_id,
        term_attempted_credits: record.term_attempted_credits,
        term_earned_credits: record.term_earned_credits,
        term_points: record.term_points,
        term_gpa: record.term_gpa,
        cumulative_attempted_credits: record.cumulative_attempted_credits,
        cumulative_earned_credits: record.cumulative_earned_credits,
        cumulative_points: record.cumulative_points,
        cumulative_gpa: record.cumulative_gpa,
        standing: format!("{:?}", record.standing),
        calculated_at: record.calculated_at,
    }))
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
    Ok(ApiResponse::ok(attendance_to_dto(a)))
}

#[utoipa::path(post, path = "/api/v1/attendance/batch", request_body = BatchMarkAttendanceRequest,
    responses((status = 200, body = BatchMarkAttendanceResponseDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn batch_mark_attendance(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<BatchMarkAttendanceRequest>,
) -> Result<Json<ApiResponse<BatchMarkAttendanceResponseDto>>, ApiError> {
    let items = p.items.into_iter().map(|i| BatchAttendanceItem {
        student_id: i.student_id,
        status: i.status,
        notes: i.notes,
        class_subject_id: i.class_subject_id,
        scheduled_lesson_id: i.scheduled_lesson_id,
    }).collect();

    let list = state.assessment_service.batch_mark_attendance(&ctx, ctx.tenant_id, p.class_id, p.date, items).await?;
    let dtos: Vec<AttendanceDto> = list.into_iter().map(attendance_to_dto).collect();
    let count = dtos.len();

    Ok(ApiResponse::ok(BatchMarkAttendanceResponseDto {
        attendances: dtos,
        count,
    }))
}

#[utoipa::path(delete, path = "/api/v1/attendance/{id}",
    params(("id" = Uuid, Path, description = "Attendance ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_attendance(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.assessment_service.delete_attendance(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/attendance",
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_attendance(
    _ctx: AuthContext, State(state): State<AppState>, Query(q): Query<AttendanceQuery>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let list = state.attendance_repo.find_by_class_and_date(q.class_id, q.date).await?;
    let dtos = list.into_iter().map(attendance_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/attendance",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_student_attendance(
    ctx: AuthContext, State(state): State<AppState>, Path(student_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let list = state.assessment_service.list_student_attendance(&ctx, student_id).await?;
    let dtos = list.into_iter().map(attendance_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/behavior", request_body = RecordBehaviorRequest,
    responses((status = 201, body = BehaviorRecordDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn record_behavior(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<RecordBehaviorRequest>,
) -> Result<Json<ApiResponse<BehaviorRecordDto>>, ApiError> {
    let r = state.assessment_service.record_behavior(
        &ctx,
        ctx.tenant_id,
        p.student_id,
        p.record_type,
        p.description,
        p.points,
    ).await?;
    Ok(ApiResponse::ok(BehaviorRecordDto {
        id: r.id, student_id: r.student_id, org_id: r.org_id,
        reporter_id: r.reporter_id, record_type: r.record_type,
        description: r.description, points: r.points,
        created_at: r.created_at, updated_at: r.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/behavior/{id}",
    params(("id" = Uuid, Path, description = "Behavior Record ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_behavior(
    ctx: AuthContext, State(state): State<AppState>, Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state.assessment_service.delete_behavior(&ctx, ctx.tenant_id, id).await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/behavior",
    responses((status = 200, body = Vec<BehaviorRecordDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_all_behavior(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<BehaviorRecordDto>>>, ApiError> {
    let list = state.assessment_service.list_behavior_records(&ctx, ctx.tenant_id).await?;
    let dtos = list.into_iter().map(|r| BehaviorRecordDto {
        id: r.id, student_id: r.student_id, org_id: r.org_id,
        reporter_id: r.reporter_id, record_type: r.record_type,
        description: r.description, points: r.points,
        created_at: r.created_at, updated_at: r.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/behavior",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = Vec<BehaviorRecordDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_student_behavior(
    ctx: AuthContext, State(state): State<AppState>, Path(student_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<BehaviorRecordDto>>>, ApiError> {
    let list = state.assessment_service.list_student_behavior_records(&ctx, student_id).await?;
    let dtos = list.into_iter().map(|r| BehaviorRecordDto {
        id: r.id, student_id: r.student_id, org_id: r.org_id,
        reporter_id: r.reporter_id, record_type: r.record_type,
        description: r.description, points: r.points,
        created_at: r.created_at, updated_at: r.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/assessment-schemes", post(create_scheme).get(list_schemes))
        .route("/api/v1/assessment-schemes/:id", get(get_scheme).put(update_scheme).delete(delete_scheme))
        .route("/api/v1/grades", post(record_grade).get(list_grades))
        .route("/api/v1/grades/batch", post(batch_record_grades))
        .route("/api/v1/grades/:id", delete(delete_grade))
        .route("/api/v1/grades/actions/calculate", post(calculate_grades))
        .route("/api/v1/grades/actions/calculate-gpa", post(calculate_gpa))
        .route("/api/v1/grades/actions/calculate-rankings", post(calculate_rankings))
        .route("/api/v1/attendance", post(mark_attendance).get(list_attendance))
        .route("/api/v1/attendance/batch", post(batch_mark_attendance))
        .route("/api/v1/attendance/:id", delete(delete_attendance))
        .route("/api/v1/students/:id/attendance", get(list_student_attendance))
        .route("/api/v1/behavior", post(record_behavior).get(list_all_behavior))
        .route("/api/v1/behavior/:id", delete(delete_behavior))
        .route("/api/v1/students/:id/behavior", get(list_student_behavior))
}
