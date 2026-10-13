//! Academic routes — Subjects, Classes, SectionSubjects.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
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
    pub credit_hours: Option<Decimal>,
    pub is_core: bool,
    pub is_elective: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSubjectRequest {
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub unit_id: Option<Uuid>,
    pub credit_hours: Option<Decimal>,
    pub is_core: Option<bool>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ClassDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: Option<String>,
    pub division: Option<String>,
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
    pub unit_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub capacity: Option<i32>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SectionSubjectDto {
    pub id: Uuid,
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub is_active: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignSectionSubjectRequest {
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Uuid,
    pub weekly_hours: Option<i32>,
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
        credit_hours: s.credit_units,
        is_core: s.is_core,
        is_elective: s.is_elective,
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
        name: c.name,
        name_ar: c.name_ar,
        level: c.level,
        division: c.track,
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
            p.unit_id,
            p.code,
            p.name,
            p.name_ar,
            p.credit_hours,
            p.is_core.unwrap_or(true),
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
            p.unit_id,
            p.period_id,
            p.name,
            p.name_ar,
            p.level,
            p.capacity,
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

#[utoipa::path(get, path = "/api/v1/teachers/me/classes",
    params(("period_id" = Option<Uuid>, Query, description = "Active academic period ID")),
    responses((status = 200, body = Vec<ClassDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_my_teaching_classes(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<TeacherClassesQuery>,
) -> Result<Json<ApiResponse<Vec<ClassDto>>>, ApiError> {
    let Some(user_id) = ctx.user_id() else {
        return Err(ApiError::unauthorized("No user identity in token"));
    };
    let Some(period_id) = query.period_id else {
        return Ok(ApiResponse::ok(Vec::new()));
    };
    let Some(employee) = state
        .employee_service
        .employee_repo
        .find_by_user_id(ctx.tenant_id, user_id)
        .await?
    else {
        return Ok(ApiResponse::ok(Vec::new()));
    };

    let assignments = state
        .section_subject_repo
        .find_by_teacher(employee.id, period_id)
        .await?;
    let mut seen_section_ids = HashSet::new();
    let mut classes = Vec::new();

    for assignment in assignments
        .into_iter()
        .filter(|assignment| assignment.is_active)
    {
        if !seen_section_ids.insert(assignment.section_id) {
            continue;
        }
        if let Some(section) = state.class_repo.find_by_id(assignment.section_id).await? {
            if section.org_id == ctx.tenant_id {
                classes.push(section_to_dto(section));
            }
        }
    }

    Ok(ApiResponse::ok(classes))
}

#[derive(Deserialize)]
struct TeacherClassesQuery {
    period_id: Option<Uuid>,
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

#[utoipa::path(post, path = "/api/v1/Section-subjects", request_body = AssignSectionSubjectRequest,
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
            p.section_id,
            p.subject_id,
            p.period_id,
            p.instructor_id,
            p.weekly_hours,
        )
        .await?;
    Ok(ApiResponse::ok(SectionSubjectDto {
        id: cs.id,
        section_id: cs.section_id,
        subject_id: cs.subject_id,
        period_id: cs.period_id,
        instructor_id: cs.instructor_id,
        weekly_hours: cs.weekly_hours,
        is_active: cs.is_active,
        created_at: cs.created_at,
        updated_at: cs.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/Section-subjects",
    params(("class_id" = Uuid, Query, description = "Section ID"), ("period_id" = Uuid, Query, description = "Period ID")),
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
    let dtos = items
        .items
        .into_iter()
        .map(|cs| SectionSubjectDto {
            id: cs.id,
            section_id: cs.section_id,
            subject_id: cs.subject_id,
            period_id: cs.period_id,
            instructor_id: cs.instructor_id,
            weekly_hours: cs.weekly_hours,
            is_active: cs.is_active,
            created_at: cs.created_at,
            updated_at: cs.updated_at,
        })
        .collect();
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
        .route("/api/v1/subjects/:id", get(get_subject))
        .route("/api/v1/classes", post(create_class).get(list_classes))
        .route(
            "/api/v1/teachers/me/classes",
            get(list_my_teaching_classes),
        )
        .route("/api/v1/classes/:id", get(get_class))
        .route(
            "/api/v1/Section-subjects",
            post(assign_section_subject).get(list_section_subjects),
        )
}
