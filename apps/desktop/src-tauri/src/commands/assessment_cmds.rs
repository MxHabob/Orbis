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
    pub formula: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateSchemeInput {
    pub name: String,
    pub description: Option<String>,
    pub max_score: f64,
    pub formula: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateSchemeInput {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub max_score: Option<f64>,
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
    pub period_id: String,
    pub student_id: String,
    pub enrollment_id: Option<String>,
    pub class_subject_id: String,
    pub component_id: Option<String>,
    pub score: Decimal,
    pub max_score: Option<Decimal>,
    #[serde(default)]
    pub is_excused: bool,
    #[serde(default)]
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
    pub student_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CalculateGradesInput {
    pub scheme_id: String,
    pub student_id: String,
    pub class_subject_id: Option<String>,
}

#[derive(Serialize)]
pub struct CalculationResultDto {
    pub final_score: Decimal,
    pub message: String,
}

#[derive(Deserialize, Default)]
pub struct GradeFilter {
    pub student_id: Option<String>,
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
        formula: s.formula,
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
        formula: s.formula,
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
        formula: s.formula,
    }))
}

#[tauri::command]
pub async fn assessment_update_scheme(request: UpdateSchemeInput, state: State<'_, AppState>) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&request.id).map_err(|_| "Invalid ID")?;
    let s = state.assessment_service.update_assessment_scheme(
        &ctx,
        ctx.tenant_id,
        uid,
        request.name,
        request.description,
        request.max_score,
        request.formula,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(ok(AssessmentSchemeDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        name: s.name,
        description: s.description,
        max_score: s.max_score,
        formula: s.formula,
    }))
}

#[tauri::command]
pub async fn assessment_delete_scheme(id: String, state: State<'_, AppState>) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state.assessment_service.delete_assessment_scheme(&ctx, ctx.tenant_id, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
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
pub async fn assessment_list_student_attendance(
    student_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AttendanceDto>>, String> {
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let domain_filter = orbis_domain::queries::filters::AttendanceFilter {
        student_id: Some(sid),
        class_id: None,
        class_subject_id: None,
        date: None,
        status: None,
        pagination: Some(orbis_domain::queries::filters::PaginationParams {
            page: Some(1),
            page_size: Some(1000),
            sort_by: None,
            sort_dir: None,
        }),
    };
    let ctx = make_ctx(&state);
    let items = state.attendance_repo.list(ctx.tenant_id, domain_filter).await.map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(|a| AttendanceDto {
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

    Ok(ok(CalculationResultDto {
        final_score,
        message: "Calculated successfully".to_string(),
    }))
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
    pub student_id: String,
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

#[derive(Deserialize)]
pub struct ScoreItemInput {
    pub id: String,
    pub score: Decimal,
}

#[derive(Deserialize)]
pub struct CalculateRankingsInput {
    pub items: Vec<ScoreItemInput>,
    pub tie_policy: Option<String>,
}

#[derive(Serialize)]
pub struct RankingItemDto {
    pub id: String,
    pub rank: u32,
}

#[tauri::command]
pub async fn assessment_calculate_rankings(
    request: CalculateRankingsInput,
) -> Result<ApiResponse<Vec<RankingItemDto>>, String> {
    use orbis_domain::entities::academic_policy::TiePolicy;
    use orbis_app::services::calculation_engine::AcademicCalculationEngine;

    let tie_policy = match request.tie_policy.as_deref() {
        Some("dense") => TiePolicy::Dense,
        Some("fractional") => TiePolicy::Fractional,
        _ => TiePolicy::StandardCompetition,
    };

    let mut scores = Vec::new();
    for item in request.items {
        let uid = Uuid::parse_str(&item.id).map_err(|_| "Invalid student ID in score items")?;
        scores.push((uid, item.score));
    }

    let ranked = AcademicCalculationEngine::compute_rankings(&scores, tie_policy);
    let dtos = ranked
        .into_iter()
        .map(|(id, rank)| RankingItemDto {
            id: id.to_string(),
            rank: rank as u32,
        })
        .collect();

    Ok(ok(dtos))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BehaviorRecordDto {
    pub id: String,
    pub student_id: String,
    pub org_id: String,
    pub reporter_id: String,
    pub record_type: String,
    pub description: String,
    pub points: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
pub struct RecordBehaviorInput {
    pub student_id: String,
    pub record_type: String,
    pub description: String,
    pub points: i32,
}

fn behavior_to_dto(r: orbis_domain::entities::behavior_record::BehaviorRecord) -> BehaviorRecordDto {
    BehaviorRecordDto {
        id: r.id.to_string(),
        student_id: r.student_id.to_string(),
        org_id: r.org_id.to_string(),
        reporter_id: r.reporter_id.to_string(),
        record_type: r.record_type,
        description: r.description,
        points: r.points,
        created_at: r.created_at.to_rfc3339(),
        updated_at: r.updated_at.to_rfc3339(),
    }
}

#[tauri::command]
pub async fn assessment_record_behavior(
    request: RecordBehaviorInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BehaviorRecordDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let record = state.assessment_service
        .record_behavior(&ctx, ctx.tenant_id, student_id, request.record_type, request.description, request.points)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(behavior_to_dto(record)))
}

#[tauri::command]
pub async fn assessment_list_student_behavior_records(
    student_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<BehaviorRecordDto>>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid studentId")?;
    let records = state.assessment_service
        .list_student_behavior_records(&ctx, sid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(records.into_iter().map(behavior_to_dto).collect()))
}

