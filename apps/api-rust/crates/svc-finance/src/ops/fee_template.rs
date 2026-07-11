//! Fee Template operations.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::fee_template;

#[derive(serde::Deserialize)]
pub struct CreateFeeTemplatePayload {
    pub name: String,
    pub name_ar: Option<String>,
    pub category: String,
    pub default_amount: f64,
    pub revenue_account_id: String,
    pub receivable_account_id: String,
    pub frequency: String,
    pub is_mandatory: bool,
}

#[derive(serde::Deserialize)]
pub struct UpdateFeeTemplatePayload {
    pub name: Option<String>,
    pub default_amount: Option<f64>,
    pub is_active: Option<bool>,
}

pub async fn list(ctx: &ServiceContext) -> Result<Vec<fee_template::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceRead).await?;
    Ok(fee_template::Entity::find()
        .filter(fee_template::Column::OrgId.eq(&ctx.org_id))
        .order_by_asc(fee_template::Column::SortOrder)
        .all(&ctx.db).await?)
}

pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let req: CreateFeeTemplatePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid create fee template payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let model = fee_template::Model {
        id: id.clone(),
        org_id: ctx.org_id.clone(),
        name: req.name,
        name_ar: req.name_ar,
        code: None,
        category: req.category,
        default_amount: req.default_amount,
        revenue_account_id: req.revenue_account_id,
        receivable_account_id: req.receivable_account_id,
        applicable_levels: None,
        frequency: req.frequency,
        installment_config: None,
        late_fee_rules: None,
        is_mandatory: if req.is_mandatory { 1 } else { 0 },
        is_active: 1,
        sort_order: None,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: fee_template::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

pub async fn update(
    ctx: &ServiceContext,
    template_id: &str,
    data: serde_json::Value,
) -> Result<fee_template::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::FinanceWrite).await?;

    let req: UpdateFeeTemplatePayload = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update fee template payload: {e}")))?;

    let existing = fee_template::Entity::find_by_id(template_id.to_string())
        .one(&ctx.db).await?
        .ok_or_else(|| AppError::not_found("fee_template", template_id))?;

    let mut active: fee_template::ActiveModel = existing.into();
    if let Some(n) = req.name { active.name = Set(n); }
    if let Some(a) = req.default_amount { active.default_amount = Set(a); }
    if let Some(ia) = req.is_active { active.is_active = Set(if ia { 1 } else { 0 }); }
    active.updated_at = Set(Utc::now().to_rfc3339());

    Ok(active.update(&ctx.db).await?)
}
