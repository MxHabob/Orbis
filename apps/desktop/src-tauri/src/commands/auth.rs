use tauri::State;
use std::sync::Arc;

use core_domain::service::auth::LoginResponse;
use core_domain::entities::user;

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Request types ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub org_slug: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// ── Commands ───────────────────────────────────────────────────────────────

/// Register a new user and create their organization if it doesn't exist.
#[tauri::command]
pub async fn register(
    state: State<'_, Arc<AppState>>,
    request: RegisterRequest,
) -> CmdResult<user::Model> {
    state
        .service
        .register(
            &request.email,
            &request.password,
            &request.full_name,
            &request.org_slug,
        )
        .await
        .map_err(CommandError::from)
}

/// Authenticate with email + password. Returns user model and JWT tokens.
#[tauri::command]
pub async fn login(
    state: State<'_, Arc<AppState>>,
    request: LoginRequest,
) -> CmdResult<LoginResponse> {
    state
        .service
        .login(&request.email, &request.password)
        .await
        .map_err(CommandError::from)
}

/// Validate a JWT token and return the associated user.
/// Called on app startup to restore session.
#[tauri::command]
pub async fn validate_token(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<user::Model> {
    state
        .service
        .validate_token(&token)
        .await
        .map_err(CommandError::from)
}

/// Invalidate the current session token.
#[tauri::command]
pub async fn logout(
    state: State<'_, Arc<AppState>>,
    token: String,
) -> CmdResult<()> {
    state
        .service
        .logout(&token)
        .await
        .map_err(CommandError::from)
}
