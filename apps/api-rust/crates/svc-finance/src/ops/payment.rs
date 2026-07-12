//! Payment operations — record payments with double-entry accounting.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{CreateJournalEntryCmd, RecordPaymentCmd};
use crate::contracts::events::event_types;
use crate::entities::{payment, student_fee, treasury, treasury_transaction, currency};
use svc_academic::entities::enrollment;

use crate::ops::ledger;

pub async fn list(
    ctx: &ServiceContext,
) -> Result<Vec<payment::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;

    Ok(payment::Entity::find()
        .filter(payment::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(payment::Column::PaidAt)
        .all(&ctx.db)
        .await?)
}

pub async fn record(
    ctx: &ServiceContext,
    cmd: RecordPaymentCmd,
) -> Result<payment::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let fee_id = cmd.fee_id.ok_or_else(|| AppError::validation("fee_id is required"))?;
    let treasury_id = cmd.treasury_id.ok_or_else(|| AppError::validation("treasury_id is required"))?;

    let fee = student_fee::Entity::find_by_id(fee_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("student_fee", &fee_id))?;

    let enroll = enrollment::Entity::find_by_id(fee.enrollment_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", &fee.enrollment_id))?;

    let student_account = ledger::get_or_create_sub_account(
        ctx,
        "student",
        &enroll.student_id,
        None,
        None,
    ).await?;

    let trsy = treasury::Entity::find_by_id(treasury_id.clone())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("treasury", &treasury_id))?;

    if cmd.amount <= 0.0 {
        return Err(AppError::validation("Payment amount must be greater than zero"));
    }

    let exchange_rate = if cmd.currency == student_account.currency {
        1.0
    } else {
        let cur = currency::Entity::find()
            .filter(currency::Column::OrgId.eq(&ctx.org_id))
            .filter(currency::Column::Code.eq(&cmd.currency))
            .one(&ctx.db)
            .await?
            .ok_or_else(|| AppError::validation(format!("Currency {} not supported", cmd.currency)))?;
        cur.exchange_rate
    };

    let base_amount = cmd.amount * exchange_rate;
    let payment_id = Uuid::new_v4().to_string();
    let receipt_number = format!("REC-{}", Utc::now().timestamp_millis());
    let journal_entry_id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    let p = payment::Model {
        id: payment_id.clone(),
        org_id: ctx.org_id.clone(),
        fee_id: Some(fee_id.clone()),
        enrollment_id: Some(fee.enrollment_id.clone()),
        account_id: student_account.id.clone(),
        amount: cmd.amount,
        currency: cmd.currency.clone(),
        exchange_rate,
        base_amount,
        method: cmd.method.clone(),
        reference: None,
        receipt_number: Some(receipt_number.clone()),
        journal_entry_id: Some(journal_entry_id.clone()),
        treasury_id: Some(treasury_id.clone()),
        payer_name: cmd.payer_name.clone(),
        payer_phone: None,
        payment_type: "receipt".to_string(),
        related_payment_id: None,
        notes: cmd.notes.clone(),
        paid_at: now.clone(),
        created_at: now.clone(),
        created_by: Some(ctx.actor.as_str().to_string()),
    };

    let active: payment::ActiveModel = p.into();
    let payment_record = active.insert(&ctx.db).await?;

    let description = format!("Fee payment received via {}: {}", cmd.method, receipt_number);
    let description_ar = Some(format!("سداد رسوم عبر {}: {}", cmd.method, receipt_number));

    let je_cmd = CreateJournalEntryCmd {
        transaction_ref: receipt_number.clone(),
        accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
        fiscal_period_id: fee.period_id.clone(),
        amount: cmd.amount,
        debit_account_id: trsy.account_id.clone(),
        credit_account_id: student_account.id.clone(),
        currency: cmd.currency.clone(),
        exchange_rate: Some(exchange_rate),
        description: description.clone(),
        description_ar,
        source_type: "payment".to_string(),
        source_id: Some(payment_id.clone()),
        source_module: Some("finance".to_string()),
    };
    
    // ops::ledger::create_journal_entry already updates ledger balances and emits events
    ledger::create_journal_entry(ctx, je_cmd).await?;

    let tx = treasury_transaction::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        treasury_id: treasury_id.clone(),
        journal_entry_id: journal_entry_id.clone(),
        item_type: "inflow".to_string(),
        category: "tuition".to_string(),
        amount: cmd.amount,
        currency: cmd.currency.clone(),
        exchange_rate: Some(exchange_rate),
        reference_type: Some("payment".to_string()),
        reference_id: Some(payment_id.clone()),
        description: "Tuition payment for student fee".to_string(),
        notes: cmd.notes.clone(),
        attachments: None,
        created_by: Some(ctx.actor.as_str().to_string()),
        approved_by: Some(ctx.actor.as_str().to_string()),
        approved_at: Some(now.clone()),
        transaction_date: Utc::now().format("%Y-%m-%d").to_string(),
        created_at: now.clone(),
    };
    
    let tx_active: treasury_transaction::ActiveModel = tx.into();
    tx_active.insert(&ctx.db).await?;

    let current_treasury_bal = trsy.current_balance.unwrap_or(0.0);
    let mut tr_active: treasury::ActiveModel = trsy.into();
    tr_active.current_balance = Set(Some(current_treasury_bal + cmd.amount));
    tr_active.update(&ctx.db).await?;

    let payments = payment::Entity::find()
        .filter(payment::Column::OrgId.eq(&ctx.org_id))
        .filter(payment::Column::FeeId.eq(fee_id.clone()))
        .all(&ctx.db).await?;

    let total_paid: f64 = payments.iter()
        .map(|p| p.base_amount)
        .sum();

    let new_status = if total_paid >= fee.net_amount {
        "paid".to_string()
    } else if total_paid > 0.0 {
        "partially_paid".to_string()
    } else {
        "unpaid".to_string()
    };

    let mut fee_active: student_fee::ActiveModel = fee.into();
    fee_active.status = Set(new_status);
    fee_active.update(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::PAYMENT_RECORDED.into(),
        entity_type: "payment".into(), entity_id: payment_id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&payment_record).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "payment.record".into(), entity_type: "payment".into(),
        entity_id: payment_id.clone(), old_data: None,
        new_data: Some(serde_json::to_value(&payment_record).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(payment_record)
}
