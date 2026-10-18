//! Progression routes — Student Cohort Progression & Promotion.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::progression_service::StudentProgressionItem;
use orbis_domain::entities::progression::{
    ProgressionJob, ProgressionJobStatus, PromotionDecision,
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
pub struct StudentProgressionItemInput {
    pub student_id: Uuid,
    #[serde(default)]
    pub student_number: Option<String>,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
    pub decision: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PromoteCohortRequest {
    pub source_period_id: Uuid,
    pub target_period_id: Uuid,
    pub job_name: String,
    pub items: Vec<StudentProgressionItemInput>,
}

#[derive(Deserialize, ToSchema)]
pub struct EvaluateCohortRequest {
    pub source_period_id: Uuid,
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

fn parse_decision(s: &str) -> PromotionDecision {
    match s.to_lowercase().as_str() {
        "retain" | "retained" => PromotionDecision::Retain,
        "graduate" | "graduated" => PromotionDecision::Graduate,
        "exclude" | "excluded" => PromotionDecision::Exclude,
        _ => PromotionDecision::Promote,
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
        .evaluate_cohort_progression(&ctx, ctx.tenant_id, p.source_period_id)
        .await?;
    let dtos = items
        .into_iter()
        .map(|i| StudentProgressionItemInput {
            student_id: i.student_id,
            student_number: i.student_number,
            enrollment_id: i.enrollment_id,
            source_section_id: i.source_section_id,
            target_section_id: i.target_section_id,
            decision: i.decision.to_string(),
            reason: i.reason,
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
        .map(|i| StudentProgressionItem {
            student_id: i.student_id,
            student_number: i.student_number,
            enrollment_id: i.enrollment_id,
            source_section_id: i.source_section_id,
            target_section_id: i.target_section_id,
            decision: parse_decision(&i.decision),
            reason: i.reason,
        })
        .collect();

    let job = state
        .progression_service
        .promote_cohort(
            &ctx,
            ctx.tenant_id,
            p.source_period_id,
            p.target_period_id,
            p.job_name,
            items,
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
    let jobs = state
        .progression_service
        .progression_repo
        .find_jobs_by_org(ctx.tenant_id)
        .await?;
    let dtos = jobs.into_iter().map(job_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/progression/jobs/{id}", params(("id" = Uuid, Path, description = "Promotion Job ID")),
    responses((status = 200, body = ProgressionJobDto)), security(("bearer" = [])), tag = "Progression")]
pub async fn get_job(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ProgressionJobDto>>, ApiError> {
    let job = state
        .progression_service
        .progression_repo
        .find_job_by_id(id)
        .await?
        .ok_or_else(|| ApiError::not_found("Job not found"))?;
    Ok(ApiResponse::ok(job_to_dto(job)))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/progression/evaluate", post(evaluate_cohort))
        .route("/api/v1/progression/promote", post(promote_cohort))
        .route("/api/v1/progression/jobs", get(list_jobs))
        .route("/api/v1/progression/jobs/:id", get(get_job))
}
