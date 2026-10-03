//! Infrastructure implementation of SettingsRepository.

use async_trait::async_trait;
use orbis_app::ports::repositories::SettingsRepository;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;

use crate::models::organization;

pub struct SettingsRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SettingsRepository for SettingsRepoImpl {
    async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError> {
        let org = organization::Entity::find_by_id(org_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Organization not found".into()))?;

        if let Some(ref settings_val) = org.settings {
            let parsed: OrganizationSettings = serde_json::from_value(settings_val.clone())
                .unwrap_or_else(|_| OrganizationSettings::default());
            Ok(parsed)
        } else {
            Ok(OrganizationSettings::default())
        }
    }

    async fn save_settings(
        &self,
        org_id: Uuid,
        settings: &OrganizationSettings,
    ) -> Result<(), DomainError> {
        let org = organization::Entity::find_by_id(org_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Organization not found".into()))?;

        let json_val = serde_json::to_value(settings)
            .map_err(|e| DomainError::Internal(format!("Failed to serialize settings: {}", e)))?;

        let mut active: organization::ActiveModel = org.into();
        active.settings = Set(Some(json_val));
        active.updated_at = Set(chrono::Utc::now());
        active
            .update(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(())
    }
}
