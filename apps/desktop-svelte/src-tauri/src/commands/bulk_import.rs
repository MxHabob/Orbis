use tauri::State;
use std::sync::Arc;

use core_domain::service::bulk_import::{BulkStudentRow, BulkGradeRow, BulkImportResult};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn bulk_create_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    rows: Vec<BulkStudentRow>,
) -> CmdResult<BulkImportResult> {
    state.service().await
        .bulk_create_users(&org_id, &created_by, rows, "student")
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn bulk_create_teachers(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    rows: Vec<BulkStudentRow>,
) -> CmdResult<BulkImportResult> {
    state.service().await
        .bulk_create_users(&org_id, &created_by, rows, "teacher")
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn bulk_create_grades(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    cycle_id: Option<String>,
    rows: Vec<BulkGradeRow>,
) -> CmdResult<BulkImportResult> {
    state.service().await
        .bulk_create_grades(&org_id, &graded_by, cycle_id, rows)
        .await
        .map_err(CommandError::from)
}
