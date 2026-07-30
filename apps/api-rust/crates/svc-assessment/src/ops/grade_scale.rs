//! Grade Scale CRUD operations.
//!
//! Provides management of grade scales and their items.
//! Each organization can have multiple scales (percentage, GPA, pass/fail).

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::{grade_scale, grade_scale_item};

// ── Queries ─────────────────────────────────────────────────────

/// List all grade scales for the current organization.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<grade_scale::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    Ok(grade_scale::Entity::find()
        .filter(grade_scale::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?)
}

/// Get a single grade scale by ID.
pub async fn get(ctx: &ServiceContext, scale_id: &str) -> Result<grade_scale::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    grade_scale::Entity::find_by_id(scale_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("grade_scale", scale_id))
}

/// List all items for a given grade scale.
pub async fn list_items(
    ctx: &ServiceContext,
    scale_id: &str,
) -> Result<Vec<grade_scale_item::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    Ok(grade_scale_item::Entity::find()
        .filter(grade_scale_item::Column::ScaleId.eq(scale_id))
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Create a new grade scale.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<grade_scale::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: String,
        #[serde(rename = "type")]
        scale_type: String,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid grade scale payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = grade_scale::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        name: req.name,
        scale_type: req.scale_type,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: grade_scale::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

/// Update a grade scale.
pub async fn update(
    ctx: &ServiceContext,
    scale_id: &str,
    data: serde_json::Value,
) -> Result<grade_scale::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = grade_scale::Entity::find_by_id(scale_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("grade_scale", scale_id))?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: Option<String>,
        #[serde(rename = "type")]
        scale_type: Option<String>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid update payload: {e}")))?;

    let mut active: grade_scale::ActiveModel = existing.into();
    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(t) = req.scale_type {
        active.scale_type = Set(t);
    }
    active.updated_at = Set(Utc::now().to_rfc3339());


    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

/// Delete a grade scale and all its items.
pub async fn delete(ctx: &ServiceContext, scale_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    // Delete items first
    grade_scale_item::Entity::delete_many()
        .filter(grade_scale_item::Column::ScaleId.eq(scale_id))
        .exec(&ctx.db)
        .await?;

    grade_scale::Entity::delete_by_id(scale_id.to_string())
        .exec(&ctx.db)
        .await?;

    Ok(())
}

// ── Scale Item Commands ─────────────────────────────────────────

/// Add a new item to a grade scale.
pub async fn create_item(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<grade_scale_item::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        scale_id: String,
        grade_label: String,
        min_score: f64,
        max_score: f64,
        gpa_points: Option<f64>,
        is_passing: bool,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid scale item payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = grade_scale_item::Model {
        id: Uuid::new_v4().to_string(),
        scale_id: req.scale_id,
        grade_label: req.grade_label,
        min_score: req.min_score,
        max_score: req.max_score,
        gpa_points: Some(req.gpa_points.unwrap_or(0.0)),
        is_passing: if req.is_passing { 1 } else { 0 },
        created_at: now.clone(),
        updated_at: now,
    };

    let active: grade_scale_item::ActiveModel = model.into();
    let created = active.insert(&ctx.db).await?;
    Ok(created)
}

/// Delete a grade scale item.
pub async fn delete_item(ctx: &ServiceContext, item_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    grade_scale_item::Entity::delete_by_id(item_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}

