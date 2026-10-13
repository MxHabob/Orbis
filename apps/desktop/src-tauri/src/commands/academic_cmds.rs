//! Tauri IPC commands — Academic (Subjects, Classes, SectionSubjects, Prerequisites).

use super::make_ctx;
use super::org_cmds::ApiResponse;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;
fn ok<T: Serialize>(data: T) -> ApiResponse<T> {
    ApiResponse { data }
}

use orbis_app::services::academic_service::{
    AssignSectionSubjectInput as ServiceAssignSectionSubjectInput,
    CreateSectionInput as ServiceCreateSectionInput,
    CreateSubjectInput as ServiceCreateSubjectInput,
    UpdateSectionInput as ServiceUpdateSectionInput,
    UpdateSectionSubjectInput as ServiceUpdateSectionSubjectInput,
    UpdateSubjectInput as ServiceUpdateSubjectInput,
};
use orbis_domain::queries::filters::{SectionFilter, SectionSubjectFilter, SubjectFilter};
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
    pub subject_type: String,
    pub credit_hours: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_core: bool,
    pub is_elective: bool,
    pub is_lab_required: bool,
    pub max_students_per_section: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateSubjectInput {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub unit_id: Option<String>,
    pub description: Option<String>,
    pub subject_type: Option<String>,
    pub credit_hours: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    #[serde(default)]
    pub is_core: bool,
    pub is_elective: Option<bool>,
    pub is_lab_required: Option<bool>,
    pub max_students_per_section: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateSubjectInput {
    pub id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub unit_id: Option<String>,
    pub description: Option<String>,
    pub subject_type: Option<String>,
    pub credit_hours: Option<Decimal>,
    pub workload_units: Option<Decimal>,
    pub theory_hours: Option<Decimal>,
    pub practical_hours: Option<Decimal>,
    pub is_core: Option<bool>,
    pub is_elective: Option<bool>,
    pub is_lab_required: Option<bool>,
    pub max_students_per_section: Option<u32>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct SectionDto {
    pub id: String,
    pub org_id: String,
    pub unit_id: String,
    pub period_id: String,
    pub section_type: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub track: Option<String>,
    pub homeroom_teacher_id: Option<String>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub room_id: Option<String>,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateSectionInput {
    pub unit_id: Option<String>,
    pub period_id: Option<String>,
    pub section_type: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSectionInput {
    pub id: String,
    pub unit_id: Option<String>,
    pub section_type: Option<String>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Serialize)]
pub struct SectionSubjectDto {
    pub id: String,
    pub org_id: String,
    pub class_id: String,
    pub section_id: String,
    pub subject_id: String,
    pub period_id: String,
    pub primary_teacher_id: String,
    pub instructor_id: Option<String>,
    pub assistant_instructor_id: Option<String>,
    pub substitute_instructor_id: Option<String>,
    pub scheme_id: Option<String>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct AssignSectionSubjectInput {
    pub class_id: Option<String>,
    pub section_id: Option<String>,
    pub subject_id: String,
    pub period_id: String,
    pub primary_teacher_id: Option<String>,
    pub instructor_id: Option<String>,
    pub assistant_instructor_id: Option<String>,
    pub substitute_instructor_id: Option<String>,
    pub scheme_id: Option<String>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateSectionSubjectInput {
    pub id: String,
    pub instructor_id: Option<String>,
    pub assistant_instructor_id: Option<String>,
    pub substitute_instructor_id: Option<String>,
    pub scheme_id: Option<String>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
    pub is_active: Option<bool>,
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

// ── Helpers ─────────────────────────────────────────────────────

fn subject_dto(s: orbis_domain::entities::subject::Subject) -> SubjectDto {
    SubjectDto {
        id: s.id.to_string(),
        org_id: s.org_id.to_string(),
        unit_id: s.unit_id.map(|u| u.to_string()),
        code: s.code,
        name: s.name,
        name_ar: s.name_ar,
        description: s.description,
        subject_type: s.subject_type,
        credit_hours: s.credit_units,
        workload_units: s.workload_units,
        theory_hours: s.theory_hours,
        practical_hours: s.practical_hours,
        is_core: s.is_core,
        is_elective: s.is_elective,
        is_lab_required: s.is_lab_required,
        max_students_per_section: s.max_students_per_section,
        color: s.color,
        icon: s.icon,
        sort_order: s.sort_order,
        is_active: s.is_active,
        created_at: s.created_at.to_rfc3339(),
        updated_at: s.updated_at.to_rfc3339(),
    }
}

fn section_dto(c: orbis_domain::entities::section::Section) -> SectionDto {
    SectionDto {
        id: c.id.to_string(),
        org_id: c.org_id.to_string(),
        unit_id: c.org_unit_id.map(|u| u.to_string()).unwrap_or_default(),
        period_id: String::new(),
        section_type: c.section_type.clone(),
        code: c.code.clone(),
        name: c.name,
        name_ar: c.name_ar,
        level: c.level.unwrap_or_default(),
        division: c.track.clone(),
        track: c.track,
        homeroom_teacher_id: None,
        capacity: c.capacity,
        current_students: None,
        room_id: None,
        color: c.color,
        is_active: c.is_active,
        created_at: c.created_at.to_rfc3339(),
        updated_at: c.updated_at.to_rfc3339(),
    }
}

fn section_subject_dto(
    cs: orbis_domain::entities::section_subject::SectionSubject,
) -> SectionSubjectDto {
    let instructor_str = cs.instructor_id.map(|t| t.to_string());
    SectionSubjectDto {
        id: cs.id.to_string(),
        org_id: cs.org_id.to_string(),
        class_id: cs.section_id.to_string(),
        section_id: cs.section_id.to_string(),
        subject_id: cs.subject_id.to_string(),
        period_id: cs.period_id.to_string(),
        primary_teacher_id: instructor_str.clone().unwrap_or_default(),
        instructor_id: instructor_str,
        assistant_instructor_id: cs.assistant_instructor_id.map(|u| u.to_string()),
        substitute_instructor_id: cs.substitute_instructor_id.map(|u| u.to_string()),
        scheme_id: cs.scheme_id.map(|u| u.to_string()),
        weekly_hours: cs.weekly_hours,
        theory_hours: cs.theory_hours,
        practical_hours: cs.practical_hours,
        assessment_config: cs.assessment_config,
        grade_groups: cs.grade_groups,
        capacity: cs.capacity,
        status: cs.status,
        is_active: cs.is_active,
        created_at: cs.created_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
        updated_at: cs.updated_at.map(|t| t.to_rfc3339()).unwrap_or_default(),
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[tauri::command]
pub async fn academic_create_subject(
    request: CreateSubjectInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SubjectDto>, String> {
    let ctx = make_ctx(&state);
    let unit_id = request.unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let s = state
        .academic_service
        .create_subject(
            &ctx,
            ctx.tenant_id,
            ServiceCreateSubjectInput {
                unit_id,
                code: request.code,
                name: request.name,
                name_ar: request.name_ar,
                description: request.description,
                subject_type: request.subject_type,
                credit_hours: request.credit_hours,
                workload_units: request.workload_units,
                theory_hours: request.theory_hours,
                practical_hours: request.practical_hours,
                is_core: Some(request.is_core),
                is_elective: request.is_elective,
                is_lab_required: request.is_lab_required,
                max_students_per_section: request.max_students_per_section,
                color: request.color,
                icon: request.icon,
                sort_order: request.sort_order,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(subject_dto(s)))
}

#[tauri::command]
pub async fn academic_update_subject(
    request: UpdateSubjectInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SubjectDto>, String> {
    let ctx = make_ctx(&state);
    let id = Uuid::parse_str(&request.id).map_err(|_| "Invalid subject ID")?;
    let unit_id = request.unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let s = state
        .academic_service
        .update_subject(
            &ctx,
            id,
            ServiceUpdateSubjectInput {
                unit_id,
                code: request.code,
                name: request.name,
                name_ar: request.name_ar,
                description: request.description,
                subject_type: request.subject_type,
                credit_hours: request.credit_hours,
                workload_units: request.workload_units,
                theory_hours: request.theory_hours,
                practical_hours: request.practical_hours,
                is_core: request.is_core,
                is_elective: request.is_elective,
                is_lab_required: request.is_lab_required,
                max_students_per_section: request.max_students_per_section,
                color: request.color,
                icon: request.icon,
                sort_order: request.sort_order,
                is_active: request.is_active,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(subject_dto(s)))
}

#[tauri::command]
pub async fn academic_list_subjects(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<SubjectDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .academic_service
        .list_subjects(ctx.tenant_id, SubjectFilter::default())
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(subject_dto).collect()))
}

#[tauri::command]
pub async fn academic_get_subject(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SubjectDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let s = state
        .academic_service
        .get_subject(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Subject not found")?;
    Ok(ok(subject_dto(s)))
}

#[tauri::command]
pub async fn academic_create_section(
    request: CreateSectionInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionDto>, String> {
    let ctx = make_ctx(&state);
    let unit_id = request.unit_id.and_then(|u| Uuid::parse_str(&u).ok());
    let period_id = request.period_id.and_then(|p| Uuid::parse_str(&p).ok());

    let c = state
        .academic_service
        .create_section(
            &ctx,
            ctx.tenant_id,
            ServiceCreateSectionInput {
                unit_id,
                period_id,
                section_type: request.section_type,
                code: request.code,
                name: request.name,
                name_ar: request.name_ar,
                level: request.level,
                track: request.track,
                capacity: request.capacity,
                color: request.color,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(section_dto(c)))
}

#[tauri::command]
pub async fn academic_update_section(
    request: UpdateSectionInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionDto>, String> {
    let ctx = make_ctx(&state);
    let id = Uuid::parse_str(&request.id).map_err(|_| "Invalid section ID")?;
    let unit_id = request.unit_id.and_then(|u| Uuid::parse_str(&u).ok());

    let c = state
        .academic_service
        .update_section(
            &ctx,
            id,
            ServiceUpdateSectionInput {
                unit_id,
                section_type: request.section_type,
                code: request.code,
                name: request.name,
                name_ar: request.name_ar,
                level: request.level,
                track: request.track,
                capacity: request.capacity,
                color: request.color,
                is_active: request.is_active,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(section_dto(c)))
}

#[tauri::command]
pub async fn academic_list_sections(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<SectionDto>>, String> {
    let ctx = make_ctx(&state);
    let items = state
        .academic_service
        .list_sections(ctx.tenant_id, SectionFilter::default())
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(section_dto).collect()))
}

#[tauri::command]
pub async fn academic_get_section(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let c = state
        .academic_service
        .get_section(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Section not found")?;
    Ok(ok(section_dto(c)))
}

#[tauri::command]
pub async fn academic_assign_section_subject(
    request: AssignSectionSubjectInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionSubjectDto>, String> {
    let ctx = make_ctx(&state);
    let section_id_str = request
        .section_id
        .or(request.class_id)
        .ok_or("section_id or class_id required")?;
    let section_id = Uuid::parse_str(&section_id_str).map_err(|_| "Invalid section_id")?;
    let subject_id = Uuid::parse_str(&request.subject_id).map_err(|_| "Invalid subject_id")?;
    let period_id = Uuid::parse_str(&request.period_id).map_err(|_| "Invalid period_id")?;

    let instructor_id = request
        .instructor_id
        .or(request.primary_teacher_id)
        .and_then(|t| Uuid::parse_str(&t).ok());

    let assistant_instructor_id = request
        .assistant_instructor_id
        .and_then(|u| Uuid::parse_str(&u).ok());
    let substitute_instructor_id = request
        .substitute_instructor_id
        .and_then(|u| Uuid::parse_str(&u).ok());
    let scheme_id = request.scheme_id.and_then(|u| Uuid::parse_str(&u).ok());

    let cs = state
        .academic_service
        .assign_section_subject(
            &ctx,
            ServiceAssignSectionSubjectInput {
                section_id,
                subject_id,
                period_id,
                instructor_id,
                assistant_instructor_id,
                substitute_instructor_id,
                scheme_id,
                weekly_hours: request.weekly_hours,
                theory_hours: request.theory_hours,
                practical_hours: request.practical_hours,
                assessment_config: request.assessment_config,
                grade_groups: request.grade_groups,
                capacity: request.capacity,
                status: request.status,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(section_subject_dto(cs)))
}

#[tauri::command]
pub async fn academic_get_section_subject(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionSubjectDto>, String> {
    let uid = Uuid::parse_str(&id).map_err(|_| "Invalid ID")?;
    let cs = state
        .academic_service
        .get_section_subject(uid)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("SectionSubject not found")?;
    Ok(ok(section_subject_dto(cs)))
}

#[tauri::command]
pub async fn academic_update_section_subject(
    request: UpdateSectionSubjectInput,
    state: State<'_, AppState>,
) -> Result<ApiResponse<SectionSubjectDto>, String> {
    let ctx = make_ctx(&state);
    let id = Uuid::parse_str(&request.id).map_err(|_| "Invalid SectionSubject ID")?;
    let instructor_id = request
        .instructor_id
        .and_then(|t| Uuid::parse_str(&t).ok());
    let assistant_instructor_id = request
        .assistant_instructor_id
        .and_then(|u| Uuid::parse_str(&u).ok());
    let substitute_instructor_id = request
        .substitute_instructor_id
        .and_then(|u| Uuid::parse_str(&u).ok());
    let scheme_id = request.scheme_id.and_then(|u| Uuid::parse_str(&u).ok());

    let cs = state
        .academic_service
        .update_section_subject(
            &ctx,
            id,
            ServiceUpdateSectionSubjectInput {
                instructor_id,
                assistant_instructor_id,
                substitute_instructor_id,
                scheme_id,
                weekly_hours: request.weekly_hours,
                theory_hours: request.theory_hours,
                practical_hours: request.practical_hours,
                assessment_config: request.assessment_config,
                grade_groups: request.grade_groups,
                capacity: request.capacity,
                status: request.status,
                is_active: request.is_active,
            },
        )
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(section_subject_dto(cs)))
}

#[tauri::command]
pub async fn academic_list_section_subjects(
    class_id: String,
    period_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<SectionSubjectDto>>, String> {
    let ctx = make_ctx(&state);
    let cid = Uuid::parse_str(&class_id).map_err(|_| "Invalid class ID")?;
    let pid = Uuid::parse_str(&period_id).map_err(|_| "Invalid period ID")?;
    let filter = SectionSubjectFilter {
        section_id: Some(cid),
        period_id: Some(pid),
        ..Default::default()
    };
    let items = state
        .academic_service
        .list_section_subjects(ctx.tenant_id, filter)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ok(items.items.into_iter().map(section_subject_dto).collect()))
}

#[tauri::command]
pub async fn academic_check_prerequisites(
    student_id: String,
    subject_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<PrerequisiteCheckResultDto>>, String> {
    let st_id = Uuid::parse_str(&student_id).map_err(|_| "Invalid studentId")?;
    let sub_id = Uuid::parse_str(&subject_id).map_err(|_| "Invalid subjectId")?;
    let results = state
        .academic_service
        .check_prerequisites(st_id, sub_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(results
        .into_iter()
        .map(|r| PrerequisiteCheckResultDto {
            subject_id: r.subject_id.to_string(),
            prerequisite_subject_id: r.prerequisite_subject_id.to_string(),
            requirement_type: format!("{:?}", r.requirement_type),
            is_satisfied: r.is_satisfied,
            min_score_required: r.min_score_required,
            current_score: r.current_score,
            failure_reason: r.failure_reason,
        })
        .collect()))
}
