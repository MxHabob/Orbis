use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::organization::CreateOrganizationUseCase;
use orbis_app::usecases::org_unit::CreateOrgUnitUseCase;
use uuid::Uuid;
use crate::state::AppState;
use orbis_infra::adapters::bootstrap_adapter::{BootstrapPayload, bootstrap_full};

use utoipa::ToSchema;

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

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub locale: String,
}

#[derive(Serialize, ToSchema)]
pub struct OrgResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateOrgUnitRequest {
    pub org_id: Uuid,
    pub name: String,
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub data_isolation: String,
}

#[derive(Serialize, ToSchema)]
pub struct OrgUnitResponse {
    pub id: Uuid,
    pub path: String,
    pub depth: i32,
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations",
    request_body = CreateOrgRequest,
    responses(
        (status = 200, description = "Organization created successfully", body = OrgResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Organization"
)]
async fn create_org(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgRequest>,
) -> Result<Json<OrgResponse>, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateOrganizationUseCase {
        org_repo: state.org_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.name, payload.slug, payload.timezone, payload.locale).await {
        Ok(org) => Ok(Json(OrgResponse {
            id: org.id,
            name: org.name,
            slug: org.slug,
        })),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/bootstrap",
    request_body(content = BootstrapPayload, description = "The initialization data for a new organization"),
    responses(
        (status = 200, description = "Organization bootstrapped successfully")
    ),
    tag = "Organization"
)]
async fn bootstrap_org(
    State(state): State<AppState>,
    Json(payload): Json<BootstrapPayload>,
) -> Result<Json<serde_json::Value>, String> {
    match bootstrap_full(&state.db, payload).await {
        Ok(res) => Ok(Json(serde_json::json!(res))),
        Err(e) => Err(e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/organizations/units",
    request_body = CreateOrgUnitRequest,
    responses(
        (status = 200, description = "Organization Unit created successfully", body = OrgUnitResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Organization"
)]
async fn create_org_unit(
    State(state): State<AppState>,
    Json(payload): Json<CreateOrgUnitRequest>,
) -> Result<Json<OrgUnitResponse>, String> {
    let ctx = RequestContext {
        tenant_id: payload.org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let use_case = CreateOrgUnitUseCase {
        org_unit_repo: state.org_unit_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match use_case.execute(&ctx, payload.org_id, payload.name, payload.unit_type, payload.parent_id, payload.data_isolation).await {
        Ok(unit) => Ok(Json(OrgUnitResponse {
            id: unit.id,
            path: unit.path,
            depth: unit.depth,
        })),
        Err(e) => Err(e.to_string()),
    }
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/organizations", post(create_org))
        .route("/api/v1/organizations/bootstrap", post(bootstrap_org))
        .route("/api/v1/organizations/units", post(create_org_unit))
}
