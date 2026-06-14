use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "academic_cycles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub cycle_type: String,
    pub parent_id: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_date: Option<String>,
    pub hijri_end_date: Option<String>,
    pub exam_start_date: Option<String>,
    pub exam_end_date: Option<String>,
    pub weight: Option<i32>,
    pub grading_system: String,
    pub passing_grade: Option<i32>,
    pub max_grade: Option<i32>,
    pub is_current: i32,
    pub is_locked: i32,
    pub is_enrollment_open: i32,
    pub settings: Option<String>,
    pub sort_order: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Organization,
    #[sea_orm(has_many = "super::class::Entity")]
    Class,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::class::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Class.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
