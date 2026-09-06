//! Tauri IPC commands — Student (Enrollments, Guardians).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct EnrollmentDto {
    pub id: String, pub student_id: String, pub class_id: Option<String>,
    pub period_id: String, pub org_id: String, pub student_number: String,
    pub enrollment_type: String, pub status: String, pub enrollment_date: String,
}

#[derive(Serialize)]
pub struct GuardianDto {
    pub id: String, pub student_id: String, pub guardian_id: String,
    pub relationship: String, pub is_primary: bool,
}

#[derive(Deserialize, Default)]
pub struct EnrollmentFilter {
    #[serde(rename = "classId")]
    pub class_id: Option<String>,
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
}

#[tauri::command]
pub async fn student_list_enrollments(filter: Option<EnrollmentFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<EnrollmentDto>>, String> {
    let ctx = make_ctx(&state);
    let f = filter.unwrap_or_default();
    let enrollments = if let Some(cid) = f.class_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.enrollment_repo.find_by_class(cid).await
    } else if let Some(sid) = f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.enrollment_repo.find_by_student(sid).await
    } else {
        state.enrollment_repo.find_by_org(ctx.tenant_id).await
    }.map_err(|e| e.to_string())?;

    Ok(ok(enrollments.into_iter().map(|e| EnrollmentDto {
        id: e.id.to_string(), student_id: e.student_id.to_string(), class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(), org_id: e.org_id.to_string(),
        student_number: e.student_number, enrollment_type: e.enrollment_type,
        status: e.status, enrollment_date: e.enrollment_date.to_rfc3339(),
    }).collect()))
}

#[tauri::command]
pub async fn student_get_enrollment(id: String, state: State<'_, AppState>) -> Result<ApiResponse<EnrollmentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let e = state.enrollment_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Enrollment not found")?;
    Ok(ok(EnrollmentDto {
        id: e.id.to_string(), student_id: e.student_id.to_string(), class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(), org_id: e.org_id.to_string(),
        student_number: e.student_number, enrollment_type: e.enrollment_type,
        status: e.status, enrollment_date: e.enrollment_date.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn student_list_guardians(student_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<GuardianDto>>, String> {
    let uid = Uuid::parse_str(&student_id).map_err(|_| "Invalid ID")?;
    let items = state.student_guardian_repo.find_by_student(uid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|g| GuardianDto {
        id: g.id.to_string(), student_id: g.student_id.to_string(),
        guardian_id: g.guardian_id.to_string(), relationship: g.relationship,
        is_primary: g.is_primary,
    }).collect()))
}
