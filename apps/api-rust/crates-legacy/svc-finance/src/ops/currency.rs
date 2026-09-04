//! Currency operations.

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::currency;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<currency::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(currency::Entity::find()
        .filter(currency::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db).await?)
}

pub async fn get(ctx: &ServiceContext, code: &str) -> Result<currency::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    currency::Entity::find()
        .filter(currency::Column::OrgId.eq(&ctx.org_id))
        .filter(currency::Column::Code.eq(code))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("currency", code))
}

pub async fn create(
    ctx: &ServiceContext,
    code: String, name: String, name_ar: Option<String>,
    symbol: String, symbol_position: String,
    decimal_places: i32, exchange_rate: f64, is_base: bool,
) -> Result<currency::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    if is_base {
        use sea_orm::ConnectionTrait;
        let stmt = sea_orm::Statement::from_string(
            ctx.db.get_database_backend(),
            format!("UPDATE currencies SET is_base = 0 WHERE org_id = '{}'", ctx.org_id),
        );
        ctx.db.execute(stmt).await.ok();
    }

    let model = currency::Model {
        code: code.clone(), org_id: ctx.org_id.clone(),
        name, name_ar, symbol, symbol_position, decimal_places,
        exchange_rate, is_base: if is_base { 1 } else { 0 },
        is_active: 1, updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let active: currency::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "currency.create".into(), entity_type: "currency".into(),
        entity_id: code, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, code: &str,
    exchange_rate: Option<f64>, is_active: Option<bool>,
) -> Result<currency::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = get(ctx, code).await?;
    let mut active: currency::ActiveModel = existing.into();
    if let Some(er) = exchange_rate { active.exchange_rate = Set(er); }
    if let Some(ia) = is_active { active.is_active = Set(if ia { 1 } else { 0 }); }
    active.updated_at = Set(chrono::Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, code: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;
    let existing = get(ctx, code).await?;
    existing.delete(&ctx.db).await?;
    Ok(())
}
