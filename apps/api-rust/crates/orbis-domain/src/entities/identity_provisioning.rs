use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    Student,
    Guardian,
    Employee,
    Admin,
    Custom,
}

impl std::fmt::Display for ActorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorType::Student => write!(f, "student"),
            ActorType::Guardian => write!(f, "guardian"),
            ActorType::Employee => write!(f, "employee"),
            ActorType::Admin => write!(f, "admin"),
            ActorType::Custom => write!(f, "custom"),
        }
    }
}

impl From<&str> for ActorType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "student" => ActorType::Student,
            "guardian" | "parent" => ActorType::Guardian,
            "employee" | "staff" | "teacher" => ActorType::Employee,
            "admin" | "super_admin" | "org_admin" => ActorType::Admin,
            _ => ActorType::Custom,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryChannel {
    Auto,
    Email,
    Sms,
    WhatsApp,
    Manual,
}

impl std::fmt::Display for DeliveryChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeliveryChannel::Auto => write!(f, "auto"),
            DeliveryChannel::Email => write!(f, "email"),
            DeliveryChannel::Sms => write!(f, "sms"),
            DeliveryChannel::WhatsApp => write!(f, "whatsapp"),
            DeliveryChannel::Manual => write!(f, "manual"),
        }
    }
}

impl From<&str> for DeliveryChannel {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "email" => DeliveryChannel::Email,
            "sms" => DeliveryChannel::Sms,
            "whatsapp" => DeliveryChannel::WhatsApp,
            "manual" | "none" => DeliveryChannel::Manual,
            _ => DeliveryChannel::Auto,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialDeliveryStatus {
    Delivered,
    Failed(String),
    NotConfigured,
    Skipped,
}

impl std::fmt::Display for CredentialDeliveryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CredentialDeliveryStatus::Delivered => write!(f, "delivered"),
            CredentialDeliveryStatus::Failed(reason) => write!(f, "failed: {}", reason),
            CredentialDeliveryStatus::NotConfigured => write!(f, "not_configured"),
            CredentialDeliveryStatus::Skipped => write!(f, "skipped"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProvisioningResult {
    pub user_id: Uuid,
    pub username: String,
    /// Temporary plaintext password returned ONLY at the moment of creation/reset.
    /// Never persisted plaintext in the database or logged.
    pub temporary_password: Option<String>,
    pub must_change_password: bool,
    pub delivery_channel: DeliveryChannel,
    pub delivery_status: CredentialDeliveryStatus,
    pub created_at: DateTime<Utc>,
}
