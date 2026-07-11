/// Thin Tauri command wrappers for AI domain (autopilot + agent).
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn autopilot_get_status(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
    use autopilot::entities::autopilot_config;
    let db = state.active_db().await;
    let config = autopilot_config::Entity::find()
        .filter(autopilot_config::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;
    let config = match config {
        Some(c) => c,
        None => {
            let now = chrono::Utc::now().to_rfc3339();
            let c = autopilot_config::ActiveModel {
                id: Set(uuid::Uuid::new_v4().to_string()),
                org_id: Set(org_id.clone()),
                is_enabled: Set(1),
                mode: Set("suggested".to_string()),
                check_interval_minutes: Set(60),
                timezone: Set("Asia/Riyadh".to_string()),
                require_backup_before_critical: Set(1),
                max_auto_retries: Set(3),
                pause_on_error: Set(1),
                last_tick_at: Set(None),
                last_tick_result: Set(None),
                paused_at: Set(None),
                pause_reason: Set(None),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };
            c.insert(&db).await.map_err(CommandError::from)?
        }
    };
    Ok(serde_json::to_value(config).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_tasks(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
    use autopilot::entities::autopilot_task;
    let db = state.active_db().await;
    let tasks = autopilot_task::Entity::find()
        .filter(autopilot_task::Column::OrgId.eq(&org_id))
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(tasks).unwrap())
}

#[tauri::command]
pub async fn autopilot_get_diagnostics(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
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
    use autopilot::entities::autopilot_config;
    let db = state.active_db().await;
    let config = autopilot_config::Entity::find()
        .filter(autopilot_config::Column::OrgId.eq(&org_id))
        .one(&db)
        .await
        .map_err(CommandError::from)?;

    let now = chrono::Utc::now().to_rfc3339();
    let active = match config {
        Some(c) => {
            let mut a: autopilot_config::ActiveModel = c.into();
            a.is_enabled = Set(if is_enabled { 1 } else { 0 });
            a.mode = Set(mode);
            a.paused_at = Set(if paused { Some(now.clone()) } else { None });
            a.updated_at = Set(now);
            a
        }
        None => autopilot_config::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            org_id: Set(org_id.clone()),
            is_enabled: Set(if is_enabled { 1 } else { 0 }),
            mode: Set(mode),
            check_interval_minutes: Set(60),
            timezone: Set("Asia/Riyadh".to_string()),
            require_backup_before_critical: Set(1),
            max_auto_retries: Set(3),
            pause_on_error: Set(1),
            last_tick_at: Set(None),
            last_tick_result: Set(None),
            paused_at: Set(if paused { Some(now.clone()) } else { None }),
            pause_reason: Set(None),
            created_at: Set(now.clone()),
            updated_at: Set(now),
        },
    };

    active.save(&db).await.map_err(CommandError::from)?;
    Ok(())
}

#[tauri::command]
pub async fn autopilot_run_tick(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<()> {
    use autopilot::engine::AutopilotEngine;
    let db = state.active_db().await;
    let engine = AutopilotEngine::new(state.platform.event_bus.clone());
    engine.tick(&db, &db, &org_id).await;
    Ok(())
}

#[tauri::command]
pub async fn agent_get_sessions(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
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
    let db = state.active_db().await;
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
    let db = state.active_db().await;
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
    let db = state.active_db().await;
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
