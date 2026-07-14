/// Thin Tauri command wrappers for the IAM domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_iam::ops;

#[tauri::command]
pub async fn login(state: State<'_, Arc<AppState>>, identifier: String, password_hash: String) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let (user, session) = ops::auth::login(&db, &identifier, &password_hash).await.map_err(CommandError::from)?;

    // Lock the session org_id in TenantGuard
    let roles: Vec<String> = vec![user.primary_role.clone()];
    state.set_session(user.id.clone(), user.org_id.clone(), roles).await;

    Ok(serde_json::json!({
        "user": user,
        "access_token": session.token
    }))
}

#[tauri::command]
pub async fn logout_session(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<()> {
    let db = state.db();
    
    // session_id here is actually the token from the frontend
    if ops::auth::logout(&db, &session_id).await.is_ok() {
        state.clear_session().await;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn restore_session(state: State<'_, Arc<AppState>>, token: String) -> CmdResult<serde_json::Value> {
    let db = state.db();
    
    let (user, session) = ops::auth::restore_session(&db, &token).await.map_err(CommandError::from)?;

    let roles: Vec<String> = vec![user.primary_role.clone()];
    state.set_session(user.id.clone(), user.org_id.clone(), roles).await;

    Ok(serde_json::json!({
        "user": user,
        "access_token": session.token
    }))
}

#[tauri::command]
pub async fn list_users(
    state: State<'_, Arc<AppState>>, 
    filter: Option<serde_json::Value>
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let f = filter.and_then(|v| serde_json::from_value(v).ok());
    let r = ops::user::list(&ctx, f).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_users_paginated(
    state: State<'_, Arc<AppState>>,
    cursor: Option<String>,
    limit: Option<u64>,
    filter: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let request = shared_kernel::pagination::CursorRequest {
        cursor,
        limit: limit.unwrap_or(50),
    };
    let f = filter.and_then(|v| serde_json::from_value(v).ok());
    let r = ops::user::list_paginated(&ctx, request, f).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_user(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::user::get(&ctx, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_user(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::user::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_user(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::user::update(&ctx, &user_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_user(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<()> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::user::delete(&ctx, &user_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_password(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<()> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::auth::change_password(&ctx, &user_id, data).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn change_role(state: State<'_, Arc<AppState>>, user_id: String, data: serde_json::Value) -> CmdResult<()> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::user::change_role(&ctx, &user_id, data).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_user_sessions(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::auth::list_sessions(&ctx, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_user_roles(state: State<'_, Arc<AppState>>, user_id: String) -> CmdResult<serde_json::Value> {

    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::role::list_by_user(&ctx, &user_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn save_user_roles(state: State<'_, Arc<AppState>>, user_id: String, roles: Vec<String>) -> CmdResult<()> {
    let db = state.db();
    ops::role::save_for_user(&db, &user_id, roles).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn assign_role(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::role::assign(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn revoke_role(state: State<'_, Arc<AppState>>, role_id: String) -> CmdResult<()> {
    let db = state.db();
    ops::role::revoke(&db, &role_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_audit_logs(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::audit::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn search_users(
    state: State<'_, Arc<AppState>>,
    query: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let result = ops::user::search(&ctx, &query).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

// ── Role Permission Commands ─────────────────────────────────────────

#[tauri::command]
pub async fn get_role_permissions(state: State<'_, Arc<AppState>>, role: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
    let db = state.db();
    let perms = svc_iam::entities::role_permission::Entity::find()
        .filter(svc_iam::entities::role_permission::Column::Role.eq(&role))
        .all(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(perms).unwrap())
}

#[tauri::command]
pub async fn save_role_permissions(state: State<'_, Arc<AppState>>, role: String, permissions: Vec<serde_json::Value>) -> CmdResult<()> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
    let db = state.db();

    // Delete existing permissions for this role
    svc_iam::entities::role_permission::Entity::delete_many()
        .filter(svc_iam::entities::role_permission::Column::Role.eq(&role))
        .exec(&db).await.map_err(CommandError::from)?;

    // Insert new permissions
    let now = chrono::Utc::now().to_rfc3339();
    for perm in permissions {
        let active = svc_iam::entities::role_permission::ActiveModel {
            role: Set(role.clone()),
            document_type: Set(perm["document_type"].as_str().unwrap_or_default().to_string()),
            level: Set(perm["level"].as_i64().unwrap_or(0) as i32),
            only_if_creator: Set(perm["only_if_creator"].as_i64().unwrap_or(0) as i32),
            actions: Set(perm["actions"].as_str().unwrap_or("read").to_string()),
            created_at: Set(now.clone()),
        };
        active.insert(&db).await.map_err(CommandError::from)?;
    }
    Ok(())
}

