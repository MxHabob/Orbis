use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "user_permissions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub permission_id: i32,
    pub grant_type: String,
    pub conditions: Option<String>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub reason: Option<String>,
    pub granted_by: Option<String>,
    pub granted_at: String,
    pub is_active: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
