use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "attendance")]
#[schema(as = Attendance)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub cycle_id: String,
    pub enrollment_id: String,
    pub class_id: String,
    pub date: String,
    pub status: String,
    pub check_in_time: Option<String>,
    pub check_out_time: Option<String>,
    pub period: Option<String>,
    pub subject_name: Option<String>,
    pub recorded_by: String,
    pub notes: Option<String>,
    pub excuse_type: Option<String>,
    pub excuse_document: Option<String>,
    pub is_excused: i32,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::enrollment::Entity",
        from = "Column::EnrollmentId",
        to = "super::enrollment::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Enrollment,
    #[sea_orm(
        belongs_to = "super::class::Entity",
        from = "Column::ClassId",
        to = "super::class::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Class,
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
