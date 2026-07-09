use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "document_templates")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub name: String,
    pub entity_type: String,
    pub layout_type: String,
    pub content: String,
    pub page_size: String,
    pub orientation: String,
    pub margins: String,
    pub is_default: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
