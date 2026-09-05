//! Tauri IPC commands — Exam (Sessions, Schedules).

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use uuid::Uuid;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct ExamSessionDto {
    pub id: String, pub org_id: String, pub period_id: String,
    pub name: String, pub name_ar: Option<String>,
    pub start_date: String, pub end_date: String,
    pub status: String, pub is_active: bool,
}

#[derive(Serialize)]
pub struct ExamScheduleDto {
    pub id: String, pub session_id: String, pub subject_id: String,
    pub date: String, pub max_score: f64, pub passing_score: f64,
}

#[tauri::command]
pub async fn exam_list_sessions(period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamSessionDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.exam_session_repo.find_by_period(pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), period_id: s.period_id.to_string(),
        name: s.name, name_ar: s.name_ar,
        start_date: s.start_date.to_rfc3339(), end_date: s.end_date.to_rfc3339(),
        status: s.status, is_active: s.is_active,
    }).collect()))
}

#[tauri::command]
pub async fn exam_get_session(id: String, state: State<'_, AppState>) -> Result<ApiResponse<ExamSessionDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.exam_session_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Session not found")?;
    Ok(ok(ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), period_id: s.period_id.to_string(),
        name: s.name, name_ar: s.name_ar,
        start_date: s.start_date.to_rfc3339(), end_date: s.end_date.to_rfc3339(),
        status: s.status, is_active: s.is_active,
    }))
}

#[tauri::command]
pub async fn exam_list_schedules(session_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamScheduleDto>>, String> {
    let sid = Uuid::parse_str(&session_id).map_err(|_| "Invalid session ID")?;
    let items = state.exam_schedule_repo.find_by_session(sid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamScheduleDto {
        id: s.id.to_string(), session_id: s.session_id.to_string(),
        subject_id: s.subject_id.to_string(), date: s.date.to_string(),
        max_score: s.max_score, passing_score: s.passing_score,
    }).collect()))
}
