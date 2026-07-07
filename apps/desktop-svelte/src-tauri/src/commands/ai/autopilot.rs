use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_autopilot_status(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Option<autopilot::entities::autopilot_config::Model>> {
    let db = state.provider.registry();
    let config = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(org_id))
        .one(db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(config)
}

#[tauri::command]
pub async fn get_autopilot_tasks(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<autopilot::entities::autopilot_task::Model>> {
    let db = state.provider.registry();
    let tasks = autopilot::entities::autopilot_task::Entity::find()
        .filter(autopilot::entities::autopilot_task::Column::OrgId.eq(org_id))
        .all(db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(tasks)
}

#[tauri::command]
pub async fn get_autopilot_diagnostics(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<autopilot::diagnostics::HealthReport> {
    let db = state.provider.registry();
    let report = autopilot::diagnostics::Diagnostics::health_check(db, &org_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(report)
}

#[tauri::command]
pub async fn configure_autopilot(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    is_enabled: bool,
    paused: bool,
    mode: String,
) -> CmdResult<autopilot::entities::autopilot_config::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.provider.registry();
    let now = chrono::Utc::now().to_rfc3339();
    
    let existing = autopilot::entities::autopilot_config::Entity::find()
        .filter(autopilot::entities::autopilot_config::Column::OrgId.eq(&org_id))
        .one(db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
        
    let is_new = existing.is_none();
    let mut active = match existing {
        Some(model) => {
            let active: autopilot::entities::autopilot_config::ActiveModel = model.into();
            active
        }
        None => {
            let model = autopilot::entities::autopilot_config::Model {
                id: format!("{}_config", org_id),
                org_id: org_id.clone(),
                mode: mode.clone(),
                check_interval_minutes: 60,
                timezone: "Asia/Riyadh".to_string(),
                require_backup_before_critical: 1,
                max_auto_retries: 3,
                pause_on_error: 1,
                is_enabled: if is_enabled { 1 } else { 0 },
                last_tick_at: None,
                last_tick_result: None,
                paused_at: if paused { Some(now.clone()) } else { None },
                pause_reason: if paused { Some("Manual pause".to_string()) } else { None },
                created_at: now.clone(),
                updated_at: now.clone(),
            };
            let active: autopilot::entities::autopilot_config::ActiveModel = model.into();
            active
        }
    };
    
    active.is_enabled = Set(if is_enabled { 1 } else { 0 });
    active.mode = Set(mode);
    if paused {
        active.paused_at = Set(Some(now.clone()));
        active.pause_reason = Set(Some("Manual pause".to_string()));
    } else {
        active.paused_at = Set(None);
        active.pause_reason = Set(None);
    }
    active.updated_at = Set(now);
    
    let saved = if !is_new {
        active.update(db).await.map_err(|e| CommandError::new(e.to_string()))?
    } else {
        active.insert(db).await.map_err(|e| CommandError::new(e.to_string()))?
    };
    
    Ok(saved)
}

#[tauri::command]
pub async fn run_autopilot_tick(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<autopilot::engine::TickResult> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let registry_db = state.provider.registry();
    let year_db = state.active_db().await;
    
    let bus = std::sync::Arc::new(shared_kernel::event_bus::EventBus::new());
    let engine = autopilot::engine::AutopilotEngine::new(bus);
    
    let result = engine.tick(registry_db, &year_db, &org_id).await;
    Ok(result)
}
