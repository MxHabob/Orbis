use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_schedules")]
#[schema(as = ExamSchedule)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub exam_session_id: String,
    pub exam_subject_id: String,
    pub exam_date: String,
    pub start_time: String,
    pub end_time: String,
    pub period: String,
    pub applicable_class_ids: Option<String>,
    pub room_ids: Option<String>,
    pub status: String,
    pub postponed_to: Option<String>,
    pub postponement_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exam_session::Entity",
        from = "Column::ExamSessionId",
        to = "super::exam_session::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ExamSession,
    #[sea_orm(
        belongs_to = "super::exam_subject::Entity",
        from = "Column::ExamSubjectId",
        to = "super::exam_subject::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ExamSubject,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::exam_subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSubject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
