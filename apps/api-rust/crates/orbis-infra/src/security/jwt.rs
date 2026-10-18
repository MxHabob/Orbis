use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use orbis_app::ports::security::{TokenClaims, TokenService};
use orbis_domain::errors::DomainError;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    org_id: String,
    role: String,
    exp: usize,
    iat: usize,
    #[serde(default)]
    purpose: Option<String>,
}

pub struct JwtTokenService {
    secret: String,
    access_token_expiry_hours: usize,
    refresh_token_expiry_days: usize,
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        let access_token_expiry_hours = std::env::var("JWT_ACCESS_EXPIRY_HOURS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(24 * 7); // 7 days default for seamless mobile session persistence

        let refresh_token_expiry_days = std::env::var("JWT_REFRESH_EXPIRY_DAYS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Self {
            secret,
            access_token_expiry_hours,
            refresh_token_expiry_days,
        }
    }
}

#[async_trait]
impl TokenService for JwtTokenService {
    async fn generate_access_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let exp = now + (self.access_token_expiry_hours * 3600);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            org_id: org_id.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
            purpose: None,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::Internal(format!("Failed to generate token: {}", e)))
    }

    async fn generate_refresh_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let exp = now + (self.refresh_token_expiry_days * 24 * 3600);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            org_id: org_id.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
            purpose: None,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::Internal(format!("Failed to generate token: {}", e)))
    }

    async fn generate_mfa_token(
        &self,
        user_id: &str,
        org_id: &str,
        role: &str,
    ) -> Result<String, DomainError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DomainError::Internal(format!("System clock error: {}", e)))?
            .as_secs() as usize;
        let claims = JwtClaims {
            sub: user_id.to_string(),
            org_id: org_id.to_string(),
            role: role.to_string(),
            exp: now + 5 * 60,
            iat: now,
            purpose: Some("mfa_challenge".to_string()),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| DomainError::Internal(format!("Failed to generate MFA token: {}", e)))
    }

    async fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::Unauthorized("Invalid or expired token".to_string()))?;

        if token_data.claims.purpose.is_some() {
            return Err(DomainError::Unauthorized(
                "Token cannot be used as an access token".to_string(),
            ));
        }

        Ok(TokenClaims {
            sub: token_data.claims.sub,
            org_id: token_data.claims.org_id,
            role: token_data.claims.role,
        })
    }

    async fn validate_mfa_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| DomainError::Unauthorized("Invalid or expired MFA token".to_string()))?;

        if token_data.claims.purpose.as_deref() != Some("mfa_challenge") {
            return Err(DomainError::Unauthorized(
                "Invalid MFA challenge token".to_string(),
            ));
        }

        Ok(TokenClaims {
            sub: token_data.claims.sub,
            org_id: token_data.claims.org_id,
            role: token_data.claims.role,
        })
    }
}
