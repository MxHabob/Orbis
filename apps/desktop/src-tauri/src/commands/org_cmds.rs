use tauri::State;
use crate::state::AppState;
use orbis_app::usecases::organization::CreateOrganizationUseCase;
use orbis_app::usecases::org_unit::CreateOrgUnitUseCase;
use orbis_app::context::RequestContext;
use uuid::Uuid;

#[tauri::command]
pub async fn create_organization(
    name: String,
    slug: String,
    timezone: String,
    locale: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let ctx = RequestContext {
        tenant_id: Uuid::new_v4(),
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateOrganizationUseCase {
        org_repo: state.org_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, name, slug, timezone, locale).await {
        Ok(org) => Ok(org.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn create_org_unit(
    org_id: String,
    name: String,
    unit_type: String,
    parent_id: Option<String>,
    data_isolation: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let parsed_org_id = match Uuid::parse_str(&org_id) {
        Ok(uuid) => uuid,
        Err(_) => return Err("Invalid org ID format".into()),
    };

    let parsed_parent_id = match parent_id {
        Some(id_str) => match Uuid::parse_str(&id_str) {
            Ok(uuid) => Some(uuid),
            Err(_) => return Err("Invalid parent ID format".into()),
        },
        None => None,
    };

    let ctx = RequestContext {
        tenant_id: parsed_org_id,
        actor: orbis_domain::iam::actor::Actor::System,
        request_id: Uuid::new_v4().to_string(),
    };

    let usecase = CreateOrgUnitUseCase {
        org_unit_repo: state.org_unit_repo.clone(),
        event_bus: state.event_bus.clone(),
    };

    match usecase.execute(&ctx, parsed_org_id, name, unit_type, parsed_parent_id, data_isolation).await {
        Ok(unit) => Ok(unit.id.to_string()),
        Err(e) => Err(e.to_string()),
    }
}
