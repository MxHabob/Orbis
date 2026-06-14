use axum::{
    extract::{State, Json},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub org_slug: String,
}

#[derive(Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Registration successful")
    )
)]
pub async fn post_register(
    State(service): State<Service>,
    Json(req): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if req.email.is_empty() || req.password.is_empty() || req.full_name.is_empty() || req.org_slug.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "email, password, full_name, and org_slug are required" })),
        ));
    }

    let user = service.register(&req.email, &req.password, &req.full_name, &req.org_slug).await
        .map_err(|e| (StatusCode::CONFLICT, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::CREATED,
        Json(json!({ "success": true, "data": user })),
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse)
    )
)]
pub async fn post_login(
    State(service): State<Service>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if req.email.is_empty() || req.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "email and password are required" })),
        ));
    }

    let resp = service.login(&req.email, &req.password).await
        .map_err(|e| (StatusCode::UNAUTHORIZED, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "data": resp })),
    ))
}

#[utoipa::path(
    get,
    path = "/api/auth/me",
    responses(
        (status = 200, description = "Me info retrieved successfully")
    )
)]
pub async fn get_me(
    AuthenticatedUser(user): AuthenticatedUser,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "success": true, "data": user })),
    )
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 200, description = "Logged out successfully")
    )
)]
pub async fn post_logout(
    State(service): State<Service>,
    headers: axum::http::HeaderMap,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let auth_str = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "missing authorization header" }))))?;

    let parts: Vec<&str> = auth_str.split_whitespace().collect();
    if parts.len() != 2 || parts[0] != "Bearer" {
        return Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "invalid token format" }))));
    }

    let token = parts[1];
    service.logout(token).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;

    Ok((
        StatusCode::OK,
        Json(json!({ "success": true, "message": "logged out successfully" })),
    ))
}
