use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::academic_override_request,
    service::override_approval::{
        CreateOverrideRequest,
        ApprovalAction,
        OverrideListParams,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn create_override_request(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateOverrideRequest,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .create_override_request(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn process_approval(
    state: State<'_, Arc<AppState>>,
    request_id: String,
    action: ApprovalAction,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .process_approval(&request_id, action)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn cancel_override_request(
    state: State<'_, Arc<AppState>>,
    request_id: String,
    user_id: String,
) -> CmdResult<academic_override_request::Model> {
    state.provider.ensure_writable().await.map_err(CommandError::from)?;
    state.service().await
        .cancel_override_request(&request_id, &user_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn list_override_requests(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: OverrideListParams,
) -> CmdResult<Vec<academic_override_request::Model>> {
    state.service().await
        .list_override_requests(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_override_request(
    state: State<'_, Arc<AppState>>,
    request_id: String,
) -> CmdResult<academic_override_request::Model> {
    state.service().await
        .get_override_request(&request_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_pending_approvals(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    approver_role: String,
) -> CmdResult<Vec<academic_override_request::Model>> {
    state.service().await
        .get_pending_approvals(&org_id, &approver_role)
        .await
        .map_err(CommandError::from)
}
