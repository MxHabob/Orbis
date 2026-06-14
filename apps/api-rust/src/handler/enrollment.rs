use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::enrollment::{CreateEnrollmentRequest, WithdrawRequest, TransferRequest, PromoteRequest};
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/enrollments", responses((status = 200, description = "Enrollments list")))]
pub async fn get_enrollments(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::EnrollmentParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollments = service.get_enrollments(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": enrollments }))))
}

#[utoipa::path(post, path = "/api/enrollments", request_body = CreateEnrollmentRequest, responses((status = 201, description = "Student enrolled")))]
pub async fn post_enrollments(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateEnrollmentRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollment = service.create_enrollment(&user.org_id, req).await
        .map_err(|e| (StatusCode::CONFLICT, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": enrollment }))))
}

#[utoipa::path(get, path = "/api/enrollments/{id}", responses((status = 200, description = "Enrollment details")))]
pub async fn get_enrollment(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentRead, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollment = service.get_enrollment(&id).await
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": enrollment }))))
}

#[utoipa::path(post, path = "/api/enrollments/{id}/withdraw", request_body = WithdrawRequest, responses((status = 200, description = "Student withdrawn")))]
pub async fn post_withdraw(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<WithdrawRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentWrite, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollment = service.withdraw_enrollment(&id, &req.reason).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": enrollment }))))
}

#[utoipa::path(post, path = "/api/enrollments/{id}/transfer", request_body = TransferRequest, responses((status = 200, description = "Student transferred")))]
pub async fn post_transfer(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<TransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentTransfer, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let enrollment = service.transfer_enrollment(&id, &req.new_class_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": enrollment }))))
}

#[utoipa::path(post, path = "/api/enrollments/promote", request_body = PromoteRequest, responses((status = 200, description = "Promotion results")))]
pub async fn post_promote(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<PromoteRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.authz.check_permission(&user, crate::service::authz::Permission::EnrollmentPromote, None)
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let result = service.promote_students(&user.org_id, req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": result }))))
}
