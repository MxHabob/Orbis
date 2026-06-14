use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "attendance_summary")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub enrollment_id: String,
    pub cycle_id: String,
    pub total_lessons: Option<i32>,
    pub present_lessons: Option<i32>,
    pub absent_lessons: Option<i32>,
    pub late_lessons: Option<i32>,
    pub excused_lessons: Option<i32>,
    pub late_minutes: Option<i32>,
    pub attendance_rate: Option<f64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
