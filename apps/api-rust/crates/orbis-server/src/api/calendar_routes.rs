//! Calendar routes — Academic Years & Periods.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;

// ── DTOs ────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AcademicYearDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub status: String,
    pub current_period_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AcademicPeriodDto {
    pub id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub period_type: String,
    pub period_kind: String,
    pub status: String,
    pub is_locked: bool,
    pub is_enrollment_open: bool,
    pub is_grade_entry_open: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, ToSchema)]
pub struct AcademicContextResponse {
    pub current_year: Option<AcademicYearDto>,
    pub all_years: Vec<AcademicYearDto>,
    pub periods: Vec<AcademicPeriodDto>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateYearRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateYearRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct CreatePeriodRequest {
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String,
    pub period_kind: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdatePeriodRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub period_kind: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct PeriodTemplateDto {
    pub id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub institution_type: String,
    pub structure: serde_json::Value,
    pub is_default: bool,
}

// ── Helpers ─────────────────────────────────────────────────────

fn year_to_dto(y: orbis_domain::entities::academic_year::AcademicYear) -> AcademicYearDto {
    AcademicYearDto {
        id: y.id,
        org_id: y.org_id,
        name: y.name,
        name_ar: y.name_ar,
        start_date: y.start_date,
        end_date: y.end_date,
        is_active: y.status == "active",
        status: y.status,
        current_period_id: y.current_period_id,
        notes: y.notes,
        created_at: y.created_at,
        updated_at: y.updated_at,
    }
}

fn period_to_dto(p: orbis_domain::entities::academic_period::AcademicPeriod) -> AcademicPeriodDto {
    AcademicPeriodDto {
        id: p.id,
        year_id: p.year_id,
        parent_id: p.parent_id,
        name: p.name,
        name_ar: p.name_ar,
        code: p.code,
        start_date: p.start_date,
        end_date: p.end_date,
        period_type: p.period_type.to_string(),
        period_kind: p.period_kind.to_string(),
        status: p.state.to_string(),
        is_locked: p.is_locked,
        is_enrollment_open: p.is_enrollment_open,
        is_grade_entry_open: p.is_grade_entry_open,
        created_at: p.created_at,
        updated_at: p.updated_at,
    }
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/academic-years", request_body = CreateYearRequest,
    responses((status = 201, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn create_year(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<CreateYearRequest>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let year = state
        .calendar_service
        .create_year(
            &ctx,
            ctx.tenant_id,
            payload.name,
            payload.name_ar,
            payload.start_date,
            payload.end_date,
        )
        .await?;
    Ok(ApiResponse::ok(year_to_dto(year)))
}

#[utoipa::path(get, path = "/api/v1/academic-years",
    responses((status = 200, body = Vec<AcademicYearDto>)),
    security(("bearer" = [])), tag = "Calendar")]
async fn list_years(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AcademicYearDto>>>, ApiError> {
    let years = state.calendar_service.list_years(ctx.tenant_id).await?;
    let dtos = years.into_iter().map(year_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/academic-years/current",
    responses((status = 200, body = AcademicContextResponse)),
    security(("bearer" = [])), tag = "Calendar")]
async fn get_current_context(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<AcademicContextResponse>>, ApiError> {
    let all_years = state.calendar_service.list_years(ctx.tenant_id).await?;
    let current = state
        .calendar_service
        .get_current_year(ctx.tenant_id)
        .await?;
    let periods = state
        .calendar_service
        .list_periods_by_org(ctx.tenant_id)
        .await?;

    let year_dtos: Vec<AcademicYearDto> = all_years.into_iter().map(year_to_dto).collect();
    let period_dtos: Vec<AcademicPeriodDto> = periods.into_iter().map(period_to_dto).collect();
    let current_dto = current.map(year_to_dto);

    Ok(ApiResponse::ok(AcademicContextResponse {
        current_year: current_dto,
        all_years: year_dtos,
        periods: period_dtos,
    }))
}

#[utoipa::path(get, path = "/api/v1/academic-years/{id}",
    params(("id" = Uuid, Path, description = "Academic Year ID")),
    responses((status = 200, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn get_year(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let y = state
        .calendar_service
        .get_year(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Academic year {} not found", id)))?;
    Ok(ApiResponse::ok(year_to_dto(y)))
}

#[utoipa::path(put, path = "/api/v1/academic-years/{id}",
    params(("id" = Uuid, Path, description = "Academic Year ID")),
    request_body = UpdateYearRequest,
    responses((status = 200, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn update_year(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateYearRequest>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let y = state
        .calendar_service
        .update_year(
            &ctx,
            id,
            payload.name,
            payload.name_ar,
            payload.start_date,
            payload.end_date,
            payload.notes,
        )
        .await?;
    Ok(ApiResponse::ok(year_to_dto(y)))
}

#[utoipa::path(post, path = "/api/v1/academic-years/{id}/activate",
    params(("id" = Uuid, Path, description = "Academic Year ID")),
    responses((status = 200, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn activate_year(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let y = state.calendar_service.activate_year(&ctx, id).await?;
    Ok(ApiResponse::ok(year_to_dto(y)))
}

#[utoipa::path(post, path = "/api/v1/academic-years/{id}/close",
    params(("id" = Uuid, Path, description = "Academic Year ID")),
    responses((status = 200, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn close_year_handler(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let y = state.calendar_service.close_year(&ctx, id).await?;
    Ok(ApiResponse::ok(year_to_dto(y)))
}

#[utoipa::path(post, path = "/api/v1/academic-years/{id}/archive",
    params(("id" = Uuid, Path, description = "Academic Year ID")),
    responses((status = 200, body = AcademicYearDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn archive_year_handler(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicYearDto>>, ApiError> {
    let y = state.calendar_service.archive_year(&ctx, id).await?;
    Ok(ApiResponse::ok(year_to_dto(y)))
}

#[utoipa::path(post, path = "/api/v1/academic-periods", request_body = CreatePeriodRequest,
    responses((status = 201, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn create_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(payload): Json<CreatePeriodRequest>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state
        .calendar_service
        .create_period(
            &ctx,
            ctx.tenant_id,
            payload.year_id,
            payload.parent_id,
            payload.name,
            payload.name_ar,
            payload.code,
            payload.period_type,
            payload.period_kind,
            payload.start_date,
            payload.end_date,
        )
        .await?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(get, path = "/api/v1/academic-periods",
    responses((status = 200, body = Vec<AcademicPeriodDto>)),
    security(("bearer" = [])), tag = "Calendar")]
async fn list_periods(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<AcademicPeriodDto>>>, ApiError> {
    let periods = state
        .calendar_service
        .list_periods_by_org(ctx.tenant_id)
        .await?;
    let dtos = periods.into_iter().map(period_to_dto).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/academic-periods/{id}",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    responses((status = 200, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn get_period(
    _ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state
        .period_repo
        .find_by_id(id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Academic period {} not found", id)))?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(put, path = "/api/v1/academic-periods/{id}",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    request_body = UpdatePeriodRequest,
    responses((status = 200, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn update_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePeriodRequest>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state
        .calendar_service
        .update_period(
            &ctx,
            id,
            payload.name,
            payload.name_ar,
            payload.code,
            payload.start_date,
            payload.end_date,
            payload.period_kind,
        )
        .await?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(delete, path = "/api/v1/academic-periods/{id}",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "Calendar")]
async fn delete_period_handler(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.calendar_service.delete_period(&ctx, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(post, path = "/api/v1/academic-periods/{id}/lock",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    responses((status = 200, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn lock_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state.calendar_service.lock_period(&ctx, id).await?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(post, path = "/api/v1/academic-periods/{id}/unlock",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    responses((status = 200, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn unlock_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state.calendar_service.unlock_period(&ctx, id).await?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(post, path = "/api/v1/academic-periods/{id}/close",
    params(("id" = Uuid, Path, description = "Academic Period ID")),
    responses((status = 200, body = AcademicPeriodDto)),
    security(("bearer" = [])), tag = "Calendar")]
async fn close_period(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<AcademicPeriodDto>>, ApiError> {
    let p = state.calendar_service.close_period(&ctx, id).await?;
    Ok(ApiResponse::ok(period_to_dto(p)))
}

#[utoipa::path(get, path = "/api/v1/period-templates",
    responses((status = 200, body = Vec<PeriodTemplateDto>)),
    security(("bearer" = [])), tag = "Calendar")]
async fn list_period_templates(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<PeriodTemplateDto>>>, ApiError> {
    let templates = state
        .period_template_repo
        .find_by_org(ctx.tenant_id)
        .await?;
    let dtos = templates
        .into_iter()
        .map(|t| PeriodTemplateDto {
            id: t.id,
            name: t.name,
            name_ar: t.name_ar,
            institution_type: t.institution_type,
            structure: t.structure,
            is_default: t.is_default,
        })
        .collect();
    Ok(ApiResponse::ok(dtos))
}

// ── Router ──────────────────────────────────────────────────────

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/academic-years", post(create_year).get(list_years))
        .route("/api/v1/academic-years/current", get(get_current_context))
        .route("/api/v1/academic-years/:id", get(get_year).put(update_year))
        .route("/api/v1/academic-years/:id/activate", post(activate_year))
        .route("/api/v1/academic-years/:id/close", post(close_year_handler))
        .route("/api/v1/academic-years/:id/archive", post(archive_year_handler))
        .route(
            "/api/v1/academic-periods",
            post(create_period).get(list_periods),
        )
        .route(
            "/api/v1/academic-periods/:id",
            get(get_period).put(update_period).delete(delete_period_handler),
        )
        .route("/api/v1/academic-periods/:id/lock", post(lock_period))
        .route("/api/v1/academic-periods/:id/unlock", post(unlock_period))
        .route("/api/v1/academic-periods/:id/close", post(close_period))
        .route("/api/v1/period-templates", get(list_period_templates))
}
