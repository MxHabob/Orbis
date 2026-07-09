use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a single academic year with its associated database file.
/// Lives in `orbis_registry.db`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "academic_years")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    #[sea_orm(unique)]
    pub db_filename: String,
    pub status: String,
    pub is_current: i32,
    pub archived_at: Option<String>,
    pub archived_by: Option<String>,
    pub promotion_source_year_id: Option<String>,
    pub db_size_bytes: Option<i64>,
    pub student_count: Option<i32>,
    pub teacher_count: Option<i32>,
    pub class_count: Option<i32>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
