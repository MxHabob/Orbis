//! Autopilot routes — Task Orchestration & Policy Automation.

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::autopilot::task::AutopilotTask;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AutopilotTaskDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub task_type: String,
    pub status: String,
    pub target_id: Option<Uuid>,
    pub target_type: Option<String>,
    pub risk_level: String,
    pub reason: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub scheduled_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub attempts: i32,
    pub max_retries: i32,
    pub error_log: Option<String>,
    pub idempotency_key: Option<String>,
    pub triggered_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AutopilotTask> for AutopilotTaskDto {
    fn from(t: AutopilotTask) -> Self {
        Self {
            id: t.id,
            org_id: t.org_id,
            task_type: t.task_type.as_str().to_string(),
            status: t.status.as_str().to_string(),
            target_id: t.target_id,
            target_type: t.target_type,
            risk_level: t.risk_level.as_str().to_string(),
            reason: t.reason,
            parameters: t.parameters,
            scheduled_at: t.scheduled_at,
            executed_at: t.executed_at,
            attempts: t.attempts,
            max_retries: t.max_retries,
            error_log: t.error_log,
            idempotency_key: t.idempotency_key,
            triggered_by: t.triggered_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AutopilotDiagnosticsDto {
    pub is_healthy: bool,
    pub status: String,
    pub mode: String,
    pub stuck_tasks: Vec<AutopilotTaskDto>,
    pub overdue_tasks: Vec<AutopilotTaskDto>,
    pub total_tasks_count: u64,
    pub pending_tasks_count: u64,
    pub completed_tasks_count: u64,
    pub failed_tasks_count: u64,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct RunTickRequest {
    pub dry_run: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct ListTasksQuery {
    pub status: Option<String>,
    pub task_type: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct ListTasksResponse {
    pub total_count: u64,
    pub items: Vec<AutopilotTaskDto>,
}

// ── Handlers ───────────────────────────────────────────────────

/// List autopilot tasks with optional filters
#[utoipa::path(
    get,
    path = "/api/v1/autopilot/tasks",
    tag = "Autopilot",
    params(
        ("status" = Option<String>, Query, description = "Filter by task status (pending, executing, completed, failed, suggested, cancelled)"),
        ("task_type" = Option<String>, Query, description = "Filter by task type"),
        ("limit" = Option<u64>, Query, description = "Pagination limit (default 20)"),
        ("offset" = Option<u64>, Query, description = "Pagination offset (default 0)"),
    ),
    responses(
        (status = 200, description = "Paginated autopilot tasks list", body = ApiResponse<ListTasksResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_tasks(
    State(state): State<AppState>,
    auth: AuthContext,
    Query(query): Query<ListTasksQuery>,
) -> Result<Json<ApiResponse<ListTasksResponse>>, ApiError> {
    let ctx = auth.0;
    let limit = query.limit.unwrap_or(20);
    let offset = query.offset.unwrap_or(0);

    let (tasks, total) = state
        .autopilot_service
        .list_tasks(
            &ctx,
            ctx.tenant_id,
            query.status.as_deref(),
            query.task_type.as_deref(),
            limit,
            offset,
        )
        .await?;

    let items = tasks.into_iter().map(AutopilotTaskDto::from).collect();
    Ok(ApiResponse::ok(ListTasksResponse {
        total_count: total,
        items,
    }))
}

/// Trigger an immediate Autopilot evaluation tick
#[utoipa::path(
    post,
    path = "/api/v1/autopilot/tick",
    tag = "Autopilot",
    request_body = Option<RunTickRequest>,
    responses(
        (status = 200, description = "Evaluated autopilot tasks list", body = ApiResponse<Vec<AutopilotTaskDto>>),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn run_tick(
    State(state): State<AppState>,
    auth: AuthContext,
    payload: Option<Json<RunTickRequest>>,
) -> Result<Json<ApiResponse<Vec<AutopilotTaskDto>>>, ApiError> {
    let ctx = auth.0;
    let dry_run = payload.and_then(|p| p.dry_run).unwrap_or(false);

    let tasks = state
        .autopilot_service
        .run_tick(&ctx, ctx.tenant_id, dry_run)
        .await?;

    let dtos = tasks.into_iter().map(AutopilotTaskDto::from).collect();
    Ok(ApiResponse::ok(dtos))
}

/// Get details of an Autopilot task
#[utoipa::path(
    get,
    path = "/api/v1/autopilot/tasks/{id}",
    tag = "Autopilot",
    params(
        ("id" = Uuid, Path, description = "Task UUID")
    ),
    responses(
        (status = 200, description = "Autopilot task detail", body = ApiResponse<AutopilotTaskDto>),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_task(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutopilotTaskDto>>, ApiError> {
    let ctx = auth.0;
    let task = state
        .autopilot_service
        .get_task(&ctx, id)
        .await?
        .ok_or_else(|| ApiError::not_found("Task not found"))?;

    Ok(ApiResponse::ok(AutopilotTaskDto::from(task)))
}

/// Manually execute a pending or suggested Autopilot task
#[utoipa::path(
    post,
    path = "/api/v1/autopilot/tasks/{id}/execute",
    tag = "Autopilot",
    params(
        ("id" = Uuid, Path, description = "Task UUID")
    ),
    responses(
        (status = 200, description = "Executed task result", body = ApiResponse<AutopilotTaskDto>),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn execute_task(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutopilotTaskDto>>, ApiError> {
    let ctx = auth.0;
    let task = state.autopilot_service.execute_task(&ctx, id).await?;
    Ok(ApiResponse::ok(AutopilotTaskDto::from(task)))
}

/// Retry a failed Autopilot task
#[utoipa::path(
    post,
    path = "/api/v1/autopilot/tasks/{id}/retry",
    tag = "Autopilot",
    params(
        ("id" = Uuid, Path, description = "Task UUID")
    ),
    responses(
        (status = 200, description = "Retried task result", body = ApiResponse<AutopilotTaskDto>),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn retry_task(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutopilotTaskDto>>, ApiError> {
    let ctx = auth.0;
    let task = state.autopilot_service.retry_task(&ctx, id).await?;
    Ok(ApiResponse::ok(AutopilotTaskDto::from(task)))
}

/// Dismiss / Cancel a suggested Autopilot task
#[utoipa::path(
    post,
    path = "/api/v1/autopilot/tasks/{id}/dismiss",
    tag = "Autopilot",
    params(
        ("id" = Uuid, Path, description = "Task UUID")
    ),
    responses(
        (status = 200, description = "Dismissed task result", body = ApiResponse<AutopilotTaskDto>),
        (status = 404, description = "Task not found"),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn dismiss_task(
    State(state): State<AppState>,
    auth: AuthContext,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AutopilotTaskDto>>, ApiError> {
    let ctx = auth.0;
    let task = state.autopilot_service.dismiss_task(&ctx, id).await?;
    Ok(ApiResponse::ok(AutopilotTaskDto::from(task)))
}

/// Get Autopilot system diagnostics and health status
#[utoipa::path(
    get,
    path = "/api/v1/autopilot/diagnostics",
    tag = "Autopilot",
    responses(
        (status = 200, description = "Autopilot diagnostics summary", body = ApiResponse<AutopilotDiagnosticsDto>),
        (status = 401, description = "Unauthorized")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_diagnostics(
    State(state): State<AppState>,
    auth: AuthContext,
) -> Result<Json<ApiResponse<AutopilotDiagnosticsDto>>, ApiError> {
    let ctx = auth.0;
    let diag = state
        .autopilot_service
        .get_diagnostics(&ctx, ctx.tenant_id)
        .await?;

    let dto = AutopilotDiagnosticsDto {
        is_healthy: diag.is_healthy,
        status: diag.status,
        mode: diag.mode,
        stuck_tasks: diag.stuck_tasks.into_iter().map(AutopilotTaskDto::from).collect(),
        overdue_tasks: diag.overdue_tasks.into_iter().map(AutopilotTaskDto::from).collect(),
        total_tasks_count: diag.total_tasks_count,
        pending_tasks_count: diag.pending_tasks_count,
        completed_tasks_count: diag.completed_tasks_count,
        failed_tasks_count: diag.failed_tasks_count,
    };

    Ok(ApiResponse::ok(dto))
}

// ── Router ─────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/autopilot/tasks", get(list_tasks))
        .route("/api/v1/autopilot/tick", post(run_tick))
        .route("/api/v1/autopilot/tasks/:id", get(get_task))
        .route("/api/v1/autopilot/tasks/:id/execute", post(execute_task))
        .route("/api/v1/autopilot/tasks/:id/retry", post(retry_task))
        .route("/api/v1/autopilot/tasks/:id/dismiss", post(dismiss_task))
        .route("/api/v1/autopilot/diagnostics", get(get_diagnostics))
}
