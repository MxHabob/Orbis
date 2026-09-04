//! Audit operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;
use shared_kernel::traits::Permission;
use crate::entities::audit_log;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, QueryOrder, QuerySelect};

pub async fn list(ctx: &ServiceContext) -> Result<Vec<audit_log::Model>, AppError> {
    ctx.permissions.check(&ctx.actor, Permission::AuditRead).await?;

    Ok(audit_log::Entity::find()
        .filter(audit_log::Column::OrgId.eq(&ctx.org_id))
        .order_by_desc(audit_log::Column::CreatedAt)
        .limit(100)
        .all(&ctx.db)
        .await?)
}
