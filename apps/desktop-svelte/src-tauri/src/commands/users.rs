use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{user, organization, user_session},
    params::UserParams,
    service::user::{
        CreateUserRequest, UpdateUserRequest,
        ChangeRoleRequest, ChangePasswordRequest,
    },
    service::organization::UpdateOrganizationRequest,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Organization ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_organization(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<organization::Model> {
    state
        .service
        .get_organization(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_organization(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: UpdateOrganizationRequest,
) -> CmdResult<organization::Model> {
    state
        .service
        .update_organization(&org_id, request)
        .await
        .map_err(CommandError::from)
}

// ── Users ──────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_users(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: UserParams,
) -> CmdResult<Vec<user::Model>> {
    state
        .service
        .get_users(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<user::Model> {
    state
        .service
        .get_user(&user_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_user(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    request: CreateUserRequest,
) -> CmdResult<user::Model> {
    state
        .service
        .create_user_in_org(&org_id, request, &created_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: UpdateUserRequest,
) -> CmdResult<user::Model> {
    state
        .service
        .update_user(&user_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_user(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    deleted_by: String,
) -> CmdResult<()> {
    state
        .service
        .delete_user(&user_id, &deleted_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_user_role(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: ChangeRoleRequest,
) -> CmdResult<user::Model> {
    state
        .service
        .change_user_role(&user_id, &request.role)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_user_password(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    request: ChangePasswordRequest,
) -> CmdResult<user::Model> {
    state
        .service
        .change_user_password(&user_id, &request.new_password)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_user_roles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<Vec<String>> {
    let roles = state
        .service
        .get_user_roles(&user_id)
        .await
        .map_err(CommandError::from)?;
    Ok(roles.into_iter().map(|ur| ur.role).collect())
}

#[tauri::command]
pub async fn save_user_roles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    roles: Vec<String>,
) -> CmdResult<()> {
    state
        .service
        .save_user_roles(&user_id, roles)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_user_sessions(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> CmdResult<Vec<user_session::Model>> {
    state
        .service
        .get_user_sessions(&user_id)
        .await
        .map_err(CommandError::from)
}
