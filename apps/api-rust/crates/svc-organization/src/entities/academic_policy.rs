use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Unified academic policies — consolidates:
///   - org_academic_policies (grading, enrollment rules)
///   - promotion_policies (passing score, attendance)
///   - autopilot_config (automation mode, toggles)
///
/// One row per organization. All academic behavior is configured here.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_policies")]
#[schema(as = AcademicPolicy)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    /// Branch-level policy override. NULL = org-wide default.
    pub org_unit_id: Option<String>,

    // ── Grading defaults ──
    pub default_grading_system: String,
    pub default_passing_grade: i32,
    pub default_max_grade: i32,

    // ── Enrollment policies ──
    pub enrollment_auto_close: i32,
    pub allow_late_enrollment: i32,
    pub late_enrollment_approval: i32,

    // ── Grade entry policies ──
    pub grade_entry_auto_close: i32,
    pub allow_grade_edit_after_close: i32,
    pub grade_edit_approval: i32,

    // ── Promotion policies ──
    pub passing_score: i32,
    pub min_attendance_rate: f64,
    pub auto_graduate_final_level: i32,
    pub final_level: Option<String>,
    pub exclude_withdrawn: i32,

    // ── Autopilot policies ──
    pub autopilot_mode: String, // automatic, suggested, manual
    pub autopilot_enabled: i32,
    pub max_auto_retries: i32,
    pub pause_on_error: i32,
    pub auto_open_enrollment: i32,
    pub auto_close_enrollment: i32,
    pub auto_open_grade_entry: i32,
    pub auto_close_grade_entry: i32,
    pub auto_lock_periods: i32,
    pub auto_advance_periods: i32,
    pub auto_archive_years: i32,
    pub grace_days_after_deadline: i32,
    pub notify_before_deadline_days: i32,

    // ── Period template ──
    pub default_template_id: Option<String>,

    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
