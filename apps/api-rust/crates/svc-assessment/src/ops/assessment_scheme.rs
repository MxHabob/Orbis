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

use shared_kernel::query_filter::UniversalFilter;

/// List all assessment schemes for the current organization.
pub async fn list(ctx: &ServiceContext, filter: Option<UniversalFilter>) -> Result<Vec<assessment_scheme::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentRead).await?;

    let mut query = assessment_scheme::Entity::find()
        .filter(assessment_scheme::Column::OrgId.eq(&ctx.org_id));

    if let Some(f) = filter {
        let condition = f.build_condition(|field| match field {
            "name" => Some(assessment_scheme::Column::Name.into_expr().into()),
            "period_id" => Some(assessment_scheme::Column::PeriodId.into_expr().into()),
            _ => None,
        });
        query = query.filter(condition);
    }

    Ok(query.all(&ctx.db).await?)
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

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateAssessmentSchemePayload {
    pub name: String,
    pub description: Option<String>,
    pub period_id: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdateAssessmentSchemePayload {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct CreateSchemeComponentPayload {
    pub scheme_id: String,
    pub name: String,
    pub component_type: String,
    pub weight: f64,
    pub max_score: f64,
    pub passing_score: Option<f64>,
}

#[derive(serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct UpdateSchemeComponentPayload {
    pub name: Option<String>,
    pub component_type: Option<String>,
    pub weight: Option<f64>,
    pub max_score: Option<f64>,
    pub passing_score: Option<f64>,
}

/// Create a new assessment scheme.
pub async fn create(
    ctx: &ServiceContext,
    req: CreateAssessmentSchemePayload,
) -> Result<assessment_scheme::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let now = Utc::now().to_rfc3339();
    let model = assessment_scheme::Model {
        id: Uuid::new_v4().to_string(),
        org_id: ctx.org_id.clone(),
        period_id: req.period_id,
        name: req.name,
        description: req.description,
        is_archived: 0,
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
    req: UpdateAssessmentSchemePayload,
) -> Result<assessment_scheme::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = get(ctx, scheme_id).await?;
    let mut active: assessment_scheme::ActiveModel = existing.into();

    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if req.description.is_some() {
        active.description = Set(req.description);
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
    req: CreateSchemeComponentPayload,
) -> Result<assessment_scheme_component::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

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
    req: UpdateSchemeComponentPayload,
) -> Result<assessment_scheme_component::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AssessmentWrite).await?;

    let existing = assessment_scheme_component::Entity::find_by_id(component_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("assessment_scheme_component", component_id))?;

    let mut active: assessment_scheme_component::ActiveModel = existing.into();

    if let Some(name) = req.name {
        active.name = Set(name);
    }
    if let Some(ct) = req.component_type {
        active.component_type = Set(ct);
    }
    if let Some(w) = req.weight {
        active.weight = Set(w);
    }
    if let Some(ms) = req.max_score {
        active.max_score = Set(ms);
    }
    if req.passing_score.is_some() {
        active.passing_score = Set(req.passing_score);
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

/// Resolve scheme components for a class offering (class_subject → period → org default).
pub async fn list_components_for_class_subject(
    ctx: &ServiceContext,
    class_subject_id: &str,
    period_id: &str,
) -> Result<Vec<assessment_scheme_component::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::GradeRead).await?;
    let scheme_id = crate::ops::resolution::resolve_scheme_id(
        &ctx.db,
        &ctx.org_id,
        class_subject_id,
        period_id,
    )
    .await?;
    match scheme_id {
        Some(sid) => crate::ops::resolution::scheme_components(&ctx.db, &sid).await,
        None => Ok(Vec::new()),
    }
}

