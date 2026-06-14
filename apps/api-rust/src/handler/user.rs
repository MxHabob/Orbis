use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::user::{UpdateUserRequest, ChangeRoleRequest, ChangePasswordRequest, CreateUserRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/users", responses((status = 200, description = "Users list")))]
pub async fn get_users(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::UserParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::UserRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let users = service.get_users(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": users }))))
}

#[utoipa::path(post, path = "/api/users", request_body = CreateUserRequest, responses((status = 201, description = "User created")))]
pub async fn post_users(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::UserWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let created = service.create_user_in_org(&user.org_id, req, &user.id).await
        .map_err(|e| (StatusCode::CONFLICT, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": created }))))
}

#[utoipa::path(get, path = "/api/users/{id}", responses((status = 200, description = "User details")))]
pub async fn get_user(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_user_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::UserRead, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let u = service.get_user(&id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": u }))))
}

#[utoipa::path(put, path = "/api/users/{id}", request_body = UpdateUserRequest, responses((status = 200, description = "User updated")))]
pub async fn put_user(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_user_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::UserWrite, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let updated = service.update_user(&id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": updated }))))
}

#[utoipa::path(delete, path = "/api/users/{id}", responses((status = 200, description = "User deleted")))]
pub async fn delete_user(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::UserDelete, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_user(&id, &user.id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "user deleted" }))))
}

#[utoipa::path(post, path = "/api/users/{id}/role", request_body = ChangeRoleRequest, responses((status = 200, description = "Role changed")))]
pub async fn post_user_role(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<ChangeRoleRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::UserRoleWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let updated = service.change_user_role(&id, &req.role).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": updated }))))
}

#[utoipa::path(post, path = "/api/users/{id}/password", request_body = ChangePasswordRequest, responses((status = 200, description = "Password changed")))]
pub async fn post_user_password(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let ctx = crate::service::authz::AuthzContext {
        target_user_id: Some(id.clone()),
        ..Default::default()
    };
    service.authz.check_permission(&user, crate::service::authz::Permission::UserPasswordWrite, Some(&ctx))
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let updated = service.change_user_password(&id, &req.new_password).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": updated }))))
}
