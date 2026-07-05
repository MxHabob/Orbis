use tauri::State;
use std::sync::Arc;
use crate::state::AppState;
use super::error::{CmdResult, CommandError};
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
    let params = ListTemplatesParams {
        org_id,
        status,
        document_type,
        entity_type,
        category_id,
        search,
    };

    TemplateService::list(&state.service().await.repo.db, params)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    TemplateService::get_by_id(&state.service().await.repo.db, &org_id, &id)
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
    TemplateService::create(&state.service().await.repo.db, &org_id, &user_id, request)
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
    TemplateService::update(&state.service().await.repo.db, &org_id, &id, &user_id, request)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn delete_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<()> {
    TemplateService::delete(&state.service().await.repo.db, &org_id, &id)
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
    TemplateService::publish(&state.service().await.repo.db, &org_id, &id, &user_id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn archive_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    TemplateService::archive(&state.service().await.repo.db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn restore_document_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<TemplateRecord> {
    TemplateService::restore(&state.service().await.repo.db, &org_id, &id)
        .await
        .map_err(|e| CommandError::new(e))
}

#[tauri::command]
pub async fn get_document_template_versions(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<Vec<TemplateVersionRecord>> {
    TemplateService::get_versions(&state.service().await.repo.db, &id)
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
    TemplateService::revert_to_version(&state.service().await.repo.db, &org_id, &id, version, &user_id)
        .await
        .map_err(|e| CommandError::new(e))
}
