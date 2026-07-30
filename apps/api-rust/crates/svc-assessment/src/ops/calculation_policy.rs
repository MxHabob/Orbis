//! Calculation Policy CRUD operations.
//!
//! Manages formula-based calculation policies that determine how grades
//! are computed for specific academic periods.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::calculation_policy;

// ── Queries ─────────────────────────────────────────────────────

/// List all calculation policies for the current organization.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<calculation_policy::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SettingsRead).await?;

    Ok(calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?)
}

/// Get a single calculation policy by ID.
pub async fn get(
    ctx: &ServiceContext,
    policy_id: &str,
) -> Result<calculation_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SettingsRead).await?;

    calculation_policy::Entity::find_by_id(policy_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("calculation_policy", policy_id))
}

// ── Commands ────────────────────────────────────────────────────

/// Create a new calculation policy.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<calculation_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SettingsWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: String,
        target_period_id: Option<String>,
        rule_type: String,
        formula_expression: String,
        rounding_policy: Option<String>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid calculation policy payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = calculation_policy::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        target_period_id: req.target_period_id.unwrap_or_default(),
        name: req.name,
        version: 1,
        rule_type: req.rule_type,
        formula_expression: req.formula_expression,
        rounding_policy: req.rounding_policy.unwrap_or_else(|| "round_half_up".into()),
        created_at: now.clone(),
        updated_at: now,
    };

    let active: calculation_policy::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

/// Update a calculation policy.
pub async fn update(
    ctx: &ServiceContext,
    policy_id: &str,
    data: serde_json::Value,
) -> Result<calculation_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SettingsWrite).await?;

    let existing = calculation_policy::Entity::find_by_id(policy_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("calculation_policy", policy_id))?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: Option<String>,
        target_period_id: Option<String>,
        rule_type: Option<String>,
        formula_expression: Option<String>,
        rounding_policy: Option<String>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update payload: {e}")))?;

    let old_version = existing.version;
    let mut active: calculation_policy::ActiveModel = existing.into();
    if let Some(name) = req.name { active.name = Set(name); }
    if let Some(pid) = req.target_period_id { active.target_period_id = Set(pid); }
    if let Some(rt) = req.rule_type { active.rule_type = Set(rt); }
    if let Some(fe) = req.formula_expression { active.formula_expression = Set(fe); }
    if let Some(rp) = req.rounding_policy { active.rounding_policy = Set(rp); }
    active.version = Set(old_version + 1);
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

/// Delete a calculation policy.
pub async fn delete(ctx: &ServiceContext, policy_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::SettingsWrite).await?;

    calculation_policy::Entity::delete_by_id(policy_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}
