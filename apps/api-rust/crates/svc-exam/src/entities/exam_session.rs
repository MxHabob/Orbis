use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_sessions")]
#[schema(as = ExamSession)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub session_type: String,
    pub status: String,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub applicable_levels: Option<String>,
    pub applicable_class_ids: Option<String>,
    pub settings: Option<String>,
    pub grading_deadline: Option<String>,
    pub results_publish_date: Option<String>,
    pub is_results_published: i32,
    pub created_by: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
