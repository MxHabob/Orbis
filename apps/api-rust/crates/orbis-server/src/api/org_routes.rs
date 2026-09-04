use axum::{extract::State, Json, routing::post, Router};
use serde::{Deserialize, Serialize};
use orbis_app::context::RequestContext;
use orbis_app::usecases::organization::{CreateOrganizationUseCase, UpdateOrganizationUseCase};
use orbis_app::usecases::org_unit::CreateOrgUnitUseCase;
use uuid::Uuid;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CreateOrgRequest {
    pub name: String,
    pub slug: String,
    pub timezone: String,
    pub locale: String,
}

#[derive(Serialize)]
pub struct OrgResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct CreateOrgUnitRequest {
    pub org_id: Uuid,
    pub name: String,
    pub unit_type: String,
    pub parent_id: Option<Uuid>,
    pub data_isolation: String,
}

#[derive(Serialize)]
pub struct OrgUnitResponse {
    pub id: Uuid,
    pub path: String,
    pub depth: i32,
}

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
        .route("/organizations", post(create_org))
        .route("/organizations/units", post(create_org_unit))
}
