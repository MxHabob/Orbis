use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MfaType {
    Totp,
    WebAuthn,
    Sms,
}

impl std::fmt::Display for MfaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Totp => write!(f, "totp"),
            Self::WebAuthn => write!(f, "webauthn"),
            Self::Sms => write!(f, "sms"),
        }
    }
}

impl From<&str> for MfaType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "totp" => Self::Totp,
            "webauthn" => Self::WebAuthn,
            "sms" => Self::Sms,
            _ => Self::Totp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaFactor {
    pub id: Uuid,
    pub user_id: Uuid,
    pub factor_type: MfaType,
    pub factor_name: String,
    pub secret_encrypted: String,
    pub is_verified: bool,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaRecoveryCode {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code_hash: String,
    pub is_used: bool,
    pub created_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaEnrollmentResult {
    pub factor_id: Uuid,
    pub secret: String,
    pub qr_code_uri: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaChallengeResponse {
    pub status: String, // "mfa_required"
    pub mfa_token: String,
    pub available_factors: Vec<String>,
}
