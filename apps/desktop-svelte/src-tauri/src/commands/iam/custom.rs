use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_iam::ops;

/// Un-guarded login endpoint. Validates credentials and elevates AppState session.
#[tauri::command]
pub async fn iam_auth_login(
    state: State<'_, Arc<AppState>>,
    identifier: String,
    password_hash: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let (user, session) = ops::auth::login(&db, &identifier, &password_hash)
        .await
        .map_err(CommandError::from)?;

    // Elevate active session context in memory
    state
        .set_session(user.id.clone(), user.org_id.clone(), vec![user.primary_role.clone()])
        .await;

    Ok(serde_json::json!({
        "user": user,
        "access_token": session.token,
        "session": session
    }))
}

/// Un-guarded restore session endpoint. Re-validates active token on startup and elevates session.
#[tauri::command]
pub async fn iam_auth_restore_session(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let (user, session) = ops::auth::restore_session(&db, &token)
        .await
        .map_err(CommandError::from)?;

    // Elevate active session context in memory
    state
        .set_session(user.id.clone(), user.org_id.clone(), vec![user.primary_role.clone()])
        .await;

    Ok(serde_json::json!({
        "user": user,
        "access_token": session.token,
        "session": session
    }))
}

/// Invalidate session and clear AppState session context.
#[tauri::command]
pub async fn iam_auth_logout(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    ops::auth::logout(&db, &token).await.map_err(CommandError::from)?;
    state.clear_session().await;
    Ok(serde_json::json!({}))
}

/// Un-guarded registration endpoint.
#[tauri::command]
pub async fn iam_auth_register(
    state: State<'_, Arc<AppState>>,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let data_parsed = serde_json::from_value(data).map_err(|e| CommandError::new(e.to_string()))?;
    let result = ops::auth::register(&db, data_parsed).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}
