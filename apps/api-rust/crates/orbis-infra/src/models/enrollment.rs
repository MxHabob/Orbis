use sea_orm::entity::prelude::*;

/// Unified enrollment — student registration in a section for a period.
/// Replaces both old `enrollments` and `enrollments`.
/// - Schools: student enrolled in "Grade 1A" for "Semester 1"
/// - Universities: student enrolled in "CS101 Section 01" for "Fall 2025" (attempt 1)
/// - Institutes: student enrolled in "Cohort 2025-A" for "Module Block 1"
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "enrollments")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Uuid,
    pub student_id: Uuid,
    pub section_id: Option<Uuid>,
    pub student_number: String,
    /// active, withdrawn, completed, failed, suspended
    pub status: String,
    pub study_mode: Option<String>,
    /// For university re-enrollment: attempt 1, 2, 3...
    pub attempt_number: i32,
    /// initial, transfer, readmission, repeat
    pub enrollment_type: Option<String>,
    pub is_attendance_exempt: Option<bool>,
    pub is_grade_exempt: Option<bool>,
    pub enrolled_at: DateTimeWithTimeZone,
    pub withdrawn_at: Option<DateTimeWithTimeZone>,
    pub withdrawal_reason: Option<String>,
    pub meta: Option<Json>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
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
        belongs_to = "super::student::Entity",
        from = "Column::StudentId",
        to = "super::student::Column::Id"
    )]
    Student,
    #[sea_orm(
        belongs_to = "super::academic_period::Entity",
        from = "Column::PeriodId",
        to = "super::academic_period::Column::Id"
    )]
    Period,
    #[sea_orm(
        belongs_to = "super::section::Entity",
        from = "Column::SectionId",
        to = "super::section::Column::Id"
    )]
    Section,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::student::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Student.def()
    }
}

impl Related<super::academic_period::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Period.def()
    }
}

impl Related<super::section::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Section.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
