//! Tauri IPC commands — Academic (Subjects, Classes, ClassSubjects).

use tauri::State;
use serde::Serialize;
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct SubjectDto {
    pub id: String, pub org_id: String, pub code: String,
    pub name: String, pub credit_hours: Option<i32>, pub is_core: bool,
}

#[derive(Serialize)]
pub struct ClassDto {
    pub id: String, pub org_id: String, pub unit_id: String,
    pub period_id: String, pub name: String, pub level: String,
}

#[derive(Serialize)]
pub struct ClassSubjectDto {
    pub id: String, pub class_id: String, pub subject_id: String,
    pub period_id: String, pub primary_teacher_id: String, pub weekly_hours: Option<i32>,
}

#[tauri::command]
pub async fn academic_list_subjects(state: State<'_, AppState>) -> Result<ApiResponse<Vec<SubjectDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.subject_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| SubjectDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), code: s.code,
        name: s.name, credit_hours: s.credit_hours, is_core: s.is_core,
    }).collect()))
}

#[tauri::command]
pub async fn academic_get_subject(id: String, state: State<'_, AppState>) -> Result<ApiResponse<SubjectDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.subject_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Subject not found")?;
    Ok(ok(SubjectDto { id: s.id.to_string(), org_id: s.org_id.to_string(), code: s.code, name: s.name, credit_hours: s.credit_hours, is_core: s.is_core }))
}

#[tauri::command]
pub async fn academic_list_classes(state: State<'_, AppState>) -> Result<ApiResponse<Vec<ClassDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.class_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|c| ClassDto {
        id: c.id.to_string(), org_id: c.org_id.to_string(), unit_id: c.unit_id.to_string(),
        period_id: c.period_id.to_string(), name: c.name, level: c.level,
    }).collect()))
}

#[tauri::command]
pub async fn academic_get_class(id: String, state: State<'_, AppState>) -> Result<ApiResponse<ClassDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let c = state.class_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Class not found")?;
    Ok(ok(ClassDto { id: c.id.to_string(), org_id: c.org_id.to_string(), unit_id: c.unit_id.to_string(), period_id: c.period_id.to_string(), name: c.name, level: c.level }))
}

#[tauri::command]
pub async fn academic_list_class_subjects(class_id: String, period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ClassSubjectDto>>, String> {
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let items = state.class_subject_repo.find_by_class_and_period(cid, pid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|cs| ClassSubjectDto {
        id: cs.id.to_string(), class_id: cs.class_id.to_string(), subject_id: cs.subject_id.to_string(),
        period_id: cs.period_id.to_string(), primary_teacher_id: cs.primary_teacher_id.to_string(),
        weekly_hours: cs.weekly_hours,
    }).collect()))
}
