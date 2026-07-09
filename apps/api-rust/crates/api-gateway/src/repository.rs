use sea_orm::{entity::prelude::*, DbErr, EntityTrait, QueryFilter, ColumnTrait, QueryOrder};
use crate::entities::{enrollment, audit_log};

#[derive(Clone)]
pub struct Repository {
    pub db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn get_enrollments_by_class(&self, class_id: &str) -> Result<Vec<enrollment::Model>, DbErr> {
        enrollment::Entity::find()
            .filter(enrollment::Column::ClassId.eq(class_id))
            .filter(enrollment::Column::Status.eq("active"))
            .all(&self.db)
            .await
    }

    pub async fn get_audit_logs(&self, org_id: &str) -> Result<Vec<audit_log::Model>, DbErr> {
        audit_log::Entity::find()
            .filter(audit_log::Column::OrgId.eq(org_id))
            .order_by_desc(audit_log::Column::CreatedAt)
            .limit(100)
            .all(&self.db)
            .await
    }
}
