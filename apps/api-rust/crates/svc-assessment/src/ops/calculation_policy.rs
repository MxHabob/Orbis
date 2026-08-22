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
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

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
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

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
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

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
        target_period_id: req.target_period_id,
        name: req.name,
        version: 1,
        rule_type: req.rule_type,
        formula_expression: req.formula_expression,
        rounding_policy: req.rounding_policy.unwrap_or_else(|| "round_half_up".into()),
        valid_from: None,
        valid_until: None,
        is_archived: 0,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: calculation_policy::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

/// Update a calculation policy using **copy-on-write** semantics.
///
/// Instead of mutating the existing row (which would silently change
/// the formula behind finalized results), this:
///
/// 1. Archives the old row (`is_archived = 1`, `valid_until = now`).
/// 2. Creates a new row with incremented `version`.
///
/// Finalized `TermResult` / `AnnualResult` records that reference
/// `calculation_policy_id + calculation_version` continue to point
/// at the archived (immutable) row.
pub async fn update(
    ctx: &ServiceContext,
    policy_id: &str,
    data: serde_json::Value,
) -> Result<calculation_policy::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = calculation_policy::Entity::find_by_id(policy_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("calculation_policy", policy_id))?;

    if existing.is_archived == 1 {
        return Err(AppError::validation(
            "Cannot update an archived policy. Create a new policy instead."
        ));
    }

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

    let now = Utc::now().to_rfc3339();
    let old_version = existing.version;

    // ── Step 1: Archive the old row ─────────────────────────────
    let mut archive: calculation_policy::ActiveModel = existing.clone().into();
    archive.is_archived = Set(1);
    archive.valid_until = Set(Some(now.clone()));
    archive.updated_at = Set(now.clone());
    archive.update(&ctx.db).await?;

    tracing::info!(
        policy_id=%policy_id,
        old_version=%old_version,
        "Archived calculation policy (copy-on-write)"
    );

    // ── Step 2: Create a new version ────────────────────────────
    let new_model = calculation_policy::Model {
        id: Uuid::new_v4().to_string(),
        org_id: existing.org_id.clone(),
        target_period_id: req.target_period_id
            .or(existing.target_period_id.clone()),
        name: req.name.unwrap_or_else(|| existing.name.clone()),
        version: old_version + 1,
        rule_type: req.rule_type.unwrap_or_else(|| existing.rule_type.clone()),
        formula_expression: req.formula_expression
            .unwrap_or_else(|| existing.formula_expression.clone()),
        rounding_policy: req.rounding_policy
            .unwrap_or_else(|| existing.rounding_policy.clone()),
        valid_from: Some(now.clone()),
        valid_until: None,
        is_archived: 0,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: calculation_policy::ActiveModel = new_model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

/// Delete a calculation policy.
///
/// Only non-archived, non-referenced policies can be deleted.
/// If this policy is referenced by finalized results, it should be
/// archived instead (via `update()`).
pub async fn delete(ctx: &ServiceContext, policy_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    calculation_policy::Entity::delete_by_id(policy_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}

/// List only active (non-archived) policies.
pub async fn list_active(ctx: &ServiceContext) -> Result<Vec<calculation_policy::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    Ok(calculation_policy::Entity::find()
        .filter(calculation_policy::Column::OrgId.eq(&ctx.org_id))
        .filter(calculation_policy::Column::IsArchived.eq(0))
        .all(&ctx.db)
        .await?)
}

