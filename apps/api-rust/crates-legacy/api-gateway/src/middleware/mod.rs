use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::app_state::AppState;

/// Authenticated user extracted from the Authorization header.
pub struct AuthenticatedUser(pub crate::entities::user::Model);

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "success": false, "error": "missing authorization header" })),
                )
                    .into_response()
            })?;

        let parts_vec: Vec<&str> = auth_header.split_whitespace().collect();
        if parts_vec.len() != 2 || parts_vec[0] != "Bearer" {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "success": false, "error": "invalid authorization header schema" })),
            )
                .into_response());
        }

        let token = parts_vec[1];
        let user = state.validate_token(token).await.map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "success": false, "error": format!("unauthorized: {}", e) })),
            )
                .into_response()
        })?;

        Ok(AuthenticatedUser(user))
    }
}
