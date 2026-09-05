//! Tauri IPC commands — Organization, OrgUnits, Settings, Dashboard.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use orbis_app::usecases::organization::{CreateOrganizationUseCase, UpdateOrganizationUseCase};
use orbis_app::usecases::org_unit::CreateOrgUnitUseCase;
use orbis_app::usecases::settings::{GetSettingsUseCase, UpdateSettingsUseCase};
use orbis_app::usecases::dashboard::GetDashboardUseCase;
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
    pub id: String, pub name: String, pub slug: String,
    pub timezone: String, pub locale: String, pub primary_color: Option<String>,
}

#[tauri::command]
pub async fn org_get_current(state: State<'_, AppState>) -> Result<ApiResponse<OrgDto>, String> {
    let ctx = make_ctx(&state);
    let org = state.org_repo.find_by_id(ctx.tenant_id).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "Organization not found".to_string())?;
    Ok(ok(OrgDto {
        id: org.id.to_string(), name: org.name, slug: org.slug,
        timezone: org.timezone, locale: org.locale, primary_color: org.primary_color,
    }))
}

#[tauri::command]
pub async fn org_update_current(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<OrgDto>, String> {
    let ctx = make_ctx(&state);
    let uc = UpdateOrganizationUseCase { org_repo: state.org_repo.clone(), event_bus: state.event_bus.clone() };
    let name = request.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let tz = request.get("timezone").and_then(|v| v.as_str()).map(|s| s.to_string());
    let locale = request.get("locale").and_then(|v| v.as_str()).map(|s| s.to_string());
    let color = request.get("primaryColor").and_then(|v| v.as_str()).map(|s| s.to_string());
    let org = uc.execute(&ctx, ctx.tenant_id, name, tz, locale, color).await.map_err(|e| e.to_string())?;
    Ok(ok(OrgDto {
        id: org.id.to_string(), name: org.name, slug: org.slug,
        timezone: org.timezone, locale: org.locale, primary_color: org.primary_color,
    }))
}

// ── Settings ────────────────────────────────────────────────────

#[tauri::command]
pub async fn org_get_settings(state: State<'_, AppState>) -> Result<ApiResponse<OrganizationSettings>, String> {
    let ctx = make_ctx(&state);
    let uc = GetSettingsUseCase { settings_repo: state.settings_repo.clone() };
    let settings = uc.execute(&ctx).await.map_err(|e| e.to_string())?;
    Ok(ok(settings))
}

#[tauri::command]
pub async fn org_update_settings(settings: OrganizationSettings, state: State<'_, AppState>) -> Result<ApiResponse<OrganizationSettings>, String> {
    let ctx = make_ctx(&state);
    let uc = UpdateSettingsUseCase { settings_repo: state.settings_repo.clone(), permission_checker: state.permission_checker.clone() };
    let saved = uc.execute(&ctx, settings).await.map_err(|e| e.to_string())?;
    Ok(ok(saved))
}

// ── Dashboard ───────────────────────────────────────────────────

#[tauri::command]
pub async fn org_get_dashboard(state: State<'_, AppState>) -> Result<ApiResponse<DashboardCounts>, String> {
    let ctx = make_ctx(&state);
    let uc = GetDashboardUseCase { dashboard_repo: state.dashboard_repo.clone() };
    let counts = uc.execute(&ctx).await.map_err(|e| e.to_string())?;
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
    let uc = CreateOrgUnitUseCase { org_unit_repo: state.org_unit_repo.clone(), event_bus: state.event_bus.clone() };
    let name = request["name"].as_str().unwrap_or_default().to_string();
    let unit_type = request["unitType"].as_str().unwrap_or_default().to_string();
    let parent_id = request["parentId"].as_str().and_then(|s| Uuid::parse_str(s).ok());
    let data_isolation = request["dataIsolation"].as_str().unwrap_or("none").to_string();
    let unit = uc.execute(&ctx, ctx.tenant_id, name, unit_type, parent_id, data_isolation).await.map_err(|e| e.to_string())?;
    Ok(ok(unit_to_dto(unit)))
}

#[tauri::command]
pub async fn org_list_units(state: State<'_, AppState>) -> Result<ApiResponse<Vec<OrgUnitDto>>, String> {
    let ctx = make_ctx(&state);
    let units = state.org_unit_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(units.into_iter().map(unit_to_dto).collect()))
}

#[tauri::command]
pub async fn org_get_unit(id: String, state: State<'_, AppState>) -> Result<ApiResponse<OrgUnitDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let u = state.org_unit_repo.find_by_id(uid).await.map_err(|e| e.to_string())?
        .ok_or_else(|| "OrgUnit not found".to_string())?;
    Ok(ok(unit_to_dto(u)))
}

#[tauri::command]
pub async fn org_delete_unit(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state.org_unit_repo.delete(uid).await.map_err(|e| e.to_string())
}
