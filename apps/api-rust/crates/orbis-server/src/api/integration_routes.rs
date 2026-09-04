use axum::{
    extract::{State, Path},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::state::AppState;
use orbis_app::usecases::integrations::connect_app::ConnectAppUseCase;
use orbis_app::usecases::integrations::get_connected_apps::GetConnectedAppsUseCase;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_connected_apps))
        .route("/connect", post(connect_app))
}

#[derive(Deserialize)]
pub struct ConnectAppRequest {
    pub provider_name: String,
    pub category: String,
    pub env: String,
    pub settings: serde_json::Value,
    pub secrets: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct IntegrationConfigResponse {
    pub id: Uuid,
    pub provider_name: String,
    pub category: String,
    pub is_enabled: bool,
    pub env: String,
    pub settings: Option<serde_json::Value>,
}

async fn get_connected_apps(
    State(state): State<AppState>,
) -> Result<Json<Vec<IntegrationConfigResponse>>, String> {
    // We assume the middleware extracts user/tenant into extensions, but for simplicity, we mock ctx
    use orbis_app::context::RequestContext;
    let ctx = RequestContext {
        actor_id: Uuid::new_v4(), // Mocked
        tenant_id: Uuid::nil(),   // Mocked
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = GetConnectedAppsUseCase {
        integration_repo: state.integration_repo.clone(),
    };

    let configs = use_case.execute(&ctx).await.map_err(|e| e.to_string())?;

    let response: Vec<_> = configs.into_iter().map(|c| IntegrationConfigResponse {
        id: c.id,
        provider_name: c.provider_name,
        category: c.category,
        is_enabled: c.is_enabled,
        env: c.env,
        settings: c.settings,
    }).collect();

    Ok(Json(response))
}

async fn connect_app(
    State(state): State<AppState>,
    Json(payload): Json<ConnectAppRequest>,
) -> Result<Json<IntegrationConfigResponse>, String> {
    use orbis_app::context::RequestContext;
    let ctx = RequestContext {
        actor_id: Uuid::new_v4(),
        tenant_id: Uuid::nil(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = ConnectAppUseCase {
        integration_repo: state.integration_repo.clone(),
    };

    let config = use_case.execute(
        &ctx,
        payload.provider_name,
        payload.category,
        payload.env,
        payload.settings,
        payload.secrets,
    ).await.map_err(|e| e.to_string())?;

    Ok(Json(IntegrationConfigResponse {
        id: config.id,
        provider_name: config.provider_name,
        category: config.category,
        is_enabled: config.is_enabled,
        env: config.env,
        settings: config.settings,
    }))
}
