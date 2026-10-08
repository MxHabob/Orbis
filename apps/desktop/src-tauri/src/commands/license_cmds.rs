//! License Tauri IPC commands — product key activation, trial, deactivation, and status.
//!
//! These commands expose the Mulverse license runtime to the Desktop UI.
//! They delegate to the `LicenseRuntime` port — no direct Mulverse HTTP calls.

use crate::state::AppState;
use orbis_domain::entities::license::LicenseStateDto;
use serde::{Deserialize, Serialize};
use tauri::State;

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateKeyRequest {
    pub product_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationResultDto {
    pub success: bool,
    pub license_state: LicenseStateDto,
    pub message: String,
}

// ═══════════════════════════════════════════════════════════════
// License IPC Handlers
// ═══════════════════════════════════════════════════════════════

/// Get the current license state for UI display.
#[tauri::command]
pub async fn license_get_state(
    state: State<'_, AppState>,
) -> Result<LicenseStateDto, String> {
    Ok(state.license_runtime.current_state_dto())
}

/// Activate a product key with Mulverse Platform.
#[tauri::command]
pub async fn license_activate_key(
    state: State<'_, AppState>,
    request: ActivateKeyRequest,
) -> Result<ActivationResultDto, String> {
    match state
        .license_runtime
        .activate_product_key(&request.product_key)
        .await
    {
        Ok(_response) => {
            let license_state = state.license_runtime.current_state_dto();
            Ok(ActivationResultDto {
                success: true,
                license_state,
                message: "Product key activated successfully".to_string(),
            })
        }
        Err(e) => {
            let license_state = state.license_runtime.current_state_dto();
            Ok(ActivationResultDto {
                success: false,
                license_state,
                message: e.to_string(),
            })
        }
    }
}

/// Start a trial evaluation period.
#[tauri::command]
pub async fn license_start_trial(
    state: State<'_, AppState>,
) -> Result<LicenseStateDto, String> {
    state
        .license_runtime
        .start_trial()
        .await
        .map_err(|e| e.to_string())?;

    Ok(state.license_runtime.current_state_dto())
}

/// Deactivate the current license (release device seat).
#[tauri::command]
pub async fn license_deactivate(
    state: State<'_, AppState>,
) -> Result<LicenseStateDto, String> {
    state
        .license_runtime
        .deactivate()
        .await
        .map_err(|e| e.to_string())?;

    Ok(state.license_runtime.current_state_dto())
}

/// Force a license refresh with Mulverse Platform.
#[tauri::command]
pub async fn license_refresh(
    state: State<'_, AppState>,
) -> Result<LicenseStateDto, String> {
    state
        .license_runtime
        .refresh()
        .await
        .map_err(|e| e.to_string())?;

    Ok(state.license_runtime.current_state_dto())
}

/// Check if a specific feature is enabled by the license.
#[tauri::command]
pub async fn license_check_feature(
    state: State<'_, AppState>,
    feature_key: String,
) -> Result<bool, String> {
    Ok(state.license_runtime.is_feature_enabled(&feature_key))
}

/// Get the installation identity for support/diagnostics.
#[tauri::command]
pub async fn license_get_installation_id(
    state: State<'_, AppState>,
) -> Result<String, String> {
    Ok(state.license_runtime.installation_id())
}
