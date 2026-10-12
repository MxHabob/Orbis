use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

/// Unified subject-section-instructor assignment per academic period.
/// Replaces both section_subjects (school) and section_subjects (university).
/// - Schools: Math assigned to "Grade 1A" in Semester 1 with Teacher X
/// - Universities: CS101 offered as Section 01 in Fall 2025 with Prof Y
/// - Institutes: Module M1 assigned to Cohort 2025 in Block 1 with Trainer Z
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "section_subjects")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub section_id: Uuid,
    pub subject_id: Uuid,
    pub period_id: Uuid,
    pub instructor_id: Option<Uuid>,
    pub assistant_instructor_id: Option<Uuid>,
    pub substitute_instructor_id: Option<Uuid>,
    pub scheme_id: Option<Uuid>,
    pub weekly_hours: Option<i32>,
    pub theory_hours: Option<i32>,
    pub practical_hours: Option<i32>,
    pub assessment_config: Option<serde_json::Value>,
    pub grade_groups: Option<serde_json::Value>,
    pub capacity: Option<i32>,
    pub status: String,
    pub is_active: bool,
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
        belongs_to = "super::section::Entity",
        from = "Column::SectionId",
        to = "super::section::Column::Id"
    )]
    Section,
    #[sea_orm(
        belongs_to = "super::subject::Entity",
        from = "Column::SubjectId",
        to = "super::subject::Column::Id"
    )]
    Subject,
    #[sea_orm(
        belongs_to = "super::academic_period::Entity",
        from = "Column::PeriodId",
        to = "super::academic_period::Column::Id"
    )]
    Period,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::section::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Section.def()
    }
}

impl Related<super::subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subject.def()
    }
}

impl Related<super::academic_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Period.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
