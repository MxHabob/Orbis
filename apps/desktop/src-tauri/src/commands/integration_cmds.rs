//! Tauri IPC commands — Integration.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct IntegrationDto {
    pub id: String,
    pub org_id: String,
    pub provider_name: String,
    pub is_enabled: bool,
    pub settings: Option<serde_json::Value>,
}

#[tauri::command]
pub async fn integration_list(state: State<'_, AppState>) -> Result<ApiResponse<Vec<IntegrationDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.integration_repo.find_configs_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| IntegrationDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        provider_name: c.provider_name,
        is_enabled: c.is_enabled,
        settings: c.settings,
    }).collect()))
}
