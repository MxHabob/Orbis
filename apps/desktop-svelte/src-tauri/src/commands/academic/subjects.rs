use tauri::State;
use std::sync::Arc;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, ActiveValue::Set};
use domain_academic::entities::subject;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_subjects(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<subject::Model>> {
    let db = state.active_db().await;
    subject::Entity::find()
        .filter(subject::Column::OrgId.eq(org_id))
        .filter(subject::Column::IsActive.eq(1))
        .all(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<subject::Model> {
    let db = state.active_db().await;
    subject::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Subject not found".to_string()))
}

#[derive(serde::Deserialize)]
pub struct CreateSubjectRequest {
    pub name: String,
    pub code: Option<String>,
    pub color: Option<String>,
    pub sort_order: Option<i32>,
}

#[tauri::command]
pub async fn create_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateSubjectRequest,
) -> CmdResult<subject::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let now = chrono::Utc::now().to_rfc3339();
    
    let new_subject = subject::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        org_id: Set(org_id),
        unit_id: Set(None),
        code: Set(request.code.unwrap_or_default()),
        name: Set(request.name),
        name_ar: Set(None),
        name_en: Set(None),
        description: Set(None),
        short_description: Set(None),
        is_core: Set(1),
        is_elective: Set(0),
        credit_hours: Set(None),
        requires_lab: Set(0),
        requires_practical: Set(0),
        max_students_per_class: Set(None),
        default_grade_groups: Set(None),
        prerequisites: Set(None),
        learning_outcomes: Set(None),
        color: Set(request.color.or(Some("#FFF3E0".to_string()))),
        icon: Set(None),
        cover_image: Set(None),
        sort_order: Set(request.sort_order),
        is_active: Set(1),
        created_at: Set(now.clone()),
        updated_at: Set(now),
    };
    new_subject
        .insert(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn update_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
    request: CreateSubjectRequest,
) -> CmdResult<subject::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let subj = subject::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Subject not found".to_string()))?;

    let mut active: subject::ActiveModel = subj.into();
    active.name = Set(request.name);
    if let Some(code) = request.code {
        active.code = Set(code);
    }
    if let Some(color) = request.color {
        active.color = Set(Some(color));
    }
    if let Some(sort_order) = request.sort_order {
        active.sort_order = Set(Some(sort_order));
    }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());

    active
        .update(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn delete_subject(
    state: State<'_, Arc<AppState>>,
    id: String,
) -> CmdResult<()> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let subj = subject::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?
        .ok_or_else(|| CommandError::new("Subject not found".to_string()))?;

    let mut active: subject::ActiveModel = subj.into();
    active.is_active = Set(0);
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());

    active
        .update(&db)
        .await
        .map_err(|e| CommandError::new(e.to_string()))?;
    Ok(())
}
