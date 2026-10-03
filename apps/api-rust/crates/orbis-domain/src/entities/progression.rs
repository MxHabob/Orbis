use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionDecision {
    Promote,
    Retain,
    Graduate,
    Exclude,
}

impl std::fmt::Display for PromotionDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PromotionDecision::Promote => write!(f, "promote"),
            PromotionDecision::Retain => write!(f, "retain"),
            PromotionDecision::Graduate => write!(f, "graduate"),
            PromotionDecision::Exclude => write!(f, "exclude"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionJobStatus {
    Draft,
    PreviewReady,
    Executing,
    Completed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionJob {
    pub id: Uuid,
    pub org_id: Uuid,
    pub source_period_id: Uuid,
    pub target_period_id: Option<Uuid>,
    pub name: String,
    pub status: PromotionJobStatus,
    pub total_students: u32,
    pub promoted_count: u32,
    pub retained_count: u32,
    pub graduated_count: u32,
    pub excluded_count: u32,
    pub executed_by: Option<Uuid>,
    pub executed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionDetail {
    pub id: Uuid,
    pub job_id: Uuid,
    pub student_id: Uuid,
    pub enrollment_id: Uuid,
    pub source_class_id: Option<Uuid>,
    pub target_class_id: Option<Uuid>,
    pub decision: PromotionDecision,
    pub reason: String,
    pub manual_override: bool,
    pub override_by: Option<Uuid>,
    pub override_reason: Option<String>,
    pub final_gpa: Option<rust_decimal::Decimal>,
    pub attendance_rate: Option<rust_decimal::Decimal>,
    pub created_at: DateTime<Utc>,
}
