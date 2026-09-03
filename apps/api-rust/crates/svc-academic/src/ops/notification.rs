//! Notification operations — in-app notifications for the org.

use sea_orm::{ActiveModelTrait, ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;

use crate::entities::notification;

// ── Queries ─────────────────────────────────────────────────────

pub async fn list(
    ctx: &ServiceContext,
) -> Result<Vec<notification::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::NotificationRead).await?;

    use sea_orm::QuerySelect;
    let query = notification::Entity::find()
        .filter(notification::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(notification::Column::CreatedAt)
        .limit(100);

    Ok(query.all(&ctx.db).await?)
}

// ── Commands ────────────────────────────────────────────────────

/// Mark a notification as read.
pub async fn mark_read(
    ctx: &ServiceContext,
    notification_id: &str,
) -> Result<notification::Model, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::NotificationRead).await?;

    let existing = notification::Entity::find()
        .filter(notification::Column::Id.eq(notification_id.to_string()))
        .filter(notification::Column::OrgId.eq(&ctx.org_id))
        .filter(notification::Column::UserId.eq(ctx.actor.as_str()))
        .one(&ctx.db)
        .await?
        .ok_or_else(|| AppError::not_found("notification", notification_id))?;

    let mut active: notification::ActiveModel = existing.into();
    active.is_read = Set(1);

    Ok(active.update(&ctx.db).await?)
}
