/// Thin Tauri command wrappers for the Academic domain.
///
/// Each command is a 1-line delegation to `svc_academic::ops::*`.
/// NO business logic lives here.

use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

use svc_academic::ops;

// ── Cycles ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_cycles(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::cycle::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::cycle::get(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::cycle::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::cycle::update(&ctx, &cycle_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn activate_cycle(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    cycle_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::cycle::set_current(&ctx, &cycle_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn lock_cycle(
    state: State<'_, Arc<AppState>>,
    cycle_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context("_").await.map_err(CommandError::from)?;
    ops::cycle::lock(&ctx, &cycle_id).await.map_err(CommandError::from)
}

// ── Classes ─────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_classes(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class::get(&ctx, &class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class::update(&ctx, &class_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_class(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::class::delete(&ctx, &class_id).await.map_err(CommandError::from)
}

// ── Subjects ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_subjects(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::subject::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::subject::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    subject_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::subject::update(&ctx, &subject_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    subject_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::subject::delete(&ctx, &subject_id).await.map_err(CommandError::from)
}

// ── Enrollments ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_enrollments(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::enrollment::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn get_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::enrollment::get(&ctx, &enrollment_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::enrollment::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn transfer_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: String,
    new_class_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::enrollment::transfer(&ctx, &enrollment_id, &new_class_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn withdraw_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::enrollment::withdraw(&ctx, &enrollment_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn check_duplicate_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    national_id: Option<String>,
    email: Option<String>,
    phone: Option<String>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::enrollment::check_duplicate(&ctx, national_id, email, phone).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

// ── Class Subjects ──────────────────────────────────────────────────

#[tauri::command]
pub async fn list_class_subjects(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_id: String,
    cycle_id: Option<String>,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class_subject::list(&ctx, &class_id, cycle_id.as_deref()).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_class_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class_subject::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_class_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_subject_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::class_subject::update(&ctx, &class_subject_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_class_subject(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    class_subject_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::class_subject::delete(&ctx, &class_subject_id).await.map_err(CommandError::from)
}

// ── Guardians ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_guardians_by_student(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    student_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::guardian::list_by_student(&ctx, &student_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn search_guardians(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    query: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::guardian::search(&ctx, &query).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_guardian_link(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::guardian::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn update_guardian_link(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    link_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::guardian::update(&ctx, &link_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_guardian_link(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    link_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::guardian::delete(&ctx, &link_id).await.map_err(CommandError::from)
}

// ── Org Units ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_org_units(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::org_unit::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_org_unit(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::org_unit::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

// ── Documents ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_documents(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    owner_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::document::list(&ctx, &owner_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn create_document(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    data: serde_json::Value,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::document::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn delete_document(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    document_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::document::delete(&ctx, &document_id).await.map_err(CommandError::from)
}

// ── Notifications ───────────────────────────────────────────────────

#[tauri::command]
pub async fn list_notifications(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let result = ops::notification::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(result).unwrap())
}

#[tauri::command]
pub async fn mark_notification_read(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    notification_id: String,
) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::notification::mark_read(&ctx, &notification_id).await.map_err(CommandError::from)
}
