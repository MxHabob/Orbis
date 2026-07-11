use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_assessment::ops;

#[utoipa::path(get, path = "/api/assessments")]
pub async fn get_assessments(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let period_id = body["period_id"].as_str().unwrap_or_default();
    let r = ops::assessment::list(&ctx, period_id, None).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/assessments")]
pub async fn post_assessments(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::assessment::create(&ctx, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(put, path = "/api/assessments/{id}")]
pub async fn put_assessment(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::assessment::update(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(delete, path = "/api/assessments/{id}")]
pub async fn delete_assessment(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::assessment::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}
