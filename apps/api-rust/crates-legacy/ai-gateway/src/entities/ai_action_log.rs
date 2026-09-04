use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// Every operation executed or suggested by an AI agent — with reasoning and rollback data.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "ai_action_log")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub session_id: String,
    pub correlation_id: Option<String>,
    pub action_code: String,
    pub input_data: Option<String>,
    pub output_data: Option<String>,
    pub reasoning: String,
    pub confidence: Option<f64>,
    /// "pending" | "approved" | "executed" | "rejected" | "failed" | "rolled_back"
    pub status: String,
    pub risk_level: String,
    pub approval_required: i32,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub affected_entities: Option<String>,
    pub idempotency_key: Option<String>,
    pub duration_ms: Option<i32>,
    pub error: Option<String>,
    pub is_reversible: i32,
    pub rollback_data: Option<String>,
    pub reversed_at: Option<String>,
    pub reversed_by: Option<String>,
    pub executed_at: String,
    pub created_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub mod action_status {
    pub const PENDING: &str = "pending";
    pub const APPROVED: &str = "approved";
    pub const EXECUTED: &str = "executed";
    pub const REJECTED: &str = "rejected";
    pub const FAILED: &str = "failed";
    pub const ROLLED_BACK: &str = "rolled_back";
}
