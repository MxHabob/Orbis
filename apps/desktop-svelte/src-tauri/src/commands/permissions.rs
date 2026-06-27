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

#[tauri::command]
pub async fn get_role_permissions(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<HashMap<String, HashMap<String, String>>> {
    let perms = state.service.get_permissions().await.map_err(CommandError::from)?;
    let role_perms = state.service.get_role_permissions().await.map_err(CommandError::from)?;

    // Build mapping of permission_id -> permission_code
    let id_to_code: HashMap<i32, String> = perms.into_iter().map(|p| (p.id, p.code)).collect();

    // Aggregate into nested map: Role -> PermissionCode -> Scope
    let mut result: HashMap<String, HashMap<String, String>> = HashMap::new();
    for rp in role_perms {
        if let Some(code) = id_to_code.get(&rp.permission_id) {
            let scope = rp.conditions.unwrap_or_else(|| "none".to_string());
            result
                .entry(rp.role)
                .or_insert_with(HashMap::new)
                .insert(code.clone(), scope);
        }
    }

    Ok(result)
}

#[tauri::command]
pub async fn save_role_permissions(
    state: State<'_, Arc<AppState>>,
    role: String,
    permissions: HashMap<String, String>,
) -> CmdResult<()> {
    state
        .service
        .save_role_permissions(&role, permissions)
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
