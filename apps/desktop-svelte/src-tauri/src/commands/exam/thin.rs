/// Thin Tauri command wrappers for the Exam domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_exam::ops;

#[tauri::command]
pub async fn list_exam_sessions(state: State<'_, Arc<AppState>>, period_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let period = svc_organization::entities::academic_period::Entity::find_by_id(period_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Period not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::session::list(&ctx, &period_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_exam_session(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::session::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_exam_session(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::session::get(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[derive(serde::Deserialize)]
pub struct UpdateExamSessionPayload {
    pub name: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[tauri::command]
pub async fn update_exam_session(
    state: State<'_, Arc<AppState>>,
    session_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let payload: UpdateExamSessionPayload = serde_json::from_value(data)
        .map_err(|e| CommandError::new(format!("Invalid payload: {}", e)))?;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::session::update(
        &ctx,
        &session_id,
        payload.name,
        payload.status,
        payload.start_date,
        payload.end_date,
    ).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_exam_subjects(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::exam_subject::list(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_exam_subject(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::exam_subject::add(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_exam_halls(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::hall::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_exam_hall(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let db = state.db();
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::hall::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn generate_ordering(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::seating::generate(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_seating(state: State<'_, Arc<AppState>>, exam_subject_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let subject = svc_exam::entities::exam_subject::Entity::find_by_id(exam_subject_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam subject not found"))?;
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(subject.exam_session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::seating::list(&ctx, &subject.exam_session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn generate_exam_schedule(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::schedule::generate(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_schedule(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let sess = svc_exam::entities::exam_session::Entity::find_by_id(session_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Exam session not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::schedule::get(&ctx, &session_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

/// Helper function to resolve org_id from the latest active session.
async fn resolve_org_id(db: &sea_orm::DatabaseConnection) -> Result<String, CommandError> {
    use sea_orm::{EntityTrait, QueryOrder};
    let session = svc_iam::entities::user_session::Entity::find()
        .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("No active session"))?;
    let user = svc_iam::entities::user::Entity::find_by_id(session.user_id)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("Active user not found"))?;
    Ok(user.org_id)
}

// ── Student Ordering Commands ────────────────────────────────────────

#[tauri::command]
pub async fn get_student_ordering(state: State<'_, Arc<AppState>>, period_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::ordering::list_ordering(&ctx, &period_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn rebuild_student_ordering(state: State<'_, Arc<AppState>>, session_id: String) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::ordering::generate_ordering(&ctx, &session_id).await.map_err(CommandError::from)
}

