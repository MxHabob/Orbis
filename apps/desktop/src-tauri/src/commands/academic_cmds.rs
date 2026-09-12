//! Tauri IPC commands — Academic (Subjects, Classes, ClassSubjects, Prerequisites).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use super::make_ctx;
use super::org_cmds::ApiResponse;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

use orbis_domain::queries::filters::{SubjectFilter, ClassFilter, ClassSubjectFilter};
use rust_decimal::Decimal;

#[derive(Serialize)]
pub struct SubjectDto {
    pub id: String,
    pub org_id: String,
    pub unit_id: Option<String>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub credit_hours: Option<Decimal>,
    pub is_core: bool,
    pub is_elective: bool,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct CreateSubjectInput {
    pub code: String,
    pub name: String,
    #[serde(rename = "unitId")]
    pub unit_id: Option<String>,
    #[serde(rename = "creditHours")]
    pub credit_hours: Option<Decimal>,
    #[serde(rename = "isCore", default)]
    pub is_core: bool,
}

#[derive(Serialize)]
pub struct ClassDto {
    pub id: String,
    pub org_id: String,
    pub unit_id: String,
    pub period_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub homeroom_teacher_id: Option<String>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct CreateClassInput {
    #[serde(rename = "unitId")]
    pub unit_id: String,
    #[serde(rename = "periodId")]
    pub period_id: String,
    pub name: String,
    pub level: String,
}

#[derive(Serialize)]
pub struct ClassSubjectDto {
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub period_id: String,
    pub primary_teacher_id: String,
    pub weekly_hours: Option<i32>,
}

#[derive(Deserialize)]
pub struct AssignClassSubjectInput {
    #[serde(rename = "classId")]
    pub class_id: String,
    #[serde(rename = "subjectId")]
    pub subject_id: String,
    #[serde(rename = "periodId")]
    pub period_id: String,
    #[serde(rename = "primaryTeacherId")]
    pub primary_teacher_id: String,
    #[serde(rename = "weeklyHours")]
    pub weekly_hours: Option<i32>,
}

#[derive(Serialize)]
pub struct PrerequisiteCheckResultDto {
    pub subject_id: String,
    pub prerequisite_subject_id: String,
    pub requirement_type: String,
    pub is_satisfied: bool,
    pub min_score_required: Option<Decimal>,
    pub current_score: Option<Decimal>,
    pub failure_reason: Option<String>,
}

#[tauri::command]
pub async fn academic_create_subject(request: CreateSubjectInput, state: State<'_, AppState>) -> Result<ApiResponse<SubjectDto>, String> {
    let ctx = make_ctx(&state);
    let unit_id = request.unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let s = state.academic_service.create_subject(&ctx, ctx.tenant_id, unit_id, request.code, request.name, request.credit_hours, request.is_core)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(SubjectDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        unit_id: s.unit_id.map(|u| u.to_string()),
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        credit_hours: s.credit_hours,
        is_core: s.is_core,
        is_elective: s.is_elective,
        is_active: s.is_active,
    }))
}

#[tauri::command]
pub async fn academic_list_subjects(state: State<'_, AppState>) -> Result<ApiResponse<Vec<SubjectDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.academic_service.list_subjects(ctx.tenant_id, SubjectFilter::default()).await.map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(|s| SubjectDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        unit_id: s.unit_id.map(|u| u.to_string()),
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        credit_hours: s.credit_hours,
        is_core: s.is_core,
        is_elective: s.is_elective,
        is_active: s.is_active,
    }).collect()))
}

#[tauri::command]
pub async fn academic_get_subject(id: String, state: State<'_, AppState>) -> Result<ApiResponse<SubjectDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state.academic_service.get_subject(uid).await.map_err(|e| e.to_string())?.ok_or("Subject not found")?;
    Ok(ok(SubjectDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        unit_id: s.unit_id.map(|u| u.to_string()),
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        credit_hours: s.credit_hours,
        is_core: s.is_core,
        is_elective: s.is_elective,
        is_active: s.is_active,
    }))
}

#[tauri::command]
pub async fn academic_create_class(request: CreateClassInput, state: State<'_, AppState>) -> Result<ApiResponse<ClassDto>, String> {
    let ctx = make_ctx(&state);
    let unit_id = Uuid::parse_str(&request.unit_id).map_err(|_| "Invalid unitId")?;
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;

    let c = state.academic_service.create_class(&ctx, ctx.tenant_id, unit_id, period_id, request.name, request.level)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ClassDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        unit_id: c.unit_id.to_string(),
        period_id: c.period_id.to_string(),
        name: c.name,
        name_ar: c.name_ar,
        level: c.level,
        division: c.division,
        homeroom_teacher_id: c.homeroom_teacher_id.map(|h| h.to_string()),
        capacity: c.capacity,
        current_students: c.current_students,
        is_active: c.is_active,
    }))
}

