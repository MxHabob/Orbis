use async_trait::async_trait;
use orbis_app::ports::repositories::IntegrationRepository;
use orbis_domain::entities::integration::{IntegrationConfig as DomainIntegrationConfig, IntegrationSecret as DomainIntegrationSecret};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};
use uuid::Uuid;

use crate::models::integration::{integration_config, integration_secret};

pub struct IntegrationRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl IntegrationRepository for IntegrationRepoImpl {
    async fn find_configs_by_org(&self, org_id: Uuid) -> Result<Vec<DomainIntegrationConfig>, DomainError> {
        let models = integration_config::Entity::find()
            .filter(integration_config::Column::OrgId.eq(org_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
            
        Ok(models.into_iter().map(|m| DomainIntegrationConfig {
            id: m.id, org_id: m.org_id, provider_name: m.provider_name,
            category: m.category, is_enabled: m.is_enabled, env: m.env,
            settings: m.settings.map(|v| v.into()),
            created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }).collect())
    }

    async fn get_secrets_by_config(&self, config_id: Uuid) -> Result<Vec<DomainIntegrationSecret>, DomainError> {
        let models = integration_secret::Entity::find()
            .filter(integration_secret::Column::ConfigId.eq(config_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models.into_iter().map(|m| DomainIntegrationSecret {
            id: m.id, config_id: m.config_id, secret_key: m.secret_key,
            key_name: m.key_name, created_at: m.created_at.into(), updated_at: m.updated_at.into(),
        }).collect())
    }

    async fn save_config(&self, config: &DomainIntegrationConfig) -> Result<(), DomainError> {
        let active = integration_config::ActiveModel {
            id: Set(config.id), org_id: Set(config.org_id), provider_name: Set(config.provider_name.clone()),
            category: Set(config.category.clone()), is_enabled: Set(config.is_enabled),
            env: Set(config.env.clone()), settings: Set(config.settings.clone()),
            created_at: Set(config.created_at.into()), updated_at: Set(config.updated_at.into()),
        };

        if integration_config::Entity::find_by_id(config.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }

    async fn save_secret(&self, secret: &DomainIntegrationSecret) -> Result<(), DomainError> {
        let active = integration_secret::ActiveModel {
            id: Set(secret.id), config_id: Set(secret.config_id), secret_key: Set(secret.secret_key.clone()),
            key_name: Set(secret.key_name.clone()), created_at: Set(secret.created_at.into()),
            updated_at: Set(secret.updated_at.into()),
        };

        if integration_secret::Entity::find_by_id(secret.id).one(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?.is_some() {
            active.update(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active.insert(&self.db).await.map_err(|e| DomainError::Internal(e.to_string()))?;
        }
        Ok(())
    }
}
