use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::service::Service;
use crate::middleware::AuthenticatedUser;

#[utoipa::path(get, path = "/api/dashboard/stats", responses((status = 200, description = "Dashboard statistics")))]
pub async fn get_dashboard_stats(
    State(service): State<Service>,
    AuthenticatedUser(user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let stats = service.get_dashboard_stats(&user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))))?;
    Ok((StatusCode::OK, Json(json!({ "success": true, "data": stats }))))
}
