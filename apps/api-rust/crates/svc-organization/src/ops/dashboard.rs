//! Dashboard stats operations.

use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait};
use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn get_stats(
    ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    use crate::entities::{academic_year, organization};
    use svc_academic::entities::{class, enrollment};
    use svc_iam::entities::user_role;

    // Get current year name
    let org = organization::Entity::find_by_id(&ctx.org_id).one(&ctx.db).await?;
    let mut current_period = "N/A".to_string();
    if let Some(org) = org {
        if let Some(year_id) = org.current_year_id {
            if let Some(year) = academic_year::Entity::find_by_id(year_id).one(&ctx.db).await? {
                current_period = year.name;
            }
        }
    }

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
        "active_period": current_period
    }))
}
