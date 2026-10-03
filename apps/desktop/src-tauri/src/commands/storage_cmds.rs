//! Tauri IPC commands — Storage, Physical File Upload, Download, and Document Archiving.

use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

use crate::state::AppState;
use super::document_cmds::{doc_dto, ok, DocumentDto};
use super::make_ctx;
use super::org_cmds::ApiResponse;
use orbis_app::services::UploadFileInput;

#[derive(Deserialize)]
pub struct StorageUploadInput {
    pub name: String,
    pub folder_id: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub org_unit_id: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub description: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    pub mime_type: String,
    pub data: Vec<u8>,
}

#[derive(Serialize)]
pub struct StorageDownloadResponse {
    pub document: DocumentDto,
    pub data: Vec<u8>,
}

#[tauri::command]
pub async fn storage_upload_file(
    request: StorageUploadInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let entity_id = request.entity_id.and_then(|e| Uuid::parse_str(&e).ok());
    let org_unit_id = request.org_unit_id.and_then(|e| Uuid::parse_str(&e).ok());
    let folder_id = request.folder_id.and_then(|f| Uuid::parse_str(&f).ok());
    let academic_year_id = request.academic_year_id.and_then(|y| Uuid::parse_str(&y).ok());
    let term_id = request.term_id.and_then(|t| Uuid::parse_str(&t).ok());

    let input = UploadFileInput {
        name: request.name,
        folder_id,
        entity_type: request.entity_type,
        entity_id,
        org_unit_id,
        category: request.category,
        asset_type: request.asset_type,
        description: request.description,
        academic_year_id,
        term_id,
        tags: request.tags,
        visibility: request.visibility,
        mime_type: request.mime_type,
        data: request.data,
    };

    let doc = state
        .file_storage_service
        .upload_file(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn storage_download_file(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<StorageDownloadResponse>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;

    let (doc, bytes) = state
        .file_storage_service
        .download_file_bytes(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(StorageDownloadResponse {
        document: doc_dto(doc),
        data: bytes,
    }))
}

#[tauri::command]
pub async fn storage_get_url(
    id: String,
    expires_in_seconds: Option<u64>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<String>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;
    let expires = expires_in_seconds.unwrap_or(3600);

    let url = state
        .file_storage_service
        .generate_access_url(&ctx, uid, expires)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(url))
}

#[tauri::command]
pub async fn storage_archive_document(
    id: String,
    reason: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;

    let doc = state
        .file_storage_service
        .archive_document(&ctx, uid, reason)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn storage_restore_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;

    let doc = state
        .file_storage_service
        .restore_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn storage_trash_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;

    state
        .file_storage_service
        .trash_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn storage_purge_document(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid Document ID")?;

    state
        .file_storage_service
        .purge_document(&ctx, uid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(()))
}

#[tauri::command]
pub async fn storage_list_archived(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);

    let docs = state
        .document_repo
        .find_archived(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(docs.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn storage_list_trash(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);

    let docs = state
        .document_repo
        .find_trash(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(docs.into_iter().map(doc_dto).collect()))
}
