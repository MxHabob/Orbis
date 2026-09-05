//! Infrastructure implementation of SettingsRepository.

use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, Set, ColumnTrait, QueryFilter};
use orbis_app::ports::repositories::SettingsRepository;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use uuid::Uuid;

pub struct SettingsRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl SettingsRepository for SettingsRepoImpl {
    async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError> {
        use crate::sea_orm_entities::organization_settings;

        let model = organization_settings::Entity::find()
            .filter(organization_settings::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        match model {
            Some(m) => {
                let settings: OrganizationSettings = serde_json::from_value(m.settings_json)
                    .map_err(|e| DomainError::Internal(format!("Failed to parse settings: {}", e)))?;
                Ok(settings)
            }
            None => {
                // Return defaults if no settings row exists
                Ok(OrganizationSettings::default())
            }
        }
    }

    async fn save_settings(&self, org_id: Uuid, settings: &OrganizationSettings) -> Result<(), DomainError> {
        use crate::sea_orm_entities::organization_settings;

        let json_val = serde_json::to_value(settings)
            .map_err(|e| DomainError::Internal(format!("Failed to serialize settings: {}", e)))?;

        let existing = organization_settings::Entity::find()
            .filter(organization_settings::Column::OrgId.eq(org_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        match existing {
            Some(model) => {
                let mut active: organization_settings::ActiveModel = model.into();
                active.settings_json = Set(json_val);
                active.updated_at = Set(chrono::Utc::now());
                active.update(&self.db).await
                    .map_err(|e| DomainError::Internal(e.to_string()))?;
            }
            None => {
                let active = organization_settings::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    org_id: Set(org_id),
                    settings_json: Set(json_val),
                    created_at: Set(chrono::Utc::now()),
                    updated_at: Set(chrono::Utc::now()),
                    ..Default::default()
                };
                active.insert(&self.db).await
                    .map_err(|e| DomainError::Internal(e.to_string()))?;
            }
        }

        Ok(())
    }
}
