//! Assessment routes — Schemes, Grades, Attendance, Behavior.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, post},
};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::assessment_service::{BatchAttendanceItem, BatchGradeItem};
use orbis_domain::entities::academic_policy::{
    AcademicPolicy, AcademicResultView, ResultSnapshot, StudentAnnualReportCard,
    StudentCertificateReportCard, StudentTermReportCard, TiePolicy,
};
use orbis_domain::entities::academic_calculation::{
    CalculationPreview, CalculationOutputOutcome,
};
use orbis_domain::entities::assessment_scheme::{
    AssessmentComponent, AssessmentLevel, AssessmentScheme, InstitutionScope,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AssessmentSchemeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Decimal,
    pub formula: Option<serde_json::Value>,
    pub level: AssessmentLevel,
    pub institution_scope: InstitutionScope,
    pub components: Vec<AssessmentComponent>,
    pub is_default: bool,
    pub is_system: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSchemeRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Decimal,
    pub formula: Option<serde_json::Value>,
    pub level: Option<AssessmentLevel>,
    pub institution_scope: Option<InstitutionScope>,
    pub components: Option<Vec<AssessmentComponent>>,
    pub is_default: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSchemeRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Option<Decimal>,
    pub formula: Option<serde_json::Value>,
    pub level: Option<AssessmentLevel>,
    pub institution_scope: Option<InstitutionScope>,
    pub components: Option<Vec<AssessmentComponent>>,
    pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GradeDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub section_subject_id: Uuid,
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
    pub section_subject_id: Uuid,
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
    pub section_subject_id: Uuid,
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
    pub section_id: Option<Uuid>,
    pub section_subject_id: Option<Uuid>,
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
    pub section_id: Uuid,
    pub date: NaiveDate,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct MarkAttendanceItemRequest {
    pub student_id: Uuid,
    pub status: String,
    pub notes: Option<String>,
    pub section_subject_id: Option<Uuid>,
    pub scheduled_lesson_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct BatchMarkAttendanceRequest {
    pub section_id: Uuid,
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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AcademicPolicyDto {
    pub org_id: Uuid,
    pub passing_threshold_percentage: Decimal,
    pub honors_threshold_percentage: Decimal,
    pub minimum_final_exam_score: Option<Decimal>,
    pub max_failed_subjects_for_conditional_pass: usize,
    pub rounding_mode: String,
    pub decimal_places: u32,
    pub ranking_scope: String,
    pub tie_policy: String,
    pub ranking_enabled: bool,
    pub max_displayed_rank: Option<u32>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AcademicResultViewDto {
    pub snapshot_id: Uuid,
    pub org_id: Uuid,
    pub learner_id: Uuid,
    pub subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub result_type: String,
    pub result_key: String,
    pub policy_version: u32,
    pub revision: u32,
    pub state: String,
    pub produced_at: chrono::DateTime<chrono::Utc>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub payload: serde_json::Value,
}

#[derive(Deserialize, ToSchema)]
pub struct AcademicResultPreviewRequest {
    pub learner_id: Uuid,
    pub result_type: String,
    pub result_key: String,
    pub subject_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub payload: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CalculationPreviewRequest {
    pub values: HashMap<String, Decimal>,
}

#[derive(Deserialize, ToSchema)]
pub struct CalculateGradesRequest {
    pub scheme_id: Uuid,
    pub student_id: Uuid,
    pub section_subject_id: Option<Uuid>,
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
    pub tie_policy: Option<TiePolicy>,
}

#[derive(Deserialize, ToSchema)]
pub struct ScoreItemRequest {
    pub id: Uuid,
    pub score: Decimal,
}

#[derive(Serialize, ToSchema)]
pub struct RankingItemDto {
    pub id: Uuid,
    pub rank: Decimal,
}

#[derive(Deserialize)]
pub struct GradeQuery {
    pub student_id: Option<Uuid>,
    pub section_subject_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct AttendanceQuery {
    pub section_id: Uuid,
    pub date: NaiveDate,
}

fn grade_to_dto(g: orbis_domain::entities::grade::Grade) -> GradeDto {
    GradeDto {
        id: g.id,
        org_id: g.org_id,
        period_id: g.period_id,
        student_id: g.student_id,
        enrollment_id: g.enrollment_id,
        section_subject_id: g.section_subject_id,
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
        section_id: a.section_id,
        section_subject_id: a.section_subject_id,
        scheduled_lesson_id: a.scheduled_lesson_id,
        date: a.attendance_date,
        status: a.status,
        notes: a.notes,
        marked_by: a.marked_by,
        created_at: a.created_at,
        updated_at: a.updated_at,
    }
}

fn scheme_to_dto(s: AssessmentScheme) -> AssessmentSchemeDto {
    AssessmentSchemeDto {
        id: s.id,
        org_id: s.org_id,
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        max_score: s.max_score,
        formula: s.formula,
        level: s.level,
        institution_scope: s.institution_scope,
        components: s.components,
        is_default: s.is_default,
        is_system: s.is_system,
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}

fn academic_policy_to_dto(policy: AcademicPolicy) -> AcademicPolicyDto {
    AcademicPolicyDto {
        org_id: policy.org_id,
        passing_threshold_percentage: policy.passing_threshold_percentage,
        honors_threshold_percentage: policy.honors_threshold_percentage,
        minimum_final_exam_score: policy.minimum_final_exam_score,
        max_failed_subjects_for_conditional_pass: policy.max_failed_subjects_for_conditional_pass,
        rounding_mode: format!("{:?}", policy.rounding_mode),
        decimal_places: policy.decimal_places,
        ranking_scope: format!("{:?}", policy.ranking_scope),
        tie_policy: format!("{:?}", policy.tie_policy),
        ranking_enabled: policy.ranking_enabled,
        max_displayed_rank: policy.max_displayed_rank,
    }
}

fn academic_result_view_to_dto(view: AcademicResultView) -> AcademicResultViewDto {
    AcademicResultViewDto {
        snapshot_id: view.snapshot_id,
        org_id: view.org_id,
        learner_id: view.learner_id,
        subject_id: view.subject_id,
        period_id: view.period_id,
        calendar_id: view.calendar_id,
        result_type: view.result_type,
        result_key: view.result_key,
        policy_version: view.policy_version,
        revision: view.revision,
        state: format!("{:?}", view.state),
        produced_at: view.produced_at,
        approved_by: view.approved_by,
        approved_at: view.approved_at,
        payload: view.payload,
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/assessment-schemes", request_body = CreateSchemeRequest,
    responses((status = 201, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn create_scheme(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateSchemeRequest>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state
        .assessment_service
        .create_assessment_scheme(
            &ctx,
            ctx.tenant_id,
            p.name,
            p.name_ar,
            p.description,
            p.max_score,
            p.formula,
            p.level,
            p.institution_scope,
            p.components,
            p.is_default.unwrap_or(false),
        )
        .await?;
    Ok(ApiResponse::ok(scheme_to_dto(s)))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes",
    responses((status = 200, body = Vec<AssessmentSchemeDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_schemes(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AssessmentSchemeDto>>>, ApiError> {
    let schemes = state
        .assessment_service
        .list_assessment_schemes(ctx.tenant_id)
        .await?;
    let dtos = schemes.into_iter().map(scheme_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/assessment-schemes/{id}",
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_scheme(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state
        .assessment_scheme_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Scheme not found"))?;
    Ok(ApiResponse::ok(scheme_to_dto(s)))
}

#[utoipa::path(put, path = "/api/v1/assessment-schemes/{id}", request_body = UpdateSchemeRequest,
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = AssessmentSchemeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn update_scheme(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateSchemeRequest>,
) -> Result<Json<ApiResponse<AssessmentSchemeDto>>, ApiError> {
    let s = state
        .assessment_service
        .update_assessment_scheme(
            &ctx,
            ctx.tenant_id,
            id,
            p.name,
            p.name_ar,
            p.description,
            p.max_score,
            p.formula,
            p.level,
            p.institution_scope,
            p.components,
            p.is_default,
        )
        .await?;
    Ok(ApiResponse::ok(scheme_to_dto(s)))
}

#[utoipa::path(delete, path = "/api/v1/assessment-schemes/{id}",
    params(("id" = Uuid, Path, description = "Assessment Scheme ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_scheme(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .assessment_service
        .delete_assessment_scheme(&ctx, ctx.tenant_id, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/academic-policy",
    responses((status = 200, body = AcademicPolicyDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_current_academic_policy(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AcademicPolicyDto>>, ApiError> {
    let settings = state.settings_repo.get_settings(ctx.tenant_id).await?;
    let policy = settings.academic.to_academic_policy(ctx.tenant_id);
    Ok(ApiResponse::ok(academic_policy_to_dto(policy)))
}

#[utoipa::path(post, path = "/api/v1/academic-results/preview", request_body = AcademicResultPreviewRequest,
    responses((status = 200, body = AcademicResultViewDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn preview_academic_result(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AcademicResultPreviewRequest>,
) -> Result<Json<ApiResponse<AcademicResultViewDto>>, ApiError> {
    let settings = state.settings_repo.get_settings(ctx.tenant_id).await?;
    let _policy = settings.academic.to_academic_policy(ctx.tenant_id);
    let snapshot = ResultSnapshot::new(
        ctx.tenant_id,
        p.learner_id,
        p.result_type,
        p.result_key,
        1,
        Utc::now(),
        p.payload,
    )
    .with_context(
        p.subject_id,
        p.period_id,
        p.calendar_id,
        ctx.user_id(),
        vec![],
    );

    let view: AcademicResultView = snapshot.into();
    let dto = academic_result_view_to_dto(view);
    Ok(ApiResponse::ok(dto))
}

pub async fn preview_configured_calculation(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(request): Json<CalculationPreviewRequest>,
) -> Result<Json<ApiResponse<CalculationPreview>>, ApiError> {
    let settings = state.settings_repo.get_settings(ctx.tenant_id).await?;
    let plan = settings
        .academic
        .calculation_plan
        .as_ref()
        .ok_or_else(|| ApiError::bad_request("No calculation plan is configured"))?;
    let result = orbis_app::services::calculation_engine::AcademicCalculationEngine::calculate_plan(
        plan,
        &request.values,
    )?;
    let mut outcomes = Vec::new();
    if let Some(policy) = settings.academic.outcome_policy.as_ref() {
        for output in &result.outputs {
            if output.unit == policy.unit {
                outcomes.push(CalculationOutputOutcome {
                    output_name: output.name.clone(),
                    outcome: orbis_app::services::calculation_engine::AcademicCalculationEngine::evaluate_outcome(
                        output.value,
                        &output.unit,
                        policy,
                    )?,
                });
            }
        }
    }

    Ok(ApiResponse::ok(CalculationPreview { result, outcomes }))
}

#[utoipa::path(post, path = "/api/v1/grades", request_body = RecordGradeRequest,
    responses((status = 201, body = GradeDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn record_grade(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<RecordGradeRequest>,
) -> Result<Json<ApiResponse<GradeDto>>, ApiError> {
    let g = state
        .assessment_service
        .record_grade(
            &ctx,
            ctx.tenant_id,
            p.period_id,
            p.student_id,
            p.section_subject_id,
            p.component_id,
            p.score,
            p.max_score.unwrap_or_else(|| Decimal::from(100)),
            false,
        )
        .await?;

    Ok(ApiResponse::ok(grade_to_dto(g)))
}

#[utoipa::path(post, path = "/api/v1/grades/batch", request_body = BatchRecordGradesRequest,
    responses((status = 200, body = BatchRecordGradesResponseDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn batch_record_grades(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BatchRecordGradesRequest>,
) -> Result<Json<ApiResponse<BatchRecordGradesResponseDto>>, ApiError> {
    let items = p
        .items
        .into_iter()
        .map(|i| BatchGradeItem {
            period_id: i.period_id,
            student_id: i.student_id,
            enrollment_id: i.enrollment_id,
            section_subject_id: i.section_subject_id,
            component_id: i.component_id,
            score: i.score,
            max_score: i.max_score,
            is_excused: i.is_excused.unwrap_or(false),
            is_exempt: i.is_exempt.unwrap_or(false),
            notes: i.notes,
        })
        .collect();

    let grades = state
        .assessment_service
        .batch_record_grades(&ctx, ctx.tenant_id, items)
        .await?;
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
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .assessment_service
        .delete_grade(&ctx, ctx.tenant_id, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/grades",
    responses((status = 200, body = Vec<GradeDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_grades(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<GradeQuery>,
) -> Result<Json<ApiResponse<Vec<GradeDto>>>, ApiError> {
    let grades = if let Some(sid) = q.student_id {
        state.grade_repo.find_by_student(sid).await?
    } else if let Some(csid) = q.section_subject_id {
        state.grade_repo.find_by_section_subject(csid).await?
    } else {
        vec![]
    };
    let dtos = grades.into_iter().map(grade_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate", request_body = CalculateGradesRequest,
    responses((status = 200, body = CalculationResultDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn calculate_grades(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CalculateGradesRequest>,
) -> Result<Json<ApiResponse<CalculationResultDto>>, ApiError> {
    let section_subject_id = p.section_subject_id.unwrap_or_else(Uuid::nil);
    let score = state
        .assessment_service
        .calculate_scheme_grades(&ctx, p.scheme_id, p.student_id, section_subject_id)
        .await?;
    Ok(ApiResponse::ok(CalculationResultDto {
        final_score: score,
        message: "Grade calculated successfully".into(),
    }))
}

#[utoipa::path(post, path = "/api/v1/grades/actions/calculate-gpa", request_body = CalculateGpaRequest,
    responses((status = 200, body = CumulativeGpaRecordDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn calculate_gpa(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CalculateGpaRequest>,
) -> Result<Json<ApiResponse<CumulativeGpaRecordDto>>, ApiError> {
    let record = state
        .assessment_service
        .calculate_gpa(&ctx, ctx.tenant_id, p.period_id, p.student_id)
        .await?;
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
    _ctx: AuthContext,
    State(_state): State<AppState>,
    Json(p): Json<CalculateRankingsRequest>,
) -> Result<Json<ApiResponse<Vec<RankingItemDto>>>, ApiError> {
    use orbis_app::services::calculation_engine::AcademicCalculationEngine;
    let tie_policy = p.tie_policy.unwrap_or_default();

    let items_tuples: Vec<(Uuid, Decimal)> = p.items.into_iter().map(|i| (i.id, i.score)).collect();
    let rankings = AcademicCalculationEngine::compute_rankings(&items_tuples, tie_policy);

    let dtos = rankings
        .into_iter()
        .map(|(id, rank)| RankingItemDto { id, rank })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/attendance", request_body = MarkAttendanceRequest,
    responses((status = 201, body = AttendanceDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn mark_attendance(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<MarkAttendanceRequest>,
) -> Result<Json<ApiResponse<AttendanceDto>>, ApiError> {
    let a = state
        .assessment_service
        .mark_attendance(
            &ctx,
            ctx.tenant_id,
            Uuid::nil(),
            p.student_id,
            Some(p.section_id),
            None,
            p.date,
            p.status,
            p.notes,
        )
        .await?;
    Ok(ApiResponse::ok(attendance_to_dto(a)))
}

#[utoipa::path(post, path = "/api/v1/attendance/batch", request_body = BatchMarkAttendanceRequest,
    responses((status = 200, body = BatchMarkAttendanceResponseDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn batch_mark_attendance(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BatchMarkAttendanceRequest>,
) -> Result<Json<ApiResponse<BatchMarkAttendanceResponseDto>>, ApiError> {
    let items = p
        .items
        .into_iter()
        .map(|i| BatchAttendanceItem {
            student_id: i.student_id,
            status: i.status,
            notes: i.notes,
            section_subject_id: i.section_subject_id,
            scheduled_lesson_id: i.scheduled_lesson_id,
        })
        .collect();

    let list = state
        .assessment_service
        .batch_mark_attendance(&ctx, ctx.tenant_id, p.section_id, p.date, items)
        .await?;
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
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .assessment_service
        .delete_attendance(&ctx, ctx.tenant_id, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/attendance",
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_attendance(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<AttendanceQuery>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let list = state
        .attendance_repo
        .find_by_section_and_date(q.section_id, q.date)
        .await?;
    let dtos = list.into_iter().map(attendance_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/attendance",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = Vec<AttendanceDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_student_attendance(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<AttendanceDto>>>, ApiError> {
    let list = state
        .assessment_service
        .list_student_attendance(&ctx, student_id)
        .await?;
    let dtos = list.into_iter().map(attendance_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/behavior", request_body = RecordBehaviorRequest,
    responses((status = 201, body = BehaviorRecordDto)), security(("bearer" = [])), tag = "Assessment")]
pub async fn record_behavior(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<RecordBehaviorRequest>,
) -> Result<Json<ApiResponse<BehaviorRecordDto>>, ApiError> {
    let r = state
        .assessment_service
        .record_behavior(
            &ctx,
            ctx.tenant_id,
            p.student_id,
            p.record_type,
            p.description,
            p.points,
        )
        .await?;
    Ok(ApiResponse::ok(BehaviorRecordDto {
        id: r.id,
        student_id: r.student_id,
        org_id: r.org_id,
        reporter_id: r.reporter_id,
        record_type: r.record_type,
        description: r.description,
        points: r.points,
        created_at: r.created_at,
        updated_at: r.updated_at,
    }))
}

#[utoipa::path(delete, path = "/api/v1/behavior/{id}",
    params(("id" = Uuid, Path, description = "Behavior Record ID")),
    responses((status = 200, body = ())), security(("bearer" = [])), tag = "Assessment")]
pub async fn delete_behavior(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    state
        .assessment_service
        .delete_behavior(&ctx, ctx.tenant_id, id)
        .await?;
    Ok(ApiResponse::ok(()))
}

#[utoipa::path(get, path = "/api/v1/behavior",
    responses((status = 200, body = Vec<BehaviorRecordDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_all_behavior(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<BehaviorRecordDto>>>, ApiError> {
    let list = state
        .assessment_service
        .list_behavior_records(&ctx, ctx.tenant_id)
        .await?;
    let dtos = list
        .into_iter()
        .map(|r| BehaviorRecordDto {
            id: r.id,
            student_id: r.student_id,
            org_id: r.org_id,
            reporter_id: r.reporter_id,
            record_type: r.record_type,
            description: r.description,
            points: r.points,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/behavior",
    params(("id" = Uuid, Path, description = "Student ID")),
    responses((status = 200, body = Vec<BehaviorRecordDto>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn list_student_behavior(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<BehaviorRecordDto>>>, ApiError> {
    let list = state
        .assessment_service
        .list_student_behavior_records(&ctx, student_id)
        .await?;
    let dtos = list
        .into_iter()
        .map(|r| BehaviorRecordDto {
            id: r.id,
            student_id: r.student_id,
            org_id: r.org_id,
            reporter_id: r.reporter_id,
            record_type: r.record_type,
            description: r.description,
            points: r.points,
            created_at: r.created_at,
            updated_at: r.updated_at,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[derive(Deserialize, ToSchema)]
pub struct StudentTermResultQuery {
    pub period_id: Uuid,
    pub section_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct StudentAnnualResultQuery {
    pub year_id: Uuid,
    pub section_id: Option<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct ClassTermResultQuery {
    pub period_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct ClassAnnualResultQuery {
    pub year_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct StudentCertificateResultQuery {
    pub calendar_id: Uuid,
    pub section_id: Option<Uuid>,
    /// "basic_grade9" or "secondary_thanawiya"
    pub certificate_type: String,
}

#[utoipa::path(get, path = "/api/v1/students/{id}/term-results",
    params(
        ("id" = Uuid, Path, description = "Student ID"),
        ("period_id" = Uuid, Query, description = "Term Period ID"),
        ("class_id" = Option<Uuid>, Query, description = "Optional Class ID"),
    ),
    responses((status = 200, body = StudentTermReportCard)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_student_term_results(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    Query(q): Query<StudentTermResultQuery>,
) -> Result<Json<ApiResponse<StudentTermReportCard>>, ApiError> {
    let report = state
        .assessment_service
        .calculate_student_term_report_card(
            &ctx,
            ctx.tenant_id,
            student_id,
            q.period_id,
            q.section_id,
            None,
        )
        .await?;
    Ok(ApiResponse::ok(report))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/annual-results",
    params(
        ("id" = Uuid, Path, description = "Student ID"),
        ("year_id" = Uuid, Query, description = "Academic Year ID"),
        ("class_id" = Option<Uuid>, Query, description = "Optional Class ID"),
    ),
    responses((status = 200, body = StudentAnnualReportCard)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_student_annual_results(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    Query(q): Query<StudentAnnualResultQuery>,
) -> Result<Json<ApiResponse<StudentAnnualReportCard>>, ApiError> {
    let report = state
        .assessment_service
        .calculate_student_annual_report_card(
            &ctx,
            ctx.tenant_id,
            student_id,
            q.year_id,
            q.section_id,
            None,
        )
        .await?;
    Ok(ApiResponse::ok(report))
}

#[utoipa::path(get, path = "/api/v1/students/{id}/certificate-results",
    params(
        ("id" = Uuid, Path, description = "Student ID"),
        ("calendar_id" = Uuid, Query, description = "Academic Year/Calendar ID"),
        ("section_id" = Option<Uuid>, Query, description = "Optional Class ID"),
        ("certificate_type" = String, Query, description = "Certificate type: basic_grade9 or secondary_thanawiya"),
    ),
    responses((status = 200, body = StudentCertificateReportCard)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_student_certificate_results(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(student_id): Path<Uuid>,
    Query(q): Query<StudentCertificateResultQuery>,
) -> Result<Json<ApiResponse<StudentCertificateReportCard>>, ApiError> {
    let report = state
        .assessment_service
        .calculate_student_certificate_report_card(
            &ctx,
            ctx.tenant_id,
            student_id,
            q.calendar_id,
            q.section_id,
            q.certificate_type,
            None,
        )
        .await?;
    Ok(ApiResponse::ok(report))
}

#[utoipa::path(get, path = "/api/v1/classes/{id}/term-results",
    params(
        ("id" = Uuid, Path, description = "Class ID"),
        ("period_id" = Uuid, Query, description = "Term Period ID"),
    ),
    responses((status = 200, body = Vec<StudentTermReportCard>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_class_term_results(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(section_id): Path<Uuid>,
    Query(q): Query<ClassTermResultQuery>,
) -> Result<Json<ApiResponse<Vec<StudentTermReportCard>>>, ApiError> {
    let reports = state
        .assessment_service
        .calculate_class_term_report_cards(&ctx, ctx.tenant_id, section_id, q.period_id, None)
        .await?;
    Ok(ApiResponse::ok(reports))
}

#[utoipa::path(get, path = "/api/v1/classes/{id}/annual-results",
    params(
        ("id" = Uuid, Path, description = "Class ID"),
        ("year_id" = Uuid, Query, description = "Academic Year ID"),
    ),
    responses((status = 200, body = Vec<StudentAnnualReportCard>)), security(("bearer" = [])), tag = "Assessment")]
pub async fn get_class_annual_results(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(section_id): Path<Uuid>,
    Query(q): Query<ClassAnnualResultQuery>,
) -> Result<Json<ApiResponse<Vec<StudentAnnualReportCard>>>, ApiError> {
    let reports = state
        .assessment_service
        .calculate_class_annual_report_cards(&ctx, ctx.tenant_id, section_id, q.year_id, None)
        .await?;
    Ok(ApiResponse::ok(reports))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/assessment-schemes",
            post(create_scheme).get(list_schemes),
        )
        .route(
            "/api/v1/assessment-schemes/:id",
            get(get_scheme).put(update_scheme).delete(delete_scheme),
        )
        .route("/api/v1/academic-policy", get(get_current_academic_policy))
        .route(
            "/api/v1/academic-results/preview",
            post(preview_academic_result),
        )
        .route(
            "/api/v1/academic-calculations/preview",
            post(preview_configured_calculation),
        )
        .route("/api/v1/grades", post(record_grade).get(list_grades))
        .route("/api/v1/grades/batch", post(batch_record_grades))
        .route("/api/v1/grades/:id", delete(delete_grade))
        .route("/api/v1/grades/actions/calculate", post(calculate_grades))
        .route("/api/v1/grades/actions/calculate-gpa", post(calculate_gpa))
        .route(
            "/api/v1/grades/actions/calculate-rankings",
            post(calculate_rankings),
        )
        .route(
            "/api/v1/attendance",
            post(mark_attendance).get(list_attendance),
        )
        .route("/api/v1/attendance/batch", post(batch_mark_attendance))
        .route("/api/v1/attendance/:id", delete(delete_attendance))
        .route(
            "/api/v1/students/:id/attendance",
            get(list_student_attendance),
        )
        .route(
            "/api/v1/students/:id/term-results",
            get(get_student_term_results),
        )
        .route(
            "/api/v1/students/:id/annual-results",
            get(get_student_annual_results),
        )
        .route(
            "/api/v1/classes/:id/term-results",
            get(get_class_term_results),
        )
        .route(
            "/api/v1/classes/:id/annual-results",
            get(get_class_annual_results),
        )
        .route(
            "/api/v1/behavior",
            post(record_behavior).get(list_all_behavior),
        )
        .route("/api/v1/behavior/:id", delete(delete_behavior))
        .route("/api/v1/students/:id/behavior", get(list_student_behavior))
        .route(
            "/api/v1/students/:id/certificate-results",
            get(get_student_certificate_results),
        )
}
