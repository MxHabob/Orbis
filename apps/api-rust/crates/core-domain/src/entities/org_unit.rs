use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "org_units")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    #[sea_orm(column_name = "type")]
    pub unit_type: String,
    pub parent_id: Option<String>,
    pub path: String,
    pub depth: i32,
    pub head_id: Option<String>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub settings: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: i32,
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
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
