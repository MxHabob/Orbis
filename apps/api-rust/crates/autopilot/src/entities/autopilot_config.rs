use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "autopilot_config")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    /// "automatic" | "suggested" | "manual"
    pub mode: String,
    pub check_interval_minutes: i32,
    pub timezone: String,
    pub require_backup_before_critical: i32,
    pub max_auto_retries: i32,
    pub pause_on_error: i32,
    pub is_enabled: i32,
    pub last_tick_at: Option<String>,
    pub last_tick_result: Option<String>,
    pub paused_at: Option<String>,
    pub pause_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
