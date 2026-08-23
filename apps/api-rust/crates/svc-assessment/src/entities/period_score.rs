use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "period_scores")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub student_id: String,
    pub period_id: String,
    pub class_subject_id: String,
    pub total_score: Option<f64>,
    pub grade_label: Option<String>,
    pub gpa_points: Option<f64>,
    pub is_passing: Option<i32>,
    pub rank_in_class: Option<i32>,
    pub rank_in_level: Option<i32>,
    pub calculation_version: i32,
    pub status: String,
    pub approved_at: Option<String>,
    pub approved_by: Option<String>,
    pub metadata: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
