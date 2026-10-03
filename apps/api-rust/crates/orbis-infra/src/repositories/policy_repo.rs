use async_trait::async_trait;
use orbis_app::ports::repositories::PolicyRepository;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use sea_orm::{DatabaseConnection, EntityTrait};
use uuid::Uuid;
use crate::models::organization;

pub struct PgPolicyRepo {
    pub db: DatabaseConnection,
}

impl PgPolicyRepo {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PolicyRepository for PgPolicyRepo {
    async fn get_settings(&self, org_id: Uuid) -> Result<OrganizationSettings, DomainError> {
        let org = organization::Entity::find_by_id(org_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .ok_or_else(|| DomainError::NotFound("Organization not found".into()))?;

        if let Some(ref settings_val) = org.settings {
            let parsed_settings: OrganizationSettings = serde_json::from_value(settings_val.clone())
                .unwrap_or_else(|_| OrganizationSettings::default());
            Ok(parsed_settings)
        } else {
            Ok(OrganizationSettings::default())
        }
    }
}
