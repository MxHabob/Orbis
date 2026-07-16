use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "autopilot_tasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub org_id: String,
    pub calendar_event_id: Option<String>,
    pub task_type: String,
    pub scheduled_at: String,
    pub deadline_at: Option<String>,
    pub target_entity_type: Option<String>,
    pub target_entity_id: Option<String>,
    pub params: Option<String>,
    pub prerequisites: Option<String>,
    /// "pending" | "ready" | "executing" | "completed" | "failed"
    /// | "skipped" | "suggested" | "approved" | "cancelled"
    pub status: String,
    pub execution_mode: String,
    pub attempts: i32,
    pub last_attempt_at: Option<String>,
    pub last_error: Option<String>,
    pub suggested_at: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub completed_at: Option<String>,
    pub result: Option<String>,
    pub idempotency_key: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Task status constants.
pub mod status {
    pub const PENDING: &str = "pending";
    pub const READY: &str = "ready";
    pub const EXECUTING: &str = "executing";
    pub const COMPLETED: &str = "completed";
    pub const FAILED: &str = "failed";
    pub const SKIPPED: &str = "skipped";
    pub const REQUIRES_APPROVAL: &str = "requires_approval";
    pub const APPROVED: &str = "approved";
    pub const CANCELLED: &str = "cancelled";
}
