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
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub version: i32,
    pub created_at: String,
    pub updated_at: String,
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
        created_by: org.created_by.map(|u| u.to_string()),
        updated_by: org.updated_by.map(|u| u.to_string()),
        version: org.version,
        created_at: org.created_at.to_rfc3339(),
        updated_at: org.updated_at.to_rfc3339(),
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
    let name_ar = request.get("name_ar").or_else(|| request.get("nameAr")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let code = request.get("code").and_then(|v| v.as_str()).map(|s| s.to_string());
    let tz = request.get("timezone").and_then(|v| v.as_str()).map(|s| s.to_string());
    let locale = request.get("locale").and_then(|v| v.as_str()).map(|s| s.to_string());
    let date_format = request.get("date_format").or_else(|| request.get("dateFormat")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let is_hijri_enabled = request.get("is_hijri_enabled").or_else(|| request.get("isHijriEnabled")).or_else(|| request.get("hijri_enabled")).and_then(|v| v.as_bool());
    let base_currency_code = request.get("base_currency_code").or_else(|| request.get("baseCurrencyCode")).or_else(|| request.get("currency")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let color = request.get("primary_color").or_else(|| request.get("primaryColor")).and_then(|v| v.as_str()).map(|s| s.to_string());
    let logo = request.get("logo").and_then(|v| v.as_str()).map(|s| s.to_string());
    let favicon = request.get("favicon").and_then(|v| v.as_str()).map(|s| s.to_string());

    let req = orbis_app::services::UpdateOrgRequest {
        name,
        name_ar,
        code,
        timezone: tz,
        locale,
        date_format,
        is_hijri_enabled,
        base_currency_code,
        primary_color: color,
        logo,
        favicon,
    };
    let org = state.organization_service.update_organization(&ctx, ctx.tenant_id, req).await.map_err(|e| e.to_string())?;
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
    pub id: String,
    pub org_id: String,
    pub parent_id: Option<String>,
    pub path: String,
    pub depth: i32,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String,
    pub head_id: Option<String>,
    pub capacity: Option<i32>,
    pub building: Option<String>,
    pub floor: Option<i32>,
    pub room_number: Option<String>,
    pub data_isolation: String,
    pub children_count: i32,
    pub descendant_count: i32,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

fn unit_to_dto(u: orbis_domain::entities::org_unit::OrgUnit) -> OrgUnitDto {
    OrgUnitDto {
        id: u.id.to_string(),
        org_id: u.org_id.to_string(),
        parent_id: u.parent_id.map(|p| p.to_string()),
        path: u.path,
        depth: u.depth,
        name: u.name,
        name_ar: u.name_ar,
        code: u.code,
        unit_type: u.unit_type,
        head_id: u.head_id.map(|h| h.to_string()),
        capacity: u.capacity,
        building: u.building,
        floor: u.floor,
        room_number: u.room_number,
        data_isolation: u.data_isolation,
        children_count: u.children_count,
        descendant_count: u.descendant_count,
        sort_order: u.sort_order,
        is_active: u.is_active,
        created_by: u.created_by.map(|c| c.to_string()),
        updated_by: u.updated_by.map(|c| c.to_string()),
        created_at: u.created_at.to_rfc3339(),
        updated_at: u.updated_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn org_create_unit(request: serde_json::Value, state: State<'_, AppState>) -> Result<ApiResponse<OrgUnitDto>, String> {
    let ctx = make_ctx(&state);
    let name = request["name"].as_str().unwrap_or_default().to_string();
    let name_ar = request["nameAr"].as_str().or_else(|| request["name_ar"].as_str()).map(|s| s.to_string());
    let code = request["code"].as_str().map(|s| s.to_string());
    let unit_type = request["unitType"].as_str().unwrap_or("division").to_string();
    let parent_id = request["parentId"].as_str().and_then(|s| Uuid::parse_str(s).ok());
    let data_isolation = request["dataIsolation"].as_str().unwrap_or("none").to_string();
    let mut unit = state.organization_service.create_org_unit(&ctx, ctx.tenant_id, name, unit_type, parent_id, data_isolation).await.map_err(|e| e.to_string())?;
    
    // Set additional optional metadata if provided
    let mut needs_save = false;
    if name_ar.is_some() {
        unit.name_ar = name_ar;
        needs_save = true;
    }
    if code.is_some() {
        unit.code = code;
        needs_save = true;
    }
    if let Some(cap) = request["capacity"].as_i64() {
        unit.capacity = Some(cap as i32);
        needs_save = true;
    }
    if let Some(bld) = request["building"].as_str() {
        unit.building = Some(bld.to_string());
        needs_save = true;
    }
    if let Some(flr) = request["floor"].as_i64() {
        unit.floor = Some(flr as i32);
        needs_save = true;
    }
    if let Some(rm) = request["roomNumber"].as_str().or_else(|| request["room_number"].as_str()) {
        unit.room_number = Some(rm.to_string());
        needs_save = true;
    }
    if needs_save {
        state.org_unit_repo.save(&unit).await.map_err(|e| e.to_string())?;
    }

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
