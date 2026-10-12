use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "exam_schedules")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub exam_session_id: Uuid,
    pub section_subject_id: Uuid,
    pub exam_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exam_session::Entity",
        from = "Column::ExamSessionId",
        to = "super::exam_session::Column::Id"
    )]
    ExamSession,
    #[sea_orm(
        belongs_to = "super::section_subject::Entity",
        from = "Column::SectionSubjectId",
        to = "super::section_subject::Column::Id"
    )]
    SectionSubject,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::section_subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SectionSubject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
