/// Thin Tauri command wrappers for the Finance domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_finance::ops;

#[tauri::command]
pub async fn list_fee_templates(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::fee_template::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_fee_template(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let cmd: svc_finance::contracts::commands::CreateFeeTemplateCmd = serde_json::from_value(data).map_err(|e| CommandError::with_code(e.to_string(), "VALIDATION_ERROR"))?;
    let r = ops::fee_template::create(&ctx, cmd).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_fee_template(state: State<'_, Arc<AppState>>, template_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _template = svc_finance::entities::fee_template::Entity::find_by_id(template_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Fee template not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let cmd: svc_finance::contracts::commands::UpdateFeeTemplateCmd = serde_json::from_value(data).map_err(|e| CommandError::with_code(e.to_string(), "VALIDATION_ERROR"))?;
    let r = ops::fee_template::update(&ctx, &template_id, cmd).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_discount_templates(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::discount::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn assign_fee(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let cmd: svc_finance::contracts::commands::AssignFeeCmd = serde_json::from_value(data).map_err(|e| CommandError::with_code(e.to_string(), "VALIDATION_ERROR"))?;
    let r = ops::student_fee::assign(&ctx, cmd).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_student_fees(state: State<'_, Arc<AppState>>, enrollment_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _enrollment = svc_academic::entities::enrollment::Entity::find_by_id(enrollment_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Enrollment not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::student_fee::list_by_enrollment(&ctx, &enrollment_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_all_student_fees(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::student_fee::list_by_org(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn record_payment(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let cmd: svc_finance::contracts::commands::RecordPaymentCmd = serde_json::from_value(data).map_err(|e| CommandError::with_code(e.to_string(), "VALIDATION_ERROR"))?;
    let r = ops::payment::record(&ctx, cmd).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_payments(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::payment::list(&ctx, None, None).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_ledger_accounts(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::ledger::list_accounts(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_journal_entries(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let entries = ops::ledger::get_journal_entries(&ctx, None, None).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(entries).unwrap())
}

#[tauri::command]
pub async fn list_treasuries(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::treasury::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_treasury(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let cmd: svc_finance::contracts::commands::CreateTreasuryCmd = serde_json::from_value(data).map_err(|e| CommandError::with_code(e.to_string(), "VALIDATION_ERROR"))?;
    let r = ops::treasury::create(&ctx, cmd).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_transaction_accounts(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::ledger::list_transaction_accounts(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn get_account_statement(
    state: State<'_, Arc<AppState>>,
    account_id: String,
    start_date: Option<String>,
    end_date: Option<String>,
) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.db();
    let _account = svc_finance::entities::account::Entity::find_by_id(account_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Ledger account not found"))?;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let sd = start_date.unwrap_or_else(|| "2000-01-01".to_string());
    let ed = end_date.unwrap_or_else(|| "2100-01-01".to_string());
    let r = ops::ledger::get_account_statement(&ctx, &account_id, &sd, &ed).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_active_fee_templates(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let templates = ops::fee_template::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(templates).unwrap())
}

#[tauri::command]
pub async fn delete_fee_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<()> {
    use sea_orm::{EntityTrait, ModelTrait};
    let db = state.db();
    let template = svc_finance::entities::fee_template::Entity::find_by_id(template_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Fee template not found"))?;
    template.delete(&db).await.map_err(CommandError::from)?;
    Ok(())
}



// ── Currency Commands ────────────────────────────────────────────────

#[tauri::command]
pub async fn list_currencies(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let r = ops::currency::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_currency(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let code = data["code"].as_str().unwrap_or_default().to_string();
    let name = data["name"].as_str().unwrap_or_default().to_string();
    let name_ar = data["name_ar"].as_str().map(|s| s.to_string());
    let symbol = data["symbol"].as_str().unwrap_or("$").to_string();
    let symbol_position = data["symbol_position"].as_str().unwrap_or("before").to_string();
    let decimal_places = data["decimal_places"].as_i64().unwrap_or(2) as i32;
    let exchange_rate = data["exchange_rate"].as_f64().unwrap_or(1.0);
    let is_base = data["is_base"].as_bool().unwrap_or(false);
    let r = ops::currency::create(&ctx, code, name, name_ar, symbol, symbol_position, decimal_places, exchange_rate, is_base)
        .await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_currency(state: State<'_, Arc<AppState>>, code: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let exchange_rate = data["exchange_rate"].as_f64();
    let is_active = data["is_active"].as_bool();
    let r = ops::currency::update(&ctx, &code, exchange_rate, is_active)
        .await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn delete_currency(state: State<'_, Arc<AppState>>, code: String) -> CmdResult<()> {
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    ops::currency::delete(&ctx, &code).await.map_err(CommandError::from)
}

// ── Fiscal Year Commands ─────────────────────────────────────────────

#[tauri::command]
pub async fn list_fiscal_years(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let db = state.db();
    let r = svc_finance::entities::fiscal_year::Entity::find()
        .filter(svc_finance::entities::fiscal_year::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(svc_finance::entities::fiscal_year::Column::StartDate)
        .all(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_fiscal_year(state: State<'_, Arc<AppState>>, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    use sea_orm::ActiveModelTrait;
    let ctx = state.build_guarded_context().await.map_err(CommandError::from)?;
    let db = state.db();
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let model = svc_finance::entities::fiscal_year::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        name: data["name"].as_str().unwrap_or_default().to_string(),
        start_date: data["start_date"].as_str().unwrap_or_default().to_string(),
        end_date: data["end_date"].as_str().unwrap_or_default().to_string(),
        status: data["status"].as_str().unwrap_or("active").to_string(),
        created_at: now.clone(),
        updated_at: now,
    };
    let active: svc_finance::entities::fiscal_year::ActiveModel = model.into();
    let created = active.insert(&db).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(created).unwrap())
}


