use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "currencies")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub code: String,
    #[sea_orm(primary_key, auto_increment = false)]
    pub org_id: String,
    pub name: String,
    pub name_ar: Option<String>,
    pub symbol: String,
    pub symbol_position: String,
    pub decimal_places: i32,
    pub exchange_rate: f64,
    pub is_base: i32,
    pub is_active: i32,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
