/// Thin Tauri command wrappers for the Finance domain.
use std::sync::Arc;
use tauri::State;
use crate::state::AppState;
use crate::commands::error::{CmdResult, CommandError};
use svc_finance::ops;

#[tauri::command]
pub async fn list_fee_templates(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::fee_template::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_fee_template(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::fee_template::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn update_fee_template(state: State<'_, Arc<AppState>>, template_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let template = svc_finance::entities::fee_template::Entity::find_by_id(template_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Fee template not found"))?;
    let ctx = state.build_context(&template.org_id).await.map_err(CommandError::from)?;
    let r = ops::fee_template::update(&ctx, &template_id, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_discount_templates(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::discount::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn assign_fee(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::student_fee::assign(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_student_fees(state: State<'_, Arc<AppState>>, enrollment_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::EntityTrait;
    let db = state.active_db().await;
    let enrollment = svc_academic::entities::enrollment::Entity::find_by_id(enrollment_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Enrollment not found"))?;
    let ctx = state.build_context(&enrollment.org_id).await.map_err(CommandError::from)?;
    let r = ops::student_fee::list_by_enrollment(&ctx, &enrollment_id).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn record_payment(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::payment::record(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_payments(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::payment::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_ledger_accounts(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::ledger::list_accounts(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_journal_entries(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
    use svc_finance::entities::journal_entry;
    let db = state.active_db().await;
    let entries = journal_entry::Entity::find()
        .filter(journal_entry::Column::OrgId.eq(&org_id))
        .order_by_desc(journal_entry::Column::AccountingDate)
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(entries).unwrap())
}

#[tauri::command]
pub async fn list_treasuries(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::treasury::list(&ctx).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn create_treasury(state: State<'_, Arc<AppState>>, org_id: String, data: serde_json::Value) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    let r = ops::treasury::create(&ctx, data).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_transaction_accounts(state: State<'_, Arc<AppState>>, org_id: String) -> CmdResult<serde_json::Value> {
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
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
    let db = state.active_db().await;
    let account = svc_finance::entities::ledger_account::Entity::find_by_id(account_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Ledger account not found"))?;
    let ctx = state.build_context(&account.org_id).await.map_err(CommandError::from)?;
    let r = ops::ledger::get_statement(&ctx, &account_id, start_date, end_date).await.map_err(CommandError::from)?;
    Ok(serde_json::to_value(r).unwrap())
}

#[tauri::command]
pub async fn list_active_fee_templates(state: State<'_, Arc<AppState>>) -> CmdResult<serde_json::Value> {
    use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder};
    let db = state.active_db().await;
    let org_id = resolve_org_id(&db).await?;
    let ctx = state.build_context(&org_id).await.map_err(CommandError::from)?;
    ctx.permissions.check(&ctx.actor, shared_kernel::traits::Permission::FinanceRead).await.map_err(CommandError::from)?;
    let templates = svc_finance::entities::fee_template::Entity::find()
        .filter(svc_finance::entities::fee_template::Column::OrgId.eq(&org_id))
        .filter(svc_finance::entities::fee_template::Column::IsActive.eq(1))
        .order_by_asc(svc_finance::entities::fee_template::Column::SortOrder)
        .all(&db)
        .await
        .map_err(CommandError::from)?;
    Ok(serde_json::to_value(templates).unwrap())
}

#[tauri::command]
pub async fn delete_fee_template(
    state: State<'_, Arc<AppState>>,
    template_id: String,
) -> CmdResult<()> {
    use sea_orm::{EntityTrait, ModelTrait};
    let db = state.active_db().await;
    let template = svc_finance::entities::fee_template::Entity::find_by_id(template_id.clone())
        .one(&db).await.map_err(CommandError::from)?
        .ok_or_else(|| CommandError::not_found("Fee template not found"))?;
    template.delete(&db).await.map_err(CommandError::from)?;
    Ok(())
}

/// Helper function to resolve org_id from the latest active session.
async fn resolve_org_id(db: &sea_orm::DatabaseConnection) -> Result<String, CommandError> {
    use sea_orm::{EntityTrait, QueryOrder};
    let session = svc_iam::entities::user_session::Entity::find()
        .order_by_desc(svc_iam::entities::user_session::Column::CreatedAt)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("No active session"))?;
    let user = svc_iam::entities::user::Entity::find_by_id(session.user_id)
        .one(db)
        .await
        .map_err(CommandError::from)?
        .ok_or_else(|| CommandError::unauthorized("Active user not found"))?;
    Ok(user.org_id)
}
