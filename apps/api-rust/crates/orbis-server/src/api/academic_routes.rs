//! Academic routes — Subjects, Classes, SectionSubjects.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::academic_service::{
    AssignSectionSubjectInput, CreateClassInput, CreateSubjectInput, UpdateClassInput,
    UpdateSectionSubjectInput, UpdateSubjectInput,
};
use orbis_domain::queries::filters::{ClassFilter, SectionSubjectFilter, SubjectFilter};

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SubjectDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSubjectRequest {
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
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
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSubjectRequest {
    pub unit_id: Option<Uuid>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub name_ar: Option<String>,
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

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ClassDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub section_type: String,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub division: Option<String>,
    pub track: Option<String>,
    pub homeroom_teacher_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub room_id: Option<Uuid>,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateClassRequest {
    pub unit_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub section_type: Option<String>,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub track: Option<String>,
    pub capacity: Option<i32>,
    pub color: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateClassRequest {
    pub unit_id: Option<Uuid>,
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

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SectionSubjectDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: String,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignSectionSubjectRequest {
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSectionSubjectRequest {
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: Option<String>,
    pub is_active: Option<bool>,
}

// ── Helpers ─────────────────────────────────────────────────────

fn subject_to_dto(s: orbis_domain::entities::subject::Subject) -> SubjectDto {
    SubjectDto {
        id: s.id,
        org_id: s.org_id,
        unit_id: s.unit_id,
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
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}

fn section_to_dto(c: orbis_domain::entities::section::Section) -> ClassDto {
    ClassDto {
        id: c.id,
        org_id: c.org_id,
        unit_id: c.org_unit_id,
        period_id: None,
        section_type: c.section_type.clone(),
        code: c.code.clone(),
        name: c.name,
        name_ar: c.name_ar,
        level: c.level,
        division: c.track.clone(),
        track: c.track,
        homeroom_teacher_id: None,
        capacity: c.capacity,
        current_students: None,
        room_id: None,
        color: c.color,
        is_active: c.is_active,
        created_at: c.created_at,
        updated_at: c.updated_at,
    }
}

fn section_subject_to_dto(
    cs: orbis_domain::entities::section_subject::SectionSubject,
) -> SectionSubjectDto {
    SectionSubjectDto {
        id: cs.id,
        org_id: cs.org_id,
        section_id: cs.section_id,
        subject_id: cs.subject_id,
        period_id: cs.period_id,
        instructor_id: cs.instructor_id,
        assistant_instructor_id: cs.assistant_instructor_id,
        substitute_instructor_id: cs.substitute_instructor_id,
        scheme_id: cs.scheme_id,
        weekly_hours: cs.weekly_hours,
        theory_hours: cs.theory_hours,
        practical_hours: cs.practical_hours,
        assessment_config: cs.assessment_config,
        grade_groups: cs.grade_groups,
        capacity: cs.capacity,
        status: cs.status,
        is_active: cs.is_active,
        created_at: cs.created_at,
        updated_at: cs.updated_at,
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/subjects", request_body = CreateSubjectRequest,
    responses((status = 201, body = SubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn create_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateSubjectRequest>,
) -> Result<Json<ApiResponse<SubjectDto>>, ApiError> {
    let s = state
        .academic_service
        .create_subject(
            &ctx,
            ctx.tenant_id,
            CreateSubjectInput {
                unit_id: p.unit_id,
                code: p.code,
                name: p.name,
                name_ar: p.name_ar,
                description: p.description,
                subject_type: p.subject_type,
                credit_hours: p.credit_hours,
                workload_units: p.workload_units,
                theory_hours: p.theory_hours,
                practical_hours: p.practical_hours,
                is_core: p.is_core,
                is_elective: p.is_elective,
                is_lab_required: p.is_lab_required,
                max_students_per_section: p.max_students_per_section,
                color: p.color,
                icon: p.icon,
                sort_order: p.sort_order,
            },
        )
        .await?;
    Ok(ApiResponse::ok(subject_to_dto(s)))
}

#[utoipa::path(put, path = "/api/v1/subjects/{id}",
    params(("id" = Uuid, Path, description = "Subject ID")),
    request_body = UpdateSubjectRequest,
    responses((status = 200, body = SubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn update_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateSubjectRequest>,
) -> Result<Json<ApiResponse<SubjectDto>>, ApiError> {
    let s = state
        .academic_service
        .update_subject(
            &ctx,
            id,
            UpdateSubjectInput {
                unit_id: p.unit_id,
                code: p.code,
                name: p.name,
                name_ar: p.name_ar,
                description: p.description,
                subject_type: p.subject_type,
                credit_hours: p.credit_hours,
                workload_units: p.workload_units,
                theory_hours: p.theory_hours,
                practical_hours: p.practical_hours,
                is_core: p.is_core,
                is_elective: p.is_elective,
                is_lab_required: p.is_lab_required,
                max_students_per_section: p.max_students_per_section,
                color: p.color,
                icon: p.icon,
                sort_order: p.sort_order,
                is_active: p.is_active,
            },
        )
        .await?;
    Ok(ApiResponse::ok(subject_to_dto(s)))
}

#[utoipa::path(get, path = "/api/v1/subjects",
    responses((status = 200, body = Vec<SubjectDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_subjects(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<SubjectDto>>>, ApiError> {
    let subjects = state
        .academic_service
        .list_subjects(ctx.tenant_id, SubjectFilter::default())
        .await?;
    let dtos = subjects.items.into_iter().map(subject_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/subjects/{id}",
    params(("id" = Uuid, Path, description = "Subject ID")),
    responses((status = 200, body = SubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn get_subject(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SubjectDto>>, ApiError> {
    let s = state
        .academic_service
        .get_subject(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Subject {} not found", id)))?;
    Ok(ApiResponse::ok(subject_to_dto(s)))
}

#[utoipa::path(post, path = "/api/v1/classes", request_body = CreateClassRequest,
    responses((status = 201, body = ClassDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn create_class(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateClassRequest>,
) -> Result<Json<ApiResponse<ClassDto>>, ApiError> {
    let c = state
        .academic_service
        .create_class(
            &ctx,
            ctx.tenant_id,
            CreateClassInput {
                unit_id: p.unit_id,
                period_id: p.period_id,
                section_type: p.section_type,
                code: p.code,
                name: p.name,
                name_ar: p.name_ar,
                level: p.level,
                track: p.track,
                capacity: p.capacity,
                color: p.color,
            },
        )
        .await?;
    Ok(ApiResponse::ok(section_to_dto(c)))
}

#[utoipa::path(put, path = "/api/v1/classes/{id}",
    params(("id" = Uuid, Path, description = "Class ID")),
    request_body = UpdateClassRequest,
    responses((status = 200, body = ClassDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn update_class(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateClassRequest>,
) -> Result<Json<ApiResponse<ClassDto>>, ApiError> {
    let c = state
        .academic_service
        .update_class(
            &ctx,
            id,
            UpdateClassInput {
                unit_id: p.unit_id,
                section_type: p.section_type,
                code: p.code,
                name: p.name,
                name_ar: p.name_ar,
                level: p.level,
                track: p.track,
                capacity: p.capacity,
                color: p.color,
                is_active: p.is_active,
            },
        )
        .await?;
    Ok(ApiResponse::ok(section_to_dto(c)))
}

#[utoipa::path(get, path = "/api/v1/classes",
    responses((status = 200, body = Vec<ClassDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_classes(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ClassDto>>>, ApiError> {
    let classes = state
        .academic_service
        .list_classes(ctx.tenant_id, ClassFilter::default())
        .await?;
    let dtos = classes.items.into_iter().map(section_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/classes/{id}",
    params(("id" = Uuid, Path, description = "Section ID")),
    responses((status = 200, body = ClassDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn get_class(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ClassDto>>, ApiError> {
    let c = state
        .academic_service
        .get_class(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Section {} not found", id)))?;
    Ok(ApiResponse::ok(section_to_dto(c)))
}

#[utoipa::path(post, path = "/api/v1/section-subjects", request_body = AssignSectionSubjectRequest,
    responses((status = 201, body = SectionSubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn assign_section_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignSectionSubjectRequest>,
) -> Result<Json<ApiResponse<SectionSubjectDto>>, ApiError> {
    let cs = state
        .academic_service
        .assign_section_subject(
            &ctx,
            AssignSectionSubjectInput {
                section_id: p.section_id,
                subject_id: p.subject_id,
                period_id: p.period_id,
                instructor_id: p.instructor_id,
                assistant_instructor_id: p.assistant_instructor_id,
                substitute_instructor_id: p.substitute_instructor_id,
                scheme_id: p.scheme_id,
                weekly_hours: p.weekly_hours,
                theory_hours: p.theory_hours,
                practical_hours: p.practical_hours,
                assessment_config: p.assessment_config,
                grade_groups: p.grade_groups,
                capacity: p.capacity,
                status: p.status,
            },
        )
        .await?;
    Ok(ApiResponse::ok(section_subject_to_dto(cs)))
}

#[utoipa::path(get, path = "/api/v1/section-subjects/{id}",
    params(("id" = Uuid, Path, description = "SectionSubject ID")),
    responses((status = 200, body = SectionSubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn get_section_subject(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<SectionSubjectDto>>, ApiError> {
    let cs = state
        .academic_service
        .get_section_subject(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("SectionSubject {} not found", id)))?;
    Ok(ApiResponse::ok(section_subject_to_dto(cs)))
}

#[utoipa::path(put, path = "/api/v1/section-subjects/{id}",
    params(("id" = Uuid, Path, description = "SectionSubject ID")),
    request_body = UpdateSectionSubjectRequest,
    responses((status = 200, body = SectionSubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn update_section_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateSectionSubjectRequest>,
) -> Result<Json<ApiResponse<SectionSubjectDto>>, ApiError> {
    let cs = state
        .academic_service
        .update_section_subject(
            &ctx,
            id,
            UpdateSectionSubjectInput {
                instructor_id: p.instructor_id,
                assistant_instructor_id: p.assistant_instructor_id,
                substitute_instructor_id: p.substitute_instructor_id,
                scheme_id: p.scheme_id,
                weekly_hours: p.weekly_hours,
                theory_hours: p.theory_hours,
                practical_hours: p.practical_hours,
                assessment_config: p.assessment_config,
                grade_groups: p.grade_groups,
                capacity: p.capacity,
                status: p.status,
                is_active: p.is_active,
            },
        )
        .await?;
    Ok(ApiResponse::ok(section_subject_to_dto(cs)))
}

#[utoipa::path(get, path = "/api/v1/section-subjects",
    params(("section_id" = Uuid, Query, description = "Section ID"), ("period_id" = Uuid, Query, description = "Period ID")),
    responses((status = 200, body = Vec<SectionSubjectDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_section_subjects(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<SectionSubjectQuery>,
) -> Result<Json<ApiResponse<Vec<SectionSubjectDto>>>, ApiError> {
    let filter = SectionSubjectFilter {
        section_id: Some(params.section_id),
        period_id: Some(params.period_id),
        ..Default::default()
    };
    let items = state
        .academic_service
        .list_section_subjects(ctx.tenant_id, filter)
        .await?;
    let dtos = items.items.into_iter().map(section_subject_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[derive(Deserialize)]
pub struct SectionSubjectQuery {
    pub section_id: Uuid,
    pub period_id: Uuid,
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/subjects", post(create_subject).get(list_subjects))
        .route(
            "/api/v1/subjects/:id",
            get(get_subject).put(update_subject),
        )
        .route("/api/v1/classes", post(create_class).get(list_classes))
        .route("/api/v1/classes/:id", get(get_class).put(update_class))
        .route(
            "/api/v1/section-subjects",
            post(assign_section_subject).get(list_section_subjects),
        )
        .route(
            "/api/v1/section-subjects/:id",
            get(get_section_subject).put(update_section_subject),
        )
        // Backward-compatibility alias
        .route(
            "/api/v1/Section-subjects",
            post(assign_section_subject).get(list_section_subjects),
        )
}
