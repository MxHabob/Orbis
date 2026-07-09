use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "user_roles")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: String,
    pub role: String,
    pub org_id: String,
    pub scope_type: String,
    pub scope_id: Option<String>,
    pub valid_from: Option<String>,
    pub valid_until: Option<String>,
    pub is_primary: i32,
    pub assigned_by: Option<String>,
    pub assigned_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
