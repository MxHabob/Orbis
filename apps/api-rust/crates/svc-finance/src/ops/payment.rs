//! Payment operations — record payments with double-entry accounting.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::payment;

pub async fn list(
    ctx: &ServiceContext,
    enrollment_id: Option<String>,
    fee_id: Option<String>,
) -> Result<Vec<payment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    let mut query = payment::Entity::find()
        .filter(payment::Column::OrgId.eq(&ctx.org_id));

    if let Some(eid) = enrollment_id {
        query = query.filter(payment::Column::EnrollmentId.eq(eid));
    }
    if let Some(fid) = fee_id {
        query = query.filter(payment::Column::FeeId.eq(fid));
    }

    Ok(query.order_by_desc(payment::Column::PaidAt).all(&ctx.db).await?)
}

pub async fn record(
    ctx: &ServiceContext,
    fee_id: Option<String>, enrollment_id: Option<String>,
    account_id: String, amount: f64, currency: String,
    exchange_rate: f64, method: String,
    treasury_id: Option<String>, payer_name: Option<String>,
    notes: Option<String>,
) -> Result<payment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();
    let base_amount = amount * exchange_rate;

    let model = payment::Model {
        id: id.clone(), org_id: ctx.org_id.clone(),
        fee_id, enrollment_id, account_id, amount, currency,
        exchange_rate, base_amount, method,
        reference: None, receipt_number: None, journal_entry_id: None,
        treasury_id, payer_name, payer_phone: None,
        payment_type: "payment".into(), related_payment_id: None,
        notes, paid_at: now.clone(),
        created_at: now, created_by: Some(ctx.actor.as_str().to_string()),
    };

    let active: payment::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::PAYMENT_RECORDED.into(),
        entity_type: "payment".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "payment.record".into(), entity_type: "payment".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}
