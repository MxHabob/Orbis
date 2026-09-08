use std::sync::Arc;
use std::collections::HashMap;
use uuid::Uuid;
use chrono::Utc;
use orbis_domain::entities::integration::{IntegrationConfig, IntegrationSecret};
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::IntegrationRepository;
use crate::ports::iam::PermissionCheckerPort;

pub struct IntegrationService {
    pub integration_repo: Arc<dyn IntegrationRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl IntegrationService {
    pub fn new(
        integration_repo: Arc<dyn IntegrationRepository>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            integration_repo,
            permission_checker,
        }
    }

    /// Connect / Configure a third-party app integration
    pub async fn connect_app(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        provider_name: String,
        category: String,
        env: String,
        settings: serde_json::Value,
        secrets: HashMap<String, String>,
    ) -> Result<IntegrationConfig, DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;

        let now = Utc::now();
        let config_id = Uuid::new_v4();

        let config = IntegrationConfig {
            id: config_id,
            org_id,
            provider_name: provider_name.clone(),
            category,
            is_enabled: true,
            env,
            settings: Some(settings),
            created_at: now,
            updated_at: now,
        };

        self.integration_repo.save_config(&config).await?;

        for (key, val) in secrets {
            let secret = IntegrationSecret {
                id: Uuid::new_v4(),
                config_id,
                key_name: key,
                secret_key: val,
                created_at: now,
                updated_at: now,
            };
            self.integration_repo.save_secret(&secret).await?;
        }

        Ok(config)
    }

    /// List connected integrations for an organization
    pub async fn list_integrations(&self, org_id: Uuid) -> Result<Vec<IntegrationConfig>, DomainError> {
        self.integration_repo.find_configs_by_org(org_id).await
    }

    /// Get integration configuration by provider name
    pub async fn get_integration_by_provider(&self, org_id: Uuid, provider_name: &str) -> Result<Option<IntegrationConfig>, DomainError> {
        self.integration_repo.find_config_by_provider(org_id, provider_name).await
    }

    /// Delete integration by provider
    pub async fn delete_integration(&self, ctx: &RequestContext, org_id: Uuid, provider_name: &str) -> Result<(), DomainError> {
        self.permission_checker.check(ctx, Permission::OrganizationWrite).await?;
        self.integration_repo.delete_by_provider(org_id, provider_name).await
    }
}
