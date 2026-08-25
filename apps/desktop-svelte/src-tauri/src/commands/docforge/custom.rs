use std::sync::Arc;

use tauri::State;

use crate::commands::error::{CmdResult, CommandError};
use crate::state::AppState;

#[tauri::command]
pub async fn create_template(
    state: State<'_, Arc<AppState>>,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let request = request
        .ok_or_else(|| CommandError::new("Missing template request"))
        .and_then(|value| {
            serde_json::from_value::<docforge_orbis::template_service::CreateTemplateRequest>(value)
                .map_err(|error| CommandError::new(format!("Invalid template request: {error}")))
        })?;

    let template = docforge_orbis::template_service::TemplateService::create(
        &ctx.db,
        &ctx.org_id,
        ctx.actor.as_str(),
        request,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn get_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::get_by_id(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn update_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let request = request
        .ok_or_else(|| CommandError::new("Missing template update request"))
        .and_then(|value| {
            serde_json::from_value::<docforge_orbis::template_service::UpdateTemplateRequest>(value)
                .map_err(|error| CommandError::new(format!("Invalid template update request: {error}")))
        })?;

    let template = docforge_orbis::template_service::TemplateService::update(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        ctx.actor.as_str(),
        request,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn publish_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::publish(
        &ctx.db,
        &ctx.org_id,
        &template_id,
        ctx.actor.as_str(),
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn archive_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::archive(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn restore_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let template = docforge_orbis::template_service::TemplateService::restore(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    serde_json::to_value(template)
        .map_err(|error| CommandError::new(format!("Failed to serialize template: {error}")))
}

#[tauri::command]
pub async fn delete_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;

    docforge_orbis::template_service::TemplateService::delete(
        &ctx.db,
        &ctx.org_id,
        &template_id,
    )
    .await
    .map_err(CommandError::from)?;

    Ok(serde_json::json!({ "deleted": true }))
}

#[tauri::command]
pub async fn list_templates(
    state: State<'_, Arc<AppState>>,
    request: Option<serde_json::Value>,
) -> CmdResult<serde_json::Value> {
    let ctx = state
        .build_guarded_context()
        .await
        .map_err(CommandError::from)?;
    let mut params = request
        .map(|value| {
            serde_json::from_value::<docforge_orbis::template_service::ListTemplatesParams>(value)
                .map_err(|error| CommandError::new(format!("Invalid template filters: {error}")))
        })
        .transpose()?;

    let mut params = params.unwrap_or(docforge_orbis::template_service::ListTemplatesParams {
        org_id: ctx.org_id.clone(),
        status: None,
        document_type: None,
        entity_type: None,
        category_id: None,
        search: None,
    });
    params.org_id = ctx.org_id.clone();

    let templates = docforge_orbis::template_service::TemplateService::list(&ctx.db, params)
        .await
        .map_err(CommandError::from)?;

    serde_json::to_value(templates)
        .map_err(|error| CommandError::new(format!("Failed to serialize templates: {error}")))
}
