use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "grades")]
#[schema(as = Grade)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub enrollment_id: String,
    pub assessment_id: Option<String>,
    pub class_subject_id: Option<String>,
    pub subject_name: String,
    pub subject_name_ar: Option<String>,
    pub score: f64,
    pub max_score: f64,
    pub weight: f64,
    pub grade_letter: Option<String>,
    pub grade_points: Option<f64>,
    pub exam_type: String,
    pub semester: String,
    pub notes: Option<String>,
    pub graded_by: String,
    pub graded_at: String,
    pub is_final: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
