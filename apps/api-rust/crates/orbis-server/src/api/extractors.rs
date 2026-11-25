//! Auth extractors for the HTTP API layer.
//!
//! Provides `AuthContext` — an Axum extractor that reads the `Authorization: Bearer <token>`
//! header, validates the session via the shared authentication service, and produces
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
/// - **Authenticated path**: validates the bearer token and retrieves the active user.
/// - **Unauthenticated path**: rejected unconditionally.
#[derive(Clone)]
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
        if let Some(context) = parts.extensions.get::<Self>() {
            return Ok(context.clone());
        }
        let request_id = Uuid::new_v4();

        // EventSource clients cannot supply an Authorization header.
        let token_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(|s| s.to_string());

        let token = token_header.or_else(|| {
            if parts.uri.path() != "/api/v1/notifications/stream" {
                return None;
            }
            parts.uri.query().and_then(|q| {
                q.split('&').find_map(|pair| {
                    let mut split = pair.splitn(2, '=');
                    let key = split.next()?;
                    let val = split.next()?;
                    if key == "token" {
                        Some(val.to_string())
                    } else {
                        None
                    }
                })
            })
        });

        match token.as_deref() {
            Some(token_str) => {
                let user = state
                    .auth_service
                    .authenticated_user(token_str)
                    .await
                    .map_err(|error| {
                        ApiError::from(error).with_request_id(request_id.to_string())
                    })?;

                let context = AuthContext(RequestContext {
                    tenant_id: user.org_id,
                    actor: Actor::User(user.id),
                    request_id: request_id.to_string(),
                    source: orbis_app::context::ActionSource::Api,
                    org_unit_path: None,
                });
                parts.extensions.insert(context.clone());
                Ok(context)
            }
            None => Err(ApiError::unauthorized("Missing Authorization header")
                .with_request_id(request_id.to_string())),
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
