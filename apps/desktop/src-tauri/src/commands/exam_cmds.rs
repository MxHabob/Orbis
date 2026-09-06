//! Tauri IPC commands — Exam (Sessions, Schedules).

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use uuid::Uuid;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct ExamSessionDto {
    pub id: String, pub org_id: String, pub academic_period_id: String,
    pub name: String,
    pub start_date: String, pub end_date: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct ExamScheduleDto {
    pub id: String, pub exam_session_id: String, pub class_subject_id: String,
    pub exam_date: String, pub start_time: String, pub end_time: String,
}

#[tauri::command]
pub async fn exam_list_sessions(period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamSessionDto>>, String> {
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.exam_session_repo.find_by_period(pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), academic_period_id: s.academic_period_id.to_string(),
        name: s.name,
        start_date: s.start_date.to_string(), end_date: s.end_date.to_string(),
        status: s.status,
    }).collect()))
}

#[tauri::command]
pub async fn exam_get_session(id: String, state: State<'_, AppState>) -> Result<ApiResponse<ExamSessionDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.exam_session_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Session not found")?;
    Ok(ok(ExamSessionDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), academic_period_id: s.academic_period_id.to_string(),
        name: s.name,
        start_date: s.start_date.to_string(), end_date: s.end_date.to_string(),
        status: s.status,
    }))
}

#[tauri::command]
pub async fn exam_list_schedules(session_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ExamScheduleDto>>, String> {
    let sid = Uuid::parse_str(&session_id).map_err(|_| "Invalid session ID")?;
    let items = state.exam_schedule_repo.find_by_session(sid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| ExamScheduleDto {
        id: s.id.to_string(), exam_session_id: s.exam_session_id.to_string(),
        class_subject_id: s.class_subject_id.to_string(),
        exam_date: s.exam_date.to_string(),
        start_time: s.start_time.to_string(), end_time: s.end_time.to_string(),
    }).collect()))
}
