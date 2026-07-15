use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_document::ops;

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
    owner_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let target = if owner_id == "all" { None } else { Some(owner_id.as_str()) };
    
    let docs = ops::document::list(&ctx, target)
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::to_value(docs).unwrap())
}

#[tauri::command]
pub async fn delete_document(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    
    ops::document::delete(&ctx, &id)
        .await
        .map_err(CommandError::from)?;
        
    Ok(serde_json::json!({ "deleted": true }))
}
