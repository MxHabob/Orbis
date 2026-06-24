use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{ledger_account, treasury, student_fee, payment, fee_template, journal_entry, discount_template},
    service::finance::{
        CreateLedgerAccountRequest, CreateTreasuryRequest,
        AssignStudentFeeRequest, RecordPaymentRequest, CreateFeeTemplateRequest,
        CreateDiscountTemplateRequest,
    },
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_ledger_accounts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<ledger_account::Model>> {
    state
        .service
        .get_ledger_accounts(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_ledger_account(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateLedgerAccountRequest,
) -> CmdResult<ledger_account::Model> {
    state
        .service
        .create_ledger_account(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_treasuries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<treasury::Model>> {
    state
        .service
        .get_treasuries(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_treasury(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateTreasuryRequest,
) -> CmdResult<treasury::Model> {
    state
        .service
        .create_treasury(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_student_fees(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: Option<String>,
) -> CmdResult<Vec<student_fee::Model>> {
    state
        .service
        .get_student_fees(&org_id, enrollment_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_active_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<fee_template::Model>> {
    state
        .service
        .get_active_fee_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<fee_template::Model>> {
    state
        .service
        .get_fee_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateFeeTemplateRequest,
) -> CmdResult<fee_template::Model> {
    state
        .service
        .create_fee_template(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: CreateFeeTemplateRequest,
) -> CmdResult<fee_template::Model> {
    state
        .service
        .update_fee_template(&org_id, &id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_fee_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<u64> {
    state
        .service
        .delete_fee_template(&org_id, &id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn assign_student_fee(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: AssignStudentFeeRequest,
) -> CmdResult<student_fee::Model> {
    state
        .service
        .assign_student_fee(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_payments(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    enrollment_id: Option<String>,
) -> CmdResult<Vec<payment::Model>> {
    state
        .service
        .get_payments(&org_id, enrollment_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn record_payment(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    created_by: String,
    request: RecordPaymentRequest,
) -> CmdResult<payment::Model> {
    state
        .service
        .record_payment(&org_id, request, &created_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_journal_entries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<journal_entry::Model>> {
    state
        .service
        .get_journal_entries(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_active_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<discount_template::Model>> {
    state
        .service
        .get_active_discount_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<discount_template::Model>> {
    state
        .service
        .get_discount_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateDiscountTemplateRequest,
) -> CmdResult<discount_template::Model> {
    state
        .service
        .create_discount_template(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
    request: CreateDiscountTemplateRequest,
) -> CmdResult<discount_template::Model> {
    state
        .service
        .update_discount_template(&org_id, &id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_discount_template(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    id: String,
) -> CmdResult<u64> {
    state
        .service
        .delete_discount_template(&org_id, &id)
        .await
        .map_err(CommandError::from)
}
