use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_proctors")]
#[schema(as = ExamProctor)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub exam_schedule_id: String,
    pub exam_hall_id: String,
    pub teacher_id: String,
    pub role: String,
    pub is_confirmed: i32,
    pub confirmed_at: Option<String>,
    pub attendance_status: Option<String>,
    pub check_in_time: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exam_schedule::Entity",
        from = "Column::ExamScheduleId",
        to = "super::exam_schedule::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ExamSchedule,
    #[sea_orm(
        belongs_to = "super::exam_hall::Entity",
        from = "Column::ExamHallId",
        to = "super::exam_hall::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    ExamHall,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::TeacherId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Teacher,
}

impl Related<super::exam_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSchedule.def()
    }
}

impl Related<super::exam_hall::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamHall.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teacher.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
