//! IAM response types.

use serde::Serialize;

/// Authenticated user response.
#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: String,
    pub org_id: String,
    pub role: String,
    pub expires_at: String,
}
