//! Tauri IPC commands — Assessment (Schemes, Grades, Attendance).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AssessmentSchemeDto { pub id: String, pub org_id: String, pub name: String, pub grading_type: String }

#[derive(Serialize)]
pub struct GradeDto {
    pub id: String, pub student_id: String, pub class_subject_id: String,
    pub assessment_type: String, pub score: f64, pub max_score: f64,
    pub letter_grade: Option<String>,
}

#[derive(Serialize)]
pub struct AttendanceDto {
    pub id: String, pub student_id: String, pub class_id: Option<String>,
    pub class_subject_id: Option<String>, pub scheduled_lesson_id: Option<String>,
    pub date: String, pub status: String, pub note: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct GradeFilter {
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: Option<String>,
}

#[tauri::command]
pub async fn assessment_list_schemes(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AssessmentSchemeDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.assessment_scheme_repo.find_by_org(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| AssessmentSchemeDto {
        id: s.id.to_string(), org_id: s.org_id.to_string(), name: s.name, grading_type: s.grading_type,
    }).collect()))
}

#[tauri::command]
pub async fn assessment_get_scheme(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.assessment_scheme_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Scheme not found")?;
    Ok(ok(AssessmentSchemeDto { id: s.id.to_string(), org_id: s.org_id.to_string(), name: s.name, grading_type: s.grading_type }))
}

#[tauri::command]
pub async fn assessment_list_grades(filter: Option<GradeFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<GradeDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(sid) = f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.grade_repo.find_by_student(sid).await
    } else if let Some(csid) = f.class_subject_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.grade_repo.find_by_class_subject(csid).await
    } else { Ok(vec![]) }.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|g| GradeDto {
        id: g.id.to_string(), student_id: g.student_id.to_string(),
        class_subject_id: g.class_subject_id.to_string(),
        assessment_type: g.assessment_type, score: g.score, max_score: g.max_score,
        letter_grade: g.letter_grade,
    }).collect()))
}

#[tauri::command]
pub async fn assessment_list_attendance(class_id: String, date: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<AttendanceDto>>, String> {
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let parsed_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format, use YYYY-MM-DD")?;
    let items = state.attendance_repo.find_by_class_and_date(cid, parsed_date).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|a| AttendanceDto {
        id: a.id.to_string(), student_id: a.student_id.to_string(),
        class_id: a.class_id.map(|c| c.to_string()),
        class_subject_id: a.class_subject_id.map(|c| c.to_string()),
        scheduled_lesson_id: a.scheduled_lesson_id.map(|s| s.to_string()),
        date: a.date.to_string(), status: a.status, note: a.note,
    }).collect()))
}
