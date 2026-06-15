use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;
use crate::service::guardian::{CreateGuardianLinkRequest, UpdateGuardianLinkRequest};

#[utoipa::path(get, path = "/api/guardians", responses((status = 200)))]
pub async fn get_guardians(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::GuardianParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::GuardianRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let links = if let Some(student_id) = params.student_id {
        service.get_guardians_by_student(&student_id).await
    } else if let Some(guardian_id) = params.guardian_id {
        service.get_students_by_guardian(&guardian_id).await
    } else {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "student_id or guardian_id is required" }))));
    }.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((StatusCode::OK, Json(json!({ "success": true, "data": links }))))
}

#[utoipa::path(post, path = "/api/guardians", responses((status = 201)))]
pub async fn post_guardians(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateGuardianLinkRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::GuardianWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let link = service.create_guardian_link(req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": link }))))
}

#[utoipa::path(put, path = "/api/guardians/{id}", responses((status = 200)))]
pub async fn put_guardian(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateGuardianLinkRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::GuardianWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let link = service.update_guardian_link(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": link }))))
}

#[utoipa::path(delete, path = "/api/guardians/{id}", responses((status = 200)))]
pub async fn delete_guardian(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::GuardianDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_guardian_link(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "guardian link deleted" }))))
}
