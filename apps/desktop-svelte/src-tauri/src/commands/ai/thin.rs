/// Thin Tauri command wrappers for AI domain (autopilot + agent).
/// Delegates to svc_lifecycle::ops::autopilot and svc_lifecycle::ops::agent.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_lifecycle::ops;

#[tauri::command]
pub async fn autopilot_get_status(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::autopilot::get_status(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_tasks(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::autopilot::get_tasks(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_diagnostics(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::autopilot::diagnostics(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn autopilot_configure(state: State<'_, Arc<AppState>>, org_id: String, is_enabled: bool, paused: bool, mode: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::autopilot::configure(&ctx, is_enabled, paused, &mode).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn autopilot_run_tick(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::autopilot::run_tick(&ctx).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn agent_get_sessions(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::agent::list_sessions(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn agent_get_session_logs(state: State<'_, Arc<AppState>>, org_id: String, session_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::agent::get_logs(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn agent_approve_action(state: State<'_, Arc<AppState>>, org_id: String, action_log_id: String, approved_by: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::agent::approve(&ctx, &action_log_id, &approved_by).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn agent_reject_action(state: State<'_, Arc<AppState>>, org_id: String, action_log_id: String, rejected_by: String, reason: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::agent::reject(&ctx, &action_log_id, &rejected_by, &reason).await.map_err(CommandError::from)
}
