use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::enrollment,
    params::EnrollmentParams,
    service::enrollment::{
        CreateEnrollmentRequest, WithdrawRequest,
        TransferRequest, PromoteRequest, PromotionResult,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_enrollments(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    params: EnrollmentParams,
) -> CmdResult<Vec<enrollment::Model>> {
    state
        .service
        .get_enrollments(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<enrollment::Model> {
    state
        .service
        .get_enrollment(&enrollment_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_enrollment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateEnrollmentRequest,
) -> CmdResult<enrollment::Model> {
    state
        .service
        .create_enrollment(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn withdraw_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
    request: WithdrawRequest,
) -> CmdResult<enrollment::Model> {
    state
        .service
        .withdraw_enrollment(&enrollment_id, &request.reason)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn transfer_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
    request: TransferRequest,
) -> CmdResult<enrollment::Model> {
    state
        .service
        .transfer_enrollment(&enrollment_id, &request.new_class_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn promote_students(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: PromoteRequest,
) -> CmdResult<PromotionResult> {
    state
        .service
        .promote_students(&org_id, request)
        .await
        .map_err(CommandError::from)
}
