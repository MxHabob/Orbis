//! Fee template & student fee operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::event_bus::EventEnvelope;
use shared_kernel::traits::{AuditEntry, Permission};

use crate::contracts::events::event_types;
use crate::entities::{fee_template, student_fee};
use svc_academic::entities::enrollment;
use svc_academic::entities::enrollment;

// ── Fee Templates ───────────────────────────────────────────────

pub async fn list_templates(ctx: &ServiceContext) -> Result<Vec<fee_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(fee_template::Entity::find()
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(fee_template::Column::SortOrder)
        .all(&ctx.db).await?)
}

pub async fn list_active_templates(ctx: &ServiceContext) -> Result<Vec<fee_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(fee_template::Entity::find()
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .filter(fee_template::Column::IsActive.eq(1))
        .order_by_asc(fee_template::Column::SortOrder)
        .all(&ctx.db).await?)
}

pub async fn create_template(
    ctx: &ServiceContext,
    name: String, name_ar: Option<String>, category: String,
    default_amount: f64, revenue_account_id: String, receivable_account_id: String,
    frequency: String, is_mandatory: bool,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = fee_template::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), name, name_ar, code: None,
        category, default_amount, revenue_account_id, receivable_account_id,
        applicable_levels: None, frequency, installment_config: None,
        late_fee_rules: None, is_mandatory: if is_mandatory { 1 } else { 0 },
        is_active: 1, sort_order: None,
        created_at: now.clone(), updated_at: now,
    };

    let active: fee_template::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "fee_template.create".into(), entity_type: "fee_template".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}

pub async fn update_template(
    ctx: &ServiceContext,
    template_id: &str,
    name: Option<String>, default_amount: Option<f64>, is_active: Option<bool>,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = fee_template::Entity::find()
        .filter(fee_template::Column::Id.eq(template_id.to_string()))
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("fee_template", template_id))?;

    let mut active: fee_template::ActiveModel = existing.into();
    if let Some(n) = name { active.name = Set(n); }
    if let Some(a) = default_amount { active.default_amount = Set(a); }
    if let Some(ia) = is_active { active.is_active = Set(if ia { 1 } else { 0 }); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}

pub async fn delete_template(ctx: &ServiceContext, template_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let existing = fee_template::Entity::find()
        .filter(fee_template::Column::Id.eq(template_id.to_string()))
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("fee_template", template_id))?;

    let mut active: fee_template::ActiveModel = existing.into();
    active.is_active = Set(0);
    active.updated_at = Set(Utc::now().to_rfc3339());
    active.update(&ctx.db).await?;
    Ok(())
}

// ── Student Fees ────────────────────────────────────────────────

use shared_kernel::query_filter::UniversalFilter;

pub async fn list_student_fees(
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

pub async fn assign_student_fee(
    ctx: &ServiceContext,
    enrollment_id: String, fee_template_id: String,
    amount: f64, due_date: String, notes: Option<String>,
    period_id: String,
) -> Result<student_fee::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    enrollment::Entity::find()
        .filter(enrollment::Column::Id.eq(enrollment_id.clone()))
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("enrollment", &enrollment_id))?;
    fee_template::Entity::find()
        .filter(fee_template::Column::Id.eq(fee_template_id.clone()))
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("fee_template", &fee_template_id))?;
    if amount <= 0.0 {
        return Err(AppError::validation("Fee amount must be greater than zero"));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = student_fee::Model {
        id: id.clone(), org_id: ctx.org_id.clone(), period_id,
        enrollment_id, fee_template_id, original_amount: amount,
        discount_amount: 0.0, net_amount: amount,
        status: "pending".into(), due_date,
        discount_details: None, parent_fee_id: None,
        installment_number: None, notes,
        created_at: now.clone(), updated_at: now,
    };

    let active: student_fee::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;

    ctx.event_bus.publish(EventEnvelope {
        event_type: event_types::FEE_ASSIGNED.into(),
        entity_type: "student_fee".into(), entity_id: id.clone(),
        org_id: ctx.org_id.clone(), actor: ctx.actor.as_str().into(),
        actor_type: ctx.actor.actor_type().into(),
        correlation_id: ctx.correlation_id.clone(),
        payload: serde_json::to_value(&created).unwrap_or_default(),
        chain_depth: 0,
    });

    ctx.audit.log(AuditEntry {
        org_id: ctx.org_id.clone(), actor: ctx.actor.clone(),
        action: "student_fee.assign".into(), entity_type: "student_fee".into(),
        entity_id: id, old_data: None,
        new_data: Some(serde_json::to_value(&created).unwrap_or_default()),
        correlation_id: ctx.correlation_id.clone(),
    }).await?;

    Ok(created)
}
