//! Tauri IPC commands — Assessment (Schemes, Grades, Attendance).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

#[derive(Serialize)]
pub struct AssessmentSchemeDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
}

#[derive(Deserialize)]
pub struct CreateSchemeInput {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "maxScore")]
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct GradeDto {
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub class_subject_id: String,
    pub component_id: Option<String>,
    pub score: Decimal,
    pub max_score: Decimal,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct RecordGradeInput {
    #[serde(rename = "periodId")]
    pub period_id: String,
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "enrollmentId")]
    pub enrollment_id: Option<String>,
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: String,
    #[serde(rename = "componentId")]
    pub component_id: Option<String>,
    pub score: Decimal,
    #[serde(rename = "maxScore")]
    pub max_score: Option<Decimal>,
    #[serde(rename = "isExcused", default)]
    pub is_excused: bool,
    #[serde(rename = "isExempt", default)]
    pub is_exempt: bool,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct AttendanceDto {
    pub id: String,
    pub student_id: String,
    pub class_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub scheduled_lesson_id: Option<String>,
    pub date: String,
    pub status: String,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct MarkAttendanceInput {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CalculateGradesInput {
    #[serde(rename = "schemeId")]
    pub scheme_id: String,
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: Option<String>,
}

#[derive(Serialize)]
pub struct CalculationResultDto {
    pub final_score: f64,
}

#[derive(Deserialize, Default)]
pub struct GradeFilter {
    #[serde(rename = "studentId")]
    pub student_id: Option<String>,
    #[serde(rename = "classSubjectId")]
    pub class_subject_id: Option<String>,
}

#[tauri::command]
pub async fn assessment_create_scheme(request: CreateSchemeInput, state: State<'_, AppState>) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let ctx = make_ctx(&state);
    let s = state.assessment_service.create_assessment_scheme(&ctx, ctx.tenant_id, request.name, request.max_score, request.formula)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(AssessmentSchemeDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        name: s.name,
        description: s.description,
        max_score: s.max_score,
    }))
}

#[tauri::command]
pub async fn assessment_list_schemes(state: State<'_, AppState>) -> Result<ApiResponse<Vec<AssessmentSchemeDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.assessment_service.list_assessment_schemes(ctx.tenant_id).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|s| AssessmentSchemeDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        name: s.name,
        description: s.description,
        max_score: s.max_score,
    }).collect()))
}

#[tauri::command]
pub async fn assessment_get_scheme(id: String, state: State<'_, AppState>) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.assessment_scheme_repo.find_by_id(uid).await.map_err(|e| e.to_string())?.ok_or("Scheme not found")?;
    Ok(ok(AssessmentSchemeDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        name: s.name,
        description: s.description,
        max_score: s.max_score,
    }))
}

#[tauri::command]
pub async fn assessment_record_grade(request: RecordGradeInput, state: State<'_, AppState>) -> Result<ApiResponse<GradeDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_subject_id = Uuid::parse_str(&request.class_subject_id).map_err(|_| "Invalid classSubjectId")?;
    let component_id = request.component_id.and_then(|c| Uuid::parse_str(&c).ok());
    let max_score = request.max_score.unwrap_or_else(|| Decimal::from(100));

    let g = state.assessment_service.record_grade(
        &ctx,
        ctx.tenant_id,
        period_id,
        student_id,
        class_subject_id,
        component_id,
        request.score,
        max_score,
        false,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(ok(GradeDto {
        id: g.id.to_string(),
        org_id: g.org_id.to_string(),
        period_id: g.period_id.to_string(),
        student_id: g.student_id.to_string(),
        enrollment_id: g.enrollment_id.map(|e| e.to_string()),
        class_subject_id: g.class_subject_id.to_string(),
        component_id: g.component_id.map(|c| c.to_string()),
        score: g.score,
        max_score: g.max_score,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        notes: g.notes,
    }))
}

#[tauri::command]
pub async fn assessment_list_grades(filter: Option<GradeFilter>, state: State<'_, AppState>) -> Result<ApiResponse<Vec<GradeDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(sid) = f.student_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.grade_repo.find_by_student(sid).await
    } else if let Some(csid) = f.class_subject_id.as_deref().and_then(|s| Uuid::parse_str(s).ok()) {
        state.grade_repo.find_by_class_subject(csid).await
    } else {
        Ok(vec![])
    }.map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(|g| GradeDto {
        id: g.id.to_string(),
        org_id: g.org_id.to_string(),
        period_id: g.period_id.to_string(),
        student_id: g.student_id.to_string(),
        enrollment_id: g.enrollment_id.map(|e| e.to_string()),
        class_subject_id: g.class_subject_id.to_string(),
        component_id: g.component_id.map(|c| c.to_string()),
        score: g.score,
        max_score: g.max_score,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        notes: g.notes,
    }).collect()))
}

