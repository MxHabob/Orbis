//! Tauri IPC commands — Document.

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use orbis_app::services::CreateDocumentInput;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct DocumentDto {
    pub id: String,
    pub org_id: String,
    pub uploaded_by: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub asset_type: String,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: String,
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct UploadDocumentInput {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub asset_type: Option<String>,
    pub school_year: Option<String>,
    pub term: Option<String>,
    pub tags: Option<String>,
    pub visibility: Option<String>,
    #[serde(alias = "size")]
    pub file_size_bytes: Option<i64>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: Option<String>,
    #[serde(alias = "path")]
    pub file_path: String,
    pub url: Option<String>,
    pub thumbnail_path: Option<String>,
    pub preview_path: Option<String>,
}

fn doc_dto(d: orbis_domain::entities::document::Document) -> DocumentDto {
    DocumentDto {
        id: d.id.to_string(),
        org_id: d.org_id.to_string(),
        uploaded_by: d.uploaded_by.map(|u| u.to_string()),
        entity_type: d.entity_type,
        entity_id: d.entity_id.map(|e| e.to_string()),
        name: d.name,
        description: d.description,
        category: d.category,
        asset_type: d.asset_type,
        school_year: d.school_year,
        term: d.term,
        tags: d.tags,
        visibility: d.visibility,
        file_size_bytes: d.file_size_bytes,
        mime_type: d.mime_type,
        extension: d.extension,
        storage_provider: d.storage_provider,
        file_path: d.file_path,
        url: d.url,
        thumbnail_path: d.thumbnail_path,
        preview_path: d.preview_path,
        created_at: d.created_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn document_upload(request: UploadDocumentInput, state: State<'_, AppState>) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let entity_id = request.entity_id.and_then(|e| Uuid::parse_str(&e).ok());
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };

    let input = CreateDocumentInput {
        uploaded_by,
        entity_type: request.entity_type,
        entity_id,
        name: request.name,
        description: request.description,
        category: request.category.unwrap_or_else(|| "general".to_string()),
        asset_type: request.asset_type.unwrap_or_else(|| "file".to_string()),
        school_year: request.school_year,
        term: request.term,
        tags: request.tags,
        visibility: request.visibility.unwrap_or_else(|| "internal".to_string()),
        file_size_bytes: request.file_size_bytes,
        mime_type: request.mime_type,
        extension: request.extension,
        storage_provider: request.storage_provider.unwrap_or_else(|| "local".to_string()),
        file_path: request.file_path,
        url: request.url,
        thumbnail_path: request.thumbnail_path,
        preview_path: request.preview_path,
    };

    let doc = state.document_service.create_document(&ctx, ctx.tenant_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_list(state: State<'_, AppState>) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.document_service.list_documents_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn document_get(id: String, state: State<'_, AppState>) -> Result<ApiResponse<DocumentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let d = state.document_service.get_document(uid).await.map_err(|e| e.to_string())?.ok_or("Document not found")?;
    Ok(ok(doc_dto(d)))
}

#[tauri::command]
pub async fn document_delete(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state.document_service.delete_document(&ctx, uid).await.map_err(|e| e.to_string())
}
