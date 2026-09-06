//! Tauri IPC commands — Document.

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct DocumentDto {
    pub id: String, pub org_id: String,
    pub entity_type: Option<String>, pub entity_id: Option<String>,
    pub file_name: String, pub mime_type: String, pub size_bytes: i64,
    pub created_at: String,
}

fn doc_dto(d: orbis_domain::entities::document::Document) -> DocumentDto {
    DocumentDto {
        id: d.id.to_string(), org_id: d.org_id.to_string(),
        entity_type: d.entity_type, entity_id: d.entity_id.map(|e| e.to_string()),
        file_name: d.name, mime_type: d.mime_type.unwrap_or_else(|| "application/octet-stream".to_string()),
        size_bytes: d.size.unwrap_or(0) as i64,
        created_at: d.created_at.to_rfc3339(),
    }
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
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state.document_repo.delete(uid).await.map_err(|e| e.to_string())
}
