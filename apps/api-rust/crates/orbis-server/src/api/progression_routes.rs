//! Progression routes — Student Cohort Progression & Promotion.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::progression_service::StudentProgressionItem;
use orbis_domain::entities::progression::{
    ProgressionDetail, ProgressionJob, ProgressionJobStatus,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProgressionJobDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub source_period_id: Uuid,
    pub target_period_id: Option<Uuid>,
    pub name: String,
    pub status: String,
    pub total_students: u32,
    pub promoted_count: u32,
    pub retained_count: u32,
    pub graduated_count: u32,
    pub excluded_count: u32,
    pub executed_by: Option<Uuid>,
    pub executed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ProgressionDetailDto {
    pub id: Uuid,
    pub job_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
    pub decision: String,
    pub reason: String,
    pub manual_override: bool,
    pub override_by: Option<Uuid>,
    pub override_reason: Option<String>,
    pub final_gpa: Option<Decimal>,
    pub attendance_rate: Option<Decimal>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct StudentProgressionItemInput {
    pub student_id: Uuid,
    #[serde(default)]
    pub student_number: Option<String>,
    #[serde(default)]
    pub student_name: Option<String>,
    #[serde(default)]
    pub student_name_ar: Option<String>,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
    pub decision: String,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub final_gpa: Option<Decimal>,
    #[serde(default)]
    pub overall_percentage: Option<Decimal>,
    #[serde(default)]
    pub attendance_rate: Option<Decimal>,
    #[serde(default)]
    pub failed_subjects_count: u32,
    #[serde(default)]
    pub financial_balance_due: Decimal,
    #[serde(default)]
    pub has_financial_hold: bool,
    #[serde(default)]
    pub already_enrolled_in_target: bool,
    #[serde(default)]
    pub manual_override: bool,
    #[serde(default)]
    pub override_reason: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct PromoteCohortRequest {
    pub source_period_id: Uuid,
    pub target_period_id: Uuid,
    pub job_name: String,
    pub items: Vec<StudentProgressionItemInput>,
    #[serde(default)]
    pub auto_assess_fees: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
pub struct EvaluateCohortRequest {
    pub source_period_id: Uuid,
    #[serde(default)]
    pub target_period_id: Option<Uuid>,
}

fn job_to_dto(job: ProgressionJob) -> ProgressionJobDto {
    ProgressionJobDto {
        id: job.id,
        org_id: job.org_id,
        source_period_id: job.source_period_id,
        target_period_id: job.target_period_id,
        name: job.name,
        status: match job.status {
            ProgressionJobStatus::Draft => "draft".into(),
            ProgressionJobStatus::PreviewReady => "preview_ready".into(),
            ProgressionJobStatus::Executing => "executing".into(),
            ProgressionJobStatus::Completed => "completed".into(),
            ProgressionJobStatus::Failed => "failed".into(),
            ProgressionJobStatus::RolledBack => "rolled_back".into(),
        },
        total_students: job.total_students,
        promoted_count: job.promoted_count,
        retained_count: job.retained_count,
        graduated_count: job.graduated_count,
        excluded_count: job.excluded_count,
        executed_by: job.executed_by,
        executed_at: job.executed_at,
        created_at: job.created_at,
        updated_at: job.updated_at,
    }
}

fn detail_to_dto(detail: ProgressionDetail) -> ProgressionDetailDto {
    ProgressionDetailDto {
        id: detail.id,
        job_id: detail.job_id,
        student_id: detail.student_id,
        enrollment_id: detail.enrollment_id,
        source_section_id: detail.source_section_id,
        target_section_id: detail.target_section_id,
        decision: detail.decision.to_string(),
        reason: detail.reason,
        manual_override: detail.manual_override,
        override_by: detail.override_by,
        override_reason: detail.override_reason,
        final_gpa: detail.final_gpa,
        attendance_rate: detail.attendance_rate,
        created_at: detail.created_at,
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/progression/evaluate", request_body = EvaluateCohortRequest,
    responses((status = 200, body = Vec<StudentProgressionItemInput>)), security(("bearer" = [])), tag = "Progression")]
pub async fn evaluate_cohort(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<EvaluateCohortRequest>,
) -> Result<Json<ApiResponse<Vec<StudentProgressionItemInput>>>, ApiError> {
    let items = state
        .progression_service
        .evaluate_cohort_with_target(&ctx, ctx.tenant_id, p.source_period_id, p.target_period_id)
        .await?;
    let dtos = items
        .into_iter()
        .map(|i| StudentProgressionItemInput {
            student_id: i.student_id,
            student_number: i.student_number,
            student_name: i.student_name,
            student_name_ar: i.student_name_ar,
            enrollment_id: i.enrollment_id,
            source_section_id: i.source_section_id,
            target_section_id: i.target_section_id,
            decision: i.decision.to_string(),
            reason: i.reason,
            final_gpa: i.final_gpa,
            overall_percentage: i.overall_percentage,
            attendance_rate: i.attendance_rate,
            failed_subjects_count: i.failed_subjects_count,
            financial_balance_due: i.financial_balance_due,
            has_financial_hold: i.has_financial_hold,
            already_enrolled_in_target: i.already_enrolled_in_target,
            manual_override: i.manual_override,
            override_reason: i.override_reason,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/progression/promote", request_body = PromoteCohortRequest,
    responses((status = 201, body = ProgressionJobDto)), security(("bearer" = [])), tag = "Progression")]
pub async fn promote_cohort(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<PromoteCohortRequest>,
) -> Result<Json<ApiResponse<ProgressionJobDto>>, ApiError> {
    let items = p
        .items
        .into_iter()
        .map(|i| {
            Ok(StudentProgressionItem {
                student_id: i.student_id,
                student_number: i.student_number,
                student_name: i.student_name,
                student_name_ar: i.student_name_ar,
                enrollment_id: i.enrollment_id,
                source_section_id: i.source_section_id,
                target_section_id: i.target_section_id,
                decision: i.decision.parse()?,
                reason: i.reason,
                final_gpa: i.final_gpa,
                overall_percentage: i.overall_percentage,
                attendance_rate: i.attendance_rate,
                failed_subjects_count: i.failed_subjects_count,
                financial_balance_due: i.financial_balance_due,
                has_financial_hold: i.has_financial_hold,
                already_enrolled_in_target: i.already_enrolled_in_target,
                manual_override: i.manual_override,
                override_reason: i.override_reason,
            })
        })
        .collect::<Result<Vec<_>, orbis_domain::errors::DomainError>>()?;

    let job = state
        .progression_service
        .promote_cohort_with_options(
            &ctx,
            ctx.tenant_id,
            p.source_period_id,
            p.target_period_id,
            p.job_name,
            items,
            p.auto_assess_fees,
        )
        .await?;
    Ok(ApiResponse::ok(job_to_dto(job)))
}

#[utoipa::path(get, path = "/api/v1/progression/jobs",
    responses((status = 200, body = Vec<ProgressionJobDto>)), security(("bearer" = [])), tag = "Progression")]
pub async fn list_jobs(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ProgressionJobDto>>>, ApiError> {
    let jobs = state.progression_service.list_jobs(&ctx).await?;
    let dtos = jobs.into_iter().map(job_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/progression/jobs/{id}", params(("id" = Uuid, Path, description = "Promotion Job ID")),
    responses((status = 200, body = ProgressionJobDto)), security(("bearer" = [])), tag = "Progression")]
pub async fn get_job(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProgressionJobDto>>, ApiError> {
    let job = state.progression_service.get_job(&ctx, id).await?;
    Ok(ApiResponse::ok(job_to_dto(job)))
}

#[utoipa::path(get, path = "/api/v1/progression/jobs/{id}/details", params(("id" = Uuid, Path, description = "Promotion Job ID")),
    responses((status = 200, body = Vec<ProgressionDetailDto>)), security(("bearer" = [])), tag = "Progression")]
pub async fn get_job_details(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<ProgressionDetailDto>>>, ApiError> {
    let details = state.progression_service.get_job_details(&ctx, id).await?;
    Ok(ApiResponse::ok(
        details.into_iter().map(detail_to_dto).collect(),
    ))
}

#[utoipa::path(post, path = "/api/v1/progression/jobs/{id}/rollback", params(("id" = Uuid, Path, description = "Promotion Job ID")),
    responses((status = 200, body = ProgressionJobDto)), security(("bearer" = [])), tag = "Progression")]
pub async fn rollback_job(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProgressionJobDto>>, ApiError> {
    let job = state.progression_service.rollback_job(&ctx, id).await?;
    Ok(ApiResponse::ok(job_to_dto(job)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/progression/evaluate", post(evaluate_cohort))
        .route("/api/v1/progression/promote", post(promote_cohort))
        .route("/api/v1/progression/jobs", get(list_jobs))
        .route("/api/v1/progression/jobs/:id", get(get_job))
        .route("/api/v1/progression/jobs/:id/details", get(get_job_details))
        .route("/api/v1/progression/jobs/:id/rollback", post(rollback_job))
}
