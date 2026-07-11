//! Notification operations — in-app notifications for the org.

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::notification;

// ── Queries ─────────────────────────────────────────────────────

/// List notifications for the organization.
pub async fn list(
    ctx: &ServiceContext,
    limit: Option<u64>,
) -> Result<Vec<notification::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::NotificationRead).await?;

    let mut query = notification::Entity::find()
        .filter(notification::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(notification::Column::CreatedAt);

    if let Some(lim) = limit {
        use sea_orm::QuerySelect;
        query = query.limit(lim);
    }

    Ok(query.all(&ctx.db).await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Mark a notification as read.
pub async fn mark_read(
    ctx: &ServiceContext,
    notification_id: &str,
) -> Result<notification::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::NotificationRead).await?;

    let existing = notification::Entity::find_by_id(notification_id.to_string())
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("notification", notification_id))?;

    let mut active: notification::ActiveModel = existing.into();
    active.is_read = Set(1);

    Ok(active.update(&ctx.db).await?)
}
