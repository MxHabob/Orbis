//! Bulk export operations.

use shared_kernel::context::ServiceContext;
use shared_kernel::errors::AppError;

pub async fn students(
    ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
    use svc_academic::entities::{enrollment, class};
    use svc_iam::entities::user;

    let enrollments = enrollment::Entity::find()
        .filter(enrollment::Column::OrgId.eq(&ctx.org_id))
        .all(&ctx.db)
        .await?;

    let mut list = Vec::new();
    for e in enrollments {
        let u = user::Entity::find_by_id(&e.student_id)
            .one(&ctx.db)
            .await?;
            
        let c = class::Entity::find_by_id(&e.class_id)
            .one(&ctx.db)
            .await?;

        list.push(serde_json::json!({
            "id": e.id,
            "student_id": e.student_id,
            "student_number": e.student_number,
            "class_id": e.class_id,
            "status": e.status,
            "enrollment_date": e.enrollment_date,
            "student_name": u.as_ref().map(|u| u.full_name.clone()),
            "student_email": u.as_ref().and_then(|u| u.email.clone()),
            "class_name": c.as_ref().map(|c| c.name.clone()),
        }));
    }

    Ok(serde_json::json!({
        "success": true,
        "message": "Bulk export of students completed successfully",
        "data": list
    }))
}

pub async fn grades(
    _ctx: &ServiceContext,
) -> Result<serde_json::Value, AppError> {
    Err(AppError::internal("Bulk export of grades is not implemented"))
}
