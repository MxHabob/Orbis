use tauri::State;
use std::sync::Arc;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};

// ── Structs & Types ──────────────────────────────────────────────────────────

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AttendanceParams {
    pub enrollment_id: Option<String>,
    pub class_id: Option<String>,
    pub date: Option<String>,
}

// ── Commands ─────────────────────────────────────────────────────────────────

#[tauri::command]
pub async fn get_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: AttendanceParams,
) -> CmdResult<Vec<domain_assessment::entities::attendance::Model>> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::attendance::AttendanceService::new(cmd, query);

    let domain_params = domain_assessment::params::AttendanceParams {
        enrollment_id: params.enrollment_id,
        class_id: params.class_id,
        date: params.date,
        cycle_id: None,
        date_from: None,
        date_to: None,
        status: None,
        limit: None,
        offset: None,
    };

    service.get_attendance(&org_id, domain_params)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn record_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    recorded_by: String,
    request: domain_assessment::services::attendance::CreateAttendanceRequest,
) -> CmdResult<domain_assessment::entities::attendance::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::attendance::AttendanceService::new(cmd, query);

    service.record_attendance(&org_id, &recorded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn record_bulk_attendance(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    recorded_by: String,
    request: domain_assessment::services::attendance::BulkAttendanceRequest,
) -> CmdResult<Vec<domain_assessment::entities::attendance::Model>> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::attendance::AttendanceService::new(cmd, query);

    service.record_bulk_attendance(&org_id, &recorded_by, request)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_attendance_report(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<domain_assessment::services::attendance::AttendanceReport> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::attendance::AttendanceService::new(cmd, query);

    service.get_attendance_report(&enrollment_id)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}

#[tauri::command]
pub async fn get_attendance_analytics(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: AttendanceParams,
) -> CmdResult<domain_assessment::services::attendance::AttendanceAnalytics> {
    let db = state.active_db().await;
    let cmd = Arc::new(domain_assessment::repository::AssessmentCommandRepo::new(db.clone()));
    let query = Arc::new(domain_assessment::repository::AssessmentQueryRepo::new(db));
    let service = domain_assessment::services::attendance::AttendanceService::new(cmd, query);

    let domain_params = domain_assessment::params::AttendanceParams {
        enrollment_id: params.enrollment_id,
        class_id: params.class_id,
        date: params.date,
        cycle_id: None,
        date_from: None,
        date_to: None,
        status: None,
        limit: None,
        offset: None,
    };

    service.get_attendance_analytics(&org_id, domain_params)
        .await
        .map_err(|e| CommandError::new(e.to_string()))
}
