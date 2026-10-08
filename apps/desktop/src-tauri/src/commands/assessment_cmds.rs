//! Tauri IPC commands — Assessment (Schemes, Grades, Attendance, Behavior).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use chrono::NaiveDate;
use orbis_app::services::assessment_service::{BatchAttendanceItem, BatchGradeItem};
use orbis_domain::entities::assessment_scheme::{
    AssessmentComponent, AssessmentLevel, AssessmentScheme, InstitutionScope,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

#[derive(Serialize, Deserialize)]
pub struct AssessmentSchemeDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Decimal,
    pub formula: Option<serde_json::Value>,
    pub level: AssessmentLevel,
    pub institution_scope: InstitutionScope,
    pub components: Vec<AssessmentComponent>,
    pub is_default: bool,
    pub is_system: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateSchemeInput {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Decimal,
    pub formula: Option<serde_json::Value>,
    pub level: Option<AssessmentLevel>,
    pub institution_scope: Option<InstitutionScope>,
    pub components: Option<Vec<AssessmentComponent>>,
    pub is_default: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateSchemeInput {
    pub id: String,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub max_score: Option<Decimal>,
    pub formula: Option<serde_json::Value>,
    pub level: Option<AssessmentLevel>,
    pub institution_scope: Option<InstitutionScope>,
    pub components: Option<Vec<AssessmentComponent>>,
    pub is_default: Option<bool>,
}

#[derive(Serialize, Deserialize)]
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
    pub weight: Option<Decimal>,
    pub letter_grade: Option<String>,
    pub is_final: bool,
    pub is_excused: bool,
    pub is_exempt: bool,
    pub change_reason: Option<String>,
    pub recorded_by: Option<String>,
    pub updated_by: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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

#[derive(Deserialize)]
pub struct RecordGradeItemInput {
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

#[derive(Deserialize)]
pub struct BatchRecordGradesInput {
    pub items: Vec<RecordGradeItemInput>,
}

#[derive(Serialize)]
pub struct BatchRecordGradesResponseDto {
    pub grades: Vec<GradeDto>,
    pub count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AttendanceDto {
    pub id: String,
    pub student_id: String,
    pub class_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub scheduled_lesson_id: Option<String>,
    pub date: String,
    pub status: String,
    pub notes: Option<String>,
    pub marked_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
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
pub struct MarkAttendanceItemInput {
    pub student_id: String,
    pub status: String,
    pub notes: Option<String>,
    pub class_subject_id: Option<String>,
    pub scheduled_lesson_id: Option<String>,
}

#[derive(Deserialize)]
pub struct BatchMarkAttendanceInput {
    pub class_id: String,
    pub date: String,
    pub items: Vec<MarkAttendanceItemInput>,
}

#[derive(Serialize)]
pub struct BatchMarkAttendanceResponseDto {
    pub attendances: Vec<AttendanceDto>,
    pub count: usize,
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

#[derive(Serialize, Deserialize)]
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

fn grade_to_dto(g: orbis_domain::entities::grade::Grade) -> GradeDto {
    GradeDto {
        id: g.id.to_string(),
        org_id: g.org_id.to_string(),
        period_id: g.period_id.to_string(),
        student_id: g.student_id.to_string(),
        enrollment_id: g.enrollment_id.map(|e| e.to_string()),
        class_subject_id: g.class_subject_id.to_string(),
        component_id: g.component_id.map(|c| c.to_string()),
        score: g.score,
        max_score: g.max_score,
        weight: g.weight,
        letter_grade: g.letter_grade,
        is_final: g.is_final,
        is_excused: g.is_excused,
        is_exempt: g.is_exempt,
        change_reason: g.change_reason,
        recorded_by: g.recorded_by.map(|u| u.to_string()),
        updated_by: g.updated_by.map(|u| u.to_string()),
        notes: g.notes,
        created_at: g.created_at.to_rfc3339(),
        updated_at: g.updated_at.to_rfc3339(),
    }
}

fn attendance_to_dto(a: orbis_domain::entities::attendance::Attendance) -> AttendanceDto {
    AttendanceDto {
        id: a.id.to_string(),
        student_id: a.student_id.to_string(),
        class_id: a.class_id.map(|c| c.to_string()),
        class_subject_id: a.class_subject_id.map(|c| c.to_string()),
        scheduled_lesson_id: a.scheduled_lesson_id.map(|s| s.to_string()),
        date: a.attendance_date.to_string(),
        status: a.status,
        notes: a.notes,
        marked_by: a.marked_by.map(|u| u.to_string()),
        created_at: a.created_at.to_rfc3339(),
        updated_at: a.updated_at.to_rfc3339(),
    }
}

fn behavior_to_dto(
    r: orbis_domain::entities::behavior_record::BehaviorRecord,
) -> BehaviorRecordDto {
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

fn scheme_to_dto(s: AssessmentScheme) -> AssessmentSchemeDto {
    AssessmentSchemeDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        max_score: s.max_score,
        formula: s.formula,
        level: s.level,
        institution_scope: s.institution_scope,
        components: s.components,
        is_default: s.is_default,
        is_system: s.is_system,
        created_at: s.created_at.to_rfc3339(),
        updated_at: s.updated_at.to_rfc3339(),
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[tauri::command]
pub async fn assessment_create_scheme(
    request: CreateSchemeInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let ctx = make_ctx(&state);
    let s = state
        .assessment_service
        .create_assessment_scheme(
            &ctx,
            ctx.tenant_id,
            request.name,
            request.name_ar,
            request.description,
            request.max_score,
            request.formula,
            request.level,
            request.institution_scope,
            request.components,
            request.is_default.unwrap_or(false),
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(scheme_to_dto(s)))
}

#[tauri::command]
pub async fn assessment_list_schemes(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AssessmentSchemeDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .assessment_service
        .list_assessment_schemes(ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(scheme_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_get_scheme(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state
        .assessment_scheme_repo
        .find_by_id(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Scheme not found")?;
    Ok(ok(scheme_to_dto(s)))
}

#[tauri::command]
pub async fn assessment_update_scheme(
    request: UpdateSchemeInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AssessmentSchemeDto>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&request.id).map_err(|_| "Invalid ID")?;
    let s = state
        .assessment_service
        .update_assessment_scheme(
            &ctx,
            ctx.tenant_id,
            uid,
            request.name,
            request.name_ar,
            request.description,
            request.max_score,
            request.formula,
            request.level,
            request.institution_scope,
            request.components,
            request.is_default,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(scheme_to_dto(s)))
}

#[tauri::command]
pub async fn assessment_delete_scheme(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    state
        .assessment_service
        .delete_assessment_scheme(&ctx, ctx.tenant_id, uid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn assessment_record_grade(
    request: RecordGradeInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<GradeDto>, String> {
    let ctx = make_ctx(&state);
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_subject_id =
        Uuid::parse_str(&request.class_subject_id).map_err(|_| "Invalid classSubjectId")?;
    let component_id = request.component_id.and_then(|c| Uuid::parse_str(&c).ok());
    let max_score = request.max_score.unwrap_or_else(|| Decimal::from(100));

    let g = state
        .assessment_service
        .record_grade(
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

    Ok(ok(grade_to_dto(g)))
}

#[tauri::command]
pub async fn assessment_batch_record_grades(
    request: BatchRecordGradesInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BatchRecordGradesResponseDto>, String> {
    let ctx = make_ctx(&state);
    let mut items = Vec::with_capacity(request.items.len());

    for item in request.items {
        let period_id = Uuid::parse_str(&item.period_id).map_err(|_| "Invalid periodId")?;
        let student_id = Uuid::parse_str(&item.student_id).map_err(|_| "Invalid studentId")?;
        let enrollment_id = item.enrollment_id.and_then(|e| Uuid::parse_str(&e).ok());
        let class_subject_id =
            Uuid::parse_str(&item.class_subject_id).map_err(|_| "Invalid classSubjectId")?;
        let component_id = item.component_id.and_then(|c| Uuid::parse_str(&c).ok());

        items.push(BatchGradeItem {
            period_id,
            student_id,
            enrollment_id,
            class_subject_id,
            component_id,
            score: item.score,
            max_score: item.max_score,
            is_excused: item.is_excused,
            is_exempt: item.is_exempt,
            notes: item.notes,
        });
    }

    let grades = state
        .assessment_service
        .batch_record_grades(&ctx, ctx.tenant_id, items)
        .await
        .map_err(|e| e.to_string())?;

    let dtos: Vec<GradeDto> = grades.into_iter().map(grade_to_dto).collect();
    let count = dtos.len();

    Ok(ok(BatchRecordGradesResponseDto {
        grades: dtos,
        count,
    }))
}

#[tauri::command]
pub async fn assessment_delete_grade(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let grade_id = Uuid::parse_str(&id).map_err(|_| "Invalid grade ID")?;
    state
        .assessment_service
        .delete_grade(&ctx, ctx.tenant_id, grade_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn assessment_list_grades(
    filter: Option<GradeFilter>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<GradeDto>>, String> {
    let f = filter.unwrap_or_default();
    let items = if let Some(sid) = f
        .student_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        state.grade_repo.find_by_student(sid).await
    } else if let Some(csid) = f
        .class_subject_id
        .as_deref()
        .and_then(|s| Uuid::parse_str(s).ok())
    {
        state.grade_repo.find_by_class_subject(csid).await
    } else {
        Ok(vec![])
    }
    .map_err(|e| e.to_string())?;

    Ok(ok(items.into_iter().map(grade_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_calculate_grades(
    request: CalculateGradesInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<CalculationResultDto>, String> {
    let ctx = make_ctx(&state);
    let scheme_id = Uuid::parse_str(&request.scheme_id).map_err(|_| "Invalid schemeId")?;
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_subject_id = request
        .class_subject_id
        .and_then(|cs| Uuid::parse_str(&cs).ok())
        .unwrap_or_else(Uuid::nil);

    let final_score = state
        .assessment_service
        .calculate_scheme_grades(&ctx, scheme_id, student_id, class_subject_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(CalculationResultDto {
        final_score,
        message: "Calculated successfully".to_string(),
    }))
}

#[tauri::command]
pub async fn assessment_calculate_gpa(
    request: CalculateGpaInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<CumulativeGpaRecordDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;

    let record = state
        .assessment_service
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

#[tauri::command]
pub async fn assessment_calculate_rankings(
    request: CalculateRankingsInput,
) -> Result<ApiResponse<Vec<RankingItemDto>>, String> {
    use orbis_app::services::calculation_engine::AcademicCalculationEngine;
    use orbis_domain::entities::academic_policy::TiePolicy;

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

#[tauri::command]
pub async fn assessment_mark_attendance(
    request: MarkAttendanceInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<AttendanceDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let class_id = Uuid::parse_str(&request.class_id).map_err(|_| "Invalid classId")?;
    let date = NaiveDate::parse_from_str(&request.date, "%Y-%m-%d")
        .map_err(|_| "Invalid date format, use YYYY-MM-DD")?;

    let a = state
        .assessment_service
        .mark_attendance(
            &ctx,
            ctx.tenant_id,
            Uuid::nil(),
            student_id,
            Some(class_id),
            None,
            date,
            request.status,
            request.notes,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(attendance_to_dto(a)))
}

#[tauri::command]
pub async fn assessment_batch_mark_attendance(
    request: BatchMarkAttendanceInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BatchMarkAttendanceResponseDto>, String> {
    let ctx = make_ctx(&state);
    let class_id = Uuid::parse_str(&request.class_id).map_err(|_| "Invalid classId")?;
    let date = NaiveDate::parse_from_str(&request.date, "%Y-%m-%d")
        .map_err(|_| "Invalid date format, use YYYY-MM-DD")?;

    let mut items = Vec::with_capacity(request.items.len());
    for item in request.items {
        let student_id = Uuid::parse_str(&item.student_id).map_err(|_| "Invalid studentId")?;
        let class_subject_id = item
            .class_subject_id
            .and_then(|cs| Uuid::parse_str(&cs).ok());
        let scheduled_lesson_id = item
            .scheduled_lesson_id
            .and_then(|sl| Uuid::parse_str(&sl).ok());

        items.push(BatchAttendanceItem {
            student_id,
            status: item.status,
            notes: item.notes,
            class_subject_id,
            scheduled_lesson_id,
        });
    }

    let list = state
        .assessment_service
        .batch_mark_attendance(&ctx, ctx.tenant_id, class_id, date, items)
        .await
        .map_err(|e| e.to_string())?;

    let dtos: Vec<AttendanceDto> = list.into_iter().map(attendance_to_dto).collect();
    let count = dtos.len();

    Ok(ok(BatchMarkAttendanceResponseDto {
        attendances: dtos,
        count,
    }))
}

#[tauri::command]
pub async fn assessment_delete_attendance(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let attendance_id = Uuid::parse_str(&id).map_err(|_| "Invalid attendance ID")?;
    state
        .assessment_service
        .delete_attendance(&ctx, ctx.tenant_id, attendance_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn assessment_list_attendance(
    class_id: String,
    date: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AttendanceDto>>, String> {
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let parsed_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|_| "Invalid date format, use YYYY-MM-DD")?;
    let items = state
        .attendance_repo
        .find_by_class_and_date(cid, parsed_date)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(attendance_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_list_student_attendance(
    student_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<AttendanceDto>>, String> {
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let ctx = make_ctx(&state);
    let items = state
        .assessment_service
        .list_student_attendance(&ctx, sid)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.into_iter().map(attendance_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_record_behavior(
    request: RecordBehaviorInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<BehaviorRecordDto>, String> {
    let ctx = make_ctx(&state);
    let student_id = Uuid::parse_str(&request.student_id).map_err(|_| "Invalid studentId")?;
    let record = state
        .assessment_service
        .record_behavior(
            &ctx,
            ctx.tenant_id,
            student_id,
            request.record_type,
            request.description,
            request.points,
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(behavior_to_dto(record)))
}

#[tauri::command]
pub async fn assessment_delete_behavior(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<()>, String> {
    let ctx = make_ctx(&state);
    let behavior_id = Uuid::parse_str(&id).map_err(|_| "Invalid behavior record ID")?;
    state
        .assessment_service
        .delete_behavior(&ctx, ctx.tenant_id, behavior_id)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(()))
}

#[tauri::command]
pub async fn assessment_list_student_behavior_records(
    student_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<BehaviorRecordDto>>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid studentId")?;
    let records = state
        .assessment_service
        .list_student_behavior_records(&ctx, sid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(records.into_iter().map(behavior_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_list_all_behavior(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<BehaviorRecordDto>>, String> {
    let ctx = make_ctx(&state);
    let records = state
        .assessment_service
        .list_behavior_records(&ctx, ctx.tenant_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(records.into_iter().map(behavior_to_dto).collect()))
}

#[tauri::command]
pub async fn assessment_get_student_term_results(
    student_id: String,
    period_id: String,
    class_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<orbis_domain::entities::academic_policy::StudentTermReportCard>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let cid = class_id.and_then(|c| Uuid::parse_str(&c).ok());

    let report = state
        .assessment_service
        .calculate_student_term_report_card(&ctx, ctx.tenant_id, sid, pid, cid, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(report))
}

#[tauri::command]
pub async fn assessment_get_student_annual_results(
    student_id: String,
    year_id: String,
    class_id: Option<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<orbis_domain::entities::academic_policy::StudentAnnualReportCard>, String> {
    let ctx = make_ctx(&state);
    let sid = Uuid::parse_str(&student_id).map_err(|_| "Invalid student ID")?;
    let yid = Uuid::parse_str(&year_id).map_err(|_| "Invalid year ID")?;
    let cid = class_id.and_then(|c| Uuid::parse_str(&c).ok());

    let report = state
        .assessment_service
        .calculate_student_annual_report_card(&ctx, ctx.tenant_id, sid, yid, cid, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(report))
}

#[tauri::command]
pub async fn assessment_get_class_term_results(
    class_id: String,
    period_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<orbis_domain::entities::academic_policy::StudentTermReportCard>>, String>
{
    let ctx = make_ctx(&state);
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;

    let reports = state
        .assessment_service
        .calculate_class_term_report_cards(&ctx, ctx.tenant_id, cid, pid, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(reports))
}

#[tauri::command]
pub async fn assessment_get_class_annual_results(
    class_id: String,
    year_id: String,
    state: State<'_, AppState>,
) -> Result<
    ApiResponse<Vec<orbis_domain::entities::academic_policy::StudentAnnualReportCard>>,
    String,
> {
    let ctx = make_ctx(&state);
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let yid = Uuid::parse_str(&year_id).map_err(|_| "Invalid year ID")?;

    let reports = state
        .assessment_service
        .calculate_class_annual_report_cards(&ctx, ctx.tenant_id, cid, yid, None)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(reports))
}
