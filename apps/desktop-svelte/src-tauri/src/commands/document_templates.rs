use tauri::State;
use std::sync::Arc;
use sea_orm::{entity::prelude::*, QueryFilter, ActiveModelTrait, ActiveValue::Set, EntityTrait};
use core_domain::entities::document_template;
use crate::state::AppState;
use super::error::{CmdResult, CommandError};

fn map_model_to_json(doc: document_template::Model) -> serde_json::Value {
    let content: serde_json::Value = serde_json::from_str(&doc.content)
        .unwrap_or(serde_json::Value::Null);
    let margins: serde_json::Value = serde_json::from_str(&doc.margins)
        .unwrap_or(serde_json::Value::Null);

    serde_json::json!({
        "id": doc.id,
        "orgId": doc.org_id,
        "name": doc.name,
        "entityType": doc.entity_type,
        "layoutType": doc.layout_type,
        "content": content,
        "pageSize": doc.page_size,
        "orientation": doc.orientation,
        "margins": margins,
        "isDefault": doc.is_default == 1,
        "createdAt": doc.created_at,
        "updatedAt": doc.updated_at,
    })
}

#[tauri::command]
pub async fn get_document_templates(
    state: State<'_, Arc<AppState>>,
    entity_type: Option<String>,
) -> CmdResult<Vec<serde_json::Value>> {
    let mut query = document_template::Entity::find();
    if let Some(etype) = entity_type {
        query = query.filter(document_template::Column::EntityType.eq(etype));
    }
    
    let docs = query.all(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(docs.into_iter().map(map_model_to_json).collect())
}

#[tauri::command]
pub async fn get_document_template(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<serde_json::Value> {
    let doc = document_template::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Document template not found"))?;

    Ok(map_model_to_json(doc))
}

#[tauri::command]
pub async fn create_document_template(
    state: State<'_, Arc<AppState>>,
    request: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let id = request.get("id").and_then(|v| v.as_str()).map(String::from).unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let name = request.get("name").and_then(|v| v.as_str()).unwrap_or("Untitled Template").to_string();
    let entity_type = request.get("entityType").and_then(|v| v.as_str()).unwrap_or("general").to_string();
    let layout_type = request.get("layoutType").and_then(|v| v.as_str()).unwrap_or("html_css").to_string();
    let is_default = request.get("isDefault").and_then(|v| v.as_bool()).unwrap_or(false);
    
    let content_val = request.get("content").cloned().unwrap_or(serde_json::Value::Null);
    let content_str = serde_json::to_string(&content_val).unwrap_or_default();
    
    let page_size = request.get("pageSize").and_then(|v| v.as_str()).unwrap_or("A4").to_string();
    let orientation = request.get("orientation").and_then(|v| v.as_str()).unwrap_or("portrait").to_string();
    
    let margins_val = request.get("margins").cloned().unwrap_or(serde_json::json!({"top": 10, "bottom": 10, "left": 10, "right": 10}));
    let margins_str = serde_json::to_string(&margins_val).unwrap_or_default();

    let active_model = document_template::ActiveModel {
        id: Set(id),
        org_id: Set("".to_string()),
        name: Set(name),
        entity_type: Set(entity_type),
        layout_type: Set(layout_type),
        content: Set(content_str),
        page_size: Set(page_size),
        orientation: Set(orientation),
        margins: Set(margins_str),
        is_default: Set(if is_default { 1 } else { 0 }),
        created_at: Set(chrono::Utc::now().naive_utc().to_string()),
        updated_at: Set(chrono::Utc::now().naive_utc().to_string()),
    };

    let doc = active_model
        .insert(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(map_model_to_json(doc))
}

#[tauri::command]
pub async fn update_document_template(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let doc = document_template::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Document template not found"))?;

    let mut active: document_template::ActiveModel = doc.into();

    if let Some(name) = request.get("name").and_then(|v| v.as_str()) {
        active.name = Set(name.to_string());
    }
    if let Some(entity_type) = request.get("entityType").and_then(|v| v.as_str()) {
        active.entity_type = Set(entity_type.to_string());
    }
    if let Some(layout_type) = request.get("layoutType").and_then(|v| v.as_str()) {
        active.layout_type = Set(layout_type.to_string());
    }
    if let Some(is_default) = request.get("isDefault").and_then(|v| v.as_bool()) {
        active.is_default = Set(if is_default { 1 } else { 0 });
    }
    if let Some(content_val) = request.get("content") {
        let content_str = serde_json::to_string(content_val).unwrap_or_default();
        active.content = Set(content_str);
    }
    if let Some(page_size) = request.get("pageSize").and_then(|v| v.as_str()) {
        active.page_size = Set(page_size.to_string());
    }
    if let Some(orientation) = request.get("orientation").and_then(|v| v.as_str()) {
        active.orientation = Set(orientation.to_string());
    }
    if let Some(margins_val) = request.get("margins") {
        let margins_str = serde_json::to_string(margins_val).unwrap_or_default();
        active.margins = Set(margins_str);
    }
    active.updated_at = Set(chrono::Utc::now().naive_utc().to_string());

    let updated_doc = active
        .update(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(map_model_to_json(updated_doc))
}

#[tauri::command]
pub async fn delete_document_template(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    let doc = document_template::Entity::find_by_id(id)
        .one(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Document template not found"))?;

    document_template::Entity::delete_by_id(doc.id)
        .exec(&state.service.repo.db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;

    Ok(())
}
