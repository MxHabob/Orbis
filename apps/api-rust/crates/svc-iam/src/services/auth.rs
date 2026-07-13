//! Authentication service — login, register, JWT management.
//!
//! Uses `AppError` instead of `String` for structured error handling.

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use shared_kernel::config::Config;
use shared_kernel::errors::AppError;
use crate::entities::{user, user_session, audit_log};
use crate::repository::{IamCommandRepo, IamQueryRepo};

/// JWT claims payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub org_id: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

/// Login response returned to the client.
#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct LoginResponse {
    #[schema(value_type = User)]
    pub user: user::Model,
    #[serde(rename = "access_token")]
    pub token: String,
    pub refresh_token: String,
}

/// Authentication service with CQRS repositories.
pub struct AuthService {
    pub cmd: Arc<IamCommandRepo>,
    pub query: Arc<IamQueryRepo>,
    pub cfg: Arc<Config>,
}

impl AuthService {
    pub fn new(cmd: Arc<IamCommandRepo>, query: Arc<IamQueryRepo>, cfg: Arc<Config>) -> Self {
        Self { cmd, query, cfg }
    }

    /// Authenticate a user by email and password.
    pub async fn login(&self, email: &str, password: &str) -> Result<LoginResponse, AppError> {
        // 1. Get user by email
        let user = self.query.get_user_by_email(email).await?
            .ok_or_else(|| AppError::unauthorized("invalid email or password"))?;

        // 2. Verify password
        let hash_str = user.password_hash.as_ref()
            .ok_or_else(|| AppError::unauthorized("password authentication not supported"))?;

        let valid = verify(password, hash_str)
            .map_err(|e| AppError::internal(e.to_string()))?;
        if !valid {
            return Err(AppError::unauthorized("invalid email or password"));
        }

        // 3. Generate JWT tokens
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
        let exp = now + (self.cfg.jwt_expiry_hours as usize * 3600);
        let refresh_exp = now + (self.cfg.jwt_refresh_days as usize * 24 * 3600);

        let claims = Claims {
            sub: user.id.clone(),
            org_id: user.org_id.clone(),
            role: user.primary_role.clone(),
            exp,
            iat: now,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
        ).map_err(|e| AppError::internal(e.to_string()))?;

        let refresh_claims = Claims {
            sub: user.id.clone(),
            org_id: user.org_id.clone(),
            role: user.primary_role.clone(),
            exp: refresh_exp,
            iat: now,
        };

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
        ).map_err(|e| AppError::internal(e.to_string()))?;

        // 4. Create session
        let session = user_session::Model {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            token: token.clone(),
            refresh_token: Some(refresh_token.clone()),
            device_id: None,
            device_type: None,
            device_name: None,
            os: None,
            browser: None,
            ip_address: None,
            user_agent: None,
            is_current: 1,
            is_trusted: 0,
            mfa_verified: 0,
            expires_at: Utc::now()
                .checked_add_signed(chrono::Duration::hours(self.cfg.jwt_expiry_hours))
                .unwrap_or_else(Utc::now)
                .to_rfc3339(),
            refresh_expires_at: Some(
                Utc::now()
                    .checked_add_signed(chrono::Duration::days(self.cfg.jwt_refresh_days))
                    .unwrap_or_else(Utc::now)
                    .to_rfc3339(),
            ),
            last_active_at: None,
            created_at: Utc::now().to_rfc3339(),
        };
        self.cmd.create_session(session).await?;

        // 5. Audit log (fire-and-forget pattern)
        let audit = audit_log::Model {
            id: Uuid::new_v4().to_string(),
            org_id: user.org_id.clone(),
            user_id: Some(user.id.clone()),
            action: "user.login".to_string(),
            entity_type: "user".to_string(),
            entity_id: user.id.clone(),
            changes: None,
            ip_address: None,
            user_agent: None,
            actor_type: "user".to_string(),
            actor_session_id: None,
            correlation_id: None,
            created_at: Utc::now().to_rfc3339(),
        };
        let _ = self.cmd.create_audit_log(audit).await;

        Ok(LoginResponse { user, token, refresh_token })
    }

    /// Validate a JWT token and return the associated user.
    pub async fn validate_token(&self, token_str: &str) -> Result<user::Model, AppError> {
        // 1. Check session exists
        self.query.get_session_by_token(token_str).await?
            .ok_or_else(|| AppError::unauthorized("session not found or inactive"))?;

        // 2. Parse and validate JWT
        let token_data = decode::<Claims>(
            token_str,
            &DecodingKey::from_secret(self.cfg.jwt_secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| AppError::unauthorized("invalid or expired token"))?;

        // 3. Load user
        let user = self.query.get_user_by_id(&token_data.claims.sub).await?
            .ok_or_else(|| AppError::not_found("user", &token_data.claims.sub))?;

        Ok(user)
    }

    /// Hash a password for storage.
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        hash(password, DEFAULT_COST)
            .map_err(|e| AppError::internal(e.to_string()))
    }

    /// Verify a password against a hash.
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash)
            .map_err(|e| AppError::internal(e.to_string()))
    }
}
