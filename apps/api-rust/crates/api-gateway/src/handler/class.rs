use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_academic::ops;

#[utoipa::path(get, path = "/api/classes")]
pub async fn get_classes(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::class::list(&ctx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/classes")]
pub async fn post_classes(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::class::create(&ctx, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(put, path = "/api/classes/{id}")]
pub async fn put_class(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::class::update(&ctx, &id, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(delete, path = "/api/classes/{id}")]
pub async fn delete_class(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::class::delete(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"deleted": true}))
}

#[utoipa::path(get, path = "/api/classes/{id}/students")]
pub async fn get_class_students(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::class::list_students(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(get, path = "/api/classes/{id}/students/enriched")]
pub async fn get_class_students_enriched(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::class::list_enriched_students(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}
