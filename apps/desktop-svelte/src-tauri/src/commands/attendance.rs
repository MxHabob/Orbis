use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::attendance,
    params::AttendanceParams,
    service::attendance_svc::{
        CreateAttendanceRequest, BulkAttendanceRequest,
        AttendanceReport, AttendanceAnalytics,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: AttendanceParams,
) -> CmdResult<Vec<attendance::Model>> {
    state.service().await
        .get_attendance(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn record_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    recorded_by: String,
    request: CreateAttendanceRequest,
) -> CmdResult<attendance::Model> {
    state.service().await
        .record_attendance(&org_id, &recorded_by, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn record_bulk_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    recorded_by: String,
    request: BulkAttendanceRequest,
) -> CmdResult<Vec<attendance::Model>> {
    state.service().await
        .record_bulk_attendance(&org_id, &recorded_by, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_attendance_report(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<AttendanceReport> {
    state.service().await
        .get_attendance_report(&enrollment_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_attendance_analytics(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: AttendanceParams,
) -> CmdResult<AttendanceAnalytics> {
    state.service().await
        .get_attendance_analytics(&org_id, params)
        .await
        .map_err(CommandError::from)
}
