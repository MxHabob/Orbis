use tauri::State;
use std::sync::Arc;
use std::collections::HashMap;
use core_domain::entities::permission;
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_permissions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<permission::Model>> {
    state.service.get_permissions().await.map_err(CommandError::from)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PermissionRuleDto {
    pub role: String,
    pub document_type: String,
    pub level: i32,
    pub only_if_creator: bool,
    pub actions: Vec<String>,
}

#[tauri::command]
pub async fn get_role_permissions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<PermissionRuleDto>> {
    let role_perms = state.service.get_role_permissions().await.map_err(CommandError::from)?;

    let mut result = Vec::new();
    for rp in role_perms {
        let actions: Vec<String> = serde_json::from_str(&rp.actions)
            .unwrap_or_default();
        result.push(PermissionRuleDto {
            role: rp.role,
            document_type: rp.document_type,
            level: rp.level,
            only_if_creator: rp.only_if_creator == 1,
            actions,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn save_role_permissions(
    state: State<'_, Arc<AppState>>,
    role: String,
    rules: Vec<PermissionRuleDto>,
) -> CmdResult<()> {
    let now = chrono::Utc::now().naive_utc().to_string();
    let entity_models = rules
        .into_iter()
        .map(|r| core_domain::entities::role_permission::Model {
            role: r.role,
            document_type: r.document_type,
            level: r.level,
            only_if_creator: if r.only_if_creator { 1 } else { 0 },
            actions: serde_json::to_string(&r.actions).unwrap_or_default(),
            created_at: now.clone(),
        })
        .collect();

    state
        .service
        .save_role_permissions(&role, entity_models)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_user_permissions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<HashMap<String, String>> {
    state
        .service
        .get_user_permissions(&user_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn save_user_permissions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    permissions: HashMap<String, String>,
) -> CmdResult<()> {
    state
        .service
        .save_user_permissions(&user_id, permissions)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_available_roles(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<String>> {
    state
        .service
        .get_available_roles()
        .await
        .map_err(CommandError::from)
}
