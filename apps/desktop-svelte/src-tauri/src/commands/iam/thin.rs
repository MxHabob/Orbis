/// Thin Tauri command wrappers for the IAM domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_iam::ops;

#[tauri::command]
pub async fn login(state: State<'_, Arc<AppState>>, identifier: String, password_hash: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::auth::login(&db, &identifier, &password_hash).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn logout_session(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::auth::logout(&db, &session_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_users(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::user::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_user(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::user::get(&db, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_user(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::user::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_user(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::user::update(&db, &user_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_user(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::user::delete(&db, &user_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_password(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::user::change_password(&db, &user_id, data).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_role(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::user::change_role(&db, &user_id, data).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_sessions(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::session::list(&db, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_user_roles(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::role::list_for_user(&db, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn save_user_roles(state: State<'_, Arc<AppState>>, user_id: String, roles: Vec<String>) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::role::save_for_user(&db, &user_id, roles).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn assign_role(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::role::assign(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn revoke_role(state: State<'_, Arc<AppState>>, role_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::role::revoke(&db, &role_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_audit_logs(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::audit::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}
