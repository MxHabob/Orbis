use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct CreateClassRequest {
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: String,
}

use crate::service::class::UpdateClassRequest;

#[utoipa::path(get, path = "/api/classes", responses((status = 200, description = "Classes list retrieved successfully")))]
pub async fn get_classes(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::ClassParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ClassRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let classes = service.get_classes(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": classes }))))
}

#[utoipa::path(post, path = "/api/classes", request_body = CreateClassRequest, responses((status = 201, description = "Class created successfully")))]
pub async fn post_classes(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateClassRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ClassWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    if req.name.is_empty() || req.level.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "name and level are required" }))));
    }
    let class = service.create_class(&req.name, &req.name_ar, &req.level, &req.division, &user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": class }))))
}

#[utoipa::path(put, path = "/api/classes/{id}", request_body = UpdateClassRequest, responses((status = 200, description = "Class updated")))]
pub async fn put_class(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateClassRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ClassWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let class = service.update_class(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": class }))))
}

#[utoipa::path(delete, path = "/api/classes/{id}", responses((status = 200, description = "Class deleted")))]
pub async fn delete_class(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ClassDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_class(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "class deleted" }))))
}

#[utoipa::path(get, path = "/api/classes/{id}/students", responses((status = 200, description = "Class students")))]
pub async fn get_class_students(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::ClassRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollments = service.repo.get_enrollments_by_class(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e.to_string() }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": enrollments }))))
}
