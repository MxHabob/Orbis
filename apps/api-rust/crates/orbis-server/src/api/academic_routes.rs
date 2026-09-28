//! Academic routes — Subjects, Classes, ClassSubjects.

use axum::{extract::{State, Path}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::queries::filters::{SubjectFilter, ClassFilter, ClassSubjectFilter};

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
    pub credit_hours: Option<Decimal>,
    pub is_core: bool,
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
    pub unit_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub capacity: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateClassRequest {
    pub unit_id: Uuid,
    pub period_id: Uuid,
    pub name: String,
    pub level: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct ClassSubjectDto {
    pub id: Uuid,
    pub class_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub primary_teacher_id: Uuid,
    pub weekly_hours: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct AssignClassSubjectRequest {
    pub class_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub primary_teacher_id: Uuid,
    pub weekly_hours: Option<i32>,
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
    let s = state.academic_service.create_subject(
        &ctx,
        ctx.tenant_id,
        p.unit_id,
        p.code,
        p.name,
        p.credit_hours,
        p.is_core.unwrap_or(true),
    ).await?;
    Ok(ApiResponse::ok(SubjectDto {
        id: s.id, org_id: s.org_id, unit_id: s.unit_id, code: s.code, name: s.name, name_ar: s.name_ar,
        credit_hours: s.credit_hours, is_core: s.is_core, is_active: s.is_active,
        created_at: s.created_at, updated_at: s.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/subjects",
    responses((status = 200, body = Vec<SubjectDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_subjects(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<SubjectDto>>>, ApiError> {
    let subjects = state.academic_service.list_subjects(ctx.tenant_id, SubjectFilter::default()).await?;
    let dtos = subjects.items.into_iter().map(|s| SubjectDto {
        id: s.id, org_id: s.org_id, unit_id: s.unit_id, code: s.code, name: s.name, name_ar: s.name_ar,
        credit_hours: s.credit_hours, is_core: s.is_core, is_active: s.is_active,
        created_at: s.created_at, updated_at: s.updated_at,
    }).collect();
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
    let s = state.academic_service.get_subject(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Subject {} not found", id)))?;
    Ok(ApiResponse::ok(SubjectDto {
        id: s.id, org_id: s.org_id, unit_id: s.unit_id, code: s.code, name: s.name, name_ar: s.name_ar,
        credit_hours: s.credit_hours, is_core: s.is_core, is_active: s.is_active,
        created_at: s.created_at, updated_at: s.updated_at,
    }))
}

#[utoipa::path(post, path = "/api/v1/classes", request_body = CreateClassRequest,
    responses((status = 201, body = ClassDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn create_class(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateClassRequest>,
) -> Result<Json<ApiResponse<ClassDto>>, ApiError> {
    let c = state.academic_service.create_class(&ctx, ctx.tenant_id, p.unit_id, p.period_id, p.name, p.level).await?;
    Ok(ApiResponse::ok(ClassDto {
        id: c.id, org_id: c.org_id, unit_id: c.unit_id,
        period_id: c.period_id, name: c.name, name_ar: c.name_ar, level: c.level,
        division: c.division, capacity: c.capacity, is_active: c.is_active,
        created_at: c.created_at, updated_at: c.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/classes",
    responses((status = 200, body = Vec<ClassDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_classes(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<ClassDto>>>, ApiError> {
    let classes = state.academic_service.list_classes(ctx.tenant_id, ClassFilter::default()).await?;
    let dtos = classes.items.into_iter().map(|c| ClassDto {
        id: c.id, org_id: c.org_id, unit_id: c.unit_id,
        period_id: c.period_id, name: c.name, name_ar: c.name_ar, level: c.level,
        division: c.division, capacity: c.capacity, is_active: c.is_active,
        created_at: c.created_at, updated_at: c.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/classes/{id}",
    params(("id" = Uuid, Path, description = "Class ID")),
    responses((status = 200, body = ClassDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn get_class(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<ClassDto>>, ApiError> {
    let c = state.academic_service.get_class(id).await?
        .ok_or_else(|| ApiError::not_found(format!("Class {} not found", id)))?;
    Ok(ApiResponse::ok(ClassDto {
        id: c.id, org_id: c.org_id, unit_id: c.unit_id,
        period_id: c.period_id, name: c.name, name_ar: c.name_ar, level: c.level,
        division: c.division, capacity: c.capacity, is_active: c.is_active,
        created_at: c.created_at, updated_at: c.updated_at,
    }))
}

#[utoipa::path(post, path = "/api/v1/class-subjects", request_body = AssignClassSubjectRequest,
    responses((status = 201, body = ClassSubjectDto)),
    security(("bearer" = [])), tag = "Academic")]
async fn assign_class_subject(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<AssignClassSubjectRequest>,
) -> Result<Json<ApiResponse<ClassSubjectDto>>, ApiError> {
    let cs = state.academic_service.assign_class_subject(&ctx, p.class_id, p.subject_id, p.period_id, p.primary_teacher_id, p.weekly_hours).await?;
    Ok(ApiResponse::ok(ClassSubjectDto {
        id: cs.id, class_id: cs.class_id, subject_id: cs.subject_id,
        period_id: cs.period_id, primary_teacher_id: cs.primary_teacher_id,
        weekly_hours: cs.weekly_hours, is_active: cs.is_active,
        created_at: cs.created_at, updated_at: cs.updated_at,
    }))
}

#[utoipa::path(get, path = "/api/v1/class-subjects",
    params(("class_id" = Uuid, Query, description = "Class ID"), ("period_id" = Uuid, Query, description = "Period ID")),
    responses((status = 200, body = Vec<ClassSubjectDto>)),
    security(("bearer" = [])), tag = "Academic")]
async fn list_class_subjects(
    ctx: AuthContext,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<ClassSubjectQuery>,
) -> Result<Json<ApiResponse<Vec<ClassSubjectDto>>>, ApiError> {
    let filter = ClassSubjectFilter {
        class_id: Some(params.class_id),
        period_id: Some(params.period_id),
        ..Default::default()
    };
    let items = state.academic_service.list_class_subjects(ctx.tenant_id, filter).await?;
    let dtos = items.items.into_iter().map(|cs| ClassSubjectDto {
        id: cs.id, class_id: cs.class_id, subject_id: cs.subject_id,
        period_id: cs.period_id, primary_teacher_id: cs.primary_teacher_id,
        weekly_hours: cs.weekly_hours, is_active: cs.is_active,
        created_at: cs.created_at, updated_at: cs.updated_at,
    }).collect();
    Ok(ApiResponse::ok(dtos))
}

#[derive(Deserialize)]
pub struct ClassSubjectQuery {
    pub class_id: Uuid,
    pub period_id: Uuid,
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/subjects", post(create_subject).get(list_subjects))
        .route("/api/v1/subjects/:id", get(get_subject))
        .route("/api/v1/classes", post(create_class).get(list_classes))
        .route("/api/v1/classes/:id", get(get_class))
        .route("/api/v1/class-subjects", post(assign_class_subject).get(list_class_subjects))
}
