use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::academic_cycle::{CreateCycleRequest, UpdateCycleRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/cycles", responses((status = 200, description = "Cycles list")))]
pub async fn get_cycles(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::CycleParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycles = service.get_cycles(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": cycles }))))
}

#[utoipa::path(post, path = "/api/cycles", request_body = CreateCycleRequest, responses((status = 201, description = "Cycle created")))]
pub async fn post_cycles(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateCycleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle = service.create_cycle(&user.org_id, req).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": cycle }))))
}

#[utoipa::path(get, path = "/api/cycles/{id}", responses((status = 200, description = "Cycle details")))]
pub async fn get_cycle(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle = service.get_cycle(&id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": cycle }))))
}

#[utoipa::path(put, path = "/api/cycles/{id}", request_body = UpdateCycleRequest, responses((status = 200, description = "Cycle updated")))]
pub async fn put_cycle(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateCycleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle = service.update_cycle(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": cycle }))))
}

#[utoipa::path(post, path = "/api/cycles/{id}/activate", responses((status = 200, description = "Cycle activated")))]
pub async fn post_activate_cycle(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleActivate, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle = service.activate_cycle(&user.org_id, &id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": cycle }))))
}

#[utoipa::path(post, path = "/api/cycles/{id}/lock", responses((status = 200, description = "Cycle locked")))]
pub async fn post_lock_cycle(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::CycleLock, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let cycle = service.lock_cycle(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": cycle }))))
}
