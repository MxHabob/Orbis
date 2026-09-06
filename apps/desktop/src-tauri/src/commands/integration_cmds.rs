//! Tauri IPC commands — Integration.

use tauri::State;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::state::AppState;
use orbis_app::usecases::integrations::{
    connect_app::ConnectAppUseCase,
    get_connected_apps::GetConnectedAppsUseCase,
};
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct IntegrationDto {
    pub id: String,
    pub org_id: String,
    pub provider_name: String,
    pub category: String,
    pub is_enabled: bool,
    pub env: String,
    pub settings: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct ConnectAppInput {
    pub provider_name: String,
    pub category: String,
    pub env: String,
    pub settings: serde_json::Value,
    pub secrets: HashMap<String, String>,
}

#[tauri::command]
pub async fn integration_connect_app(request: ConnectAppInput, state: State<'_, AppState>) -> Result<ApiResponse<IntegrationDto>, String> {
    let ctx = make_ctx(&state);
    let uc = ConnectAppUseCase {
        integration_repo: state.integration_repo.clone(),
    };
    let config = uc.execute(&ctx, request.provider_name, request.category, request.env, request.settings, request.secrets)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(IntegrationDto {
        id: config.id.to_string(),
        org_id: config.org_id.to_string(),
        provider_name: config.provider_name,
        category: config.category,
        is_enabled: config.is_enabled,
        env: config.env,
        settings: config.settings,
    }))
}

#[tauri::command]
pub async fn integration_list(state: State<'_, AppState>) -> Result<ApiResponse<Vec<IntegrationDto>>, String> {
    let ctx = make_ctx(&state);
    let uc = GetConnectedAppsUseCase {
        integration_repo: state.integration_repo.clone(),
    };
    let configs = uc.execute(&ctx).await.map_err(|e| e.to_string())?;
    Ok(ok(configs.into_iter().map(|c| IntegrationDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        provider_name: c.provider_name,
        category: c.category,
        is_enabled: c.is_enabled,
        env: c.env,
        settings: c.settings,
    }).collect()))
}
