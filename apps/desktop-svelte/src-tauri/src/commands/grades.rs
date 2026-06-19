use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::grade,
    params::GradeParams,
    service::grade::{
        CreateGradeRequest, BulkGradeRequest, UpdateGradeRequest,
        GradeReportCard, GradeAnalytics,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_grades(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: GradeParams,
) -> CmdResult<Vec<grade::Model>> {
    state
        .service
        .get_grades(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn submit_grade(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: CreateGradeRequest,
) -> CmdResult<grade::Model> {
    state
        .service
        .submit_grade(&org_id, &graded_by, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn submit_grades_bulk(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    graded_by: String,
    request: BulkGradeRequest,
) -> CmdResult<Vec<grade::Model>> {
    state
        .service
        .submit_grades_bulk(&org_id, &graded_by, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_grade(
    state: State<'_, Arc<AppState>>,
    grade_id: String,
    request: UpdateGradeRequest,
) -> CmdResult<grade::Model> {
    state
        .service
        .update_grade(&grade_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_report_card(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<GradeReportCard> {
    state
        .service
        .generate_report_card(&enrollment_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_grade_analytics(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: GradeParams,
) -> CmdResult<GradeAnalytics> {
    state
        .service
        .get_grade_analytics(&org_id, params)
        .await
        .map_err(CommandError::from)
}
