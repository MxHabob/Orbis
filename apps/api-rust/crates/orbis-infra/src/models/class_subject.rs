use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "class_subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub class_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub primary_teacher_id: Uuid,
    pub assistant_teacher_id: Option<Uuid>,
    pub substitute_teacher_id: Option<Uuid>,
    pub grade_groups: Option<serde_json::Value>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub scheme_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
