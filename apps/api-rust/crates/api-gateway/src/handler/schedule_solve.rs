use axum::{extract::{State, Path, Json}, http::StatusCode, response::IntoResponse};
use serde_json::json;
use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_scheduling::ops;

#[utoipa::path(post, path = "/api/schedule/solve")]
pub async fn post_solve(State(state): State<AppState>, AuthenticatedUser(user): AuthenticatedUser, Json(body): Json<serde_json::Value>) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let year_id = body["year_id"].as_str().unwrap_or_default();
    let r = ops::solver::start(&ctx, year_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(get, path = "/api/schedule/solve/{year_id}")]
pub async fn get_solve_jobs(State(state): State<AppState>, Path(year_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::solver::list_jobs(&ctx, &year_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}

#[utoipa::path(post, path = "/api/schedule/solve/{job_id}/apply")]
pub async fn post_apply_solve(State(state): State<AppState>, Path(job_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    ops::solver::apply(&ctx, &job_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(json!({"applied": true}))
}

#[utoipa::path(post, path = "/api/schedule/solve/{job_id}/cancel")]
pub async fn post_cancel_solve(State(state): State<AppState>, Path(job_id): Path<String>, AuthenticatedUser(user): AuthenticatedUser) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = state.build_context(&user.org_id, &user.id);
    let r = ops::solver::cancel(&ctx, &job_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success":false,"error":e.to_string()}))))?;
    ok!(r)
}