#[tauri::command]
pub async fn assessment_mark_attendance(request: MarkAttendanceInput, state: State<'_, AppState>) -> Result<ApiResponse<AttendanceDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_id = Uuid::parse_str(&request.class_id).map_err(|_| "Invalid classId")?;
    let date = NaiveDate::parse_from_str(&request.date, "%Y-%m-%d").map_err(|_| "Invalid date format, use YYYY-MM-DD")?;

    let a = state.assessment_service.mark_attendance(&ctx, ctx.tenant_id, Uuid::nil(), student_id, Some(class_id), None, date, request.status, request.notes)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(AttendanceDto {
        id: a.id.to_string(),
        student_id: a.student_id.to_string(),
        class_id: a.class_id.map(|c| c.to_string()),
        class_subject_id: a.class_subject_id.map(|c| c.to_string()),
        scheduled_lesson_id: a.scheduled_lesson_id.map(|s| s.to_string()),
        date: a.attendance_date.to_string(),
        status: a.status,
        note: a.notes,
    }))
}

#[tauri::command]
pub async fn assessment_list_attendance(class_id: String, date: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<AttendanceDto>>, String> {
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|_| "Invalid date format, use YYYY-MM-DD")?;
    let items = state.attendance_repo.find_by_class_and_date(cid, parsed_date).await.map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(|a| AttendanceDto {
        id: a.id.to_string(),
        student_id: a.student_id.to_string(),
        class_id: a.class_id.map(|c| c.to_string()),
        class_subject_id: a.class_subject_id.map(|c| c.to_string()),
        scheduled_lesson_id: a.scheduled_lesson_id.map(|s| s.to_string()),
        date: a.attendance_date.to_string(),
        status: a.status,
        note: a.notes,
    }).collect()))
}

#[tauri::command]
pub async fn assessment_calculate_grades(request: CalculateGradesInput, state: State<'_, AppState>) -> Result<ApiResponse<CalculationResultDto>, String> {
    let ctx = make_ctx(&state);
    let scheme_id = Uuid::parse_str(&request.scheme_id).map_err(|_| "Invalid schemeId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_subject_id = request.class_subject_id.and_then(|cs| Uuid::parse_str(&cs).ok()).unwrap_or_else(Uuid::nil);

    let final_score = state.assessment_service.calculate_scheme_grades(&ctx, scheme_id, student_id, class_subject_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(CalculationResultDto { final_score }))
}

#[derive(Serialize)]
pub struct CumulativeGpaRecordDto {
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    pub period_id: String,
    pub term_attempted_credits: Decimal,
    pub term_earned_credits: Decimal,
    pub term_points: Decimal,
    pub term_gpa: Decimal,
    pub cumulative_attempted_credits: Decimal,
    pub cumulative_earned_credits: Decimal,
    pub cumulative_points: Decimal,
    pub cumulative_gpa: Decimal,
    pub standing: String,
    pub calculated_at: String,
}

#[derive(Deserialize)]
pub struct CalculateGpaInput {
    #[serde(rename = "studentId")]
    pub student_id: String,
    #[serde(rename = "periodId")]
    pub period_id: String,
}

#[tauri::command]
pub async fn assessment_calculate_gpa(
    request: CalculateGpaInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<CumulativeGpaRecordDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;

    let record = state.assessment_service
        .calculate_gpa(&ctx, ctx.tenant_id, period_id, student_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(CumulativeGpaRecordDto {
        id: record.id.to_string(),
        org_id: record.org_id.to_string(),
        student_id: record.student_id.to_string(),
        period_id: record.period_id.to_string(),
        term_attempted_credits: record.term_attempted_credits,
        term_earned_credits: record.term_earned_credits,
        term_points: record.term_points,
        term_gpa: record.term_gpa,
        cumulative_attempted_credits: record.cumulative_attempted_credits,
        cumulative_earned_credits: record.cumulative_earned_credits,
        cumulative_points: record.cumulative_points,
        cumulative_gpa: record.cumulative_gpa,
        standing: format!("{:?}", record.standing),
        calculated_at: record.calculated_at.to_rfc3339(),
    }))
}

