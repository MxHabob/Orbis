//! Bootstrap Tauri IPC commands — first-time organization initialization.
//!
//! These commands mirror the HTTP bootstrap API but run locally via Tauri IPC.
//! They are used during the desktop startup flow when no organization exists.

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;

/// DTO for bootstrap request via Tauri IPC.
#[derive(Debug, Clone, Deserialize)]
pub struct BootstrapRequest {
    pub institution_type: String,
    pub organization_name: String,
    pub organization_name_ar: Option<String>,
    pub organization_slug: String,
    pub organization_code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub is_hijri_enabled: Option<bool>,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub admin_first_name: String,
    pub admin_last_name: String,
    pub admin_first_name_ar: Option<String>,
    pub admin_last_name_ar: Option<String>,
    pub admin_email: String,
    pub admin_password: String,
    pub admin_phone: Option<String>,
}

/// DTO for bootstrap result via Tauri IPC.
#[derive(Debug, Clone, Serialize)]
pub struct BootstrapResult {
    pub success: bool,
    pub organization_id: String,
    pub organization_name: String,
    pub organization_slug: String,
    pub institution_type: String,
    pub admin_user_id: String,
    pub admin_email: String,
    pub setup_required: bool,
    pub warnings: Vec<String>,
}

/// DTO for bootstrap status via Tauri IPC.
#[derive(Debug, Clone, Serialize)]
pub struct BootstrapStatusResult {
    pub status: String,
    pub organization_id: Option<String>,
    pub organization_name: Option<String>,
    pub institution_type: Option<String>,
    pub has_admin: bool,
}

/// Get bootstrap status — check if the system needs initialization.
#[tauri::command]
pub async fn bootstrap_get_status(
    state: State<'_, AppState>,
) -> Result<BootstrapStatusResult, String> {
    let status = state.bootstrap_service.get_status().await
        .map_err(|e| e.to_string())?;

    Ok(BootstrapStatusResult {
        status: match status.status {
            orbis_app::services::BootstrapStatus::Required => "required".to_string(),
            orbis_app::services::BootstrapStatus::InProgress => "in_progress".to_string(),
            orbis_app::services::BootstrapStatus::Completed => "completed".to_string(),
        },
        organization_id: status.organization_id.map(|id| id.to_string()),
        organization_name: status.organization_name,
        institution_type: status.institution_type,
        has_admin: status.has_admin,
    })
}

/// Execute bootstrap — create the first organization and administrator.
#[tauri::command]
pub async fn bootstrap_execute(
    state: State<'_, AppState>,
    request: BootstrapRequest,
) -> Result<BootstrapResult, String> {
    let service_req = orbis_app::services::BootstrapRequest {
        institution_type: request.institution_type,
        organization_name: request.organization_name,
        organization_name_ar: request.organization_name_ar,
        organization_slug: request.organization_slug,
        organization_code: request.organization_code,
        timezone: request.timezone,
        locale: request.locale,
        date_format: request.date_format,
        is_hijri_enabled: request.is_hijri_enabled,
        base_currency_code: request.base_currency_code,
        primary_color: request.primary_color,
        admin_first_name: request.admin_first_name,
        admin_last_name: request.admin_last_name,
        admin_first_name_ar: request.admin_first_name_ar,
        admin_last_name_ar: request.admin_last_name_ar,
        admin_email: request.admin_email,
        admin_password: request.admin_password,
        admin_phone: request.admin_phone,
    };

    let result = state.bootstrap_service.execute(service_req).await
        .map_err(|e| e.to_string())?;

    Ok(BootstrapResult {
        success: result.success,
        organization_id: result.organization_id.to_string(),
        organization_name: result.organization_name,
        organization_slug: result.organization_slug,
        institution_type: result.institution_type,
        admin_user_id: result.admin_user_id.to_string(),
        admin_email: result.admin_email,
        setup_required: result.setup_required,
        warnings: result.warnings,
    })
}
