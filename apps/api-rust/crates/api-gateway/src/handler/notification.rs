use axum::{
    extract::{State, Path, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[utoipa::path(
    get,
    path = "/api/notifications",
    responses(
        (status = 200, description = "User notifications feed retrieved successfully")
    )
)]
pub async fn get_notifications(
    State(service): State<Service>,
    Query(params): Query<crate::handler::params::NotificationParams>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let notifications = service.get_notifications(&user.id, params).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "data": notifications })),
    ))
}

#[utoipa::path(
    post,
    path = "/api/notifications/{id}/read",
    params(
        ("id" = String, Path, description = "Notification ID to mark as read")
    ),
    responses(
        (status = 200, description = "Notification marked as read successfully")
    )
)]
pub async fn post_mark_read(
    State(service): State<Service>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "notification ID is required" })),
        ));
    }

    service.mark_notification_read(&id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "message": "notification marked as read" })),
    ))
}
