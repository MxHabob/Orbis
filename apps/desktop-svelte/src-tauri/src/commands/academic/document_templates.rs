use tauri::State;
use std::sync::Arc;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use docforge_orbis::template_service::{
    TemplateService, CreateTemplateRequest, UpdateTemplateRequest,
    ListTemplatesParams, TemplateRecord, TemplateVersionRecord
};

#[tauri::command]
pub async fn get_document_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    status: Option<String>,
    document_type: Option<String>,
    entity_type: Option<String>,
    category_id: Option<String>,
    search: Option<String>,
) -> CmdResult<Vec<TemplateRecord>> {
    let db = state.active_db().await;
    let params = ListTemplatesParams {
        org_id,
        status,
        document_type,
        entity_type,
        category_id,
        search,
    };

    TemplateService::list(&db, params)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    let db = state.active_db().await;
    TemplateService::get_by_id(&db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn create_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    user_id: String,
    request: CreateTemplateRequest,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::create(&db, &org_id, &user_id, request)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn update_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    user_id: String,
    request: UpdateTemplateRequest,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::update(&db, &org_id, &id, &user_id, request)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::delete(&db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn publish_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    user_id: String,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::publish(&db, &org_id, &id, &user_id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn archive_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::archive(&db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn restore_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::restore(&db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_document_template_versions(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<Vec<TemplateVersionRecord>> {
    let db = state.active_db().await;
    TemplateService::get_versions(&db, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn revert_document_template_to_version(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    version: i32,
    user_id: String,
) -> CmdResult<TemplateRecord> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    TemplateService::revert_to_version(&db, &org_id, &id, version, &user_id)
        .await
        .map_err(|e| CommandError::new(e))
}
