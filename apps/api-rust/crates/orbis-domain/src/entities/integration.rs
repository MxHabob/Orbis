use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub id: Uuid,
    pub org_id: Uuid,
    pub provider_name: String, // e.g. "stripe", "mada", "gemini", "twilio"
    pub category: String,      // e.g. "payment", "sms", "ai"
    pub is_enabled: bool,
    pub env: String,           // e.g. "sandbox", "production"
    pub settings: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationSecret {
    pub id: Uuid,
    pub config_id: Uuid,
    pub secret_key: String, // Encrypted string
    pub key_name: String,   // e.g. "api_key", "webhook_secret"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
