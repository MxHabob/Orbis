use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "academic_periods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String,
    /// Semantic function: teaching, examination, break, registration, preparatory
    pub period_kind: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,
    pub enrollment_starts_at: Option<DateTime<Utc>>,
    pub enrollment_ends_at: Option<DateTime<Utc>>,
    pub is_enrollment_open: bool,
    pub grade_entry_starts_at: Option<DateTime<Utc>>,
    pub grade_entry_ends_at: Option<DateTime<Utc>>,
    pub is_grade_entry_open: bool,
    pub exam_starts_at: Option<DateTime<Utc>>,
    pub exam_ends_at: Option<DateTime<Utc>>,
    pub status: String,
    pub is_locked: bool,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub sort_order: Option<i32>,
    pub metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
