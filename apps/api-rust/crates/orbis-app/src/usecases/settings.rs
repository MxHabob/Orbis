use std::sync::Arc;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::SettingsRepository;
use crate::ports::iam::PermissionCheckerPort;

pub struct UpdateSettingsUseCase {
    pub settings_repo: Arc<dyn SettingsRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl UpdateSettingsUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        settings: OrganizationSettings,
    ) -> Result<OrganizationSettings, DomainError> {
        // Authorization: only org admins or system can update settings
        if !ctx.actor.is_system() {
            let user_id = ctx.actor.id()
                .ok_or_else(|| DomainError::Unauthorized("No actor identity".into()))?;
            let has_perm = self.permission_checker
                .has_permission(user_id, ctx.tenant_id, "settings.manage")
                .await
                .unwrap_or(false);
            if !has_perm {
                return Err(DomainError::Unauthorized(
                    "Insufficient permissions to update organization settings".into(),
                ));
            }
        }

        // Tenant isolation: settings are always scoped to ctx.tenant_id
        self.settings_repo.save_settings(ctx.tenant_id, &settings).await?;

        // Read back to return the canonical saved state
        let saved = self.settings_repo.get_settings(ctx.tenant_id).await?;
        Ok(saved)
    }
}

pub struct GetSettingsUseCase {
    pub settings_repo: Arc<dyn SettingsRepository>,
}

impl GetSettingsUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
    ) -> Result<OrganizationSettings, DomainError> {
        self.settings_repo.get_settings(ctx.tenant_id).await
    }
}
