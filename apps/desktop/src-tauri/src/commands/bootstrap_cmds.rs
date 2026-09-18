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

/// Complete initial academic setup (year, periods, classes, subjects).
#[tauri::command]
pub async fn bootstrap_initial_setup(
    state: State<'_, AppState>,
    org_id: String,
    academic_year_name: String,
    academic_year_name_ar: Option<String>,
    academic_year_start: String,
    academic_year_end: String,
    periods: Vec<serde_json::Value>,
    classes: Vec<serde_json::Value>,
    subjects: Vec<serde_json::Value>,
) -> Result<orbis_app::services::OrganizationReadiness, String> {
    let org_uuid: uuid::Uuid = org_id.parse()
        .map_err(|_| "Invalid organization ID".to_string())?;

    let request = orbis_app::services::InitialSetupRequest {
        academic_year: orbis_app::services::bootstrap_service::AcademicYearInput {
            name: academic_year_name,
            name_ar: academic_year_name_ar,
            start_date: academic_year_start,
            end_date: academic_year_end,
        },
        periods: periods.into_iter().map(|p| {
            orbis_app::services::bootstrap_service::PeriodInput {
                name: p.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                name_ar: p.get("name_ar").and_then(|v| v.as_str()).map(|s| s.to_string()),
                period_type: p.get("period_type").and_then(|v| v.as_str()).unwrap_or("semester").to_string(),
                start_date: p.get("start_date").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                end_date: p.get("end_date").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            }
        }).collect(),
        classes: classes.into_iter().map(|c| {
            orbis_app::services::bootstrap_service::ClassInput {
                name: c.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                name_ar: c.get("name_ar").and_then(|v| v.as_str()).map(|s| s.to_string()),
                level: c.get("level").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                division: c.get("division").and_then(|v| v.as_str()).map(|s| s.to_string()),
            }
        }).collect(),
        subjects: subjects.into_iter().map(|s| {
            orbis_app::services::bootstrap_service::SubjectInput {
                code: s.get("code").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                name: s.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                name_ar: s.get("name_ar").and_then(|v| v.as_str()).map(|s| s.to_string()),
                is_core: s.get("is_core").and_then(|v| v.as_bool()).unwrap_or(true),
            }
        }).collect(),
    };

    state.bootstrap_service
        .complete_initial_setup(org_uuid, request)
        .await
        .map_err(|e| e.to_string())
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
