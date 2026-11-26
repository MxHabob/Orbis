//! Academic and Financial Approval Override Workflow routes.

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use super::extractors::AuthContext;
use super::{ApiError, ApiResponse};
use crate::state::AppState;
use orbis_app::services::SubmitOverrideInput;
use orbis_domain::entities::override_request::{AcademicOverrideRequest, OverrideType};
use orbis_domain::queries::filters::PaginationParams;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApprovalStepDto {
    pub step_order: u32,
    pub approver_role: String,
    pub approver_id: Option<Uuid>,
    pub approver_name: Option<String>,
    pub status: String,
    pub decision_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct OverrideRequestDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub override_type: String,
    pub risk_level: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
    pub requested_by: Uuid,
    pub requested_by_name: Option<String>,
    pub status: String,
    pub current_approver_role: Option<String>,
    pub approval_chain: Vec<ApprovalStepDto>,
    pub resolved_by: Option<Uuid>,
    pub resolved_by_name: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<AcademicOverrideRequest> for OverrideRequestDto {
    fn from(r: AcademicOverrideRequest) -> Self {
        Self {
            id: r.id,
            org_id: r.org_id,
            override_type: r.override_type.to_string(),
            risk_level: r.risk_level.to_string(),
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            field_name: r.field_name,
            old_value: r.old_value,
            new_value: r.new_value,
            reason: r.reason,
            requested_by: r.requested_by,
            requested_by_name: r.requested_by_name,
            status: r.status.to_string(),
            current_approver_role: r.current_approver_role,
            approval_chain: r
                .approval_chain
                .into_iter()
                .map(|s| ApprovalStepDto {
                    step_order: s.step_order,
                    approver_role: s.approver_role,
                    approver_id: s.approver_id,
                    approver_name: s.approver_name,
                    status: s.status.to_string(),
                    decision_at: s.decision_at,
                    notes: s.notes,
                })
                .collect(),
            resolved_by: r.resolved_by,
            resolved_by_name: r.resolved_by_name,
            resolved_at: r.resolved_at,
            resolution_notes: r.resolution_notes,
            expires_at: r.expires_at,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct SubmitOverrideRequest {
    pub override_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub period_id: Option<Uuid>,
    pub field_name: Option<String>,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub reason: String,
}

#[derive(Deserialize, ToSchema)]
pub struct ResolveOverrideRequest {
    pub notes: Option<String>,
}

#[derive(Deserialize, utoipa::IntoParams)]
pub struct OverrideQuery {
    pub status: Option<String>,
    pub role: Option<String>,
    pub search: Option<String>,
    pub mine: Option<bool>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedOverrideRequestsDto {
    pub items: Vec<OverrideRequestDto>,
    pub total_count: u64,
    pub page: u64,
    pub page_size: u64,
    pub total_pages: u64,
}

// ── Handlers ────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/overrides", request_body = SubmitOverrideRequest,
    responses((status = 201, body = OverrideRequestDto)),
    security(("bearer" = [])), tag = "Workflow")]
pub async fn submit_override(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<SubmitOverrideRequest>,
) -> Result<Json<ApiResponse<OverrideRequestDto>>, ApiError> {
    let input = SubmitOverrideInput {
        override_type: p
            .override_type
            .parse::<OverrideType>()
            .map_err(ApiError::bad_request)?,
        entity_type: p.entity_type,
        entity_id: p.entity_id,
        period_id: p.period_id,
        field_name: p.field_name,
        old_value: p.old_value,
        new_value: p.new_value,
        reason: p.reason,
    };

    let req = state.workflow_service.submit_override(&ctx, input).await?;

    Ok(ApiResponse::ok(req.into()))
}

#[utoipa::path(get, path = "/api/v1/overrides",
    params(OverrideQuery),
    responses((status = 200, body = PaginatedOverrideRequestsDto)),
    security(("bearer" = [])), tag = "Workflow")]
pub async fn list_overrides(
    ctx: AuthContext,
    State(state): State<AppState>,
    Query(q): Query<OverrideQuery>,
) -> Result<Json<ApiResponse<PaginatedOverrideRequestsDto>>, ApiError> {
    let reqs = state
        .workflow_service
        .list_overrides_paginated(
            &ctx,
            q.status.as_deref(),
            q.role.as_deref(),
            q.search.as_deref(),
            q.mine.unwrap_or(false),
            PaginationParams {
                page: q.page,
                page_size: q.page_size,
                ..PaginationParams::default()
            },
        )
        .await?;
    Ok(ApiResponse::ok(PaginatedOverrideRequestsDto {
        items: reqs
            .items
            .into_iter()
            .map(OverrideRequestDto::from)
            .collect(),
        total_count: reqs.total_count,
        page: reqs.page,
        page_size: reqs.page_size,
        total_pages: reqs.total_pages,
    }))
}

#[utoipa::path(get, path = "/api/v1/overrides/{id}",
    params(("id" = Uuid, Path, description = "Override Request ID")),
    responses((status = 200, body = OverrideRequestDto)),
    security(("bearer" = [])), tag = "Workflow")]
pub async fn get_override(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<OverrideRequestDto>>, ApiError> {
    let req = state
        .workflow_service
        .get_override(&ctx, id)
        .await?
        .ok_or_else(|| ApiError::not_found(format!("Override request {} not found", id)))?;
    Ok(ApiResponse::ok(req.into()))
}

#[utoipa::path(post, path = "/api/v1/overrides/{id}/approve", request_body = ResolveOverrideRequest,
    responses((status = 200, body = OverrideRequestDto)),
    security(("bearer" = [])), tag = "Workflow")]
pub async fn approve_override(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<ResolveOverrideRequest>,
) -> Result<Json<ApiResponse<OverrideRequestDto>>, ApiError> {
    let req = state
        .workflow_service
        .approve_override(&ctx, id, p.notes)
        .await?;
    Ok(ApiResponse::ok(req.into()))
}

#[utoipa::path(post, path = "/api/v1/overrides/{id}/reject", request_body = ResolveOverrideRequest,
    responses((status = 200, body = OverrideRequestDto)),
    security(("bearer" = [])), tag = "Workflow")]
pub async fn reject_override(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<ResolveOverrideRequest>,
) -> Result<Json<ApiResponse<OverrideRequestDto>>, ApiError> {
    let req = state
        .workflow_service
        .reject_override(&ctx, id, p.notes.unwrap_or_default())
        .await?;
    Ok(ApiResponse::ok(req.into()))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/overrides",
            post(submit_override).get(list_overrides),
        )
        .route("/api/v1/overrides/:id", get(get_override))
        .route("/api/v1/overrides/:id/approve", post(approve_override))
        .route("/api/v1/overrides/:id/reject", post(reject_override))
}
