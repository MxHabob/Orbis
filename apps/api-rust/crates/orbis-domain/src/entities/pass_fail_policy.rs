use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassFailPolicy {
    pub id: Uuid,
    pub assessment_scheme_id: Uuid,
    pub pass_score_threshold: f64,
    pub attendance_requirement: Option<f64>,
    pub rules_json: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
