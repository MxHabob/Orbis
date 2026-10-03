use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRule {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub trigger_event: String, // e.g. "StudentFeeGenerated"
    pub conditions: Option<serde_json::Value>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowAction {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub action_type: String, // e.g. "SendSms", "IssueCertificate"
    pub config: serde_json::Value,
    pub order_index: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub entity_id: Uuid,
    pub status: String, // e.g. "pending", "success", "failed"
    pub error_msg: Option<String>,
    pub executed_at: DateTime<Utc>,
}
