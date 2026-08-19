use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "scheduled_lessons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub class_id: String,
    pub class_subject_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub substitute_teacher_id: Option<String>,
    pub time_slot_id: String,
    pub day_of_week: i32,
    pub room_id: Option<String>,
    pub status: String,
    pub is_substituted: i32,
    pub substitution_reason: Option<String>,
    pub topic: Option<String>,
    pub topic_ar: Option<String>,
    pub objectives: Option<String>,
    pub materials: Option<String>,
    pub homework: Option<String>,
    pub homework_due_date: Option<String>,
    pub attendance_taken: i32,
    pub attendance_taken_at: Option<String>,
    pub attendance_taken_by: Option<String>,
    pub attendance_method: Option<String>,
    pub notes: Option<String>,
    pub cancellation_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
