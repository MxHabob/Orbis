use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Cross-year student identity. Tracks a student across all academic years.
/// Lives in `orbis_registry.db`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "student_registry")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub user_id: String,
    pub student_number: String,
    pub full_name: String,
    pub name_ar: Option<String>,
    pub national_id: Option<String>,
    /// Status: active | graduated | withdrawn | transferred_out
    pub status: String,
    pub first_enrollment_year_id: Option<String>,
    pub current_year_id: Option<String>,
    pub current_level: Option<String>,
    pub current_class_name: Option<String>,
    pub years_enrolled: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
