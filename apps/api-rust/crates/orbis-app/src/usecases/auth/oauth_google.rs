use std::sync::Arc;
use chrono::Utc;
use uuid::Uuid;
use orbis_domain::entities::identity::{UserIdentity, IdentityProvider};
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{UserIdentityRepository, UserRepository, SessionRepository, IntegrationRepository};
use crate::ports::security::{OidcService, TokenService};
use orbis_domain::entities::session::Session;

/// Validates organization-level identity integration settings (SSO enabled, domain restriction)
async fn verify_org_identity_config(
    integration_repo: &Option<Arc<dyn IntegrationRepository>>,
    org_id: Uuid,
    provider_name: &str,
    email: &str,
) -> Result<(), DomainError> {
    if let Some(repo) = integration_repo {
        let org_config = repo.find_config_by_provider(org_id, &format!("{}-workspace", provider_name)).await?
            .or(repo.find_config_by_provider(org_id, provider_name).await?);

        if let Some(config) = org_config {
            if !config.is_enabled {
                return Err(DomainError::Forbidden(format!(
                    "{} integration is disabled for this organization",
                    provider_name
                )));
            }

            if let Some(settings) = &config.settings {
                if let Some(enable_sso) = settings.get("enable_sso").and_then(|v| v.as_bool()) {
                    if !enable_sso {
                        return Err(DomainError::Forbidden(format!(
                            "{} SSO is disabled by organization administrator",
                            provider_name
                        )));
                    }
                }

                if let Some(domain) = settings.get("domain").and_then(|v| v.as_str()) {
                    let domain = domain.trim().trim_start_matches('@').to_lowercase();
                    if !domain.is_empty() {
                        let email_lower = email.to_lowercase();
                        let expected_suffix = format!("@{}", domain);
                        if !email_lower.ends_with(&expected_suffix) {
                            return Err(DomainError::Validation(format!(
                                "Account email ({}) does not match organization domain ({})",
                                email, domain
                            )));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

pub struct LinkGoogleAccountUseCase {
    pub identity_repo: Arc<dyn UserIdentityRepository>,
    pub oidc_service: Arc<dyn OidcService>,
    pub user_repo: Arc<dyn UserRepository>,
    pub integration_repo: Option<Arc<dyn IntegrationRepository>>,
}

impl LinkGoogleAccountUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        id_token: &str,
    ) -> Result<UserIdentity, DomainError> {
        let user_id = ctx.actor.id().ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        let user = self.user_repo.find_by_id(user_id).await?
            .ok_or_else(|| DomainError::NotFound("User not found".into()))?;

        let profile = self.oidc_service.verify_google_token(id_token).await?;
        
        let provider_user_id = profile.email.clone().unwrap_or_default();
        if provider_user_id.is_empty() {
            return Err(DomainError::Validation("Invalid Google account profile".into()));
        }

        // Per-Organization Integration Verification
        verify_org_identity_config(&self.integration_repo, user.org_id, "google", &provider_user_id).await?;

        // Check if identity is already linked to another user
        if let Some(existing) = self.identity_repo.find_by_provider("google", &provider_user_id).await? {
            if existing.user_id != user_id {
                return Err(DomainError::Conflict("This Google account is already linked to another user".into()));
            }
            return Ok(existing);
        }

        let identity = UserIdentity {
            id: Uuid::new_v4(),
            user_id,
            provider: IdentityProvider::Google,
            provider_user_id,
            email: profile.email.clone(),
            is_primary: false,
            profile_data: Some(serde_json::to_value(&profile).unwrap_or_default()),
            connected_at: Utc::now(),
            last_used_at: Some(Utc::now()),
        };

        self.identity_repo.save(&identity).await?;
        Ok(identity)
    }
}

pub struct LoginWithGoogleUseCase {
    pub identity_repo: Arc<dyn UserIdentityRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
    pub oidc_service: Arc<dyn OidcService>,
    pub token_service: Arc<dyn TokenService>,
    pub integration_repo: Option<Arc<dyn IntegrationRepository>>,
}

pub struct GoogleLoginResult {
    pub token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
    pub org_id: Uuid,
}

impl LoginWithGoogleUseCase {
    pub async fn execute(
        &self,
        id_token: &str,
    ) -> Result<GoogleLoginResult, DomainError> {
        let profile = self.oidc_service.verify_google_token(id_token).await?;
        let provider_user_id = profile.email.clone().unwrap_or_default();

        if provider_user_id.is_empty() {
            return Err(DomainError::Validation("Invalid Google account profile".into()));
        }

        // Resolve user via identity or existing email
        let user = if let Some(identity) = self.identity_repo.find_by_provider("google", &provider_user_id).await? {
            self.user_repo.find_by_id(identity.user_id).await?
                .ok_or_else(|| DomainError::NotFound("User not found".into()))?
        } else if let Some(existing_user) = self.user_repo.find_by_email(&provider_user_id).await? {
            // Auto-link Google identity to existing user with matching email
            let new_identity = UserIdentity {
                id: Uuid::new_v4(),
                user_id: existing_user.id,
                provider: IdentityProvider::Google,
                provider_user_id: provider_user_id.clone(),
                email: Some(provider_user_id.clone()),
                is_primary: false,
                profile_data: Some(serde_json::to_value(&profile).unwrap_or_default()),
                connected_at: Utc::now(),
                last_used_at: Some(Utc::now()),
            };
            let _ = self.identity_repo.save(&new_identity).await;
            existing_user
        } else {
            return Err(DomainError::NotFound("No account linked with this Google identity".into()));
        };

        // Per-Organization Integration Verification
        verify_org_identity_config(&self.integration_repo, user.org_id, "google", &provider_user_id).await?;

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
            is_trusted: true,
            is_revoked: false,
            mfa_verified: true,
            expires_at: now + chrono::Duration::hours(24),
            refresh_expires_at: Some(now + chrono::Duration::days(30)),
            last_active_at: Some(now),
            created_at: now,
        };
        self.session_repo.save(&session).await?;

        Ok(GoogleLoginResult {
            token: access_token,
            refresh_token,
            user_id: user.id,
            org_id: user.org_id,
        })
    }
}
