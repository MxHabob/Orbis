use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Suggested,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    SendDelayNotification,
    TransitionPeriod,
    GenericWorkflow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopilotTask {
    pub id: Uuid,
    pub org_id: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub parameters: Option<Value>,
    pub scheduled_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub error_log: Option<String>,
}

impl AutopilotTask {
    pub fn new(org_id: Uuid, task_type: TaskType, scheduled_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            org_id,
            task_type,
            status: TaskStatus::Pending,
            parameters: None,
            scheduled_at,
            executed_at: None,
            error_log: None,
        }
    }
}
