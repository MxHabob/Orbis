use axum::{
    extract::{State, Path, Query, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::service::Service;
use crate::service::behavior::AIBehaviorSubmission;
use crate::middleware::AuthenticatedUser;

use crate::service::behavior::UpdateBehaviorRequest;

#[utoipa::path(post, path = "/api/ai/behavior", request_body = AIBehaviorSubmission, responses((status = 201, description = "AI behavior event processed successfully")))]
pub async fn post_ai_behavior(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<AIBehaviorSubmission>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::BehaviorAI, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    if req.student_id.is_empty() || req.behavior_type.is_empty() || req.camera_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "student_id, behavior_type, and camera_id are required fields" }))));
    }
    let record = service.submit_ai_behavior(req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::CREATED, Json(json!({ "success": true, "data": record }))))
}

#[utoipa::path(get, path = "/api/behavior", responses((status = 200, description = "Behavior timeline list retrieved successfully")))]
pub async fn get_behavior(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::BehaviorParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::BehaviorRead, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let records = service.get_behavior_records(&user.org_id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": records }))))
}

#[utoipa::path(put, path = "/api/behavior/{id}", request_body = UpdateBehaviorRequest, responses((status = 200, description = "Behavior record updated")))]
pub async fn put_behavior(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<UpdateBehaviorRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::BehaviorWrite, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    let record = service.update_behavior_record(&id, req, &user.id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": record }))))
}

#[utoipa::path(delete, path = "/api/behavior/{id}", responses((status = 200, description = "Behavior record deleted")))]
pub async fn delete_behavior(
    State(service): State<Service>,
    Path(id): Path<String>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    service.check_permission_and_audit(&user, crate::service::authz::Permission::BehaviorDelete, None).await
        .map_err(|e| (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": e }))))?;

    service.delete_behavior_record(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "message": "behavior record deleted" }))))
}
