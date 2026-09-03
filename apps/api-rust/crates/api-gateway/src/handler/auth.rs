use axum::{extract::{State, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_iam::ops;

#[utoipa::path(post, path = "/api/auth/login")]
pub async fn post_login(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let identifier = body["identifier"].as_str().unwrap_or_default();
    let password = body["password"].as_str().unwrap_or_default();
    let r = ops::auth::login(&state.db, identifier, password).await
        .map_err(|e| (StatusCode::UNAUTHORIZED, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/auth/register")]
pub async fn post_register(
    State(state): State<AppState>,
    Json(body): Json<svc_iam::ops::auth::RegisterPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if !state.config.public_registration_enabled {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"success": false, "error": "Registration is disabled"})),
        ));
    }
    let r = ops::auth::register(&state.db, body).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(get, path = "/api/auth/me")]
pub async fn get_me(
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    ok!(user)
}

#[utoipa::path(post, path = "/api/auth/logout")]
pub async fn post_logout(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    ops::auth::logout_by_user(&state.db, &user.id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"logged_out": true}))
}
