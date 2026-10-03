use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Browser,
    Unknown,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Desktop => write!(f, "desktop"),
            Self::Mobile => write!(f, "mobile"),
            Self::Tablet => write!(f, "tablet"),
            Self::Browser => write!(f, "browser"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for DeviceType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "desktop" => Self::Desktop,
            "mobile" => Self::Mobile,
            "tablet" => Self::Tablet,
            "browser" => Self::Browser,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: String,
    pub device_name: String,
    pub device_type: DeviceType,
    pub os: Option<String>,
    pub browser: Option<String>,
    pub client_ip: Option<String>,
    pub user_agent: Option<String>,
    pub fingerprint_hash: String,
    pub is_trusted: bool,
    pub trusted_until: Option<DateTime<Utc>>,
    pub first_seen_at: DateTime<Utc>,
    pub last_active_at: DateTime<Utc>,
}
