use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "grade_history")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub grade_id: String,
    pub score: Option<f64>,
    pub grade_letter: Option<String>,
    pub is_absent: Option<i32>,
    pub is_excused: Option<i32>,
    pub comment: Option<String>,
    pub changed_by: Option<String>,
    pub changed_at: String,
    pub change_reason: Option<String>,
    pub change_type: String,
    pub snapshot: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
