use crate::models::user_device::{self, Entity as DeviceEntity};
use async_trait::async_trait;
use orbis_app::ports::repositories::UserDeviceRepository;
use orbis_domain::entities::device::{DeviceType, UserDevice};
use orbis_domain::errors::DomainError;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct UserDeviceRepoImpl {
    pub db: DatabaseConnection,
}

#[async_trait]
impl UserDeviceRepository for UserDeviceRepoImpl {
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<UserDevice>, DomainError> {
        let models = DeviceEntity::find()
            .filter(user_device::Column::UserId.eq(user_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(models
            .into_iter()
            .map(|m| UserDevice {
                id: m.id,
                user_id: m.user_id,
                device_id: m.device_id,
                device_name: m.device_name,
                device_type: DeviceType::from(m.device_type.as_str()),
                os: m.os,
                browser: m.browser,
                client_ip: m.client_ip,
                user_agent: m.user_agent,
                fingerprint_hash: m.fingerprint_hash,
                is_trusted: m.is_trusted,
                trusted_until: m.trusted_until,
                first_seen_at: m.first_seen_at,
                last_active_at: m.last_active_at,
            })
            .collect())
    }

    async fn find_by_device_id(
        &self,
        user_id: Uuid,
        device_id: &str,
    ) -> Result<Option<UserDevice>, DomainError> {
        let model = DeviceEntity::find()
            .filter(user_device::Column::UserId.eq(user_id))
            .filter(user_device::Column::DeviceId.eq(device_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        Ok(model.map(|m| UserDevice {
            id: m.id,
            user_id: m.user_id,
            device_id: m.device_id,
            device_name: m.device_name,
            device_type: DeviceType::from(m.device_type.as_str()),
            os: m.os,
            browser: m.browser,
            client_ip: m.client_ip,
            user_agent: m.user_agent,
            fingerprint_hash: m.fingerprint_hash,
            is_trusted: m.is_trusted,
            trusted_until: m.trusted_until,
            first_seen_at: m.first_seen_at,
            last_active_at: m.last_active_at,
        }))
    }

    async fn save(&self, device: &UserDevice) -> Result<(), DomainError> {
        let active_model = user_device::ActiveModel {
            id: Set(device.id),
            user_id: Set(device.user_id),
            device_id: Set(device.device_id.clone()),
            device_name: Set(device.device_name.clone()),
            device_type: Set(device.device_type.to_string()),
            os: Set(device.os.clone()),
            browser: Set(device.browser.clone()),
            client_ip: Set(device.client_ip.clone()),
            user_agent: Set(device.user_agent.clone()),
            fingerprint_hash: Set(device.fingerprint_hash.clone()),
            is_trusted: Set(device.is_trusted),
            trusted_until: Set(device.trusted_until),
            first_seen_at: Set(device.first_seen_at),
            last_active_at: Set(device.last_active_at),
        };

        if DeviceEntity::find_by_id(device.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?
            .is_some()
        {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        DeviceEntity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;
        Ok(())
    }
}
