//! Operations for autopilot module.

use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use shared_kernel::errors::AppError;
use crate::entities::autopilot_task;

pub async fn get_tasks(
    db: &DatabaseConnection,
    org_id: &str,
) -> Result<Vec<autopilot_task::Model>, AppError> {
    let tasks = autopilot_task::Entity::find()
        .filter(autopilot_task::Column::OrgId.eq(org_id))
        .all(db)
        .await?;
    Ok(tasks)
}
