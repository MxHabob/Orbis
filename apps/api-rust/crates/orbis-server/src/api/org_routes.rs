//! Organization routes — Org CRUD, OrgUnits, Settings, Dashboard, Bootstrap.

use axum::{extract::{State, Path}, routing::{get, post, put, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_app::usecases::organization::{CreateOrganizationUseCase, UpdateOrganizationUseCase};
use orbis_app::usecases::org_unit::CreateOrgUnitUseCase;
use orbis_app::usecases::settings::{GetSettingsUseCase, UpdateSettingsUseCase};
use orbis_app::usecases::dashboard::GetDashboardUseCase;
use orbis_app::ports::repositories::DashboardCounts;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_infra::adapters::bootstrap_adapter::{BootstrapPayload, bootstrap_full};

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgRequest {
    #[schema(example = "Orbis High School")]
    pub name: String,
    #[schema(example = "orbis-high")]
    pub slug: String,
    #[schema(example = "Asia/Riyadh")]
    pub timezone: String,
    #[schema(example = "ar-SA")]
    pub locale: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateOrgRequest {
    pub name: Option<String>,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    pub primary_color: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct OrgDto {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub locale: String,
    pub primary_color: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgUnitRequest {
    pub name: String,
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub data_isolation: String,
}

#[derive(Serialize, ToSchema)]
pub struct OrgUnitDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub depth: i32,
    pub name: String,
    pub unit_type: String,
    pub is_active: bool,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/organizations", request_body = CreateOrgRequest,
    responses((status = 201, description = "Organization created", body = OrgDto)),
    tag = "Organization")]
async fn create_org(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let ctx = orbis_app::context::RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateOrganizationUseCase {
        org_repo: state.org_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    let org = use_case.execute(&ctx, payload.name, payload.slug, payload.timezone, payload.locale).await?;

    Ok(ApiResponse::ok(OrgDto {
        id: org.id,
        name: org.name,
        slug: org.slug,
        timezone: org.timezone,
        locale: org.locale,
        primary_color: org.primary_color,
    }))
}

#[utoipa::path(get, path = "/api/v1/organizations/current",
    responses((status = 200, description = "Current organization", body = OrgDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_current_org(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let org = state.org_repo.find_by_id(ctx.tenant_id).await?
        .ok_or_else(|| ApiError::not_found("Organization not found"))?;

    Ok(ApiResponse::ok(OrgDto {
        id: org.id,
        name: org.name,
        slug: org.slug,
        timezone: org.timezone,
        locale: org.locale,
        primary_color: org.primary_color,
    }))
}

#[utoipa::path(put, path = "/api/v1/organizations/current", request_body = UpdateOrgRequest,
    responses((status = 200, description = "Organization updated", body = OrgDto)),
    security(("bearer" = [])), tag = "Organization")]
async fn update_current_org(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<UpdateOrgRequest>,
) -> Result<Json<ApiResponse<OrgDto>>, ApiError> {
    let use_case = UpdateOrganizationUseCase {
        org_repo: state.org_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    let org = use_case.execute(&ctx, ctx.tenant_id, payload.name, payload.timezone, payload.locale, payload.primary_color).await?;

    Ok(ApiResponse::ok(OrgDto {
        id: org.id,
        name: org.name,
        slug: org.slug,
        timezone: org.timezone,
        locale: org.locale,
        primary_color: org.primary_color,
    }))
}

#[utoipa::path(post, path = "/api/v1/organizations/bootstrap",
    request_body = serde_json::Value,
    responses((status = 200, description = "Organization bootstrapped")),
    tag = "Organization")]
async fn bootstrap_org(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapPayload>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    let res = bootstrap_full(&state.db, payload).await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    Ok(ApiResponse::ok(serde_json::json!(res)))
}

// ── Settings ────────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/v1/organizations/current/settings",
    responses((status = 200, description = "Organization settings")),
    security(("bearer" = [])), tag = "Organization")]
async fn get_settings(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<OrganizationSettings>>, ApiError> {
    let uc = GetSettingsUseCase { settings_repo: state.settings_repo.clone() };
    let settings = uc.execute(&ctx).await?;
    Ok(ApiResponse::ok(settings))
}

#[utoipa::path(put, path = "/api/v1/organizations/current/settings",
    request_body = OrganizationSettings,
    responses((status = 200, description = "Settings updated")),
    security(("bearer" = [])), tag = "Organization")]
