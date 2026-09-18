//! Tauri IPC commands — Organization, OrgUnits, Settings, Dashboard.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_app::ports::repositories::DashboardCounts;
use uuid::Uuid;

use super::make_ctx;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> { pub data: T }
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

// ── Org ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct OrgDto {
    pub id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub slug: String,
    pub code: Option<String>,
    pub timezone: String,
    pub locale: String,
    pub date_format: Option<String>,
    pub is_hijri_enabled: bool,
    pub base_currency_code: Option<String>,
    pub primary_color: Option<String>,
    pub logo: Option<String>,
    pub favicon: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub features: Option<serde_json::Value>,
    pub current_period_id: Option<String>,
    pub is_active: bool,
    pub is_verified: bool,
}

fn org_to_dto(org: orbis_domain::entities::organization::Organization) -> OrgDto {
    OrgDto {
        id: org.id.to_string(),
        name: org.name,
        name_ar: org.name_ar,
        slug: org.slug,
        code: org.code,
        timezone: org.timezone,
        locale: org.locale,
        date_format: org.date_format,
        is_hijri_enabled: org.is_hijri_enabled,
        base_currency_code: org.base_currency_code,
        primary_color: org.primary_color,
        logo: org.logo,
        favicon: org.favicon,
        settings: org.settings,
        features: org.features,
        current_period_id: org.current_period_id,
        is_active: org.is_active,
        is_verified: org.is_verified,
    }
}

#[tauri::command]
pub async fn org_get_current(state: State<'_, AppState>) -> Result<ApiResponse<OrgDto>, String> {
    let ctx = make_ctx(&state);
    let org = if ctx.tenant_id != Uuid::nil() {
        state.organization_service.get_organization(ctx.tenant_id).await.map_err(|e| e.to_string())?
    } else {
        state.org_repo.find_first().await.map_err(|e| e.to_string())?
    };
    let org = org.ok_or_else(|| "Organization not found".to_string())?;
    Ok(ok(org_to_dto(org)))
}

#[tauri::command]
pub async fn org_update_current(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<OrgDto>, String> {
    let ctx = make_ctx(&state);
    let name = request.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let tz = request.get("timezone").and_then(|v| v.as_str()).map(|s| s.to_string());
    let locale = request.get("locale").and_then(|v| v.as_str()).map(|s| s.to_string());
    let color = request.get("primaryColor").and_then(|v| v.as_str()).map(|s| s.to_string());
    let org = state.organization_service.update_organization(&ctx, ctx.tenant_id, name, tz, locale, color).await.map_err(|e| e.to_string())?;
    Ok(ok(org_to_dto(org)))
}

// ── Settings ────────────────────────────────────────────────────

#[tauri::command]
pub async fn org_get_settings(state: State<'_, AppState>) -> Result<ApiResponse<OrganizationSettings>, String> {
    let ctx = make_ctx(&state);
    let settings = state.organization_service.get_settings(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(settings))
}

#[tauri::command]
pub async fn org_update_settings(settings: OrganizationSettings, state: State<'_, AppState>) -> Result<ApiResponse<OrganizationSettings>, String> {
    let ctx = make_ctx(&state);
    let saved = state.organization_service.update_settings(&ctx, ctx.tenant_id, settings).await.map_err(|e| e.to_string())?;
    Ok(ok(saved))
}

// ── Dashboard ───────────────────────────────────────────────────

#[tauri::command]
pub async fn org_get_dashboard(state: State<'_, AppState>) -> Result<ApiResponse<DashboardCounts>, String> {
    let ctx = make_ctx(&state);
    let counts = state.organization_service.get_dashboard_counts(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(counts))
}

// ── OrgUnits ────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct OrgUnitDto {
    pub id: String, pub org_id: String, pub parent_id: Option<String>,
    pub path: String, pub depth: i32, pub name: String,
    pub unit_type: String, pub is_active: bool,
}

fn unit_to_dto(u: orbis_domain::entities::org_unit::OrgUnit) -> OrgUnitDto {
    OrgUnitDto {
        id: u.id.to_string(), org_id: u.org_id.to_string(),
        parent_id: u.parent_id.map(|p| p.to_string()),
        path: u.path, depth: u.depth, name: u.name,
        unit_type: u.unit_type, is_active: u.is_active,
    }
}

#[tauri::command]
pub async fn org_create_unit(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<OrgUnitDto>, String> {
    let ctx = make_ctx(&state);
    let name = request["name"].as_str().unwrap_or_default().to_string();
    let unit_type = request["unitType"].as_str().unwrap_or_default().to_string();
    let parent_id = request["parentId"].as_str().and_then(|s| Uuid::parse_str(s).ok());
    let data_isolation = request["dataIsolation"].as_str().unwrap_or("none").to_string();
    let unit = state.organization_service.create_org_unit(&ctx, ctx.tenant_id, name, unit_type, parent_id, data_isolation).await.map_err(|e| e.to_string())?;
    Ok(ok(unit_to_dto(unit)))
}

#[tauri::command]
pub async fn org_list_units(state: State<'_, AppState>) -> Result<ApiResponse<Vec<OrgUnitDto>>, String> {
    let ctx = make_ctx(&state);
    let units = state.organization_service.list_org_units(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(units.into_iter().map(unit_to_dto).collect()))
}

#[tauri::command]
pub async fn org_get_unit(id: String, state: State<'_, AppState>) -> Result<ApiResponse<OrgUnitDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let u = state.organization_service.get_org_unit(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "OrgUnit not found".to_string())?;
    Ok(ok(unit_to_dto(u)))
}

#[tauri::command]
pub async fn org_delete_unit(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state.organization_service.delete_org_unit(&ctx, uid).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn org_select_unit(unit_id: Option<String>, state: State<'_, AppState>) -> Result<(), String> {
    let uid = match unit_id {
        Some(s) if !s.is_empty() => Some(Uuid::parse_str(&s).map_err(|_| "Invalid unit ID")?),
        _ => None,
    };
    state.set_active_unit(uid);
    Ok(())
}
