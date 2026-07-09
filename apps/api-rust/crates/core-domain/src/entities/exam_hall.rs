use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_halls")]
#[schema(as = ExamHall)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub room_id: String,
    pub hall_number: String,
    pub capacity: i32,
    pub allocated_count: i32,
    pub seating_arrangement: Option<String>,
    pub floor_number: Option<i32>,
    pub building: Option<String>,
    pub is_accessible: i32,
    pub status: String,
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
        belongs_to = "super::room::Entity",
        from = "Column::RoomId",
        to = "super::room::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Room,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::room::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
