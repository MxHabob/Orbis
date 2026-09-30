//! System Tauri IPC commands — system status, health, readiness, and entitlements.

use tauri::State;
use crate::state::AppState;

/// System health check.
#[tauri::command]
pub async fn system_health(state: State<'_, AppState>) -> Result<orbis_app::services::HealthResponse, String> {
    Ok(state.system_service.health().await)
}

/// Check if any organization exists in the system.
#[tauri::command]
pub async fn system_has_organization(state: State<'_, AppState>) -> Result<bool, String> {
    state
        .system_service
        .has_organization()
        .await
        .map_err(|e| e.to_string())
}

/// Get organization readiness status.
#[tauri::command]
pub async fn system_get_readiness(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::OrganizationReadiness, String> {
    state
        .system_service
        .get_readiness()
        .await
        .map_err(|e| e.to_string())
}

/// Get system entitlement summary.
#[tauri::command]
pub async fn system_get_entitlement(
    state: State<'_, AppState>,
) -> Result<orbis_domain::entities::entitlement::EntitlementResult, String> {
    state
        .entitlement_provider
        .check_entitlement("orbis_desktop", "default")
        .await
        .map_err(|e| e.to_string())
}
