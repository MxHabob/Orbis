//! Tauri IPC commands — User Groups (CRUD, Members, Bulk Assignment, Permissions).

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use uuid::Uuid;
use orbis_domain::entities::group::{GroupDetail, GroupMemberInfo};
use orbis_app::services::group_service::{
    CreateGroupInput, UpdateGroupInput, BulkAssignGroupInput
};
use super::make_ctx;
use super::org_cmds::ApiResponse;

fn ok<T: Serialize>(data: T) -> ApiResponse<T> { ApiResponse { data } }

// ── DTOs ─────────────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupDetailDto {
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub status: String,
    pub member_count: usize,
    pub permissions: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub fn group_detail_to_dto(d: GroupDetail) -> GroupDetailDto {
    GroupDetailDto {
        id: d.group.id.to_string(),
        org_id: d.group.org_id.to_string(),
        name: d.group.name,
        name_ar: d.group.name_ar,
        description: d.group.description,
        is_system: d.group.is_system,
        status: d.group.status,
        member_count: d.member_count,
        permissions: d.permissions,
        created_at: d.group.created_at.to_rfc3339(),
        updated_at: d.group.updated_at.to_rfc3339(),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GroupMemberInfoDto {
    pub user_id: String,
    pub person_id: String,
    pub full_name: String,
    pub full_name_ar: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub user_status: String,
    pub joined_at: String,
}

pub fn member_to_dto(m: GroupMemberInfo) -> GroupMemberInfoDto {
    GroupMemberInfoDto {
        user_id: m.user_id.to_string(),
        person_id: m.person_id.to_string(),
        full_name: m.full_name,
        full_name_ar: m.full_name_ar,
        email: m.email,
        phone: m.phone,
        avatar_url: m.avatar_url,
        user_status: m.user_status,
        joined_at: m.joined_at.to_rfc3339(),
    }
}

// ── Request Inputs ───────────────────────────────────────────────

#[derive(Deserialize, Debug)]
pub struct CreateGroupRequest {
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct BulkAssignGroupRequest {
    pub group_id: String,
    pub user_ids: Vec<String>,
}

// ── Commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn group_create_group(
    request: CreateGroupRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<GroupDetailDto>, String> {
    let ctx = make_ctx(&state);
    let org_id = ctx.tenant_id;

    let input = CreateGroupInput {
        name: request.name,
        name_ar: request.name_ar,
        description: request.description,
        permissions: request.permissions,
    };

    let detail = state.group_service.create_group(&ctx, org_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(group_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn group_list_groups(
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<GroupDetailDto>>, String> {
    let ctx = make_ctx(&state);
    let org_id = ctx.tenant_id;

    let groups = state.group_service.list_groups(&ctx, org_id)
        .await
        .map_err(|e| e.to_string())?;

    let dtos = groups.into_iter().map(group_detail_to_dto).collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn group_get_group(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<GroupDetailDto>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let detail = state.group_service.get_group(&ctx, group_id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Group {} not found", id))?;

    Ok(ok(group_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn group_update_group(
    id: String,
    request: UpdateGroupRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<GroupDetailDto>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let input = UpdateGroupInput {
        name: request.name,
        name_ar: request.name_ar,
        description: request.description,
        status: request.status,
        permissions: request.permissions,
    };

    let detail = state.group_service.update_group(&ctx, group_id, input)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(group_detail_to_dto(detail)))
}

#[tauri::command]
pub async fn group_delete_group(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state.group_service.delete_group(&ctx, group_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}

#[tauri::command]
pub async fn group_get_members(
    id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<Vec<GroupMemberInfoDto>>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    let members = state.group_service.get_group_members(&ctx, group_id)
        .await
        .map_err(|e| e.to_string())?;

    let dtos = members.into_iter().map(member_to_dto).collect();
    Ok(ok(dtos))
}

#[tauri::command]
pub async fn group_add_member(
    id: String,
    user_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    state.group_service.add_member(&ctx, group_id, user_uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}

#[tauri::command]
pub async fn group_remove_member(
    id: String,
    user_id: String,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;
    let user_uuid = Uuid::parse_str(&user_id).map_err(|e| e.to_string())?;

    state.group_service.remove_member(&ctx, group_id, user_uuid)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}

#[tauri::command]
pub async fn group_bulk_assign(
    request: BulkAssignGroupRequest,
    state: State<'_, AppState>,
) -> Result<ApiResponse<u64>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&request.group_id).map_err(|e| e.to_string())?;
    let user_ids: Vec<Uuid> = request.user_ids.into_iter().filter_map(|u| Uuid::parse_str(&u).ok()).collect();

    let count = state.group_service.bulk_assign_group(&ctx, BulkAssignGroupInput {
        group_id,
        user_ids,
    }).await.map_err(|e| e.to_string())?;

    Ok(ok(count))
}

#[tauri::command]
pub async fn group_set_permissions(
    id: String,
    permissions: Vec<String>,
    state: State<'_, AppState>,
) -> Result<ApiResponse<bool>, String> {
    let ctx = make_ctx(&state);
    let group_id = Uuid::parse_str(&id).map_err(|e| e.to_string())?;

    state.group_service.set_permissions(&ctx, group_id, permissions)
        .await
        .map_err(|e| e.to_string())?;

    Ok(ok(true))
}
