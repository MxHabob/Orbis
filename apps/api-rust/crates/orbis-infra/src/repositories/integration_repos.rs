use async_trait::async_trait;
use orbis_app::ports::repositories::IntegrationRepository;
use orbis_domain::entities::integration::{
    IntegrationConfig as DomainIntegrationConfig, IntegrationSecret as DomainIntegrationSecret,
};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::models::integration::{integration_config, integration_secret};

pub struct IntegrationRepoImpl {
    pub db: DatabaseConnection,
    secret_cipher: crate::security::integration_secret::IntegrationSecretCipher,
}

impl IntegrationRepoImpl {
    pub fn new(db: DatabaseConnection, master_key: &str) -> Self {
        Self {
            db,
            secret_cipher: crate::security::integration_secret::IntegrationSecretCipher::new(
                master_key,
            ),
        }
    }
}

#[async_trait]
impl IntegrationRepository for IntegrationRepoImpl {
    async fn find_configs_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<DomainIntegrationConfig>, DomainError> {
        let models = integration_config::Entity::find()
            .filter(integration_config::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| DomainIntegrationConfig {
                id: m.id,
                org_id: m.org_id,
                provider_name: m.provider_name,
                category: m.category,
                is_enabled: m.is_enabled,
                env: m.env,
                settings: m.settings.map(|v| v.into()),
                created_at: m.created_at.into(),
                updated_at: m.updated_at.into(),
            })
            .collect())
    }

    async fn get_secrets_by_config(
        &self,
        config_id: Uuid,
    ) -> Result<Vec<DomainIntegrationSecret>, DomainError> {
        let models = integration_secret::Entity::find()
            .filter(integration_secret::Column::ConfigId.eq(config_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let mut secrets = Vec::with_capacity(models.len());
        for model in models {
            let plaintext = self.secret_cipher.decrypt(&model.secret_key)?;
            if !model.secret_key.starts_with("v1:") {
                let mut active: integration_secret::ActiveModel = model.clone().into();
                active.secret_key = Set(self.secret_cipher.encrypt(&plaintext)?);
                active.updated_at = Set(chrono::Utc::now().into());
                active
                    .update(&self.db)
                    .await
                    .map_err(|error| DomainError::Internal(error.to_string()))?;
            }
            secrets.push(DomainIntegrationSecret {
                id: model.id,
                config_id: model.config_id,
                secret_key: plaintext,
                key_name: model.key_name,
                created_at: model.created_at.into(),
                updated_at: model.updated_at.into(),
            });
        }
        Ok(secrets)
    }

    async fn save_config(&self, config: &DomainIntegrationConfig) -> Result<(), DomainError> {
        let active = integration_config::ActiveModel {
            id: Set(config.id),
            org_id: Set(config.org_id),
            provider_name: Set(config.provider_name.clone()),
            category: Set(config.category.clone()),
            is_enabled: Set(config.is_enabled),
            env: Set(config.env.clone()),
            settings: Set(config.settings.clone()),
            created_at: Set(config.created_at.into()),
            updated_at: Set(config.updated_at.into()),
        };

        if integration_config::Entity::find_by_id(config.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_secret(&self, secret: &DomainIntegrationSecret) -> Result<(), DomainError> {
        let existing = integration_secret::Entity::find()
            .filter(integration_secret::Column::ConfigId.eq(secret.config_id))
            .filter(integration_secret::Column::KeyName.eq(&secret.key_name))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        let sid = existing.as_ref().map(|e| e.id).unwrap_or(secret.id);

        let active = integration_secret::ActiveModel {
            id: Set(sid),
            config_id: Set(secret.config_id),
            secret_key: Set(self.secret_cipher.encrypt(&secret.secret_key)?),
            key_name: Set(secret.key_name.clone()),
            created_at: Set(existing
                .as_ref()
                .map(|e| e.created_at)
                .unwrap_or_else(|| secret.created_at.into())),
            updated_at: Set(secret.updated_at.into()),
        };

        if existing.is_some() {
            active
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn find_config_by_provider(
        &self,
        org_id: Uuid,
        provider_name: &str,
    ) -> Result<Option<DomainIntegrationConfig>, DomainError> {
        let model = integration_config::Entity::find()
            .filter(integration_config::Column::OrgId.eq(org_id))
            .filter(integration_config::Column::ProviderName.eq(provider_name))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| DomainIntegrationConfig {
            id: m.id,
            org_id: m.org_id,
            provider_name: m.provider_name,
            category: m.category,
            is_enabled: m.is_enabled,
            env: m.env,
            settings: m.settings.map(|v| v.into()),
            created_at: m.created_at.into(),
            updated_at: m.updated_at.into(),
        }))
    }

    async fn find_configs_by_provider(
        &self,
        provider_name: &str,
    ) -> Result<Vec<DomainIntegrationConfig>, DomainError> {
        let models = integration_config::Entity::find()
            .filter(integration_config::Column::ProviderName.eq(provider_name))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| DomainIntegrationConfig {
                id: m.id,
                org_id: m.org_id,
                provider_name: m.provider_name,
                category: m.category,
                is_enabled: m.is_enabled,
                env: m.env,
                settings: m.settings.map(|v| v.into()),
                created_at: m.created_at.into(),
                updated_at: m.updated_at.into(),
            })
            .collect())
    }

    async fn delete_config(&self, config_id: Uuid) -> Result<(), DomainError> {
        // Delete secrets first
        integration_secret::Entity::delete_many()
            .filter(integration_secret::Column::ConfigId.eq(config_id))
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        // Delete config
        integration_config::Entity::delete_by_id(config_id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_provider(
        &self,
        org_id: Uuid,
        provider_name: &str,
    ) -> Result<(), DomainError> {
        if let Some(cfg) = self.find_config_by_provider(org_id, provider_name).await? {
            self.delete_config(cfg.id).await?;
        }
        Ok(())
    }
}
