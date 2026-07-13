use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_iam::ops;

#[utoipa::path(get, path = "/api/users")]
pub async fn get_users(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::user::list(&ctx, None).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/users")]
pub async fn post_users(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::user::create(&ctx, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(get, path = "/api/users/{id}")]
pub async fn get_user(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Path(id): Path<String>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::user::get(&ctx, &id).await.map_err(|e| (StatusCode::NOT_FOUND, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(put, path = "/api/users/{id}")]
pub async fn put_user(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Path(id): Path<String>, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::user::update(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(delete, path = "/api/users/{id}")]
pub async fn delete_user(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Path(id): Path<String>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::user::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}

#[utoipa::path(post, path = "/api/users/{id}/role")]
pub async fn post_user_role(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Path(id): Path<String>, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::user::change_role(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"updated": true}))
}

#[utoipa::path(post, path = "/api/users/{id}/password")]
pub async fn post_user_password(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Path(id): Path<String>, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::auth::change_password(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"updated": true}))
}
