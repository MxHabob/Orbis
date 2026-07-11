//! Treasury operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::treasury;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<treasury::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(treasury::Entity::find()
        .filter(treasury::Column::OrgId.eq(&ctx.org_id))
        .filter(treasury::Column::IsActive.eq(1))
        .order_by_asc(treasury::Column::Name)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    name: String,
    name_ar: Option<String>,
    item_type: String,
    ledger_account_id: String,
    currency: String,
) -> Result<treasury::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = treasury::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), unit_id: None,
        name, name_ar, code: None, item_type, ledger_account_id,
        bank_name: None, bank_branch: None, account_number: None,
        account_name: None, iban: None, swift_code: None,
        opening_balance: Some(0.0), current_balance: Some(0.0),
        currency, authorized_users: None, is_default: 0, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: treasury::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "treasury.create".into(), entity_type: "treasury".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}
