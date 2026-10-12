use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "scheduled_lessons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub section_subject_id: Uuid,
    pub room_id: Option<Uuid>,
    pub time_slot_id: Uuid,
    pub teacher_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::section_subject::Entity",
        from = "Column::SectionSubjectId",
        to = "super::section_subject::Column::Id"
    )]
    SectionSubject,
}

impl Related<super::section_subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SectionSubject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
