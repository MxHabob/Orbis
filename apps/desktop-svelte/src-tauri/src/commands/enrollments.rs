use tauri::State;
use std::sync::Arc;
use serde_json;

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
) -> CmdResult<Vec<serde_json::Value>> {
    state.service().await
        .get_enrollments(&org_id, params)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_enrollment(
    state: State<'_, Arc<AppState>>,
    enrollment_id: String,
) -> CmdResult<serde_json::Value> {
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
        .promote_students(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn transition_enrollment_status(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: String,
    target_status: String,
    withdrawal_reason: Option<String>,
) -> CmdResult<enrollment::Model> {
    state.service().await
        .transition_enrollment_status(&org_id, &enrollment_id, &target_status, withdrawal_reason)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn check_student_duplicate(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    email: Option<String>,
    phone: Option<String>,
    national_id: Option<String>,
) -> CmdResult<Vec<core_domain::entities::user::Model>> {
    state.service().await
        .check_student_duplicate(&org_id, email, phone, national_id)
        .await
        .map_err(CommandError::from)
}
