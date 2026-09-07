use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "grades")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub period_id: Option<Uuid>,
    pub student_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub class_subject_id: Uuid,
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
    pub notes: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
