use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
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

#[utoipa::path(
    get,
    path = "/api/classes",
    responses(
        (status = 200, description = "Classes list retrieved successfully")
    )
)]
pub async fn get_classes(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let classes = service.get_classes(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "data": classes })),
    ))
}

#[utoipa::path(
    post,
    path = "/api/classes",
    request_body = CreateClassRequest,
    responses(
        (status = 201, description = "Class created successfully")
    )
)]
pub async fn post_classes(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(req): Json<CreateClassRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // Role Guard: admin, super_admin, principal
    if user.primary_role != "admin" && user.primary_role != "super_admin" && user.primary_role != "principal" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "success": false, "error": "unauthorized: principal or admin role required" })),
        ));
    }

    if req.name.is_empty() || req.level.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "name and level are required" })),
        ));
    }

    let class = service.create_class(&req.name, &req.name_ar, &req.level, &req.division, &user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "success": true, "data": class })),
    ))
}
