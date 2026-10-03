use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "promotion_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub job_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Uuid,
    pub source_class_id: Option<Uuid>,
    pub target_class_id: Option<Uuid>,
    pub decision: String,
    pub reason: String,
    pub manual_override: bool,
    pub override_by: Option<Uuid>,
    pub override_reason: Option<String>,
    pub final_gpa: Option<Decimal>,
    pub attendance_rate: Option<Decimal>,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
