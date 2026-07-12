//! Fee Template operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{CreateFeeTemplateCmd, UpdateFeeTemplateCmd};
use crate::contracts::events::event_types;
use crate::entities::fee_template;

pub async fn list(ctx: &ServiceContext) -> Result<Vec<fee_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(fee_template::Entity::find()
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(fee_template::Column::SortOrder)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    cmd: CreateFeeTemplateCmd,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = fee_template::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        name: cmd.name,
        name_ar: cmd.name_ar,
        code: None,
        category: cmd.category,
        default_amount: cmd.default_amount,
        revenue_account_id: cmd.revenue_account_id,
        receivable_account_id: cmd.receivable_account_id,
        applicable_levels: None,
        frequency: cmd.frequency,
        installment_config: None,
        late_fee_rules: None,
        is_mandatory: if cmd.is_mandatory { 1 } else { 0 },
        is_active: 1,
        sort_order: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    let active: fee_template::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::FEE_TEMPLATE_CREATED.into(),
        entity_type: "fee_template".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "fee_template.create".into(), entity_type: "fee_template".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext,
    template_id: &str,
    cmd: UpdateFeeTemplateCmd,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = fee_template::Entity::find_by_id(template_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("fee_template", template_id))?;

    let mut active: fee_template::ActiveModel = existing.into();
    if let Some(n) = cmd.name { active.name = Set(n); }
    if let Some(a) = cmd.default_amount { active.default_amount = Set(a); }
    if let Some(ia) = cmd.is_active { active.is_active = Set(if ia { 1 } else { 0 }); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::FEE_TEMPLATE_UPDATED.into(),
        entity_type: "fee_template".into(), entity_id: template_id.to_string(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&updated).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "fee_template.update".into(), entity_type: "fee_template".into(),
        entity_id: template_id.to_string(), old_data: None,
        new_data: Some(serde_json::to_value(&updated).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(updated)
}
