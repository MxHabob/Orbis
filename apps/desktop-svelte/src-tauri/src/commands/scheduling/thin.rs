/// Thin Tauri command wrappers for the Scheduling domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_scheduling::ops;

// ── Time Slots ──────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_time_slots(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_time_slot(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_time_slot(state: State<'_, Arc<AppState>>, org_id: String, slot_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::time_slot::update(&ctx, &slot_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_time_slot(state: State<'_, Arc<AppState>>, org_id: String, slot_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::time_slot::delete(&ctx, &slot_id).await.map_err(CommandError::from)
}

// ── Lessons ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_lessons_by_class(state: State<'_, Arc<AppState>>, org_id: String, class_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::list_by_class(&ctx, &class_id, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_lessons_by_teacher(state: State<'_, Arc<AppState>>, org_id: String, teacher_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::list_by_teacher(&ctx, &teacher_id, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_lesson(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_lesson(state: State<'_, Arc<AppState>>, org_id: String, lesson_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::lesson::update(&ctx, &lesson_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_lesson(state: State<'_, Arc<AppState>>, org_id: String, lesson_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::lesson::delete(&ctx, &lesson_id).await.map_err(CommandError::from)
}

// ── Rooms ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_rooms(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_room(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_room(state: State<'_, Arc<AppState>>, org_id: String, room_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::room::update(&ctx, &room_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_room(state: State<'_, Arc<AppState>>, org_id: String, room_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::room::delete(&ctx, &room_id).await.map_err(CommandError::from)
}

// ── Solver ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn start_solve(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::start(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_solve_jobs(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::list_jobs(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_solve_job(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::get_job(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn cancel_solve(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::cancel(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn apply_solve(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::solver::apply(&ctx, &job_id).await.map(|_| ()).map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_solve_job(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::solver::delete_job(&ctx, &job_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn check_conflicts(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::solver::check_conflicts(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}
