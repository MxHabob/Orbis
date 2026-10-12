use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

/// Unified academic period — hierarchical periods within a calendar.
/// Replaces both old `academic_periods` and `academic_periods`.
/// Supports any nesting: Calendar → Semester → Month, or Calendar → Module, etc.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "academic_periods")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub calendar_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub period_type_id: Uuid,
    pub code: Option<String>,
    pub name: String,
    pub name_ar: Option<String>,
    /// Semantic function: teaching, examination, break, registration, preparatory
    pub period_kind: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
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
    /// planning, active, completed, locked
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
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(
        belongs_to = "super::academic_calendar::Entity",
        from = "Column::CalendarId",
        to = "super::academic_calendar::Column::Id"
    )]
    Calendar,
    #[sea_orm(
        belongs_to = "super::period_type::Entity",
        from = "Column::PeriodTypeId",
        to = "super::period_type::Column::Id"
    )]
    PeriodType,
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    Parent,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::academic_calendar::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Calendar.def()
    }
}

impl Related<super::period_type::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PeriodType.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
