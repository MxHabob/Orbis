use tauri::State;
use std::sync::Arc;
use orbis_app::usecases::integrations::connect_app::ConnectAppUseCase;
use orbis_app::usecases::integrations::get_connected_apps::GetConnectedAppsUseCase;
use orbis_app::context::RequestContext;
use uuid::Uuid;
use std::collections::HashMap;

use crate::state::AppState;

#[tauri::command]
pub async fn connect_app_cmd(
    state: State<'_, AppState>,
    provider_name: String,
    category: String,
    env: String,
    settings: serde_json::Value,
    secrets: HashMap<String, String>,
) -> Result<serde_json::Value, String> {
    let ctx = RequestContext {
        actor_id: Uuid::new_v4(), // Mock
        tenant_id: Uuid::nil(),   // Mock
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = ConnectAppUseCase {
        integration_repo: state.integration_repo.clone(),
    };

    let config = use_case.execute(
        &ctx,
        provider_name,
        category,
        env,
        settings,
        secrets
    ).await.map_err(|e| e.to_string())?;

    let json = serde_json::json!({
        "id": config.id,
        "provider_name": config.provider_name,
        "category": config.category,
        "is_enabled": config.is_enabled,
        "env": config.env,
        "settings": config.settings,
    });

    Ok(json)
}

#[tauri::command]
pub async fn get_connected_apps_cmd(
    state: State<'_, AppState>,
) -> Result<serde_json::Value, String> {
    let ctx = RequestContext {
        actor_id: Uuid::new_v4(),
        tenant_id: Uuid::nil(),
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = GetConnectedAppsUseCase {
        integration_repo: state.integration_repo.clone(),
    };

    let configs = use_case.execute(&ctx).await.map_err(|e| e.to_string())?;

    let arr: Vec<_> = configs.into_iter().map(|c| {
        serde_json::json!({
            "id": c.id,
            "provider_name": c.provider_name,
            "category": c.category,
            "is_enabled": c.is_enabled,
            "env": c.env,
            "settings": c.settings,
        })
    }).collect();

    Ok(serde_json::Value::Array(arr))
}
