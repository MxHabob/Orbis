use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, utoipa::ToSchema)]
#[sea_orm(table_name = "system_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: Option<String>,
    pub level: String,
    pub category: String,
    pub message: String,
    pub details: Option<String>,
    pub stack_trace: Option<String>,
    pub user_id: Option<String>,
    pub request_id: Option<String>,
    pub endpoint: Option<String>,
    pub method: Option<String>,
    pub duration_ms: Option<i32>,
    pub memory_usage_mb: Option<i32>,
    pub cpu_usage_percent: Option<i32>,
    pub timestamp: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
