use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Academic Period — represents a time division within an academic year.
///
/// Supports hierarchical structures via `parent_id`:
///   Year → Semester → Quarter (school)
///   Year → Trimester (some schools)
///   Year → Semester → Module (university)
///   Year → Block (training institute)
///
/// Autopilot manages lifecycle transitions by reading dates directly:
///   enrollment_start/end, grade_entry_start/end, exam_start/end
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_periods")]
#[schema(as = AcademicPeriod)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub year_id: String,
    pub parent_id: Option<String>,

    // Identity
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub period_type: String, // semester, trimester, quarter, module, block, custom

    // Timeline
    pub start_date: String,
    pub end_date: String,
    pub hijri_start: Option<String>,
    pub hijri_end: Option<String>,

    // Enrollment window (Autopilot-managed)
    pub enrollment_start: Option<String>,
    pub enrollment_end: Option<String>,
    pub is_enrollment_open: i32,

    // Grade entry window (Autopilot-managed)
    pub grade_entry_start: Option<String>,
    pub grade_entry_end: Option<String>,
    pub is_grade_entry_open: i32,

    // Exam window
    pub exam_start: Option<String>,
    pub exam_end: Option<String>,

    // Grading config
    pub grading_system: String,
    pub passing_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub weight: Option<i32>,

    // Status
    pub status: String, // planned, active, grading, closed
    pub is_locked: i32,
    pub locked_at: Option<String>,
    pub locked_by: Option<String>,

    pub sort_order: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::academic_year::Entity",
        from = "Column::YearId",
        to = "super::academic_year::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AcademicYear,
}

impl Related<super::academic_year::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicYear.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
