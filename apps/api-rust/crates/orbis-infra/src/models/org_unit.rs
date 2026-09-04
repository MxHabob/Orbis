use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "org_units")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub org_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub depth: i32,
    pub name: String,
    pub name_ar: Option<String>,
    pub code: Option<String>,
    pub unit_type: String,
    pub head_id: Option<Uuid>,
    pub capacity: Option<i32>,
    pub room_number: Option<String>,
    pub floor: Option<i32>,
    pub building: Option<String>,
    pub settings: Option<serde_json::Value>,
    pub local_policies: Option<serde_json::Value>,
    pub data_isolation: String,
    pub children_count: i32,
    pub descendant_count: i32,
    pub sort_order: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
