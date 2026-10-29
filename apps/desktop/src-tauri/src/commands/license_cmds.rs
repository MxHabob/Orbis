//! License Tauri IPC commands — product key activation, trial, deactivation, and status.
//!
//! These commands expose the Mulverse license runtime to the Desktop UI.
//! They delegate to the `LicenseRuntime` port — no direct Mulverse HTTP calls.

use crate::state::AppState;
use orbis_domain::entities::license::{LicenseStateDto, TrialOrganizationProfile};
use orbis_domain::errors::DomainError;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

// ═══════════════════════════════════════════════════════════════
// DTOs
// ═══════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateKeyRequest {
    pub product_key: String,
}

// ═══════════════════════════════════════════════════════════════
// License IPC Handlers
// ═══════════════════════════════════════════════════════════════

/// Get the current license state for UI display.
#[tauri::command]
pub async fn license_get_state(state: State<'_, AppState>) -> Result<LicenseStateDto, String> {
    Ok(state.license_runtime.current_state_dto())
}

/// Activate a customer-bound key or claim and activate a reseller key.
#[tauri::command]
pub async fn license_activate_key(
    state: State<'_, AppState>,
    request: ActivateKeyRequest,
) -> Result<LicenseStateDto, String> {
    let ctx = crate::commands::make_ctx(&state);
    if ctx.tenant_id.is_nil() {
        return Err("An active organization is required to activate a product key.".to_string());
    }
    match state
        .license_runtime
        .activate_product_key(&request.product_key, ctx.tenant_id)
        .await
    {
        Ok(_) => {}
        Err(DomainError::LicenseKeyRequiresClaim) => {
            let organization = trial_organization_profile(&state, ctx.tenant_id).await?;
            state
                .license_runtime
                .redeem_product_key(&request.product_key, organization)
                .await
                .map_err(|error| error.to_string())?;
        }
        Err(error) => return Err(error.to_string()),
    }
    Ok(state.license_runtime.current_state_dto())
}

/// Start a trial evaluation period.
#[tauri::command]
pub async fn license_start_trial(state: State<'_, AppState>) -> Result<LicenseStateDto, String> {
    let ctx = crate::commands::make_ctx(&state);
    if ctx.tenant_id.is_nil() {
        return Err("An active organization is required to start a license trial.".to_string());
    }
    let organization = trial_organization_profile(&state, ctx.tenant_id).await?;
    state
        .license_runtime
        .start_trial(organization)
        .await
        .map_err(|e| e.to_string())?;

    Ok(state.license_runtime.current_state_dto())
}

/// Deactivate the current license (release device seat).
#[tauri::command]
pub async fn license_deactivate(state: State<'_, AppState>) -> Result<LicenseStateDto, String> {
    state
        .license_runtime
        .deactivate()
        .await
        .map_err(|e| e.to_string())?;

    Ok(state.license_runtime.current_state_dto())
}

/// Force a license refresh with Mulverse Platform.
#[tauri::command]
pub async fn license_refresh(state: State<'_, AppState>) -> Result<LicenseStateDto, String> {
    let ctx = crate::commands::make_ctx(&state);
    if ctx.tenant_id.is_nil() {
        return Err("An active organization is required to refresh its license.".to_string());
    }
    state
        .license_runtime
        .refresh(Some(ctx.tenant_id))
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
pub async fn license_get_installation_id(state: State<'_, AppState>) -> Result<String, String> {
    Ok(state.license_runtime.installation_id())
}

async fn trial_organization_profile(
    state: &AppState,
    organization_id: Uuid,
) -> Result<TrialOrganizationProfile, String> {
    let organization = state
        .org_repo
        .find_by_id(organization_id)
        .await
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "The active organization could not be found.".to_string())?;
    let metadata_value = |key: &str| {
        organization
            .metadata
            .as_ref()
            .and_then(|metadata| metadata.get(key))
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
    };
    let institution_type = metadata_value("institution_type")
        .ok_or_else(|| "The organization is missing its institution type.".to_string())?;
    let organization_size = metadata_value("organization_size");
    let educational_track = metadata_value("educational_track");
    let country_code = metadata_value("country_code");
    let country_name = metadata_value("country_name");
    let region_name = metadata_value("region_name");
    let city = metadata_value("city");

    Ok(TrialOrganizationProfile {
        id: organization.id,
        name: organization.name,
        name_ar: organization.name_ar,
        slug: organization.slug,
        institution_type,
        organization_size,
        educational_track,
        country_code,
        country_name,
        region_name,
        city,
    })
}
