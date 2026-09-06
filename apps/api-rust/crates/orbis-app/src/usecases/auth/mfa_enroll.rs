use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use orbis_domain::entities::mfa::{MfaFactor, MfaType, MfaRecoveryCode, MfaEnrollmentResult};
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{MfaFactorRepository, MfaRecoveryCodeRepository, UserRepository};
use crate::ports::security::{TotpService, PasswordHasher};

pub struct EnrollTotpUseCase {
    pub user_repo: Arc<dyn UserRepository>,
    pub mfa_factor_repo: Arc<dyn MfaFactorRepository>,
    pub recovery_code_repo: Arc<dyn MfaRecoveryCodeRepository>,
    pub totp_service: Arc<dyn TotpService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
}

impl EnrollTotpUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        factor_name: Option<String>,
    ) -> Result<MfaEnrollmentResult, DomainError> {
        let user_id = ctx.actor.id().ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        let secret = self.totp_service.generate_secret();
        let account_name = user.email.unwrap_or_else(|| user.full_name.clone());
        let qr_code_uri = self.totp_service.generate_qr_uri("Orbis", &account_name, &secret);
        let raw_recovery_codes = self.totp_service.generate_recovery_codes(8);

        let factor_id = Uuid::new_v4();
        let now = Utc::now();

        let factor = MfaFactor {
            id: factor_id,
            user_id,
            factor_type: MfaType::Totp,
            factor_name: factor_name.unwrap_or_else(|| "Authenticator App".to_string()),
            secret_encrypted: secret.clone(),
            is_verified: false,
            is_default: true,
            created_at: now,
            last_used_at: None,
        };
        self.mfa_factor_repo.save(&factor).await?;

        let mut recovery_code_entities = Vec::new();
        for code in &raw_recovery_codes {
            let code_hash = self.password_hasher.hash(code).await?;
            recovery_code_entities.push(MfaRecoveryCode {
                id: Uuid::new_v4(),
                user_id,
                code_hash,
                is_used: false,
                created_at: now,
                used_at: None,
            });
        }
        self.recovery_code_repo.save_all(&recovery_code_entities).await?;

        Ok(MfaEnrollmentResult {
            factor_id,
            secret,
            qr_code_uri,
            recovery_codes: raw_recovery_codes,
        })
    }
}
