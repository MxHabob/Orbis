use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "groups")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub status: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::organization::Entity",
        from = "Column::OrgId",
        to = "super::organization::Column::Id"
    )]
    Organization,
    #[sea_orm(has_many = "super::user_group_membership::Entity")]
    UserGroupMembership,
    #[sea_orm(has_many = "super::group_permission::Entity")]
    GroupPermission,
}

impl Related<super::organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Organization.def()
    }
}

impl Related<super::user_group_membership::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroupMembership.def()
    }
}

impl Related<super::group_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::GroupPermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
