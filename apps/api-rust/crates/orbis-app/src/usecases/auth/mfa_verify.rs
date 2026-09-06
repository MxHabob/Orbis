use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use orbis_domain::errors::DomainError;
use orbis_domain::entities::session::Session;
use crate::context::RequestContext;
use crate::ports::repositories::{MfaFactorRepository, MfaRecoveryCodeRepository, UserRepository, SessionRepository, UserDeviceRepository};
use crate::ports::security::{TotpService, TokenService, PasswordHasher};

pub struct VerifyTotpEnrollmentUseCase {
    pub user_repo: Arc<dyn UserRepository>,
    pub mfa_factor_repo: Arc<dyn MfaFactorRepository>,
    pub totp_service: Arc<dyn TotpService>,
}

impl VerifyTotpEnrollmentUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        factor_id: Uuid,
        code: &str,
    ) -> Result<bool, DomainError> {
        let user_id = ctx.actor.id().ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let factor = self.mfa_factor_repo.find_by_id(factor_id).await?
            .ok_or_else(|| DomainError::NotFound("Factor not found".into()))?;

        if factor.user_id != user_id {
            return Err(DomainError::Forbidden("Invalid factor".into()));
        }

        let is_valid = self.totp_service.verify_code(&factor.secret_encrypted, code);
        if !is_valid {
            return Err(DomainError::Validation("Invalid verification code".into()));
        }

        let mut updated_factor = factor;
        updated_factor.is_verified = true;
        updated_factor.last_used_at = Some(Utc::now());
        self.mfa_factor_repo.save(&updated_factor).await?;

        if let Some(mut user) = self.user_repo.find_by_id(user_id).await? {
            user.mfa_enabled = true;
            user.updated_at = Utc::now();
            self.user_repo.save(&user).await?;
        }

        Ok(true)
    }
}

pub struct VerifyMfaChallengeUseCase {
    pub user_repo: Arc<dyn UserRepository>,
    pub mfa_factor_repo: Arc<dyn MfaFactorRepository>,
    pub recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
    pub device_repo: Arc<dyn UserDeviceRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub totp_service: Arc<dyn TotpService>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
}

pub struct VerifiedSessionResponse {
    pub token: String,
    pub refresh_token: String,
}

impl VerifyMfaChallengeUseCase {
    pub async fn execute(
        &self,
        user_id: Uuid,
        code: &str,
        device_id: Option<String>,
        remember_device: bool,
    ) -> Result<VerifiedSessionResponse, DomainError> {
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        let factors = self.mfa_factor_repo.find_by_user(user_id).await?;
        let verified_factor = factors.into_iter().find(|f| f.is_verified);

        let mut authenticated = false;

        if let Some(mut factor) = verified_factor {
            if self.totp_service.verify_code(&factor.secret_encrypted, code) {
                factor.last_used_at = Some(Utc::now());
                let _ = self.mfa_factor_repo.save(&factor).await;
                authenticated = true;
            }
        }

        // If TOTP didn't match, check recovery codes
        if !authenticated {
            let active_codes = self.recovery_code_repo.find_active_by_user(user_id).await?;
            for rc in active_codes {
                if self.password_hasher.verify(code, &rc.code_hash).await.unwrap_or(false) {
                    let _ = self.recovery_code_repo.mark_used(rc.id).await;
                    authenticated = true;
                    break;
                }
            }
        }

        if !authenticated {
            return Err(DomainError::Unauthorized("Invalid authentication code or recovery code".into()));
        }

        // If remember_device is requested, update/create device trust
        if let Some(dev_id) = device_id {
            if remember_device {
                if let Ok(Some(mut dev)) = self.device_repo.find_by_device_id(user_id, &dev_id).await {
                    dev.is_trusted = true;
                    dev.trusted_until = Some(Utc::now() + chrono::Duration::days(30));
                    dev.last_active_at = Utc::now();
                    let _ = self.device_repo.save(&dev).await;
                }
            }
        }

        let user_id_str = user.id.to_string();
        let org_id_str = user.org_id.to_string();
        let role_str = "super_admin".to_string();

        let access_token = self.token_service.generate_access_token(&user_id_str, &org_id_str, &role_str).await?;
        let refresh_token = self.token_service.generate_refresh_token(&user_id_str, &org_id_str, &role_str).await?;

        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4(),
            user_id: user.id,
            device_id: None,
            token: access_token.clone(),
            refresh_token: Some(refresh_token.clone()),
            family_id: Some(Uuid::new_v4().to_string()),
            is_current: true,
            is_trusted: remember_device,
            is_revoked: false,
            mfa_verified: true,
            expires_at: now + chrono::Duration::hours(24),
            refresh_expires_at: Some(now + chrono::Duration::days(30)),
            last_active_at: Some(now),
            created_at: now,
        };
        self.session_repo.save(&session).await?;

        Ok(VerifiedSessionResponse {
            token: access_token,
            refresh_token,
        })
    }
}
