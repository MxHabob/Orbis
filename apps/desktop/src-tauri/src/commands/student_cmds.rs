//! Tauri IPC commands — Student (Enrollments, Guardians).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use orbis_domain::entities::enrollment::StudyMode;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct EnrollmentDto {
    pub id: String,
    pub student_id: String,
    pub class_id: Option<String>,
    pub period_id: String,
    pub org_id: String,
    pub student_number: String,
    pub study_mode: String,
    pub status: String,
    pub enrollment_date: String,
}

#[derive(Deserialize)]
pub struct EnrollStudentInput {
    #[serde(rename = "periodId")]
    pub period_id: String,
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "classId")]
    pub class_id: Option<String>,
    #[serde(rename = "studentNumber")]
    pub student_number: String,
    #[serde(rename = "studyMode")]
    pub study_mode: Option<String>,
}

#[derive(Serialize)]
pub struct GuardianDto {
    pub id: String,
    pub student_id: String,
    pub guardian_id: String,
    pub relationship: String,
    pub is_primary: bool,
}

#[derive(Deserialize)]
pub struct AssignGuardianInput {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "guardianId")]
    pub guardian_id: String,
    pub relationship: String,
    #[serde(rename = "isPrimary", default)]
    pub is_primary: bool,
}

#[derive(Deserialize, Default)]
pub struct EnrollmentFilter {
    #[serde(rename = "classId")]
    pub class_id: Option<String>,
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
}

#[tauri::command]
pub async fn student_enroll_student(request: EnrollStudentInput, state: State<'_, AppState>) -> Result<ApiResponse<EnrollmentDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_id = request.class_id.and_then(|c| Uuid::parse_str(&c).ok());

    let study_mode = match request.study_mode.as_deref() {
        Some("homeschooled") | Some("homeschool") => StudyMode::Homeschooled,
        Some("auditor") => StudyMode::Auditor,
        Some("visiting") => StudyMode::Visiting,
        Some("distance_learning") => StudyMode::DistanceLearning,
        Some("part_time") => StudyMode::PartTime,
        Some("exchange") => StudyMode::Exchange,
        Some("special_needs") => StudyMode::SpecialNeeds,
        _ => StudyMode::Regular,
    };

    let e = state.student_service.enroll_student(&ctx, ctx.tenant_id, period_id, student_id, class_id, request.student_number, study_mode)
        .await
        .map_err(|err| err.to_string())?;

    Ok(ok(EnrollmentDto {
        id: e.id.to_string(),
        student_id: e.student_id.to_string(),
        class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(),
        org_id: e.org_id.to_string(),
        student_number: e.student_number,
        study_mode: e.study_mode.to_string(),
        status: e.status.to_string(),
        enrollment_date: e.enrolled_at.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn student_list_enrollments(filter: Option<EnrollmentFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<EnrollmentDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(cid) = f.class_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.enrollment_repo.find_by_class(cid).await
    } else if let Some(sid) = f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.enrollment_repo.find_by_student(sid).await
    } else {
        Ok(vec![])
    }.map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(|e| EnrollmentDto {
        id: e.id.to_string(),
        student_id: e.student_id.to_string(),
        class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(),
        org_id: e.org_id.to_string(),
        student_number: e.student_number,
        study_mode: e.study_mode.to_string(),
        status: e.status.to_string(),
        enrollment_date: e.enrolled_at.to_rfc3339(),
    }).collect()))
}

#[tauri::command]
pub async fn student_get_enrollment(id: String, state: State<'_, AppState>) -> Result<ApiResponse<EnrollmentDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let e = state.enrollment_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Enrollment not found")?;
    Ok(ok(EnrollmentDto {
        id: e.id.to_string(),
        student_id: e.student_id.to_string(),
        class_id: e.class_id.map(|c| c.to_string()),
        period_id: e.period_id.to_string(),
        org_id: e.org_id.to_string(),
        student_number: e.student_number,
        study_mode: e.study_mode.to_string(),
        status: e.status.to_string(),
        enrollment_date: e.enrolled_at.to_rfc3339(),
    }))
}

#[tauri::command]
pub async fn student_assign_guardian(request: AssignGuardianInput, state: State<'_, AppState>) -> Result<ApiResponse<GuardianDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let guardian_id = Uuid::parse_str(&request.guardian_id).map_err(|_| "Invalid guardianId")?;

    let g = state.student_service.assign_guardian(&ctx, student_id, guardian_id, request.relationship, request.is_primary)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(GuardianDto {
        id: g.id.to_string(),
        student_id: g.student_id.to_string(),
        guardian_id: g.guardian_id.to_string(),
        relationship: g.relationship,
        is_primary: g.is_primary,
    }))
}

#[tauri::command]
pub async fn student_list_guardians(student_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<GuardianDto>>, String> {
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let items = state.student_guardian_repo.find_by_student(sid).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|g| GuardianDto {
        id: g.id.to_string(),
        student_id: g.student_id.to_string(),
        guardian_id: g.guardian_id.to_string(),
        relationship: g.relationship,
        is_primary: g.is_primary,
    }).collect()))
}
