use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "enrollments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub class_id: Uuid,
    #[sea_orm(unique)]
    pub student_number: String,
    pub status: String,
    pub enrollment_date: DateTime<Utc>,
    pub enrollment_type: String,
    pub withdrawal_date: Option<DateTime<Utc>>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
