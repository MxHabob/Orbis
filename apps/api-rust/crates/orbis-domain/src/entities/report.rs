use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: String, // e.g. "student_transcript", "financial_summary"
    pub template_body: String, // Base64 encoded PDF/HTML template or JSON
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportJob {
    pub id: Uuid,
    pub org_id: Uuid,
    pub template_id: Uuid,
    pub requested_by: Uuid,
    pub parameters: serde_json::Value,
    pub status: String, // "pending", "processing", "completed", "failed"
    pub file_path: Option<String>,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}
