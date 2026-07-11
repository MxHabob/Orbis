/// Thin Tauri command wrappers for Platform/Lifecycle operations.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_lifecycle::ops;

// ── Organization ────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_organization(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::get(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_main_organization(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::organization::get_main(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_organization(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::update(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::organization::get_settings(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_settings(state: State<'_, Arc<AppState>>, org_id: String, settings: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::organization::update_settings(&ctx, settings).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_dashboard_stats(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::dashboard::get_stats(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_setup_status(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<bool> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::organization::check_setup(&ctx).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn initialize_school_setup(state: State<'_, Arc<AppState>>, org_id: String, payload: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::setup::initialize(&ctx, payload).await.map_err(CommandError::from)
}

// ── Academic Years ──────────────────────────────────────────────────

#[tauri::command]
pub async fn get_current_academic_year(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let db = state.active_db().await;
    let r = ops::academic_year::get_current(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_academic_years(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::academic_year::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn switch_academic_year(state: State<'_, Arc<AppState>>, year_id: String) -> CmdResult<()> {
    let db = state.active_db().await;
    ops::academic_year::switch(&db, &year_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn is_year_archived(state: State<'_, Arc<AppState>>) -> CmdResult<bool> {
    let db = state.active_db().await;
    ops::academic_year::is_archived(&db).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_academic_context(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::academic_year::get_context(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

// ── Promotion ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_promotion_jobs(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::list_jobs(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_promotion_job(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::get_job(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_promotion_details(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::get_details(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn promotion_dry_run(state: State<'_, Arc<AppState>>, org_id: String, config: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::dry_run(&ctx, config).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn execute_promotion(state: State<'_, Arc<AppState>>, org_id: String, job_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::execute(&ctx, &job_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn rollback_promotion(state: State<'_, Arc<AppState>>, org_id: String, job_id: String, user_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::promotion::rollback(&ctx, &job_id, &user_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn override_promotion_decision(state: State<'_, Arc<AppState>>, org_id: String, request: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::promotion::override_decision(&ctx, request).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_promotion_policy(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::promotion::get_policy(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

// ── Calendar ────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_calendar_events(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::calendar::list(&ctx, &cycle_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_pending_calendar_events(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::calendar::list_pending(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_calendar_event(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::calendar::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn execute_calendar_event(state: State<'_, Arc<AppState>>, org_id: String, event_id: String, executed_by: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::calendar::execute(&ctx, &event_id, &executed_by).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_calendar_event(state: State<'_, Arc<AppState>>, org_id: String, event_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::calendar::delete(&ctx, &event_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn generate_default_calendar(state: State<'_, Arc<AppState>>, org_id: String, cycle_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::calendar::generate_defaults(&ctx, &cycle_id).await.map_err(CommandError::from)
}

// ── Overrides ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_override_requests(state: State<'_, Arc<AppState>>, org_id: String, params: Option<serde_json::Value>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::override_request::list(&ctx, params).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_override_request(state: State<'_, Arc<AppState>>, org_id: String, request_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::override_request::get(&ctx, &request_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_override_request(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::override_request::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn process_approval(state: State<'_, Arc<AppState>>, org_id: String, request_id: String, action: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::override_request::process(&ctx, &request_id, action).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn cancel_override(state: State<'_, Arc<AppState>>, org_id: String, request_id: String, user_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::override_request::cancel(&ctx, &request_id, &user_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_pending_approvals(state: State<'_, Arc<AppState>>, org_id: String, approver_role: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::override_request::list_pending(&ctx, &approver_role).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

// ── Policy ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_org_policy(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::policy::get(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_org_policy(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::policy::update(&ctx, data).await.map_err(CommandError::from)
}

// ── Document Templates ──────────────────────────────────────────────

#[tauri::command]
pub async fn get_document_templates(state: State<'_, Arc<AppState>>, org_id: String, params: Option<serde_json::Value>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::document_template::list(&ctx, params).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::document_template::get(&ctx, &template_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_document_template(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::document_template::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::document_template::update(&ctx, &template_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::document_template::delete(&ctx, &template_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn publish_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String, user_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::document_template::publish(&ctx, &template_id, &user_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn archive_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::document_template::archive(&ctx, &template_id).await.map_err(CommandError::from)
}

#[tauri::command]
pub async fn restore_document_template(state: State<'_, Arc<AppState>>, org_id: String, template_id: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::document_template::restore(&ctx, &template_id).await.map_err(CommandError::from)
}

// ── Enrollment Status ───────────────────────────────────────────────

#[tauri::command]
pub async fn transition_enrollment_status(state: State<'_, Arc<AppState>>, org_id: String, enrollment_id: String, status: String) -> CmdResult<()> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ops::enrollment_status::transition(&ctx, &enrollment_id, &status).await.map_err(CommandError::from)
}

// ── Import / Export ─────────────────────────────────────────────────

#[tauri::command]
pub async fn import_students(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_import::students(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn import_grades(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_import::grades(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn export_students(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_export::students(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn export_grades(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::bulk_export::grades(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}
