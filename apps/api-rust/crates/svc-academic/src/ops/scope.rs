//! Shared organization-unit scope helpers for academic-owned data.
//!
//! Branch ownership is derived from the canonical relationship:
//! enrollment -> class -> org_unit. Consumers should use these helpers
//! instead of reimplementing scope checks for every dependent domain.

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

use crate::entities::{class, enrollment};

/// Ensure a class belongs to the current organization and active branch scope.
pub async fn ensure_class_scope(ctx: &ServiceContext, class_id: &str) -> Result<(), AppError> {
    let mut query = class::Entity::find()
        .filter(class::Column::Id.eq(class_id))
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .filter(class::Column::IsDeleted.eq(0));

    if let Some(org_unit_id) = &ctx.org_unit_id {
        query = query.filter(class::Column::UnitId.eq(org_unit_id));
    }

    query
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("class", class_id))?;
    Ok(())
}

/// Ensure an enrollment belongs to the current organization and active branch scope.
pub async fn ensure_enrollment_scope(
    ctx: &ServiceContext,
    enrollment_id: &str,
) -> Result<enrollment::Model, AppError> {
    let enrollment = enrollment::Entity::find()
        .filter(enrollment::Column::Id.eq(enrollment_id))
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("enrollment", enrollment_id))?;

    ensure_class_scope(ctx, &enrollment.class_id).await?;
    Ok(enrollment)
}

/// Return enrollment IDs visible in the current branch scope.
pub async fn scoped_enrollment_ids(ctx: &ServiceContext) -> Result<Vec<String>, AppError> {
    let class_ids = scoped_class_ids(ctx).await?;
    if class_ids.is_empty() && ctx.org_unit_id.is_none() {
        return Ok(Vec::new());
    }

    Ok(enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(enrollment::Column::ClassId.is_in(class_ids))
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|enrollment| enrollment.id)
        .collect())
}

/// Return class IDs visible in the current branch scope.
pub async fn scoped_class_ids(ctx: &ServiceContext) -> Result<Vec<String>, AppError> {
    let Some(org_unit_id) = &ctx.org_unit_id else {
        return Ok(Vec::new());
    };

    Ok(class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .filter(class::Column::UnitId.eq(org_unit_id))
        .filter(class::Column::IsDeleted.eq(0))
        .all(&ctx.db)
        .await?
        .into_iter()
        .map(|class| class.id)
        .collect())
}
