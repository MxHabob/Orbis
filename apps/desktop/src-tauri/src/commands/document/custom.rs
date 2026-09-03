use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_document::ops;

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DocumentListRequest {
    pub owner_id: Option<String>,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub category: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<u64>,
    pub filter: Option<shared_kernel::query_filter::UniversalFilter>,
}

#[tauri::command]
pub async fn upload_document(
    state: State<'_, Arc<AppState>>,
    file_name: String,
    mime_type: String,
    bytes: Vec<u8>,
    entity_type: String,
    entity_id: String,
    category: String,
    visibility: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    
    let req = ops::document::UploadRequest {
        file_name,
        mime_type,
        data: &bytes,
        entity_type,
        entity_id,
        category,
        school_year: None,
        term: None,
        tags: None,
        visibility,
    };
    
    let doc = ops::document::upload_file(&ctx, state.platform.storage.as_ref(), req)
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::to_value(doc).unwrap())
}

#[tauri::command]
pub async fn get_documents(
    state: State<'_, Arc<AppState>>,
    request: Option<DocumentListRequest>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let filters = request.unwrap_or_default();
    let target = filters.owner_id.as_deref().filter(|s| *s != "all");
    
    let cursor_request = shared_kernel::pagination::CursorRequest {
        cursor: filters.cursor,
        limit: filters.limit.unwrap_or(50),
        filter: filters.filter,
    };
    
    let docs = ops::document::list(
        &ctx,
        target,
        filters.entity_type.as_deref(),
        filters.category.as_deref(),
        filters.entity_id.as_deref(),
        cursor_request,
    )
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::to_value(docs).unwrap())
}

#[tauri::command]
pub async fn storage_delete_document(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    
    ops::document::delete(&ctx, &id)
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::json!({ "deleted": true }))
}
