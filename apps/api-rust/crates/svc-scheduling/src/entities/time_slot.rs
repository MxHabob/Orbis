use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "time_slots")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub period_id: String,
    pub slot_number: i32,
    pub name: Option<String>,
    pub name_ar: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub duration_minutes: i32,
    pub is_break: i32,
    pub break_type: Option<String>,
    pub applicable_days: Option<String>,
    pub sort_order: Option<i32>,
    pub is_active: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
