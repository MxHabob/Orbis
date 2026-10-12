use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Per-student progression decision within a progression job.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "progression_details")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub job_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Uuid,
    pub source_section_id: Option<Uuid>,
    pub target_section_id: Option<Uuid>,
    /// promoted, retained, graduated, excluded
    pub decision: String,
    pub reason: String,
    pub manual_override: bool,
    pub override_by: Option<Uuid>,
    pub override_reason: Option<String>,
    pub final_gpa: Option<Decimal>,
    pub attendance_rate: Option<Decimal>,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::progression_job::Entity",
        from = "Column::JobId",
        to = "super::progression_job::Column::Id"
    )]
    Job,
    #[sea_orm(
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
}

impl Related<super::progression_job::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Job.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
