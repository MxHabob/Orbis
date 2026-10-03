//! Integration routes — Connect apps, List connected apps.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, put},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;

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

#[utoipa::path(post, path = "/api/v1/integrations", request_body = ConnectAppRequest,
    responses((status = 201, body = IntegrationDto)), security(("bearer" = [])), tag = "Integration")]
async fn connect_app(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<ConnectAppRequest>,
) -> Result<Json<ApiResponse<IntegrationDto>>, ApiError> {
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
        Some(c) => Ok(ApiResponse::ok(IntegrationStatusDto {
            connected: true,
            is_enabled: c.is_enabled,
            config: Some(IntegrationDto {
                id: c.id,
                org_id: c.org_id,
                provider_name: c.provider_name,
                category: c.category,
                is_enabled: c.is_enabled,
                env: c.env,
                settings: c.settings,
            }),
        })),
        None => Ok(ApiResponse::ok(IntegrationStatusDto {
            connected: false,
            is_enabled: false,
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
