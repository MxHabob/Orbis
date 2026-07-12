use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_organization::ops;

#[utoipa::path(get, path = "/api/periods")]
pub async fn get_periods(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::academic_period::list(&ctx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/periods")]
pub async fn post_periods(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::academic_period::create(&ctx, body).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(StatusCode::CREATED, r)
}

#[utoipa::path(get, path = "/api/periods/{id}")]
pub async fn get_period(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::academic_period::get(&ctx, &id).await.map_err(|e| (StatusCode::NOT_FOUND, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(get, path = "/api/periods/year/{year_id}")]
pub async fn get_periods_by_year(State(state): State<AppState>, Path(year_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::academic_period::list_by_year(&ctx, &year_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/periods/{id}/lock")]
pub async fn post_lock_period(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::lock(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"locked": true}))
}

#[utoipa::path(post, path = "/api/periods/{id}/unlock")]
pub async fn post_unlock_period(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::unlock(&ctx, &id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"unlocked": true}))
}

#[utoipa::path(post, path = "/api/periods/{id}/enrollment/open")]
pub async fn post_open_enrollment(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::toggle_enrollment(&ctx, &id, true).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"enrollment_opened": true}))
}

#[utoipa::path(post, path = "/api/periods/{id}/enrollment/close")]
pub async fn post_close_enrollment(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::toggle_enrollment(&ctx, &id, false).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"enrollment_closed": true}))
}

#[utoipa::path(post, path = "/api/periods/{id}/grade-entry/open")]
pub async fn post_open_grade_entry(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::toggle_grade_entry(&ctx, &id, true).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"grade_entry_opened": true}))
}

#[utoipa::path(post, path = "/api/periods/{id}/grade-entry/close")]
pub async fn post_close_grade_entry(State(state): State<AppState>, Path(id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::academic_period::toggle_grade_entry(&ctx, &id, false).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"grade_entry_closed": true}))
}
