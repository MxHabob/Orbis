use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_agent_sessions")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    /// "assistant" | "autopilot" | "analyst" | "guardian" | "external"
    pub agent_type: String,
    pub agent_version: Option<String>,
    pub acting_as_user_id: String,
    pub acting_as_role: String,
    pub conversation_id: Option<String>,
    /// "active" | "completed" | "suspended" | "revoked"
    pub status: String,
    pub max_risk_level: String,
    pub requires_approval_above: String,
    pub actions_executed: i32,
    pub actions_suggested: i32,
    pub started_at: String,
    pub last_active_at: String,
    pub ended_at: Option<String>,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Agent type constants.
pub mod agent_type {
    pub const ASSISTANT: &str = "assistant";
    pub const AUTOPILOT: &str = "autopilot";
    pub const ANALYST: &str = "analyst";
    pub const GUARDIAN: &str = "guardian";
    pub const EXTERNAL: &str = "external";
}

/// Session status constants.
pub mod session_status {
    pub const ACTIVE: &str = "active";
    pub const COMPLETED: &str = "completed";
    pub const SUSPENDED: &str = "suspended";
    pub const REVOKED: &str = "revoked";
}
