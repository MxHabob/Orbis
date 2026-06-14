use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{behavior_record, notification, org_unit, audit_log},
    params::{BehaviorParams, NotificationParams, OrgUnitParams},
    service::behavior::{AIBehaviorSubmission, UpdateBehaviorRequest},
    service::org_unit::{CreateOrgUnitRequest, UpdateOrgUnitRequest},
    service::dashboard::DashboardStats,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

// ── Behavior ───────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_behavior_records(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: BehaviorParams,
) -> CmdResult<Vec<behavior_record::Model>> {
    state
        .service
        .get_behavior_records(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn submit_ai_behavior(
    state: State<'_, Arc<AppState>>,
    request: AIBehaviorSubmission,
) -> CmdResult<behavior_record::Model> {
    state
        .service
        .submit_ai_behavior(request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
    reviewed_by: String,
    request: UpdateBehaviorRequest,
) -> CmdResult<behavior_record::Model> {
    state
        .service
        .update_behavior_record(&record_id, request, &reviewed_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_behavior_record(
    state: State<'_, Arc<AppState>>,
    record_id: String,
) -> CmdResult<()> {
    state
        .service
        .delete_behavior_record(&record_id)
        .await
        .map_err(CommandError::from)
}

// ── Notifications ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_notifications(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    params: NotificationParams,
) -> CmdResult<Vec<notification::Model>> {
    state
        .service
        .get_notifications(&user_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn mark_notification_read(
    state: State<'_, Arc<AppState>>,
    notification_id: String,
) -> CmdResult<()> {
    state
        .service
        .mark_notification_read(&notification_id)
        .await
        .map_err(CommandError::from)
}

// ── Org Units ──────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_org_units(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: OrgUnitParams,
) -> CmdResult<Vec<org_unit::Model>> {
    state
        .service
        .get_org_units(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_org_unit(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOrgUnitRequest,
) -> CmdResult<org_unit::Model> {
    state
        .service
        .create_org_unit(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
    request: UpdateOrgUnitRequest,
) -> CmdResult<org_unit::Model> {
    state
        .service
        .update_org_unit(&unit_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_org_unit(
    state: State<'_, Arc<AppState>>,
    unit_id: String,
) -> CmdResult<()> {
    state
        .service
        .delete_org_unit(&unit_id)
        .await
        .map_err(CommandError::from)
}

// ── Dashboard ──────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_dashboard_stats(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<DashboardStats> {
    state
        .service
        .get_dashboard_stats(&org_id)
        .await
        .map_err(CommandError::from)
}

// ── Audit Logs ─────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_audit_logs(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<audit_log::Model>> {
    state
        .service
        .repo
        .get_audit_logs(&org_id)
        .await
        .map_err(|e| CommandError::from(e.to_string()))
}
