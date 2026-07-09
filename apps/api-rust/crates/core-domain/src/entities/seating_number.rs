use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "seating_numbers")]
#[schema(as = SeatingNumber)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub exam_session_id: String,
    pub enrollment_id: String,
    pub student_id: String,
    pub class_id: String,
    pub seating_number: String,
    pub sequence_number: i32,
    pub level: String,
    pub division: Option<String>,
    pub student_name_ar: Option<String>,
    pub is_active: i32,
    pub generated_at: String,
    pub generated_by: String,
    pub voided_at: Option<String>,
    pub voided_by: Option<String>,
    pub void_reason: Option<String>,
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
        belongs_to = "super::enrollment::Entity",
        from = "Column::EnrollmentId",
        to = "super::enrollment::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Enrollment,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
