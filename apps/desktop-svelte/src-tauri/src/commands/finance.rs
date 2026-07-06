use tauri::State;
use std::sync::Arc;

use core_domain::{
    entities::{ledger_account, treasury, student_fee, payment, fee_template, journal_entry, discount_template, currency},
    service::{
        finance::{
            CreateLedgerAccountRequest, CreateTreasuryRequest,
            AssignStudentFeeRequest, RecordPaymentRequest, CreateFeeTemplateRequest,
            CreateDiscountTemplateRequest, CreateJournalEntryRequest, LedgerAccountResponse,
        },
        account::AccountStatement,
    },
    repository::currency::CurrencyUpdate,
};

use crate::state::AppState;
use super::error::{CmdResult, CommandError};

#[tauri::command]
pub async fn get_ledger_accounts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<LedgerAccountResponse>> {
    state.service().await
        .get_ledger_accounts(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_transaction_accounts(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<LedgerAccountResponse>> {
    state.service().await
        .get_transaction_accounts(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_ledger_account(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateLedgerAccountRequest,
) -> CmdResult<ledger_account::Model> {
    state.service().await
        .create_ledger_account(&org_id, request)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_treasuries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<treasury::Model>> {
    state.service().await
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
    state.service().await
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
    state.service().await
        .get_student_fees(&org_id, enrollment_id.as_deref())
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_active_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<fee_template::Model>> {
    state.service().await
        .get_active_fee_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_fee_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<fee_template::Model>> {
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
        .record_payment(&org_id, request, &created_by)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_journal_entries(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<journal_entry::Model>> {
    state.service().await
        .get_journal_entries(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_journal_entry(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    request: CreateJournalEntryRequest,
) -> CmdResult<journal_entry::Model> {
    state.service().await
        .create_journal_entry(&org_id, request)
        .await
        .map_err(CommandError::from)
}


#[tauri::command]
pub async fn get_active_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<discount_template::Model>> {
    state.service().await
        .get_active_discount_templates(&org_id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_discount_templates(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<discount_template::Model>> {
    state.service().await
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
    state.service().await
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
    state.service().await
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
    state.service().await
        .delete_discount_template(&org_id, &id)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_currencies(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<Vec<currency::Model>> {
    state.service().await
        .repo
        .get_currencies(&org_id)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn get_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
) -> CmdResult<Option<currency::Model>> {
    state.service().await
        .repo
        .get_currency(&org_id, &code)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn create_currency(
    state: State<'_, Arc<AppState>>,
    c: currency::Model,
) -> CmdResult<currency::Model> {
    state.service().await
        .repo
        .create_currency(c)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn update_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
    updates: CurrencyUpdate,
) -> CmdResult<currency::Model> {
    state.service().await
        .repo
        .update_currency(&org_id, &code, updates)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn delete_currency(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    code: String,
) -> CmdResult<u64> {
    let result = state.service().await
        .repo
        .delete_currency(&org_id, &code)
        .await
        .map_err(|e| e.to_string())
        .map_err(CommandError::from)?;
    Ok(result.rows_affected)
}

#[tauri::command]
pub async fn get_account_statement(
    state: State<'_, Arc<AppState>>,
    org_id: String,
    account_id: String,
    start_date: String,
    end_date: String,
) -> CmdResult<AccountStatement> {
    state.service().await
        .get_account_statement(&org_id, &account_id, &start_date, &end_date)
        .await
        .map_err(CommandError::from)
}

#[tauri::command]
pub async fn seed_default_financials(
    state: State<'_, Arc<AppState>>,
    org_id: String,
) -> CmdResult<()> {
    state.service().await
        .seed_default_financials(&org_id)
        .await
        .map_err(CommandError::from)
}
