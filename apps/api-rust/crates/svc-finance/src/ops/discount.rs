//! Discount template operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::entities::discount_template;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<discount_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(discount_template::Entity::find()
        .filter(discount_template::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(discount_template::Column::Priority)
        .all(&ctx.db).await?)
}

pub async fn list_active(ctx: &ServiceContext) -> Result<Vec<discount_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(discount_template::Entity::find()
        .filter(discount_template::Column::OrgId.eq(&ctx.org_id))
        .filter(discount_template::Column::IsActive.eq(1))
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    name: String, item_type: String, calculation_method: String,
    value: i32, requires_approval: bool,
) -> Result<discount_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = discount_template::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), name, code: None,
        item_type, calculation_method, value,
        eligibility_rules: None, limits: None,
        requires_approval: if requires_approval { 1 } else { 0 },
        approver_roles: None, is_auto_apply: 0, is_stackable: 0,
        priority: None, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: discount_template::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "discount_template.create".into(), entity_type: "discount_template".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext, template_id: &str,
    name: Option<String>, value: Option<i32>, is_active: Option<bool>,
) -> Result<discount_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = discount_template::Entity::find_by_id(template_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("discount_template", template_id))?;

    let mut active: discount_template::ActiveModel = existing.into();
    if let Some(n) = name { active.name = Set(n); }
    if let Some(v) = value { active.value = Set(v); }
    if let Some(ia) = is_active { active.is_active = Set(if ia { 1 } else { 0 }); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete(ctx: &ServiceContext, template_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = discount_template::Entity::find_by_id(template_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("discount_template", template_id))?;

    let mut active: discount_template::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;
    Ok(())
}
