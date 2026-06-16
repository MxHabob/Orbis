use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

use crate::middleware::AuthenticatedUser;
use crate::service::Service;
use crate::service::schedule_solver::{SolveJobResponse, StartSolveRequest};

#[utoipa::path(post, path = "/api/schedule/solve", responses((status = 202)))]
pub async fn post_schedule_solve(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<StartSolveRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let job = service
        .start_schedule_solve(&user.org_id, &user.id, req)
        .await
        .map_err(|e| {
            let status = if e.contains("already running") || e.contains("not configured") {
                StatusCode::CONFLICT
            } else if e.contains("no lesson") || e.contains("no time slots") {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "success": false, "error": e })))
        })?;

    Ok((
        StatusCode::ACCEPTED,
        Json(json!({ "success": true, "data": job })),
    ))
}

#[utoipa::path(get, path = "/api/schedule/solve", responses((status = 200)))]
pub async fn get_schedule_solve_jobs(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ScheduleParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleRead,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let jobs: Vec<SolveJobResponse> = service
        .list_schedule_solve_jobs(&user.org_id, params.cycle_id.as_deref())
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "error": e })),
            )
        })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": jobs }))))
}

#[utoipa::path(get, path = "/api/schedule/solve/{id}", responses((status = 200)))]
pub async fn get_schedule_solve_job(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleRead,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let job = service
        .get_schedule_solve_job(&user.org_id, &id)
        .await
        .map_err(|e| {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "success": false, "error": e })))
        })?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": job }))))
}

#[utoipa::path(delete, path = "/api/schedule/solve/{id}", responses((status = 200)))]
pub async fn delete_schedule_solve_job(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let job = service
        .cancel_schedule_solve(&user.org_id, &id)
        .await
        .map_err(|e| {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "success": false, "error": e })))
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "data": job, "message": "solve job cancelled" })),
    ))
}

#[utoipa::path(post, path = "/api/schedule/solve/{id}/apply", responses((status = 200)))]
pub async fn post_apply_schedule_solve(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service
        .check_permission_and_audit(
            &user,
            crate::service::authz::Permission::ScheduleWrite,
            None,
        )
        .await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let job = service
        .apply_schedule_solve(&user.org_id, &id)
        .await
        .map_err(|e| {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else if e.contains("not completed") || e.contains("conflict") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, Json(json!({ "success": false, "error": e })))
        })?;

    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": job,
            "message": format!("applied {} lessons", job.lessons_created)
        })),
    ))
}
