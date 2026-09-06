use std::sync::Arc;
use chrono::Utc;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::entities::session::Session;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::repositories::{UserRepository, SessionRepository};
use crate::ports::security::{TokenService, PasswordHasher};
use crate::ports::event_bus::EventBus;

pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

pub struct LoginUseCase {
    pub user_repo: Arc<dyn UserRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub token_service: Arc<dyn TokenService>,
    pub event_bus: Arc<dyn EventBus>,
}

impl LoginUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        email: &str,
        password: &str,
    ) -> Result<LoginResponse, DomainError> {
        let user = self.user_repo.find_by_email(email).await?
            .ok_or_else(|| DomainError::Unauthorized("Invalid email or password".into()))?;

        let hash = user.password_hash.as_ref()
            .ok_or_else(|| DomainError::Unauthorized("Password authentication not supported for this user".into()))?;

        let is_valid = self.password_hasher.verify(password, hash).await?;
        if !is_valid {
            return Err(DomainError::Unauthorized("Invalid email or password".into()));
        }

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = "super_admin".to_string(); // TODO: fetch from user_role_assignments

        let access_token = self.token_service.generate_access_token(&user_id_str, &org_id_str, &role_str).await?;
        let refresh_token = self.token_service.generate_refresh_token(&user_id_str, &org_id_str, &role_str).await?;

        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = Session {
            id: session_id,
            user_id: user.id,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            device_id: None,
            family_id: None,
            is_revoked: false,
            is_current: true,
            is_trusted: false,
            mfa_verified: false,
            expires_at: now + chrono::Duration::hours(2),
            refresh_expires_at: Some(now + chrono::Duration::days(7)),
            last_active_at: Some(now),
            created_at: now,
        };

        self.session_repo.save(&session).await?;

        let event = DomainEvent::UserLoggedIn {
            user_id: user.id,
            org_id: user.org_id,
            session_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(LoginResponse {
            token: access_token,
            refresh_token,
        })
    }
}
