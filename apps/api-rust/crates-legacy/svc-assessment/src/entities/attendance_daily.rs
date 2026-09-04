use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "attendance_daily")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub enrollment_id: String,
    pub date: String,
    pub status: String,
    pub lessons: Option<String>,
    pub total_lessons: Option<i32>,
    pub present_lessons: Option<i32>,
    pub absent_lessons: Option<i32>,
    pub late_lessons: Option<i32>,
    pub excused_lessons: Option<i32>,
    pub late_minutes: Option<i32>,
    pub excuse_type: Option<String>,
    pub excuse_document: Option<String>,
    pub excuse_notes: Option<String>,
    pub is_verified: i32,
    pub verified_by: Option<String>,
    pub verified_at: Option<String>,
    pub marked_by: String,
    pub marked_at: String,
    pub mark_method: Option<String>,
    pub device_id: Option<String>,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
