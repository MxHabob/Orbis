//! Assessment Scheme CRUD operations.
//!
//! Manages assessment schemes and their components (weighted assessment types
//! like homework, quizzes, exams, etc.). Each scheme defines the breakdown
//! structure for how final grades are composed.

use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::{assessment_scheme, assessment_scheme_component};

// ── Queries ─────────────────────────────────────────────────────

/// List all assessment schemes for the current organization.
pub async fn list(ctx: &ServiceContext) -> Result<Vec<assessment_scheme::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    Ok(assessment_scheme::Entity::find()
        .filter(assessment_scheme::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?)
}

/// Get a single assessment scheme by ID.
pub async fn get(ctx: &ServiceContext, scheme_id: &str) -> Result<assessment_scheme::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    assessment_scheme::Entity::find_by_id(scheme_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment_scheme", scheme_id))
}

/// List all components for a given assessment scheme.
pub async fn list_components(
    ctx: &ServiceContext,
    scheme_id: &str,
) -> Result<Vec<assessment_scheme_component::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    Ok(assessment_scheme_component::Entity::find()
        .filter(assessment_scheme_component::Column::SchemeId.eq(scheme_id))
        .all(&ctx.db)
        .await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Create a new assessment scheme.
pub async fn create(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<assessment_scheme::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        name: String,
        description: Option<String>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid assessment scheme payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = assessment_scheme::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        name: req.name,
        description: req.description,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: assessment_scheme::ActiveModel = model.into();
    let inserted = active.insert(&ctx.db).await?;
    Ok(inserted)
}

/// Update an existing assessment scheme.
pub async fn update(
    ctx: &ServiceContext,
    scheme_id: &str,
    data: serde_json::Value,
) -> Result<assessment_scheme::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = get(ctx, scheme_id).await?;
    let mut active: assessment_scheme::ActiveModel = existing.into();

    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
        active.name = Set(name.to_string());
    }
    if let Some(desc) = data.get("description") {
        active.description = Set(desc.as_str().map(|s| s.to_string()));
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

/// Delete an assessment scheme and its components.
pub async fn delete(ctx: &ServiceContext, scheme_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    // Delete components first
    assessment_scheme_component::Entity::delete_many()
        .filter(assessment_scheme_component::Column::SchemeId.eq(scheme_id))
        .exec(&ctx.db)
        .await?;

    // Delete scheme
    assessment_scheme::Entity::delete_by_id(scheme_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}

/// Add a component to an assessment scheme.
pub async fn create_component(
    ctx: &ServiceContext,
    data: serde_json::Value,
) -> Result<assessment_scheme_component::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    #[derive(serde::Deserialize)]
    struct Req {
        scheme_id: String,
        name: String,
        component_type: String,
        weight: f64,
        max_score: f64,
        passing_score: Option<f64>,
    }

    let req: Req = serde_json::from_value(data)
        .map_err(|e| AppError::validation(format!("Invalid scheme component payload: {e}")))?;

    let now = Utc::now().to_rfc3339();
    let model = assessment_scheme_component::Model {
        id: Uuid::new_v4().to_string(),
        scheme_id: req.scheme_id,
        name: req.name,
        component_type: req.component_type,
        weight: req.weight,
        max_score: req.max_score,
        passing_score: req.passing_score,
        created_at: now.clone(),
        updated_at: now,
    };

    let active: assessment_scheme_component::ActiveModel = model.into();
    let inserted = active.insert(&ctx.db).await?;
    Ok(inserted)
}

/// Update a scheme component.
pub async fn update_component(
    ctx: &ServiceContext,
    component_id: &str,
    data: serde_json::Value,
) -> Result<assessment_scheme_component::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = assessment_scheme_component::Entity::find_by_id(component_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment_scheme_component", component_id))?;

    let mut active: assessment_scheme_component::ActiveModel = existing.into();

    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
        active.name = Set(name.to_string());
    }
    if let Some(ct) = data.get("component_type").and_then(|v| v.as_str()) {
        active.component_type = Set(ct.to_string());
    }
    if let Some(w) = data.get("weight").and_then(|v| v.as_f64()) {
        active.weight = Set(w);
    }
    if let Some(ms) = data.get("max_score").and_then(|v| v.as_f64()) {
        active.max_score = Set(ms);
    }
    if let Some(ps) = data.get("passing_score") {
        active.passing_score = Set(ps.as_f64());
    }
    active.updated_at = Set(Utc::now().to_rfc3339());

    let updated = active.update(&ctx.db).await?;
    Ok(updated)
}

/// Delete a scheme component.
pub async fn delete_component(ctx: &ServiceContext, component_id: &str) -> Result<(), AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    assessment_scheme_component::Entity::delete_by_id(component_id.to_string())
        .exec(&ctx.db)
        .await?;
    Ok(())
}

