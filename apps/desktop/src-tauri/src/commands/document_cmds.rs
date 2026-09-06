//! Tauri IPC commands — Document.

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use orbis_app::usecases::document::{
    upload_document::UploadDocumentUseCase,
    delete_document::DeleteDocumentUseCase,
};
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
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: String,
    pub path: String,
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
    pub size: Option<i32>,
    pub mime_type: Option<String>,
    pub extension: Option<String>,
    pub storage_provider: Option<String>,
    pub path: String,
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
        size: d.size,
        mime_type: d.mime_type,
        extension: d.extension,
        storage_provider: d.storage_provider,
        path: d.path,
        url: d.url,
        thumbnail_path: d.thumbnail_path,
        preview_path: d.preview_path,
        created_at: d.created_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn document_upload(request: UploadDocumentInput, state: State<'_, AppState>) -> Result<ApiResponse<DocumentDto>, String> {
    let ctx = make_ctx(&state);
    let uc = UploadDocumentUseCase {
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    let entity_id = request.entity_id.and_then(|e| Uuid::parse_str(&e).ok());
    let uploaded_by = match ctx.actor {
        orbis_domain::iam::actor::Actor::User(uid) => Some(uid),
        _ => None,
    };

    let doc = uc.execute(
        &ctx,
        ctx.tenant_id,
        uploaded_by,
        request.entity_type,
        entity_id,
        request.name,
        request.description,
        request.category.unwrap_or_else(|| "general".to_string()),
        request.asset_type.unwrap_or_else(|| "file".to_string()),
        request.school_year,
        request.term,
        request.tags,
        request.visibility.unwrap_or_else(|| "internal".to_string()),
        request.size,
        request.mime_type,
        request.extension,
        request.storage_provider.unwrap_or_else(|| "local".to_string()),
        request.path,
        request.url,
        request.thumbnail_path,
        request.preview_path,
    ).await.map_err(|e| e.to_string())?;

    Ok(ok(doc_dto(doc)))
}

#[tauri::command]
pub async fn document_list(state: State<'_, AppState>) -> Result<ApiResponse<Vec<DocumentDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.document_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(doc_dto).collect()))
}

#[tauri::command]
pub async fn document_get(id: String, state: State<'_, AppState>) -> Result<ApiResponse<DocumentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let d = state.document_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Document not found")?;
    Ok(ok(doc_dto(d)))
}

#[tauri::command]
pub async fn document_delete(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let uc = DeleteDocumentUseCase {
        document_repo: state.document_repo.clone(),
        event_bus: state.event_bus.clone(),
    };
    uc.execute(&ctx, uid).await.map_err(|e| e.to_string())
}
