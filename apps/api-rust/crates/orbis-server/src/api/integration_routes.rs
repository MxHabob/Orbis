//! Integration routes — Connect apps, List connected apps.

use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::integrations::connect_app::ConnectAppUseCase;

#[derive(Serialize, ToSchema)]
pub struct IntegrationDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub app_key: String,
    pub app_name: String,
    pub is_active: bool,
}

#[derive(Deserialize, ToSchema)]
pub struct ConnectAppRequest {
    pub app_key: String,
    pub credentials: serde_json::Value,
}

#[utoipa::path(post, path = "/api/v1/integrations", request_body = ConnectAppRequest,
    responses((status = 201, body = IntegrationDto)), security(("bearer" = [])), tag = "Integration")]
async fn connect_app(
    ctx: AuthContext, State(state): State<AppState>, Json(p): Json<ConnectAppRequest>,
) -> Result<Json<ApiResponse<IntegrationDto>>, ApiError> {
    let uc = ConnectAppUseCase {
        integration_repo: state.integration_repo.clone(),
        integration_resolver: state.integration_resolver.clone(),
    };
    let config = uc.execute(&ctx, ctx.tenant_id, p.app_key.clone(), p.credentials).await?;
    Ok(ApiResponse::ok(IntegrationDto {
        id: config.id, org_id: config.org_id, app_key: config.app_key,
        app_name: config.app_name, is_active: config.is_active,
    }))
}

#[utoipa::path(get, path = "/api/v1/integrations",
    responses((status = 200, body = Vec<IntegrationDto>)), security(("bearer" = [])), tag = "Integration")]
async fn list_integrations(
    ctx: AuthContext, State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<IntegrationDto>>>, ApiError> {
    let configs = state.integration_repo.find_configs_by_org(ctx.tenant_id).await?;
    let dtos = configs.into_iter().map(|c| IntegrationDto {
        id: c.id, org_id: c.org_id, app_key: c.app_key,
        app_name: c.app_name, is_active: c.is_active,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/integrations", post(connect_app).get(list_integrations))
}
