//! Auth extractors for the HTTP API layer.
//!
//! Provides `AuthContext` — an Axum extractor that reads the `Authorization: Bearer <token>`
//! header, validates the JWT, and produces a `RequestContext` for downstream handlers.

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use orbis_app::context::RequestContext;
use orbis_domain::iam::actor::Actor;
use uuid::Uuid;

use super::ApiError;

/// Extractor that produces a validated `RequestContext` from the incoming request.
///
/// - **Authenticated path**: reads `Authorization: Bearer <token>`, validates JWT via
///   `TokenService`, and populates `tenant_id` + `actor` from claims.
/// - **Fallback (dev-only)**: if no token is provided and the `ORBIS_DEV_MODE` env var
///   is set, produces a system context (useful for Swagger/testing).
///
/// Usage in a handler:
/// ```ignore
/// async fn my_handler(ctx: AuthContext, ...) -> Result<..., ApiError> {
///     let tenant = ctx.tenant_id;
///     // ...
/// }
/// ```
pub struct AuthContext(pub RequestContext);

impl std::ops::Deref for AuthContext {
    type Target = RequestContext;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let request_id = Uuid::new_v4();

        // Extract bearer token from Authorization header
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        match token {
            Some(token_str) => {
                // Decode the JWT claims directly (simple decode without full TokenService DI).
                // In production, you'd inject TokenService via State, but for an extractor
                // we use the same JWT secret and decode inline.
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "super-secret-key-for-dev".to_string());
                let decoding = jsonwebtoken::DecodingKey::from_secret(secret.as_bytes());
                let validation = jsonwebtoken::Validation::default();

                let token_data = jsonwebtoken::decode::<JwtClaims>(token_str, &decoding, &validation)
                    .map_err(|e| {
                        ApiError::unauthorized(format!("Invalid or expired token: {}", e))
                            .with_request_id(request_id.to_string())
                    })?;

                let claims = token_data.claims;
                let user_id = Uuid::parse_str(&claims.sub)
                    .map_err(|_| ApiError::unauthorized("Invalid user ID in token"))?;
                let org_id = Uuid::parse_str(&claims.org_id)
                    .map_err(|_| ApiError::unauthorized("Invalid org ID in token"))?;

                Ok(AuthContext(RequestContext {
                    tenant_id: org_id,
                    actor: Actor::User(user_id),
                    request_id: request_id.to_string(),
                }))
            }
            None => {
                // Allow unauthenticated access in dev mode for Swagger testing
                if std::env::var("ORBIS_DEV_MODE").is_ok() {
                    // Check for X-Org-Id header (dev override)
                    let dev_org_id = parts
                        .headers
                        .get("x-org-id")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| Uuid::parse_str(v).ok())
                        .unwrap_or_default();

                    return Ok(AuthContext(RequestContext {
                        tenant_id: dev_org_id,
                        actor: Actor::System,
                        request_id: request_id.to_string(),
                    }));
                }

                Err(
                    ApiError::unauthorized("Missing Authorization header")
                        .with_request_id(request_id.to_string()),
                )
            }
        }
    }
}

/// Optional auth — doesn't reject if no token is present, but returns None.
pub struct OptionalAuth(pub Option<RequestContext>);

impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthContext::from_request_parts(parts, state).await {
            Ok(ctx) => Ok(OptionalAuth(Some(ctx.0))),
            Err(_) => Ok(OptionalAuth(None)),
        }
    }
}

/// JWT claims structure matching what `JwtTokenService` produces.
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct JwtClaims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub exp: usize,
}
