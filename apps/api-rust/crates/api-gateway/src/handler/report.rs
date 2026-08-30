use axum::{
    extract::{State, Path, Query, Json},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::middleware::AuthenticatedUser;
use svc_report::definition::ReportDomain;
use svc_report::request::{ReportRequest, OutputFormat, ReportLocale};
use svc_report::org_context::OrgContext;

// ── List Report Definitions ─────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ListReportsQuery {
    pub domain: Option<String>,
}

#[utoipa::path(get, path = "/api/reports")]
pub async fn get_reports(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser,
    Query(query): Query<ListReportsQuery>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let report_service = state.report_service();

    let domain = query.domain.as_deref().and_then(|d| match d {
        "finance" => Some(ReportDomain::Finance),
        "academic" => Some(ReportDomain::Academic),
        "exam" => Some(ReportDomain::Exam),
        "staff" => Some(ReportDomain::Staff),
        "system" => Some(ReportDomain::System),
        _ => None,
    });

    let reports = report_service.list_reports(domain.as_ref());
    ok!(reports)
}

// ── Get Single Report Definition ────────────────────────────────

#[utoipa::path(get, path = "/api/reports/{code}")]
pub async fn get_report_definition(
    State(state): State<AppState>,
    Path(code): Path<String>,
    AuthenticatedUser(_user): AuthenticatedUser,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let report_service = state.report_service();

    let definition = report_service.get_definition(&code)
        .map_err(|e| (StatusCode::NOT_FOUND, Json(json!({"success": false, "error": e.to_string()}))))?;

    ok!(definition)
}

// ── Generate Report (Data Only) ─────────────────────────────────

#[utoipa::path(post, path = "/api/reports/generate")]
pub async fn post_generate_report(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<ReportRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let report_service = state.report_service();
    let org = resolve_org_context(&state, &user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": e}))))?;

    let dataset = report_service.generate(&body, &org).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"success": false, "error": e.to_string()}))))?;

    ok!(dataset)
}

// ── Export Report (File Download) ───────────────────────────────

#[utoipa::path(post, path = "/api/reports/export")]
pub async fn post_export_report(
    State(state): State<AppState>,
    AuthenticatedUser(user): AuthenticatedUser,
    Json(body): Json<ReportRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let report_service = state.report_service();
    let org = resolve_org_context(&state, &user.org_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"success": false, "error": e}))))?;

    let result = report_service.export(&body, &org).await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(json!({"success": false, "error": e.to_string()}))))?;

    // Return as file download
    let headers = [
        (header::CONTENT_TYPE, result.mime_type.clone()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", result.filename),
        ),
    ];

    Ok((StatusCode::OK, headers, result.data))
}

// ── Resolve Org Context ─────────────────────────────────────────

/// Build an OrgContext from the organization entity and settings.
async fn resolve_org_context(state: &AppState, org_id: &str) -> Result<OrgContext, String> {
    let db = state.db();

    // Query the organization record
    let stmt = sea_orm::Statement::from_sql_and_values(
        db.get_database_backend(),
        "SELECT * FROM organizations WHERE id = $1",
        vec![org_id.into()],
    );

    let org_row = db.query_one(stmt).await
        .map_err(|e| format!("Failed to query organization: {}", e))?
        .ok_or_else(|| format!("Organization not found: {}", org_id))?;

    use sea_orm::ConnectionTrait;

    // Parse settings JSON for reporting settings
    let settings_json: String = org_row.try_get("", "settings").unwrap_or_default();
    let org_settings: shared_kernel::settings::OrgSettings = serde_json::from_str(&settings_json)
        .unwrap_or_default();

    let reporting = &org_settings.reporting;

    Ok(OrgContext {
        id: org_id.to_string(),
        name: org_row.try_get::<String>("", "name").unwrap_or_default(),
        name_ar: org_row.try_get::<Option<String>>("", "name_ar")
            .unwrap_or_default()
            .unwrap_or_default(),
        code: org_row.try_get::<Option<String>>("", "code").unwrap_or_default(),
        logo_url: org_row.try_get::<Option<String>>("", "logo").unwrap_or_default(),
        logo_data: None,
        address: reporting.address.clone(),
        address_ar: reporting.address_ar.clone(),
        phone: reporting.phone.clone(),
        email: reporting.email.clone(),
        website: reporting.website.clone(),
        tax_id: reporting.tax_id.clone(),
        registration_number: reporting.registration_number.clone(),
        currency: org_settings.finance.default_currency.clone(),
        currency_symbol: match org_settings.finance.default_currency.as_str() {
            "SAR" => "ر.س".to_string(),
            "YER" => "ر.ي".to_string(),
            "USD" => "$".to_string(),
            "EUR" => "€".to_string(),
            _ => org_settings.finance.default_currency.clone(),
        },
        locale: org_row.try_get::<String>("", "locale").unwrap_or_else(|_| "ar".into()),
        timezone: org_row.try_get::<String>("", "timezone").unwrap_or_else(|_| "Asia/Riyadh".into()),
        hijri_enabled: org_row.try_get::<i32>("", "hijri_enabled").unwrap_or(0) == 1,
        primary_color: org_row.try_get::<Option<String>>("", "primary_color").unwrap_or_default(),
        stamp_url: None,
        signature_url: None,
        stamp_data: None,
        signature_data: None,
        branches: vec![],
        current_period_name: None,
        current_period_name_ar: None,
    })
}
