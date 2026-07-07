use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_agent_sessions(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<ai_gateway::entities::ai_agent_session::Model>> {
    let db = state.active_db().await;
    let sessions = ai_gateway::entities::ai_agent_session::Entity::find()
        .filter(ai_gateway::entities::ai_agent_session::Column::OrgId.eq(org_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(sessions)
}

#[tauri::command]
pub async fn get_agent_session_logs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    session_id: String,
) -> CmdResult<Vec<ai_gateway::entities::ai_action_log::Model>> {
    let db = state.active_db().await;
    let logs = ai_gateway::entities::ai_action_log::Entity::find()
        .filter(ai_gateway::entities::ai_action_log::Column::OrgId.eq(org_id))
        .filter(ai_gateway::entities::ai_action_log::Column::SessionId.eq(session_id))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(logs)
}

#[tauri::command]
pub async fn approve_agent_action(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    action_log_id: String,
    approved_by: String,
) -> CmdResult<ai_gateway::entities::ai_action_log::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let action = ai_gateway::entities::ai_action_log::Entity::find_by_id(action_log_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Action log not found".to_string()))?;
        
    let mut active: ai_gateway::entities::ai_action_log::ActiveModel = action.into();
    active.status = Set(ai_gateway::entities::ai_action_log::action_status::APPROVED.to_string());
    active.approved_by = Set(Some(approved_by));
    active.approved_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}

#[tauri::command]
pub async fn reject_agent_action(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    action_log_id: String,
    rejected_by: String,
    reason: String,
) -> CmdResult<ai_gateway::entities::ai_action_log::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let action = ai_gateway::entities::ai_action_log::Entity::find_by_id(action_log_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::from("Action log not found".to_string()))?;
        
    let mut active: ai_gateway::entities::ai_action_log::ActiveModel = action.into();
    active.status = Set(ai_gateway::entities::ai_action_log::action_status::REJECTED.to_string());
    active.rejection_reason = Set(Some(reason));
    active.approved_by = Set(Some(rejected_by));
    active.approved_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    
    let updated = active.update(&db).await.map_err(|e| CommandError::new(e.to_string()))?;
    Ok(updated)
}
