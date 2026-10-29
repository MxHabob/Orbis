//! Auth extractors for the HTTP API layer.
//!
//! Provides `AuthContext` — an Axum extractor that reads the `Authorization: Bearer <token>`
//! header, validates the JWT via the shared `TokenService` in `AppState`, and produces
//! a `RequestContext` for downstream handlers.

use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};
use orbis_app::context::RequestContext;
use orbis_domain::iam::actor::Actor;
use uuid::Uuid;

use super::ApiError;
use crate::state::AppState;

/// Extractor that produces a validated `RequestContext` from the incoming request.
///
/// - **Authenticated path**: reads `Authorization: Bearer <token>`, validates JWT via
///   `AppState::token_service`, and populates `tenant_id` + `actor` from claims.
/// - **Fallback (dev-only)**: if no token is provided and the `ORBIS_DEV_MODE` env var
///   is set, produces a system context (useful for Swagger/testing).
pub struct AuthContext(pub RequestContext);

impl std::ops::Deref for AuthContext {
    type Target = RequestContext;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthContext {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let request_id = Uuid::new_v4();

        // Extract bearer token from Authorization header or query param (?token=... or ?access_token=...)
        let token_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let token = token_header.or_else(|| {
            parts.uri.query().and_then(|q| {
                q.split('&').find_map(|pair| {
                    let mut split = pair.splitn(2, '=');
                    let key = split.next()?;
                    let val = split.next()?;
                    if key == "token" || key == "access_token" {
                        Some(val.to_string())
                    } else {
                        None
                    }
                })
            })
        });

        match token.as_deref() {
            Some(token_str) => {
                // Enterprise DI: Delegate 100% of token verification to the injected TokenService
                let claims = state
                    .token_service
                    .validate_token(token_str)
                    .await
                    .map_err(|e| {
                        ApiError::unauthorized(format!("Invalid or expired token: {}", e))
                            .with_request_id(request_id.to_string())
                    })?;

                let user_id = Uuid::parse_str(&claims.sub)
                    .map_err(|_| ApiError::unauthorized("Invalid user ID in token"))?;
                let org_id = Uuid::parse_str(&claims.org_id)
                    .map_err(|_| ApiError::unauthorized("Invalid org ID in token"))?;

                Ok(AuthContext(RequestContext {
                    tenant_id: org_id,
                    actor: Actor::User(user_id),
                    request_id: request_id.to_string(),
                    source: orbis_app::context::ActionSource::Api,
                    org_unit_path: None,
                }))
            }
            None => {
                // Allow unauthenticated access in dev mode for Swagger testing
                if std::env::var("ORBIS_DEV_MODE").is_ok() {
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
                        source: orbis_app::context::ActionSource::Api,
                        org_unit_path: None,
                    }));
                }

                Err(ApiError::unauthorized("Missing Authorization header")
                    .with_request_id(request_id.to_string()))
            }
        }
    }
}

/// Optional auth — doesn't reject if no token is present, but returns None.
pub struct OptionalAuth(pub Option<RequestContext>);

#[axum::async_trait]
impl FromRequestParts<AppState> for OptionalAuth {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        match AuthContext::from_request_parts(parts, state).await {
            Ok(ctx) => Ok(OptionalAuth(Some(ctx.0))),
            Err(_) => Ok(OptionalAuth(None)),
        }
    }
}
