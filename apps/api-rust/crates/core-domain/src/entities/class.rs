use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "classes")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub unit_id: String,
    pub cycle_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub homeroom_teacher_id: Option<String>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub room_id: Option<String>,
    pub color: Option<String>,
    pub schedule_template: Option<String>,
    pub is_active: i32,
    pub is_deleted: i32,
    pub deleted_at: Option<String>,
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
    #[sea_orm(
        belongs_to = "super::academic_cycle::Entity",
        from = "Column::CycleId",
        to = "super::academic_cycle::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AcademicCycle,
    #[sea_orm(
        belongs_to = "super::org_unit::Entity",
        from = "Column::UnitId",
        to = "super::org_unit::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    OrgUnit,
    #[sea_orm(has_many = "super::enrollment::Entity")]
    Enrollment,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::academic_cycle::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AcademicCycle.def()
    }
}

impl Related<super::org_unit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OrgUnit.def()
    }
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
