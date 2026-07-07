use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use domain_academic::entities::document;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_documents(
    state: State<'_, Arc<AppState>>,
    params: Option<serde_json::Value>,
) -> CmdResult<Vec<document::Model>> {
    let db = state.active_db().await;
    let mut query = document::Entity::find().filter(document::Column::DeletedAt.is_null());
    if let Some(params) = params {
        if let Some(org_id) = params.get("orgId").or_else(|| params.get("org_id")).and_then(|v| v.as_str()) {
            query = query.filter(document::Column::OrgId.eq(org_id));
        }
        if let Some(owner_id) = params.get("ownerId").or_else(|| params.get("owner_id")).and_then(|v| v.as_str()) {
            query = query.filter(document::Column::OwnerId.eq(owner_id));
        }
        if let Some(is_template) = params.get("isTemplate").or_else(|| params.get("is_template")) {
            let val = match is_template {
                serde_json::Value::Bool(b) => if *b { 1 } else { 0 },
                serde_json::Value::Number(num) => num.as_i64().unwrap_or(0) as i32,
                _ => 0,
            };
            query = query.filter(document::Column::IsTemplate.eq(val));
        }
    }
    query
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_document(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<document::Model> {
    let db = state.active_db().await;
    document::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Document not found".to_string()))
}

#[tauri::command]
pub async fn create_document(
    state: State<'_, Arc<AppState>>,
    request: serde_json::Value,
) -> CmdResult<document::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    
    let id = request.get("id").and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let org_id = request.get("orgId").or_else(|| request.get("org_id")).and_then(|v| v.as_str()).map(String::from).unwrap_or_default();
    let owner_id = request.get("ownerId").or_else(|| request.get("owner_id")).and_then(|v| v.as_str()).map(String::from).unwrap_or_default();
    let name = request.get("name").and_then(|v| v.as_str()).map(String::from).unwrap_or_default();
    let item_type = request.get("type").and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| "file".to_string());
    let visibility = request.get("visibility").and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| "private".to_string());
    let storage_provider = request.get("storageProvider").or_else(|| request.get("storage_provider")).and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| "local".to_string());
    let key = request.get("key").and_then(|v| v.as_str()).map(String::from).unwrap_or_default();
    let url = request.get("url").and_then(|v| v.as_str()).map(String::from);
    let is_template = request.get("isTemplate").or_else(|| request.get("is_template")).and_then(|v| v.as_bool().map(|b| if b { 1 } else { 0 }).or_else(|| v.as_i64().map(|i| i as i32))).unwrap_or(0);

    let active_model = document::ActiveModel {
        id: Set(id),
        org_id: Set(org_id),
        owner_id: Set(owner_id),
        name: Set(name),
        item_type: Set(item_type),
        visibility: Set(visibility),
        storage_provider: Set(storage_provider),
        key: Set(key),
        url: Set(url),
        is_template: Set(is_template),
        uploaded_by: Set(request.get("uploadedBy").or_else(|| request.get("uploaded_by")).and_then(|v| v.as_str()).map(String::from)),
        title: Set(request.get("title").and_then(|v| v.as_str()).map(String::from)),
        description: Set(request.get("description").and_then(|v| v.as_str()).map(String::from)),
        size: Set(request.get("size").and_then(|v| v.as_i64().map(|i| i as i32))),
        mime_type: Set(request.get("mimeType").or_else(|| request.get("mime_type")).and_then(|v| v.as_str()).map(String::from)),
        extension: Set(request.get("extension").and_then(|v| v.as_str()).map(String::from)),
        bucket: Set(request.get("bucket").and_then(|v| v.as_str()).map(String::from)),
        thumbnail_url: Set(request.get("thumbnailUrl").or_else(|| request.get("thumbnail_url")).and_then(|v| v.as_str()).map(String::from)),
        parent_id: Set(request.get("parentId").or_else(|| request.get("parent_id")).and_then(|v| v.as_str()).map(String::from)),
        path: Set(request.get("path").and_then(|v| v.as_str()).map(String::from)),
        depth: Set(request.get("depth").and_then(|v| v.as_i64().map(|i| i as i32))),
        shared_with: Set(request.get("sharedWith").or_else(|| request.get("shared_with")).and_then(|v| v.as_str()).map(String::from)),
        share_link: Set(request.get("shareLink").or_else(|| request.get("share_link")).and_then(|v| v.as_str()).map(String::from)),
        share_link_expires_at: Set(request.get("shareLinkExpiresAt").or_else(|| request.get("share_link_expires_at")).and_then(|v| v.as_str()).map(String::from)),
        share_password: Set(request.get("sharePassword").or_else(|| request.get("share_password")).and_then(|v| v.as_str()).map(String::from)),
        version: Set(request.get("version").and_then(|v| v.as_i64().map(|i| i as i32))),
        previous_version_id: Set(request.get("previousVersionId").or_else(|| request.get("previous_version_id")).and_then(|v| v.as_str()).map(String::from)),
        is_latest_version: Set(1),
        processing_status: Set(request.get("processingStatus").or_else(|| request.get("processing_status")).and_then(|v| v.as_str()).map(String::from)),
        ocr_text: Set(request.get("ocrText").or_else(|| request.get("ocr_text")).and_then(|v| v.as_str()).map(String::from)),
        extracted_data: Set(request.get("extractedData").or_else(|| request.get("extracted_data")).and_then(|v| v.as_str()).map(String::from)),
        is_pinned: Set(request.get("isPinned").or_else(|| request.get("is_pinned")).and_then(|v| v.as_bool().map(|b| if b { 1 } else { 0 }).or_else(|| v.as_i64().map(|i| i as i32))).unwrap_or(0)),
        is_starred: Set(request.get("isStarred").or_else(|| request.get("is_starred")).and_then(|v| v.as_bool().map(|b| if b { 1 } else { 0 }).or_else(|| v.as_i64().map(|i| i as i32))).unwrap_or(0)),
        tags: Set(request.get("tags").and_then(|v| v.as_str()).map(String::from)),
        meta: Set(request.get("meta").and_then(|v| v.as_str()).map(String::from)),
        deleted_at: Set(None),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };
    active_model
        .insert(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_document(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let doc = document::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Document not found".to_string()))?;

    let mut active: document::ActiveModel = doc.into();
    active.deleted_at = Set(Some(chrono::Utc::now().to_rfc3339()));
    active
        .update(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}
