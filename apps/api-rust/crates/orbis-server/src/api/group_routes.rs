//! User Groups routes — CRUD, Member management, Bulk assignment, Permission matrix.

use axum::{extract::{State, Path}, routing::{get, post, put, delete}, Json, Router};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{ApiError, ApiResponse};
use super::extractors::AuthContext;
use crate::state::AppState;
use orbis_domain::entities::group::{GroupDetail, GroupMemberInfo};
use orbis_app::services::group_service::{
    CreateGroupInput, UpdateGroupInput, BulkAssignGroupInput
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GroupDetailDto {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub status: String,
    pub member_count: usize,
    pub permissions: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<GroupDetail> for GroupDetailDto {
    fn from(d: GroupDetail) -> Self {
        Self {
            id: d.group.id,
            org_id: d.group.org_id,
            name: d.group.name,
            name_ar: d.group.name_ar,
            description: d.group.description,
            is_system: d.group.is_system,
            status: d.group.status,
            member_count: d.member_count,
            permissions: d.permissions,
            created_at: d.group.created_at,
            updated_at: d.group.updated_at,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct GroupMemberInfoDto {
    pub user_id: Uuid,
    pub person_id: Uuid,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub user_status: String,
    pub joined_at: DateTime<Utc>,
}

impl From<GroupMemberInfo> for GroupMemberInfoDto {
    fn from(m: GroupMemberInfo) -> Self {
        Self {
            user_id: m.user_id,
            person_id: m.person_id,
            full_name: m.full_name,
            full_name_ar: m.full_name_ar,
            email: m.email,
            phone: m.phone,
            avatar_url: m.avatar_url,
            user_status: m.user_status,
            joined_at: m.joined_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateGroupRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Deserialize, ToSchema)]
pub struct AddGroupMemberRequest {
    pub user_id: Uuid,
}

#[derive(Deserialize, ToSchema)]
pub struct BulkAssignGroupRequest {
    pub group_id: Uuid,
    pub user_ids: Vec<Uuid>,
}

#[derive(Deserialize, ToSchema)]
pub struct SetGroupPermissionsRequest {
    pub permissions: Vec<String>,
}

// ── Handlers ─────────────────────────────────────────────────────

#[utoipa::path(post, path = "/api/v1/groups",
    request_body = CreateGroupRequest,
    responses((status = 200, body = GroupDetailDto)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn create_group(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<CreateGroupRequest>,
) -> Result<Json<ApiResponse<GroupDetailDto>>, ApiError> {
    let org_id = ctx.tenant_id;
    let input = CreateGroupInput {
        name: p.name,
        name_ar: p.name_ar,
        description: p.description,
        permissions: p.permissions,
    };
    let detail = state.group_service.create_group(&ctx, org_id, input).await?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(get, path = "/api/v1/groups",
    responses((status = 200, body = Vec<GroupDetailDto>)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn list_groups(
    ctx: AuthContext,
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<GroupDetailDto>>>, ApiError> {
    let org_id = ctx.tenant_id;
    let groups = state.group_service.list_groups(&ctx, org_id).await?;
    let dtos = groups.into_iter().map(Into::into).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(get, path = "/api/v1/groups/:id",
    responses((status = 200, body = GroupDetailDto)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn get_group(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<GroupDetailDto>>, ApiError> {
    let detail = state.group_service.get_group(&ctx, id).await?
        .ok_or_else(|| ApiError::not_found(format!("Group {} not found", id)))?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(patch, path = "/api/v1/groups/:id",
    request_body = UpdateGroupRequest,
    responses((status = 200, body = GroupDetailDto)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn update_group(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateGroupRequest>,
) -> Result<Json<ApiResponse<GroupDetailDto>>, ApiError> {
    let input = UpdateGroupInput {
        name: p.name,
        name_ar: p.name_ar,
        description: p.description,
        status: p.status,
        permissions: p.permissions,
    };
    let detail = state.group_service.update_group(&ctx, id, input).await?;
    Ok(ApiResponse::ok(detail.into()))
}

#[utoipa::path(delete, path = "/api/v1/groups/:id",
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn delete_group(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.group_service.delete_group(&ctx, id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(get, path = "/api/v1/groups/:id/members",
    responses((status = 200, body = Vec<GroupMemberInfoDto>)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn get_group_members(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<GroupMemberInfoDto>>>, ApiError> {
    let members = state.group_service.get_group_members(&ctx, id).await?;
    let dtos = members.into_iter().map(Into::into).collect();
    Ok(ApiResponse::ok(dtos))
}

#[utoipa::path(post, path = "/api/v1/groups/:id/members",
    request_body = AddGroupMemberRequest,
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn add_group_member(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<AddGroupMemberRequest>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.group_service.add_member(&ctx, id, p.user_id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(delete, path = "/api/v1/groups/:id/members/:user_id",
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn remove_group_member(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path((id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.group_service.remove_member(&ctx, id, user_id).await?;
    Ok(ApiResponse::ok(true))
}

#[utoipa::path(post, path = "/api/v1/groups/bulk-assign",
    request_body = BulkAssignGroupRequest,
    responses((status = 200, body = u64)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn bulk_assign_group(
    ctx: AuthContext,
    State(state): State<AppState>,
    Json(p): Json<BulkAssignGroupRequest>,
) -> Result<Json<ApiResponse<u64>>, ApiError> {
    let count = state.group_service.bulk_assign_group(&ctx, BulkAssignGroupInput {
        group_id: p.group_id,
        user_ids: p.user_ids,
    }).await?;
    Ok(ApiResponse::ok(count))
}

#[utoipa::path(put, path = "/api/v1/groups/:id/permissions",
    request_body = SetGroupPermissionsRequest,
    responses((status = 200, body = bool)),
    security(("bearer" = [])), tag = "IAM")]
pub async fn set_group_permissions(
    ctx: AuthContext,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<SetGroupPermissionsRequest>,
) -> Result<Json<ApiResponse<bool>>, ApiError> {
    state.group_service.set_permissions(&ctx, id, p.permissions).await?;
    Ok(ApiResponse::ok(true))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/v1/groups", post(create_group).get(list_groups))
        .route("/api/v1/groups/bulk-assign", post(bulk_assign_group))
        .route("/api/v1/groups/:id", get(get_group).patch(update_group).delete(delete_group))
        .route("/api/v1/groups/:id/members", get(get_group_members).post(add_group_member))
        .route("/api/v1/groups/:id/members/:user_id", delete(remove_group_member))
        .route("/api/v1/groups/:id/permissions", put(set_group_permissions))
}
