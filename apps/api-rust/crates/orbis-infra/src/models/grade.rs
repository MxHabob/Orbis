use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "grades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub period_id: Option<Uuid>,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub section_subject_id: Uuid,
    pub component_id: Option<Uuid>,
    pub score: Decimal,
    pub max_score: Option<Decimal>,
    pub weight: Option<Decimal>,
    pub letter_grade: Option<String>,
    pub is_final: Option<bool>,
    pub is_excused: bool,
    pub is_exempt: Option<bool>,
    pub change_reason: Option<String>,
    pub recorded_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
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
        belongs_to = "super::section_subject::Entity",
        from = "Column::SectionSubjectId",
        to = "super::section_subject::Column::Id"
    )]
    SectionSubject,
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

impl Related<super::section_subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SectionSubject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
