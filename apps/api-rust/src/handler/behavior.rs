use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::service::behavior::AIBehaviorSubmission;
use crate::middleware::AuthenticatedUser;

#[utoipa::path(
    post,
    path = "/api/ai/behavior",
    request_body = AIBehaviorSubmission,
    responses(
        (status = 201, description = "AI behavior event processed successfully")
    )
)]
pub async fn post_ai_behavior(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<AIBehaviorSubmission>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Role Guard: admin, super_admin, teacher
    if user.primary_role != "admin" && user.primary_role != "super_admin" && user.primary_role != "teacher" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "success": false, "error": "unauthorized: teacher or admin role required" })),
        ));
    }

    if req.student_id.is_empty() || req.behavior_type.is_empty() || req.camera_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "student_id, behavior_type, and camera_id are required fields" })),
        ));
    }

    let record = service.submit_ai_behavior(req).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "success": true, "data": record })),
    ))
}

#[utoipa::path(
    get,
    path = "/api/behavior",
    responses(
        (status = 200, description = "Behavior timeline list retrieved successfully")
    )
)]
pub async fn get_behavior(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let records = service.get_behavior_records(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "data": records })),
    ))
}
