//! Integration routes — Connect apps, List connected apps.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{delete, get, put},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_domain::iam::permission::Permission;

#[derive(Serialize, ToSchema)]
pub struct IntegrationDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub provider_name: String,
    pub category: String,
    pub is_enabled: bool,
    pub env: String,
    pub settings: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct IntegrationStatusDto {
    pub connected: bool,
    pub is_enabled: bool,
    pub is_healthy: Option<bool>,
    pub health_error: Option<String>,
    pub config: Option<IntegrationDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct ConnectAppRequest {
    pub provider_name: String,
    pub category: String,
    pub env: String,
    pub settings: serde_json::Value,
    pub secrets: HashMap<String, String>,
}

#[derive(Deserialize, ToSchema)]
pub struct ToggleIntegrationRequest {
    pub is_enabled: bool,
}

#[derive(Deserialize, ToSchema, utoipa::IntoParams)]
pub struct NotificationIntegrationQuery {
    pub channel: String,
}

#[derive(Serialize, ToSchema)]
pub struct NotificationIntegrationCheckDto {
    pub id: Uuid,
    pub provider_name: String,
    pub channel: String,
    pub is_enabled: bool,
    pub is_supported: bool,
    pub is_healthy: Option<bool>,
    pub is_eligible: bool,
    pub reason: Option<String>,
}

#[utoipa::path(get, path = "/api/v1/integrations/notifications", params(NotificationIntegrationQuery),
    responses((status = 200, body = Vec<NotificationIntegrationCheckDto>)), security(("bearer" = [])), tag = "Integration")]
async fn list_notification_integrations(
    ctx: AuthContext,
    Query(query): Query<NotificationIntegrationQuery>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<NotificationIntegrationCheckDto>>>, ApiError> {
    state
        .permission_checker
        .check(&ctx, Permission::NotificationWrite)
        .await?;
    let checks = state
        .notification_adapter
        .check_integrations(ctx.tenant_id, &query.channel)
        .await
        .map_err(ApiError::from)?;
    let dtos = checks
        .into_iter()
        .map(|check| NotificationIntegrationCheckDto {
            id: check.id,
            provider_name: check.provider_name,
            channel: check.channel,
            is_enabled: check.is_enabled,
            is_supported: check.is_supported,
            is_healthy: check.is_healthy,
            is_eligible: check.is_eligible,
            reason: check.reason.map(str::to_string),
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/integrations", request_body = ConnectAppRequest,
    responses((status = 201, body = IntegrationDto)), security(("bearer" = [])), tag = "Integration")]
async fn connect_app(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<ConnectAppRequest>,
) -> Result<Json<ApiResponse<IntegrationDto>>, ApiError> {
    state
        .integration_service
        .permission_checker
        .check(&ctx, Permission::OrganizationWrite)
        .await?;

    orbis_infra::adapters::verification::verify_integration_connection(
        &ctx,
        &p.provider_name,
        &p.category,
        &p.settings,
        &p.secrets,
        &state.integration_resolver,
        &state.notification_adapter,
        &state.storage_resolver,
    )
    .await
    .map_err(|error| ApiError::bad_request(error.to_string()))?;

    let config = state
        .integration_service
        .connect_app(
            &ctx,
            ctx.tenant_id,
            p.provider_name,
            p.category,
            p.env,
            p.settings,
            p.secrets,
        )
        .await?;
    Ok(ApiResponse::ok(IntegrationDto {
        id: config.id,
        org_id: config.org_id,
        provider_name: config.provider_name,
        category: config.category,
        is_enabled: config.is_enabled,
        env: config.env,
        settings: config.settings,
    }))
}

#[utoipa::path(get, path = "/api/v1/integrations",
    responses((status = 200, body = Vec<IntegrationDto>)), security(("bearer" = [])), tag = "Integration")]
async fn list_integrations(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationDto>>>, ApiError> {
    let configs = state
        .integration_service
        .list_integrations(ctx.tenant_id)
        .await?;
    let dtos = configs
        .into_iter()
        .map(|c| IntegrationDto {
            id: c.id,
            org_id: c.org_id,
            provider_name: c.provider_name,
            category: c.category,
            is_enabled: c.is_enabled,
            env: c.env,
            settings: c.settings,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/integrations/{provider_name}/status",
    responses((status = 200, body = IntegrationStatusDto)), security(("bearer" = [])), tag = "Integration")]
async fn get_integration_status(
    ctx: AuthContext,
    Path(provider_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<IntegrationStatusDto>>, ApiError> {
    let config_opt = state
        .integration_service
        .get_integration_by_provider(ctx.tenant_id, &provider_name)
        .await?;
    match config_opt {
        Some(c) => {
            let (is_healthy, health_error) = if c.category.eq_ignore_ascii_case("storage") {
                match state
                    .storage_resolver
                    .check_integration_connection(ctx.tenant_id, &c.provider_name)
                    .await
                {
                    Ok(()) => (Some(true), None),
                    Err(error) => (Some(false), Some(error.to_string())),
                }
            } else {
                (None, None)
            };
            Ok(ApiResponse::ok(IntegrationStatusDto {
                connected: true,
                is_enabled: c.is_enabled,
                is_healthy,
                health_error,
                config: Some(IntegrationDto {
                    id: c.id,
                    org_id: c.org_id,
                    provider_name: c.provider_name,
                    category: c.category,
                    is_enabled: c.is_enabled,
                    env: c.env,
                    settings: c.settings,
                }),
            }))
        }
        None => Ok(ApiResponse::ok(IntegrationStatusDto {
            connected: false,
            is_enabled: false,
            is_healthy: None,
            health_error: None,
            config: None,
        })),
    }
}

#[utoipa::path(delete, path = "/api/v1/integrations/{provider_name}",
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Integration")]
async fn disconnect_app(
    ctx: AuthContext,
    Path(provider_name): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state
        .integration_service
        .delete_integration(&ctx, ctx.tenant_id, &provider_name)
        .await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(put, path = "/api/v1/integrations/{provider_name}/toggle", request_body = ToggleIntegrationRequest,
    responses((status = 200, body = bool)), security(("bearer" = [])), tag = "Integration")]
async fn toggle_integration(
    ctx: AuthContext,
    Path(provider_name): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<ToggleIntegrationRequest>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    let toggled = state
        .integration_service
        .toggle_integration(&ctx, ctx.tenant_id, &provider_name, body.is_enabled)
        .await?;
    Ok(ApiResponse::ok(toggled))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/integrations",
            get(list_integrations).post(connect_app),
        )
        .route(
            "/api/v1/integrations/notifications",
            get(list_notification_integrations),
        )
        .route(
            "/api/v1/integrations/:provider_name/status",
            get(get_integration_status),
        )
        .route(
            "/api/v1/integrations/:provider_name",
            delete(disconnect_app),
        )
        .route(
            "/api/v1/integrations/:provider_name/toggle",
            put(toggle_integration),
        )
}
