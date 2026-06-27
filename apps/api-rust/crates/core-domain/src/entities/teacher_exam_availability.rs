use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "teacher_exam_availability")]
#[schema(as = TeacherExamAvailability)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub teacher_id: String,
    pub date: String,
    pub is_available: i32,
    pub reason: Option<String>,
    pub approved_by: Option<String>,
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
        belongs_to = "super::user::Entity",
        from = "Column::TeacherId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Teacher,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teacher.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
