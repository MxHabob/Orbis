use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_hall_allocations")]
#[schema(as = ExamHallAllocation)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub exam_hall_id: String,
    pub exam_session_id: String,
    pub enrollment_id: String,
    pub seating_number_id: Option<String>,
    pub seat_position: Option<String>,
    pub is_present: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::exam_hall::Entity",
        from = "Column::ExamHallId",
        to = "super::exam_hall::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ExamHall,
    #[sea_orm(
        belongs_to = "super::enrollment::Entity",
        from = "Column::EnrollmentId",
        to = "super::enrollment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Enrollment,
}

impl Related<super::exam_hall::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamHall.def()
    }
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
