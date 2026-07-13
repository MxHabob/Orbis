/// Thin Tauri command wrappers for AI domain (autopilot + agent).
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn autopilot_get_status(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
    use svc_organization::entities::academic_policy;
    let db = state.db();
    let config = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;
    
    match config {
        Some(c) => Ok(serde_json::to_value(c).unwrap()),
        None => {
            Ok(serde_json::json!({
                "autopilot_enabled": 0,
                "autopilot_mode": "suggested"
            }))
        }
    }
}

#[tauri::command]
pub async fn autopilot_get_tasks(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
    use autopilot::entities::autopilot_task;
    let db = state.db();
    let tasks = autopilot_task::Entity::find()
        .filter(autopilot_task::Column::OrgId.eq(&org_id))
        .all(&db)
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

#[tauri::command]
pub async fn autopilot_configure(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    is_enabled: bool,
    paused: bool,
    mode: String,
) -> CmdResult<()> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
    use svc_organization::entities::academic_policy;
    let db = state.db();
    let config = academic_policy::Entity::find()
        .filter(academic_policy::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;

    let now = chrono::Utc::now().to_rfc3339();
    if let Some(c) = config {
        let mut active: academic_policy::ActiveModel = c.into();
        active.autopilot_enabled = Set(if is_enabled && !paused { 1 } else { 0 });
        active.autopilot_mode = Set(mode);
        active.updated_at = Set(now);
        active.save(&db).await.map_err(CommandError::from)?;
    }
    Ok(())
}

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
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};
    use ai_gateway::entities::ai_action_log;
    let db = state.db();
    let logs = ai_action_log::Entity::find()
        .filter(ai_action_log::Column::OrgId.eq(&org_id))
        .filter(ai_action_log::Column::SessionId.eq(&session_id))
        .order_by_desc(ai_action_log::Column::ExecutedAt)
        .all(&db)
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
    use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue::Set};
    use ai_gateway::entities::ai_action_log;
    let db = state.db();
    let log = ai_action_log::Entity::find_by_id(action_log_id.clone())
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Action log not found"))?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut active: ai_action_log::ActiveModel = log.into();
    active.status = Set(ai_action_log::action_status::APPROVED.to_string());
    active.approved_by = Set(Some(approved_by));
    active.approved_at = Set(Some(now));
    active.update(&db).await.map_err(CommandError::from)?;
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
    use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue::Set};
    use ai_gateway::entities::ai_action_log;
    let db = state.db();
    let log = ai_action_log::Entity::find_by_id(action_log_id.clone())
        .one(&db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Action log not found"))?;

    let mut active: ai_action_log::ActiveModel = log.into();
    active.status = Set(ai_action_log::action_status::REJECTED.to_string());
    active.rejection_reason = Set(Some(reason));
    active.update(&db).await.map_err(CommandError::from)?;
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
    on_chunk: Option<tauri::ipc::Channel<String>>,
) -> CmdResult<String> {
    let db = state.db();
    let event_bus = state.platform.event_bus.clone();
    let response = ai_gateway::agent::AgentSessionManager::query(&db, event_bus, &org_id, &session_id, &text, context)
        .await
        .map_err(CommandError::from)?;

    if let Some(channel) = on_chunk {
        // Simulate streaming for the frontend
        let chunks: Vec<&str> = response.split_whitespace().collect();
        let mut current = String::new();
        for chunk in chunks {
            if !current.is_empty() {
                current.push(' ');
            }
            current.push_str(chunk);
            let _ = channel.send(current.clone());
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
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
