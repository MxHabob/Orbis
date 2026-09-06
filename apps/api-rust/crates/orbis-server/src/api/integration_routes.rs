//! Integration routes — Connect apps, List connected apps.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::integrations::{
    connect_app::ConnectAppUseCase,
    get_connected_apps::GetConnectedAppsUseCase,
};

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

#[derive(Deserialize, ToSchema)]
pub struct ConnectAppRequest {
    pub provider_name: String,
    pub category: String,
    pub env: String,
    pub settings: serde_json::Value,
    pub secrets: HashMap<String, String>,
}

#[utoipa::path(post, path = "/api/v1/integrations", request_body = ConnectAppRequest,
    responses((status = 201, body = IntegrationDto)), security(("bearer" = [])), tag = "Integration")]
async fn connect_app(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ConnectAppRequest>,
) -> Result<Json<ApiResponse<IntegrationDto>>, ApiError> {
    let uc = ConnectAppUseCase {
        integration_repo: state.integration_repo.clone(),
    };
    let config = uc.execute(&ctx, p.provider_name, p.category, p.env, p.settings, p.secrets).await?;
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
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationDto>>>, ApiError> {
    let uc = GetConnectedAppsUseCase {
        integration_repo: state.integration_repo.clone(),
    };
    let configs = uc.execute(&ctx).await?;
    let dtos = configs.into_iter().map(|c| IntegrationDto {
        id: c.id,
        org_id: c.org_id,
        provider_name: c.provider_name,
        category: c.category,
        is_enabled: c.is_enabled,
        env: c.env,
        settings: c.settings,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/integrations", post(connect_app).get(list_integrations))
}

