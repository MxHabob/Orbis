/// Thin Tauri command wrappers for the Assessment domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_assessment::ops;

#[tauri::command]
pub async fn list_assessments(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String, class_subject_id: Option<String>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::assessment::list(&ctx, &cycle_id, class_subject_id.as_deref()).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_assessment(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::assessment::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_assessment(state: State<'_, Arc<AppState>>, org_id: String, assessment_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::assessment::update(&ctx, &assessment_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_assessment(state: State<'_, Arc<AppState>>, org_id: String, assessment_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::assessment::delete(&ctx, &assessment_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn submit_grade(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::submit(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn submit_bulk_grades(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::submit_bulk(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_grades(state: State<'_, Arc<AppState>>, org_id: String, assessment_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::list(&ctx, &assessment_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn record_attendance(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::attendance::record(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn record_bulk_attendance(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::attendance::record_bulk(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_attendance(state: State<'_, Arc<AppState>>, org_id: String, class_id: String, date: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::attendance::list(&ctx, &class_id, &date).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn submit_behavior(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::behavior::submit_ai(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_behavior(state: State<'_, Arc<AppState>>, org_id: String, student_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::behavior::list(&ctx, &student_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_report_card(state: State<'_, Arc<AppState>>, org_id: String, enrollment_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::report_card(&ctx, &enrollment_id, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_grade_analytics(state: State<'_, Arc<AppState>>, org_id: String, class_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::analytics(&ctx, &class_id, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_attendance_report(state: State<'_, Arc<AppState>>, org_id: String, class_id: String, start: String, end: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::attendance::report(&ctx, &class_id, &start, &end).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn upsert_grade_cell(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::upsert_cell(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn compile_semester_grades(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String, class_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::compile_semester(&ctx, &cycle_id, &class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_grade_matrix(state: State<'_, Arc<AppState>>, org_id: String, class_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::grade::matrix(&ctx, &class_id, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_grade(
    state: State<'_, Arc<AppState>>,
    grade_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let grade = svc_assessment::entities::grade::Entity::find_by_id(grade_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Grade not found"))?;
    let ctx = state.build_context(&grade.org_id).await.map_err(CommandError::from)?;
    let req: svc_assessment::services::grade::UpdateGradeRequest = serde_json::from_value(data)
        .map_err(|e| shared_kernel::errors::AppError::validation(format!("Invalid update grade payload: {e}")))?;
    let r = ops::grade::update(&ctx, &grade_id, req).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[derive(serde::Deserialize)]
pub struct UpdateBehaviorPayload {
    pub action_taken: Option<String>,
    pub status: Option<String>,
    pub resolution_notes: Option<String>,
}

#[tauri::command]
pub async fn update_behavior(
    state: State<'_, Arc<AppState>>,
    record_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let record = svc_assessment::entities::behavior_record::Entity::find_by_id(record_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Behavior record not found"))?;
    let ctx = state.build_context(&record.org_id).await.map_err(CommandError::from)?;
    let req: UpdateBehaviorPayload = serde_json::from_value(data)
        .map_err(|e| shared_kernel::errors::AppError::validation(format!("Invalid update behavior payload: {e}")))?;
    let r = ops::behavior::update(&ctx, &record_id, req.action_taken, req.status, req.resolution_notes).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_behavior(
    state: State<'_, Arc<AppState>>,
    record_id: String,
) -> CmdResult<()> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let record = svc_assessment::entities::behavior_record::Entity::find_by_id(record_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Behavior record not found"))?;
    let ctx = state.build_context(&record.org_id).await.map_err(CommandError::from)?;
    ops::behavior::delete(&ctx, &record_id).await.map_err(CommandError::from)
}
