//! Bootstrap routes — First-time organization initialization API.
//!
//! These endpoints are intentionally unauthenticated because they are used
//! BEFORE any user exists in the system. The bootstrap service enforces
//! idempotency — once an organization + admin exist, the endpoint returns
//! 409 Conflict (AlreadyInitialized).

use axum::{
    Json, Router,
    extract::State,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use crate::state::AppState;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct BootstrapRequest {
    /// Institution type: "school", "university", "institute"
    #[schema(example = "school")]
    pub institution_type: String,
    /// Organization display name
    #[schema(example = "Orbis International School")]
    pub organization_name: String,
    /// Organization name in Arabic (optional)
    pub organization_name_ar: Option<String>,
    /// URL-safe slug (optional — auto-generated from name if omitted)
    #[schema(example = "orbis-school")]
    pub organization_slug: Option<String>,
    /// Short organization code
    pub organization_code: Option<String>,
    /// Educational curriculum track
    pub educational_track: Option<String>,
    /// Organization estimated size
    pub organization_size: Option<String>,
    /// ISO Country Code (e.g. SA, YE, AE)
    pub country_code: Option<String>,
    /// City / Province
    pub city: Option<String>,
    /// Contact phone
    pub phone: Option<String>,
    /// Official website URL
    pub website: Option<String>,
    /// Tax registration / VAT number
    pub tax_id: Option<String>,
    /// Ministry / Educational license number
    pub license_number: Option<String>,
    /// Mulverse Autonomous Node UUID
    pub mulverse_node_id: Option<String>,
    /// IANA timezone
    #[schema(example = "Asia/Riyadh")]
    pub timezone: String,
    /// BCP-47 locale
    #[schema(example = "ar-SA")]
    pub locale: String,
    /// Date format pattern
    pub date_format: Option<String>,
    /// Enable Hijri calendar support
    pub is_hijri_enabled: Option<bool>,
    /// Base currency ISO code
    pub base_currency_code: Option<String>,
    /// Theme primary color hex
    pub primary_color: Option<String>,
    /// Administrator first name
    #[schema(example = "Admin")]
    pub admin_first_name: String,
    /// Administrator last name
    #[schema(example = "User")]
    pub admin_last_name: String,
    /// Administrator first name in Arabic
    pub admin_first_name_ar: Option<String>,
    /// Administrator last name in Arabic
    pub admin_last_name_ar: Option<String>,
    /// Administrator email
    #[schema(example = "admin@orbis.school")]
    pub admin_email: String,
    /// Administrator password (min 8 characters)
    pub admin_password: String,
    /// Administrator phone number
    pub admin_phone: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapResultDto {
    pub success: bool,
    pub organization_id: Uuid,
    pub organization_name: String,
    pub organization_slug: String,
    pub institution_type: String,
    pub admin_user_id: Uuid,
    pub admin_email: String,
    pub setup_required: bool,
    pub warnings: Vec<String>,
}

#[derive(Serialize, ToSchema)]
pub struct BootstrapStatusDto {
    pub status: String,
    pub organization_id: Option<Uuid>,
    pub organization_name: Option<String>,
    pub institution_type: Option<String>,
    pub has_admin: bool,
}

// ── Handlers ────────────────────────────────────────────────────

/// Get bootstrap status — is the system initialized?
#[utoipa::path(
    get,
    path = "/api/v1/bootstrap/status",
    responses(
        (status = 200, description = "Bootstrap status", body = BootstrapStatusDto)
    ),
    tag = "Bootstrap"
)]
pub async fn get_bootstrap_status(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<BootstrapStatusDto>>, ApiError> {
    let status = state
        .bootstrap_service
        .get_status()
        .await
        .map_err(ApiError::from)?;

    Ok(ApiResponse::ok(BootstrapStatusDto {
        status: match status.status {
            orbis_app::services::BootstrapStatus::Required => "required".to_string(),
            orbis_app::services::BootstrapStatus::InProgress => "in_progress".to_string(),
            orbis_app::services::BootstrapStatus::Completed => "completed".to_string(),
        },
        organization_id: status.organization_id,
        organization_name: status.organization_name,
        institution_type: status.institution_type,
        has_admin: status.has_admin,
    }))
}

/// Execute bootstrap — create the first organization and administrator.
///
/// This endpoint is unauthenticated (no user exists yet).
/// It is idempotent: calling it after bootstrap is complete returns 409.
#[utoipa::path(
    post,
    path = "/api/v1/bootstrap",
    request_body = BootstrapRequest,
    responses(
        (status = 200, description = "Bootstrap successful", body = BootstrapResultDto),
        (status = 409, description = "System already initialized"),
        (status = 400, description = "Validation error"),
    ),
    tag = "Bootstrap"
)]
pub async fn execute_bootstrap(
    State(state): State<AppState>,
    Json(req): Json<BootstrapRequest>,
) -> Result<Json<ApiResponse<BootstrapResultDto>>, ApiError> {
    let service_req = orbis_app::services::BootstrapRequest {
        institution_type: req.institution_type,
        organization_name: req.organization_name,
        organization_name_ar: req.organization_name_ar,
        organization_slug: req.organization_slug,
        organization_code: req.organization_code,
        educational_track: req.educational_track,
        organization_size: req.organization_size,
        country_code: req.country_code,
        city: req.city,
        phone: req.phone,
        website: req.website,
        tax_id: req.tax_id,
        license_number: req.license_number,
        mulverse_node_id: req.mulverse_node_id,
        timezone: req.timezone,
        locale: req.locale,
        date_format: req.date_format,
        is_hijri_enabled: req.is_hijri_enabled,
        base_currency_code: req.base_currency_code,
        primary_color: req.primary_color,
        admin_first_name: req.admin_first_name,
        admin_last_name: req.admin_last_name,
        admin_first_name_ar: req.admin_first_name_ar,
        admin_last_name_ar: req.admin_last_name_ar,
        admin_email: req.admin_email,
        admin_password: req.admin_password,
        admin_phone: req.admin_phone,
    };

    let result = state
        .bootstrap_service
        .execute(service_req)
        .await
        .map_err(ApiError::from)?;

    Ok(ApiResponse::ok(BootstrapResultDto {
        success: result.success,
        organization_id: result.organization_id,
        organization_name: result.organization_name,
        organization_slug: result.organization_slug,
        institution_type: result.institution_type,
        admin_user_id: result.admin_user_id,
        admin_email: result.admin_email,
        setup_required: result.setup_required,
        warnings: result.warnings,
    }))
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/bootstrap/status", get(get_bootstrap_status))
        .route("/api/v1/bootstrap", post(execute_bootstrap))
}
