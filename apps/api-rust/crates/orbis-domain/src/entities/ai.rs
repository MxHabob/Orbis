use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiAgent {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub model_name: String,      // e.g. "gemini-1.5-pro", "gpt-4"
    pub temperature: f64,
    pub system_prompt: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiInsight {
    pub id: Uuid,
    pub org_id: Uuid,
    pub agent_id: Uuid,
    pub entity_type: String,     // e.g. "student", "class"
    pub entity_id: Uuid,
    pub insight_type: String,    // e.g. "academic_risk", "performance_summary"
    pub content: String,
    pub confidence_score: f64,
    pub created_at: DateTime<Utc>,
}
