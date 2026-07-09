use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "org_academic_policies")]
#[schema(as = OrgAcademicPolicy)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,

    // Grading defaults
    pub default_grading_system: String,
    pub default_passing_grade: i32,
    pub default_max_grade: i32,

    // Enrollment policies
    pub enrollment_auto_close_on_capacity: i32,
    pub allow_late_enrollment: i32,
    pub late_enrollment_requires_approval: i32,

    // Grade entry policies
    pub grade_entry_auto_close_on_deadline: i32,
    pub allow_grade_modification_after_close: i32,
    pub grade_modification_requires_approval: i32,

    // Calendar defaults
    pub default_event_execution_mode: String,

    // Extensibility
    pub settings: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
