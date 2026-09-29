//! Bootstrap & System Tauri IPC commands — organization bootstrap and system readiness.
//!
//! These commands require AppState to be initialized (database connected).

use tauri::State;
use crate::state::AppState;

/// Get system health status.
#[tauri::command]
pub async fn system_health(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::HealthResponse, String> {
    Ok(state.system_service.health().await)
}

/// Check if an organization exists.
#[tauri::command]
pub async fn system_has_organization(
    state: State<'_, AppState>,
) -> Result<bool, String> {
    state.system_service.has_organization().await.map_err(|e| e.to_string())
}

/// Get organization readiness — what setup items are pending.
#[tauri::command]
pub async fn system_get_readiness(
    state: State<'_, AppState>,
) -> Result<orbis_app::services::OrganizationReadiness, String> {
    state.system_service.get_readiness().await.map_err(|e| e.to_string())
}

/// Bootstrap a new organization with admin user.
#[tauri::command]
pub async fn bootstrap_organization(
    state: State<'_, AppState>,
    org_name: String,
    org_name_ar: Option<String>,
    org_type: String,
    timezone: String,
    locale: String,
    currency: String,
    admin_full_name: String,
    admin_email: String,
    admin_password: String,
) -> Result<orbis_app::services::BootstrapResult, String> {
    let request = orbis_app::services::BootstrapRequest {
        org_name,
        org_name_ar,
        org_type,
        timezone,
        locale,
        currency,
        admin_full_name,
        admin_email,
        admin_password,
    };

    let result = state.bootstrap_service
        .bootstrap_organization(request)
        .await
        .map_err(|e| e.to_string())?;

    // Auto-set active session after bootstrap
    state.set_active_session(crate::state::ActiveSession {
        user_id: result.admin_user_id.parse().unwrap_or_default(),
        org_id: result.org_id.parse().unwrap_or_default(),
        active_unit_id: None,
        token: result.token.clone(),
    });

    Ok(result)
}



/// Get entitlement status (licensing readiness).
#[tauri::command]
pub async fn system_get_entitlement(
    state: State<'_, AppState>,
) -> Result<orbis_domain::entities::entitlement::EntitlementResult, String> {
    state.entitlement_provider
        .check_entitlement("orbis-desktop", "default")
        .await
        .map_err(|e| e.to_string())
}

/// Complete organization onboarding atomically.
#[tauri::command]
pub async fn onboarding_complete(
    state: State<'_, AppState>,
    request: orbis_app::services::CompleteOrganizationOnboardingRequest,
) -> Result<orbis_app::services::OnboardingResult, String> {
    let target_org_id = if let Some(ref req_org_id) = request.org_id {
        req_org_id.parse::<uuid::Uuid>().map_err(|_| "Invalid organization ID in request".to_string())?
    } else {
        let session = state.get_active_session()
            .ok_or_else(|| "Not authenticated. Complete bootstrap or login first.".to_string())?;
        session.org_id
    };

    state.onboarding_service
        .complete_onboarding(target_org_id, request)
        .await
        .map_err(|e| e.to_string())
}
