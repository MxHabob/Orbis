use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Per-student promotion decision and outcome.
/// Lives in `orbis_registry.db`.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "promotion_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub job_id: String,
    pub student_id: String,
    pub student_name: String,
    pub student_name_ar: Option<String>,
    pub student_number: Option<String>,

    // Source (current year)
    pub source_enrollment_id: String,
    pub source_class_id: String,
    pub source_class_name: Option<String>,
    pub source_level: String,
    pub source_division: Option<String>,

    // Target (next year) — populated after execution
    pub target_enrollment_id: Option<String>,
    pub target_class_id: Option<String>,
    pub target_class_name: Option<String>,
    pub target_level: Option<String>,

    /// Decision: promoted | retained | graduated | excluded
    pub decision: String,
    pub reason: String,
    pub final_score: Option<i32>,
    pub attendance_rate: Option<f64>,

    // Manual override
    pub manual_override: i32,
    pub override_by: Option<String>,
    pub override_reason: Option<String>,
    /// The decision before any manual override was applied
    pub original_decision: Option<String>,

    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::promotion_job::Entity",
        from = "Column::JobId",
        to = "super::promotion_job::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    PromotionJob,
}

impl Related<super::promotion_job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PromotionJob.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
