use tauri::State;
use std::sync::Arc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait};
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_widget_preferences(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let u = domain_iam::entities::user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("User not found".to_string()))?;

    let prefs: serde_json::Value = serde_json::from_str(&u.prefs)
        .unwrap_or_else(|_| serde_json::json!({}));

    let widget_prefs = prefs.get("widget_preferences").cloned().unwrap_or_else(|| {
        serde_json::json!([
            { "id": "school_metrics", "visible": true, "w": 4, "h": 2 },
            { "id": "grade_analytics", "visible": true, "w": 4, "h": 3 },
            { "id": "attendance_overview", "visible": true, "w": 4, "h": 2 }
        ])
    });

    Ok(widget_prefs)
}

#[tauri::command]
pub async fn update_widget_preferences(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    preferences: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let u = domain_iam::entities::user::Entity::find_by_id(user_id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("User not found".to_string()))?;

    let mut prefs: serde_json::Value = serde_json::from_str(&u.prefs)
        .unwrap_or_else(|_| serde_json::json!({}));

    if let Some(obj) = prefs.as_object_mut() {
        obj.insert("widget_preferences".to_string(), preferences.clone());
    }

    let prefs_str = serde_json::to_string(&prefs).unwrap_or_else(|_| "{}".to_string());

    let mut active: domain_iam::entities::user::ActiveModel = u.into();
    active.prefs = Set(prefs_str);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());

    active
        .update(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(preferences)
}
