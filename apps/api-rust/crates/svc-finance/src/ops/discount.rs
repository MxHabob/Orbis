//! Discount template operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::CreateDiscountTemplateCmd;
use crate::contracts::events::event_types;
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
    cmd: CreateDiscountTemplateCmd,
) -> Result<discount_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let calculation_method = if cmd.is_percentage { "percentage".to_string() } else { "fixed".to_string() };

    let model = discount_template::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), name: cmd.name, code: None,
        item_type: "manual".to_string(), calculation_method, value: cmd.amount as i32,
        eligibility_rules: None, limits: None,
        requires_approval: 0,
        approver_roles: None, is_auto_apply: 0, is_stackable: 0,
        priority: None, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: discount_template::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::DISCOUNT_TEMPLATE_CREATED.into(),
        entity_type: "discount_template".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

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
