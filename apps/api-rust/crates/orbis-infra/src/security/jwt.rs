use async_trait::async_trait;
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use orbis_app::ports::security::{TokenService, TokenClaims};
use orbis_domain::errors::DomainError;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    org_id: String,
    role: String,
    exp: usize,
    iat: usize,
}

pub struct JwtTokenService {
    secret: String,
    access_token_expiry_hours: usize,
    refresh_token_expiry_days: usize,
}

impl JwtTokenService {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            access_token_expiry_hours: 2,
            refresh_token_expiry_days: 7,
        }
    }
}

#[async_trait]
impl TokenService for JwtTokenService {
    async fn generate_access_token(&self, user_id: &str, org_id: &str, role: &str) -> Result<String, DomainError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let exp = now + (self.access_token_expiry_hours * 3600);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            org_id: org_id.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| DomainError::Internal(format!("Failed to generate token: {}", e)))
    }

    async fn generate_refresh_token(&self, user_id: &str, org_id: &str, role: &str) -> Result<String, DomainError> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let exp = now + (self.refresh_token_expiry_days * 24 * 3600);

        let claims = JwtClaims {
            sub: user_id.to_string(),
            org_id: org_id.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| DomainError::Internal(format!("Failed to generate token: {}", e)))
    }

    async fn validate_token(&self, token: &str) -> Result<TokenClaims, DomainError> {
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| DomainError::Unauthorized("Invalid or expired token".to_string()))?;

        Ok(TokenClaims {
            sub: token_data.claims.sub,
            org_id: token_data.claims.org_id,
            role: token_data.claims.role,
        })
    }
}
