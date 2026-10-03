//! Tauri IPC commands — Orbis Autopilot System.

use tauri::State;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;
use orbis_domain::autopilot::task::AutopilotTask;
use super::make_ctx;
use super::org_cmds::ApiResponse;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AutopilotTaskDto {
    pub id: String,
    pub org_id: String,
    pub task_type: String,
    pub status: String,
    pub target_id: Option<String>,
    pub target_type: Option<String>,
    pub risk_level: String,
    pub reason: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub scheduled_at: String,
    pub executed_at: Option<String>,
    pub attempts: i32,
    pub max_retries: i32,
    pub error_log: Option<String>,
    pub idempotency_key: Option<String>,
    pub triggered_by: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<AutopilotTask> for AutopilotTaskDto {
    fn from(t: AutopilotTask) -> Self {
        Self {
            id: t.id.to_string(),
            org_id: t.org_id.to_string(),
            task_type: t.task_type.as_str().to_string(),
            status: t.status.as_str().to_string(),
            target_id: t.target_id.map(|id| id.to_string()),
            target_type: t.target_type,
            risk_level: t.risk_level.as_str().to_string(),
            reason: t.reason,
            parameters: t.parameters,
            scheduled_at: t.scheduled_at.to_rfc3339(),
            executed_at: t.executed_at.map(|d| d.to_rfc3339()),
            attempts: t.attempts,
            max_retries: t.max_retries,
            error_log: t.error_log,
            idempotency_key: t.idempotency_key,
            triggered_by: t.triggered_by,
            created_at: t.created_at.to_rfc3339(),
            updated_at: t.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListTasksResponse {
    pub total_count: u64,
    pub items: Vec<AutopilotTaskDto>,
}

#[tauri::command]
pub async fn autopilot_list_tasks(
    state: State<'_, AppState>,
    status: Option<String>,
    task_type: Option<String>,
    limit: Option<u64>,
    offset: Option<u64>,
) -> Result<ApiResponse<ListTasksResponse>, String> {
    let ctx = make_ctx(&state);
    let limit = limit.unwrap_or(20);
    let offset = offset.unwrap_or(0);

    let (tasks, total) = state
        .autopilot_service
        .list_tasks(
            &ctx,
            ctx.tenant_id,
            status.as_deref(),
            task_type.as_deref(),
            limit,
            offset,
        )
        .await
        .map_err(|e| e.to_string())?;

    let items = tasks.into_iter().map(AutopilotTaskDto::from).collect();
    Ok(ok(ListTasksResponse {
        total_count: total,
        items,
    }))
}

#[tauri::command]
pub async fn autopilot_run_tick(
    state: State<'_, AppState>,
    dry_run: Option<bool>,
) -> Result<ApiResponse<Vec<AutopilotTaskDto>>, String> {
    let ctx = make_ctx(&state);
    let dry_run = dry_run.unwrap_or(false);

    let tasks = state
        .autopilot_service
        .run_tick(&ctx, ctx.tenant_id, dry_run)
        .await
        .map_err(|e| e.to_string())?;

    let dtos = tasks.into_iter().map(AutopilotTaskDto::from).collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn autopilot_get_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<ApiResponse<AutopilotTaskDto>, String> {
    let ctx = make_ctx(&state);
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let task = state
        .autopilot_service
        .get_task(&ctx, uuid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "Autopilot task not found".to_string())?;

    Ok(ok(AutopilotTaskDto::from(task)))
}

#[tauri::command]
pub async fn autopilot_execute_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<ApiResponse<AutopilotTaskDto>, String> {
    let ctx = make_ctx(&state);
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let task = state
        .autopilot_service
        .execute_task(&ctx, uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(AutopilotTaskDto::from(task)))
}

#[tauri::command]
pub async fn autopilot_retry_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<ApiResponse<AutopilotTaskDto>, String> {
    let ctx = make_ctx(&state);
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let task = state
        .autopilot_service
        .retry_task(&ctx, uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(AutopilotTaskDto::from(task)))
}

#[tauri::command]
pub async fn autopilot_dismiss_task(
    state: State<'_, AppState>,
    id: String,
) -> Result<ApiResponse<AutopilotTaskDto>, String> {
    let ctx = make_ctx(&state);
    let uuid = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let task = state
        .autopilot_service
        .dismiss_task(&ctx, uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(AutopilotTaskDto::from(task)))
}

#[tauri::command]
pub async fn autopilot_get_diagnostics(
    state: State<'_, AppState>,
) -> Result<ApiResponse<AutopilotDiagnosticsDto>, String> {
    let ctx = make_ctx(&state);

    let diag = state
        .autopilot_service
        .get_diagnostics(&ctx, ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

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

    Ok(ok(dto))
}
