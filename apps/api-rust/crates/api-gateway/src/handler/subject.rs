use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_academic::ops;

#[utoipa::path(get, path = "/api/subjects")]
pub async fn get_subjects(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::subject::list(&ctx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/subjects")]
pub async fn post_subjects(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<svc_academic::ops::subject::CreateSubjectPayload>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::subject::create(&ctx, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(put, path = "/api/subjects/{id}")]
pub async fn put_subject(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<svc_academic::ops::subject::UpdateSubjectPayload>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::subject::update(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(delete, path = "/api/subjects/{id}")]
pub async fn delete_subject(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::subject::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}
