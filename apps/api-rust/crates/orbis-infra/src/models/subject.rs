use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub unit_id: Option<Uuid>,
    pub code: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub name_en: Option<String>,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub is_core: bool,
    pub is_elective: bool,
    pub credit_hours: Option<i32>,
    pub requires_lab: bool,
    pub requires_practical: bool,
    pub max_students_per_class: Option<i32>,
    pub default_grade_groups: Option<serde_json::Value>,
    pub prerequisites: Option<serde_json::Value>,
    pub learning_outcomes: Option<serde_json::Value>,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
