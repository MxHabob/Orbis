//! Ledger account operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::ledger_account;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<ledger_account::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(ledger_account::Entity::find()
        .filter(ledger_account::Column::OrgId.eq(&ctx.org_id))
        .filter(ledger_account::Column::IsActive.eq(1))
        .order_by_asc(ledger_account::Column::Code)
        .all(&ctx.db).await?)
}

pub async fn list_transaction_accounts(ctx: &ServiceContext) -> Result<Vec<ledger_account::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(ledger_account::Entity::find()
        .filter(ledger_account::Column::OrgId.eq(&ctx.org_id))
        .filter(ledger_account::Column::IsActive.eq(1))
        .filter(ledger_account::Column::IsControlAccount.eq(0))
        .order_by_asc(ledger_account::Column::Code)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    name: String,
    name_ar: Option<String>,
    code: String,
    item_type: String,
    currency: String,
    parent_id: Option<String>,
) -> Result<ledger_account::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let (depth, path) = if let Some(ref pid) = parent_id {
        let parent = ledger_account::Entity::find_by_id(pid.clone())
            .one(&ctx.db).await?
            .ok_or_else(|| AppError::not_found("ledger_account", pid))?;
        (parent.depth + 1, format!("{}/{}", parent.path, pid))
    } else {
        (0, "/".into())
    };

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = ledger_account::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), code, name, name_ar,
        item_type, sub_type: None, parent_id, path, depth, currency,
        opening_balance: Some(0.0), current_balance: Some(0.0),
        entity_type: None, entity_id: None,
        is_control_account: 0, is_system_account: 0, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: ledger_account::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "ledger_account.create".into(), entity_type: "ledger_account".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn get_statement(
    ctx: &ServiceContext,
    account_id: &str,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Vec<crate::entities::journal_entry::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    use crate::entities::journal_entry;
    let mut query = journal_entry::Entity::find()
        .filter(journal_entry::Column::OrgId.eq(&ctx.org_id))
        .filter(
            sea_orm::Condition::any()
                .add(journal_entry::Column::DebitAccountId.eq(account_id))
                .add(journal_entry::Column::CreditAccountId.eq(account_id))
        );

    if let Some(start) = start_date {
        query = query.filter(journal_entry::Column::AccountingDate.gte(start));
    }
    if let Some(end) = end_date {
        query = query.filter(journal_entry::Column::AccountingDate.lte(end));
    }

    Ok(query.order_by_asc(journal_entry::Column::AccountingDate).all(&ctx.db).await?)
}
