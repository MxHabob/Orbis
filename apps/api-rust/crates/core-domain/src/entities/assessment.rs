use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "assessments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub class_subject_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub description: Option<String>,
    #[sea_orm(column_name = "type")]
    pub item_type: String,
    pub max_score: f64,
    pub passing_score: Option<f64>,
    pub weight: f64,
    pub grade_group_name: String,
    pub assessment_date: Option<String>,
    pub due_date: Option<String>,
    pub duration_minutes: Option<i32>,
    pub is_published: i32,
    pub published_at: Option<String>,
    pub published_by: Option<String>,
    pub is_locked: i32,
    pub locked_at: Option<String>,
    pub locked_by: Option<String>,
    pub grading_method: Option<String>,
    pub rubric_id: Option<String>,
    pub instructions: Option<String>,
    pub instructions_ar: Option<String>,
    pub attachments: Option<String>,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
