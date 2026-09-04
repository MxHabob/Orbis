//! Journal entry operations.

use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::journal_entry;

use shared_kernel::query_filter::UniversalFilter;

pub async fn list(
    ctx: &ServiceContext,
    limit: Option<u64>,
    offset: Option<u64>,
    filter: Option<UniversalFilter>,
) -> Result<Vec<journal_entry::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    let limit_val = limit.unwrap_or(500);
    let offset_val = offset.unwrap_or(0);
    
    let mut query = journal_entry::Entity::find()
        .filter(journal_entry::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "source_type" => Some(journal_entry::Column::SourceType.into_expr().into()),
            "is_reversed" => Some(journal_entry::Column::IsReversed.into_expr().into()),
            "accounting_date" => Some(journal_entry::Column::AccountingDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_desc(journal_entry::Column::AccountingDate)
        .limit(limit_val)
        .offset(offset_val)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    debit_account_id: String, credit_account_id: String,
    amount: f64, currency: String, description: String,
    description_ar: Option<String>, accounting_date: String,
    fiscal_period_id: String, source_type: String,
    source_id: Option<String>,
) -> Result<journal_entry::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();
    let tx_ref = format!("JE-{}", &id[..8]);

    let model = journal_entry::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        transaction_ref: tx_ref, accounting_date, fiscal_period_id,
        amount, debit_account_id, credit_account_id,
        currency, exchange_rate: Some(1.0), base_currency_amount: Some(amount),
        source_type, source_id, source_module: None,
        description, description_ar, metadata: None,
        is_reversed: 0, reversed_by: None, reversal_reason: None,
        is_reconciled: 0, reconciled_at: None, reconciliation_ref: None,
        status: "posted".into(),
        posted_by: Some(ctx.actor.as_str().to_string()),
        posted_at: Some(now.clone()),
        approved_by: None, approved_at: None,
        created_by: Some(ctx.actor.as_str().to_string()),
        created_at: now,
    };

    let active: journal_entry::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "journal_entry.create".into(), entity_type: "journal_entry".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}
