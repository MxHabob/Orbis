//! Treasury operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::CreateTreasuryCmd;
use crate::contracts::events::event_types;
use crate::entities::treasury;
use crate::ops::ledger;

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
    cmd: CreateTreasuryCmd,
) -> Result<treasury::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let treasury_id = Uuid::new_v4().to_string();
    let entity_type = if cmd.treasury_type.to_lowercase() == "bank" {
        "treasury_bank"
    } else {
        "treasury_cash"
    };

    let sub_account = ledger::get_or_create_sub_account(
        ctx,
        entity_type,
        &treasury_id,
        Some(&cmd.name),
        cmd.name_ar.as_deref(),
    ).await?;

    let now = Utc::now().to_rfc3339();

    let model = treasury::Model {
        id: treasury_id.clone(), org_id: ctx.org_id.clone(), unit_id: None,
        name: cmd.name, name_ar: cmd.name_ar, code: Some(cmd.code), item_type: cmd.treasury_type, account_id: sub_account.id,
        bank_name: None, bank_branch: None, account_number: None,
        account_name: None, iban: None, swift_code: None,
        opening_balance: Some(0.0), current_balance: Some(0.0),
        currency: sub_account.currency, authorized_users: None, is_default: 0, is_active: 1,
        created_at: now.clone(), updated_at: now,
    };

    let active: treasury::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::TREASURY_CREATED.into(),
        entity_type: "treasury".into(), entity_id: treasury_id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "treasury.create".into(), entity_type: "treasury".into(),
        entity_id: treasury_id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}
