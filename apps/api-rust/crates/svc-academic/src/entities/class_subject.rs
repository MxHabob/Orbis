use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "class_subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub period_id: String,
    pub primary_teacher_id: String,
    pub assistant_teacher_id: Option<String>,
    pub substitute_teacher_id: Option<String>,
    pub grade_groups: Option<String>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<String>,
    pub scheme_id: Option<String>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
