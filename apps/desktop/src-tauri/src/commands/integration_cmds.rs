//! Tauri IPC commands — Integration.

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use orbis_domain::iam::permission::Permission;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

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
pub async fn integration_connect_app(
    request: ConnectAppInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<IntegrationDto>, String> {
    let ctx = make_ctx(&state);
    if request.category.eq_ignore_ascii_case("storage") {
        state
            .integration_service
            .permission_checker
            .check(&ctx, Permission::OrganizationWrite)
            .await
            .map_err(|error| error.to_string())?;
        state
            .storage_resolver
            .check_connection(
                ctx.tenant_id,
                &request.provider_name,
                &request.category,
                &request.settings,
                &request.secrets,
            )
            .await
            .map_err(|error| error.to_string())?;
    }
    let config = state
        .integration_service
        .connect_app(
            &ctx,
            ctx.tenant_id,
            request.provider_name,
            request.category,
            request.env,
            request.settings,
            request.secrets,
        )
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

#[derive(Serialize)]
pub struct IntegrationStatusDto {
    pub connected: bool,
    pub is_enabled: bool,
    pub is_healthy: Option<bool>,
    pub health_error: Option<String>,
    pub config: Option<IntegrationDto>,
}

#[tauri::command]
pub async fn integration_get_status(
    provider_name: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<IntegrationStatusDto>, String> {
    let ctx = make_ctx(&state);
    let config_opt = state
        .integration_service
        .get_integration_by_provider(ctx.tenant_id, &provider_name)
        .await
        .map_err(|e| e.to_string())?;

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
            Ok(ok(IntegrationStatusDto {
                connected: true,
                is_enabled: c.is_enabled,
                is_healthy,
                health_error,
                config: Some(IntegrationDto {
                    id: c.id.to_string(),
                    org_id: c.org_id.to_string(),
                    provider_name: c.provider_name,
                    category: c.category,
                    is_enabled: c.is_enabled,
                    env: c.env,
                    settings: c.settings,
                }),
            }))
        }
        None => Ok(ok(IntegrationStatusDto {
            connected: false,
            is_enabled: false,
            is_healthy: None,
            health_error: None,
            config: None,
        })),
    }
}

#[tauri::command]
pub async fn integration_disconnect(
    provider_name: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    state
        .integration_service
        .delete_integration(&ctx, ctx.tenant_id, &provider_name)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(true))
}

#[tauri::command]
pub async fn integration_list(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<IntegrationDto>>, String> {
    let ctx = make_ctx(&state);
    let configs = state
        .integration_service
        .list_integrations(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(configs
        .into_iter()
        .map(|c| IntegrationDto {
            id: c.id.to_string(),
            org_id: c.org_id.to_string(),
            provider_name: c.provider_name,
            category: c.category,
            is_enabled: c.is_enabled,
            env: c.env,
            settings: c.settings,
        })
        .collect()))
}

#[tauri::command]
pub async fn integration_toggle(
    provider_name: String,
    is_enabled: bool,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let toggled = state
        .integration_service
        .toggle_integration(&ctx, ctx.tenant_id, &provider_name, is_enabled)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(toggled))
}