async fn update_settings(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<OrganizationSettings>,
) -> Result<Json<ApiResponse<OrganizationSettings>>, ApiError> {
    let uc = UpdateSettingsUseCase {
        settings_repo: state.settings_repo.clone(),
        permission_checker: state.permission_checker.clone(),
    };
    let settings = uc.execute(&ctx, payload).await?;
    Ok(ApiResponse::ok(settings))
}

// ── Dashboard ───────────────────────────────────────────────────

#[utoipa::path(get, path = "/api/v1/organizations/current/dashboard",
    responses((status = 200, description = "Dashboard statistics", body = DashboardCounts)),
    security(("bearer" = [])), tag = "Organization")]
async fn get_dashboard(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<DashboardCounts>>, ApiError> {
    let uc = GetDashboardUseCase { dashboard_repo: state.dashboard_repo.clone() };
    let counts = uc.execute(&ctx).await?;
    Ok(ApiResponse::ok(counts))
}

// ── OrgUnits ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/org-units", request_body = CreateOrgUnitRequest,
    responses((status = 201, description = "OrgUnit created", body = OrgUnitDto)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn create_org_unit(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgUnitRequest>,
) -> Result<Json<ApiResponse<OrgUnitDto>>, ApiError> {
    let use_case = CreateOrgUnitUseCase {
        org_unit_repo: state.org_unit_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    let unit = use_case.execute(&ctx, ctx.tenant_id, payload.name, payload.unit_type, payload.parent_id, payload.data_isolation).await?;

    Ok(ApiResponse::ok(OrgUnitDto {
        id: unit.id,
        org_id: unit.org_id,
        parent_id: unit.parent_id,
        path: unit.path,
        depth: unit.depth,
        name: unit.name,
        unit_type: unit.unit_type,
        is_active: unit.is_active,
    }))
}

#[utoipa::path(get, path = "/api/v1/org-units",
    responses((status = 200, description = "List of org units", body = Vec<OrgUnitDto>)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn list_org_units(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<OrgUnitDto>>>, ApiError> {
    let units = state.org_unit_repo.find_by_org(ctx.tenant_id).await?;
    let dtos: Vec<OrgUnitDto> = units.into_iter().map(|u| OrgUnitDto {
        id: u.id,
        org_id: u.org_id,
        parent_id: u.parent_id,
        path: u.path,
        depth: u.depth,
        name: u.name,
        unit_type: u.unit_type,
        is_active: u.is_active,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/org-units/{id}",
    params(("id" = Uuid, Path, description = "OrgUnit ID")),
    responses((status = 200, description = "OrgUnit details", body = OrgUnitDto)),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn get_org_unit(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OrgUnitDto>>, ApiError> {
    let u = state.org_unit_repo.find_by_id(id).await?
        .ok_or_else(|| ApiError::not_found(format!("OrgUnit {} not found", id)))?;

    Ok(ApiResponse::ok(OrgUnitDto {
        id: u.id, org_id: u.org_id, parent_id: u.parent_id,
        path: u.path, depth: u.depth, name: u.name,
        unit_type: u.unit_type, is_active: u.is_active,
    }))
}

#[utoipa::path(delete, path = "/api/v1/org-units/{id}",
    params(("id" = Uuid, Path, description = "OrgUnit ID")),
    responses((status = 204, description = "OrgUnit deleted")),
    security(("bearer" = [])), tag = "OrgUnits")]
async fn delete_org_unit(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<axum::http::StatusCode, ApiError> {
    state.org_unit_repo.delete(id).await?;
    Ok(axum::http::StatusCode::NO_CONTENT)
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/organizations", post(create_org))
        .route("/api/v1/organizations/bootstrap", post(bootstrap_org))
        .route("/api/v1/organizations/current", get(get_current_org).put(update_current_org))
        .route("/api/v1/organizations/current/settings", get(get_settings).put(update_settings))
        .route("/api/v1/organizations/current/dashboard", get(get_dashboard))
        .route("/api/v1/org-units", post(create_org_unit).get(list_org_units))
        .route("/api/v1/org-units/{id}", get(get_org_unit).delete(delete_org_unit))
}
