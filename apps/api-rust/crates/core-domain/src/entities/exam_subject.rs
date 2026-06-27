use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "exam_subjects")]
#[schema(as = ExamSubject)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub exam_session_id: String,
    pub subject_id: String,
    pub class_subject_id: Option<String>,
    pub max_score: f64,
    pub passing_score: f64,
    pub practical_score: Option<f64>,
    pub oral_score: Option<f64>,
    pub coursework_score: Option<f64>,
    pub exam_written_score: Option<f64>,
    pub duration_minutes: i32,
    pub exam_type: String,
    pub is_included: i32,
    pub weight: f64,
    pub instructions: Option<String>,
    pub instructions_ar: Option<String>,
    pub sort_order: Option<i32>,
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
        belongs_to = "super::subject::Entity",
        from = "Column::SubjectId",
        to = "super::subject::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Subject,
}

impl Related<super::exam_session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ExamSession.def()
    }
}

impl Related<super::subject::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Subject.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
