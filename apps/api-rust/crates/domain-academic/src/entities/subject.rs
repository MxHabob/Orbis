use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub unit_id: Option<String>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub is_core: i32,
    pub is_elective: i32,
    pub credit_hours: Option<i32>,
    pub requires_lab: i32,
    pub requires_practical: i32,
    pub max_students_per_class: Option<i32>,
    pub default_grade_groups: Option<String>,
    pub prerequisites: Option<String>,
    pub learning_outcomes: Option<String>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
