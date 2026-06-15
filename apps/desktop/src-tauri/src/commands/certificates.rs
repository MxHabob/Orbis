use tauri::State;
use std::sync::Arc;
use sea_orm::{entity::prelude::*, QueryFilter, ActiveModelTrait, ActiveValue::Set, EntityTrait};
use core_domain::entities::document;
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

fn map_document_to_certificate(doc: document::Model) -> serde_json::Value {
    let content: serde_json::Value = doc.meta.as_deref()
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "id": doc.id,
        "name": doc.name,
        "description": doc.description,
        "category": doc.tags.clone().unwrap_or_else(|| "general".to_string()),
        "isDefault": doc.is_pinned == 1,
        "isArchived": doc.deleted_at.is_some(),
        "thumbnailUrl": doc.thumbnail_url,
        "version": doc.version.unwrap_or(1),
        "content": content,
        "createdAt": doc.created_at,
        "updatedAt": doc.updated_at,
    })
}

#[tauri::command]
pub async fn get_certificates(
    state: State<'_, Arc<AppState>>,
) -> CmdResult<Vec<serde_json::Value>> {
    let docs = document::Entity::find()
        .filter(document::Column::IsTemplate.eq(1))
        .filter(document::Column::ItemType.eq("certificate"))
        .all(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(docs.into_iter().map(map_document_to_certificate).collect())
}

#[tauri::command]
pub async fn get_certificate(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<serde_json::Value> {
    let doc = document::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Certificate template not found"))?;

    Ok(map_document_to_certificate(doc))
}

#[tauri::command]
pub async fn create_certificate(
    state: State<'_, Arc<AppState>>,
    request: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let id = request.get("id").and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let name = request.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled Certificate").to_string();
    let description = request.get("description").and_then(|v| v.as_str()).map(String::from);
    let category = request.get("category").and_then(|v| v.as_str()).unwrap_or("general").to_string();
    let is_default = request.get("isDefault").and_then(|v| v.as_bool()).unwrap_or(false);
    let content_val = request.get("content").cloned().unwrap_or(serde_json::Value::Null);
    let content_str = serde_json::to_string(&content_val).unwrap_or_default();

    let active_model = document::ActiveModel {
        id: Set(id),
        org_id: Set("".to_string()),
        owner_id: Set("".to_string()),
        name: Set(name),
        item_type: Set("certificate".to_string()),
        visibility: Set("public".to_string()),
        storage_provider: Set("local".to_string()),
        key: Set("".to_string()),
        is_template: Set(1),
        is_pinned: Set(if is_default { 1 } else { 0 }),
        tags: Set(Some(category)),
        description: Set(description),
        meta: Set(Some(content_str)),
        created_at: Set(chrono::Utc::now().naive_utc().to_string()),
        updated_at: Set(chrono::Utc::now().naive_utc().to_string()),
        ..Default::default()
    };

    let doc = active_model
        .insert(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(map_document_to_certificate(doc))
}

#[tauri::command]
pub async fn update_certificate(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let doc = document::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Certificate template not found"))?;

    let mut active: document::ActiveModel = doc.into();

    if let Some(name) = request.get("name").and_then(|v| v.as_str()) {
        active.name = Set(name.to_string());
    }
    if let Some(description) = request.get("description") {
        active.description = Set(description.as_str().map(String::from));
    }
    if let Some(category) = request.get("category").and_then(|v| v.as_str()) {
        active.tags = Set(Some(category.to_string()));
    }
    if let Some(is_default) = request.get("isDefault").and_then(|v| v.as_bool()) {
        active.is_pinned = Set(if is_default { 1 } else { 0 });
    }
    if let Some(content_val) = request.get("content") {
        let content_str = serde_json::to_string(content_val).unwrap_or_default();
        active.meta = Set(Some(content_str));
    }
    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());

    let updated_doc = active
        .update(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(map_document_to_certificate(updated_doc))
}

#[tauri::command]
pub async fn delete_certificate(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    let doc = document::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Certificate template not found"))?;

    document::Entity::delete_by_id(doc.id)
        .exec(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(())
}
