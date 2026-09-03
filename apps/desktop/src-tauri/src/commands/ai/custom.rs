/// Thin Tauri command wrappers for AI domain (autopilot + agent).
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// Deleted autopilot_get_status

#[tauri::command]
pub async fn autopilot_get_tasks(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let tasks = autopilot::ops::get_tasks(&ctx.db, &ctx.org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(tasks).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_diagnostics(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let report = autopilot::diagnostics::Diagnostics::health_check(&ctx.db, &ctx.org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(report).unwrap())
}

// Deleted autopilot_configure

#[tauri::command]
pub async fn autopilot_run_tick(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<()> {
    use autopilot::engine::AutopilotEngine;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let engine = AutopilotEngine::new(state.platform.event_bus.clone())
        .with_period_gate(state.platform.period_gate.clone());
    engine.tick(&ctx.db, &ctx.org_id, ctx.org_unit_id.as_deref()).await;
    Ok(())
}

#[tauri::command]
pub async fn agent_get_sessions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let sessions = ai_gateway::agent::AgentSessionManager::list_active_sessions(&ctx.db, &ctx.org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(sessions).unwrap())
}

#[tauri::command]
pub async fn agent_get_session_logs(
    state: State<'_, Arc<AppState>>,
    session_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let logs = ai_gateway::agent::AgentSessionManager::get_session_logs(&ctx.db, &ctx.org_id, &session_id)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(logs).unwrap())
}

#[tauri::command]
pub async fn agent_approve_action(
    state: State<'_, Arc<AppState>>,
    action_log_id: String,
    approved_by: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ai_gateway::agent::AgentSessionManager::approve_action(&ctx.db, &ctx.org_id, &action_log_id, &approved_by)
        .await
        .map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn agent_reject_action(
    state: State<'_, Arc<AppState>>,
    action_log_id: String,
    rejected_by: String,
    reason: String,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ai_gateway::agent::AgentSessionManager::reject_action(&ctx.db, &ctx.org_id, &action_log_id, &rejected_by, &reason)
        .await
        .map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn agent_create_session(
    state: State<'_, Arc<AppState>>,
    session_type: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let agent_type = ai_gateway::entities::ai_agent_session::agent_type::canonical(&session_type);
    let req = ai_gateway::agent::CreateSessionRequest {
        org_id: ctx.org_id.clone(),
        agent_type: agent_type.to_string(),
        agent_version: None,
        acting_as_user_id: ctx.actor.as_str().to_string(),
        acting_as_role: "admin".to_string(),
        conversation_id: None,
        max_risk_level: shared_kernel::types::RiskLevel::Low,
        requires_approval_above: shared_kernel::types::RiskLevel::Low,
    };
    let session = ai_gateway::agent::AgentSessionManager::create_session(&ctx.db, req)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(session).unwrap())
}

#[tauri::command]
pub async fn agent_query(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    text: String,
    context: Option<serde_json::Value>,
    on_chunk: tauri::ipc::Channel<String>,
) -> CmdResult<String> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let event_bus = state.platform.event_bus.clone();
    let response = ai_gateway::agent::AgentSessionManager::query(&ctx.db, event_bus, &ctx.org_id, &session_id, &text, context)
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
pub async fn agent_open_standalone<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> CmdResult<()> {
    crate::do_open_agent_window(&app)
        .await
        .map_err(CommandError::new)?;
    Ok(())
}

#[tauri::command]
pub async fn ai_get_autopilot_config(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = platform_policy::ops::academic_policy::get_autopilot_config(&ctx, &org_id)
        .await
        .map_err(CommandError::from)?;
    Ok(result)
}

#[tauri::command]
pub async fn ai_save_autopilot_config(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    config: serde_json::Value,
) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    platform_policy::ops::academic_policy::update_autopilot_config(&ctx, &org_id, config)
        .await
        .map_err(CommandError::from)?;
    Ok(())
}



