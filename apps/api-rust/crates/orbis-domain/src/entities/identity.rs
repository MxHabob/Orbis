use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityProvider {
    Password,
    Google,
    Apple,
    Microsoft,
    Custom(String),
}

impl std::fmt::Display for IdentityProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Password => write!(f, "password"),
            Self::Google => write!(f, "google"),
            Self::Apple => write!(f, "apple"),
            Self::Microsoft => write!(f, "microsoft"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for IdentityProvider {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "password" => Self::Password,
            "google" => Self::Google,
            "apple" => Self::Apple,
            "microsoft" => Self::Microsoft,
            other => Self::Custom(other.to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProfile {
    pub name: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub picture: Option<String>,
    pub locale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: IdentityProvider,
    pub provider_user_id: String,
    pub email: Option<String>,
    pub is_primary: bool,
    pub profile_data: Option<serde_json::Value>,
    pub connected_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}
