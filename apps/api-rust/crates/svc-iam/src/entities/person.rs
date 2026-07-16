use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "persons")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub party_id: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: Option<String>,
    pub birth_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::party::Entity",
        from = "Column::PartyId",
        to = "super::party::Column::Id"
    )]
    Party,
}

impl Related<super::party::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Party.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