#[tauri::command]
pub async fn academic_list_classes(state: State<'_, AppState>) -> Result<ApiResponse<Vec<ClassDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state.academic_service.list_classes(ctx.tenant_id, ClassFilter::default()).await.map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(|c| ClassDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        unit_id: c.unit_id.to_string(),
        period_id: c.period_id.to_string(),
        name: c.name,
        name_ar: c.name_ar,
        level: c.level,
        division: c.division,
        homeroom_teacher_id: c.homeroom_teacher_id.map(|h| h.to_string()),
        capacity: c.capacity,
        current_students: c.current_students,
        is_active: c.is_active,
    }).collect()))
}

#[tauri::command]
pub async fn academic_get_class(id: String, state: State<'_, AppState>) -> Result<ApiResponse<ClassDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let c = state.academic_service.get_class(uid).await.map_err(|e| e.to_string())?.ok_or("Class not found")?;
    Ok(ok(ClassDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        unit_id: c.unit_id.to_string(),
        period_id: c.period_id.to_string(),
        name: c.name,
        name_ar: c.name_ar,
        level: c.level,
        division: c.division,
        homeroom_teacher_id: c.homeroom_teacher_id.map(|h| h.to_string()),
        capacity: c.capacity,
        current_students: c.current_students,
        is_active: c.is_active,
    }))
}

#[tauri::command]
pub async fn academic_assign_class_subject(request: AssignClassSubjectInput, state: State<'_, AppState>) -> Result<ApiResponse<ClassSubjectDto>, String> {
    let ctx = make_ctx(&state);
    let class_id = Uuid::parse_str(&request.class_id).map_err(|_| "Invalid classId")?;
    let subject_id = Uuid::parse_str(&request.subject_id).map_err(|_| "Invalid subjectId")?;
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid periodId")?;
    let primary_teacher_id = Uuid::parse_str(&request.primary_teacher_id).map_err(|_| "Invalid primaryTeacherId")?;

    let cs = state.academic_service.assign_class_subject(&ctx, class_id, subject_id, period_id, primary_teacher_id, request.weekly_hours)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(ClassSubjectDto {
        id: cs.id.to_string(),
        class_id: cs.class_id.to_string(),
        subject_id: cs.subject_id.to_string(),
        period_id: cs.period_id.to_string(),
        primary_teacher_id: cs.primary_teacher_id.to_string(),
        weekly_hours: cs.weekly_hours,
    }))
}

#[tauri::command]
pub async fn academic_list_class_subjects(class_id: String, period_id: String, state: State<'_, AppState>) -> Result<ApiResponse<Vec<ClassSubjectDto>>, String> {
    let ctx = make_ctx(&state);
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let filter = ClassSubjectFilter {
        class_id: Some(cid),
        period_id: Some(pid),
        ..Default::default()
    };
    let items = state.academic_service.list_class_subjects(ctx.tenant_id, filter).await.map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(|cs| ClassSubjectDto {
        id: cs.id.to_string(),
        class_id: cs.class_id.to_string(),
        subject_id: cs.subject_id.to_string(),
        period_id: cs.period_id.to_string(),
        primary_teacher_id: cs.primary_teacher_id.to_string(),
        weekly_hours: cs.weekly_hours,
    }).collect()))
}

#[tauri::command]
pub async fn academic_check_prerequisites(
    student_id: String,
    subject_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<PrerequisiteCheckResultDto>>, String> {
    let st_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid studentId")?;
    let sub_id = Uuid::parse_str(&subject_id).map_err(|_| "Invalid subjectId")?;
    let results = state.academic_service
        .check_prerequisites(st_id, sub_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(results.into_iter().map(|r| PrerequisiteCheckResultDto {
        subject_id: r.subject_id.to_string(),
        prerequisite_subject_id: r.prerequisite_subject_id.to_string(),
        requirement_type: format!("{:?}", r.requirement_type),
        is_satisfied: r.is_satisfied,
        min_score_required: r.min_score_required,
        current_score: r.current_score,
        failure_reason: r.failure_reason,
    }).collect()))
}
