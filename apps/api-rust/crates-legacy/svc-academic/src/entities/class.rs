use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "classes")]
#[schema(as = Class)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub unit_id: String,
    pub period_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub level: String,
    pub division: Option<String>,
    pub homeroom_teacher_id: Option<String>,
    pub capacity: Option<i32>,
    pub current_students: Option<i32>,
    pub room_id: Option<String>,
    pub color: Option<String>,
    pub is_active: i32,
    pub is_deleted: i32,
    pub deleted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::enrollment::Entity")]
    Enrollment,
}

impl Related<super::enrollment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Enrollment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
