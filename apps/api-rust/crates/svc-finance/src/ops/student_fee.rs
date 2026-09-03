//! Student Fee operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, TransactionTrait};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::commands::{AssignFeeCmd, CreateJournalEntryCmd};
use crate::contracts::events::event_types;
use crate::entities::{student_fee, fee_template};
use svc_academic::entities::enrollment;

use crate::ops::ledger;

use shared_kernel::query_filter::UniversalFilter;

pub async fn list_by_enrollment(
    ctx: &ServiceContext,
    enrollment_id: &str,
    filter: Option<UniversalFilter>,
) -> Result<Vec<student_fee::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    let mut query = student_fee::Entity::find()
        .filter(student_fee::Column::OrgId.eq(&ctx.org_id))
        .filter(student_fee::Column::EnrollmentId.eq(enrollment_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "status" => Some(student_fee::Column::Status.into_expr().into()),
            "due_date" => Some(student_fee::Column::DueDate.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }
    
    Ok(query.order_by_asc(student_fee::Column::DueDate).all(&ctx.db).await?)
}

pub async fn list_by_org(
    ctx: &ServiceContext,
    filter: Option<UniversalFilter>,
) -> Result<Vec<student_fee::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    let mut query = student_fee::Entity::find()
        .filter(student_fee::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "status" => Some(student_fee::Column::Status.into_expr().into()),
            "due_date" => Some(student_fee::Column::DueDate.into_expr().into()),
            "enrollment_id" => Some(student_fee::Column::EnrollmentId.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.order_by_desc(student_fee::Column::CreatedAt).all(&ctx.db).await?)
}

pub async fn assign(
    ctx: &ServiceContext,
    cmd: AssignFeeCmd,
) -> Result<student_fee::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let enroll = enrollment::Entity::find()
        .filter(enrollment::Column::Id.eq(cmd.enrollment_id.clone()))
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", &cmd.enrollment_id))?;

    let student_account = ledger::get_or_create_sub_account(
        ctx,
        "student",
        &enroll.student_id,
        None,
        None,
    ).await?;

    let template = fee_template::Entity::find()
        .filter(fee_template::Column::Id.eq(cmd.fee_template_id.clone()))
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("fee_template", &cmd.fee_template_id))?;

    let amount = cmd.amount_override.unwrap_or(template.default_amount);
    if amount <= 0.0 {
        return Err(AppError::validation("Fee amount must be greater than zero"));
    }
    let now = Utc::now().to_rfc3339();
    let fee_id = Uuid::new_v4().to_string();

    let sf = student_fee::Model {
        id: fee_id.clone(),
        org_id: ctx.org_id.clone(),
        period_id: enroll.period_id.clone(),
        enrollment_id: cmd.enrollment_id.clone(),
        fee_template_id: cmd.fee_template_id.clone(),
        original_amount: amount,
        discount_amount: 0.0,
        net_amount: amount,
        status: "unpaid".to_string(),
        due_date: Utc::now().format("%Y-%m-%d").to_string(),
        discount_details: None,
        parent_fee_id: None,
        installment_number: None,
        notes: None,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    let description = format!("Tuition fee charged: {}", template.name);
    let description_ar = Some(format!("قيد رسوم دراسية: {}", template.name_ar.clone().unwrap_or(template.name.clone())));

    let je_cmd = CreateJournalEntryCmd {
        transaction_ref: format!("INV-{}", &fee_id[..8].to_uppercase()),
        accounting_date: Utc::now().format("%Y-%m-%d").to_string(),
        fiscal_period_id: enroll.period_id.clone(),
        amount,
        debit_account_id: student_account.id.clone(),
        credit_account_id: template.revenue_account_id.clone(),
        currency: student_account.currency.clone(),
        exchange_rate: Some(1.0),
        description,
        description_ar,
        source_type: "student_fee".to_string(),
        source_id: Some(fee_id.clone()),
        source_module: Some("finance".to_string()),
    };

    let txn = ctx.db.begin().await
        .map_err(|e| AppError::internal(format!("Failed to start fee transaction: {e}")))?;
    let active: student_fee::ActiveModel = sf.into();
    let created_fee = active.insert(&txn).await?;
    ledger::post_journal_entry(
        &txn,
        &ctx.org_id,
        Some(ctx.actor.as_str()),
        je_cmd,
    ).await?;
    txn.commit().await
        .map_err(|e| AppError::internal(format!("Failed to commit fee transaction: {e}")))?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::FEE_ASSIGNED.into(),
        entity_type: "student_fee".into(), entity_id: fee_id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(), correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created_fee).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "student_fee.assign".into(), entity_type: "student_fee".into(),
        entity_id: fee_id, old_data: None,
        new_data: Some(serde_json::to_value(&created_fee).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created_fee)
}
