/// Thin Tauri command wrappers for AI domain (autopilot + agent).
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// Deleted autopilot_get_status

#[tauri::command]
pub async fn autopilot_get_tasks(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let tasks = autopilot::ops::get_tasks(&db, &org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(tasks).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_diagnostics(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let report = autopilot::diagnostics::Diagnostics::health_check(&db, &org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(report).unwrap())
}

// Deleted autopilot_configure

#[tauri::command]
pub async fn autopilot_run_tick(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<()> {
    use autopilot::engine::AutopilotEngine;
    let db = state.db();
    let engine = AutopilotEngine::new(state.platform.event_bus.clone());
    engine.tick(&db, &org_id).await;
    Ok(())
}

#[tauri::command]
pub async fn agent_get_sessions(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let sessions = ai_gateway::agent::AgentSessionManager::list_active_sessions(&db, &org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(sessions).unwrap())
}

#[tauri::command]
pub async fn agent_get_session_logs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let logs = ai_gateway::agent::AgentSessionManager::get_session_logs(&db, &org_id, &session_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(logs).unwrap())
}

#[tauri::command]
pub async fn agent_approve_action(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    action_log_id: String,
    approved_by: String,
) -> CmdResult<()> {
    let db = state.db();
    ai_gateway::agent::AgentSessionManager::approve_action(&db, &_org_id, &action_log_id, &approved_by)
        .await
        .map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn agent_reject_action(
    state: State<'_, Arc<AppState>>,
    _org_id: String,
    action_log_id: String,
    _rejected_by: String,
    reason: String,
) -> CmdResult<()> {
    let db = state.db();
    ai_gateway::agent::AgentSessionManager::reject_action(&db, &_org_id, &action_log_id, &_rejected_by, &reason)
        .await
        .map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn agent_create_session(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_type: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let req = ai_gateway::agent::CreateSessionRequest {
        org_id,
        agent_type: session_type,
        agent_version: None,
        acting_as_user_id: "system".to_string(), // TODO: Get from state
        acting_as_role: "admin".to_string(),
        conversation_id: None,
        max_risk_level: shared_kernel::types::RiskLevel::Low,
        requires_approval_above: shared_kernel::types::RiskLevel::Low,
    };
    let session = ai_gateway::agent::AgentSessionManager::create_session(&db, req)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(session).unwrap())
}

#[tauri::command]
pub async fn agent_query(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
    text: String,
    context: Option<serde_json::Value>,
    on_chunk: tauri::ipc::Channel<String>,
) -> CmdResult<String> {
    let db = state.db();
    let event_bus = state.platform.event_bus.clone();
    let response = ai_gateway::agent::AgentSessionManager::query(&db, event_bus, &org_id, &session_id, &text, context)
        .await
        .map_err(CommandError::from)?;

    // Simulate streaming for the frontend
    let chunks: Vec<&str> = response.split_whitespace().collect();
    let mut current = String::new();
    for chunk in chunks {
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(chunk);
        let _ = on_chunk.send(current.clone());
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }

    Ok(response)
}

#[tauri::command]
pub async fn agent_open_standalone(
    _state: State<'_, Arc<AppState>>,
    _org_id: String,
) -> CmdResult<()> {
    // Logic to open a new tauri window would go here
    Ok(())
}
