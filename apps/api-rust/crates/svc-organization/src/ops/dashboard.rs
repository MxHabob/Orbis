//! Dashboard stats operations.

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn get_stats(
    ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    use crate::entities::academic_cycle;
    use svc_academic::entities::{class, enrollment};
    use svc_iam::entities::user_role;

    // Get current cycle name
    let current_cycle = academic_cycle::Entity::find()
        .filter(academic_cycle::Column::OrgId.eq(&ctx.org_id))
        .filter(academic_cycle::Column::IsCurrent.eq(1))
        .one(&ctx.db)
        .await?
        .map(|c| c.name)
        .unwrap_or_else(|| "N/A".to_string());

    // Count active student enrollments
    let students_count = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .filter(enrollment::Column::Status.eq("active"))
        .count(&ctx.db)
        .await?;

    // Count classes
    let classes_count = class::Entity::find()
        .filter(class::Column::OrgId.eq(&ctx.org_id))
        .count(&ctx.db)
        .await?;

    // Count unique users assigned with role 'teacher'
    let teachers_count = user_role::Entity::find()
        .filter(user_role::Column::OrgId.eq(&ctx.org_id))
        .filter(user_role::Column::Role.eq("teacher"))
        .count(&ctx.db)
        .await?;

    Ok(serde_json::json!({
        "students_count": students_count,
        "classes_count": classes_count,
        "teachers_count": teachers_count,
        "active_cycle": current_cycle
    }))
}
