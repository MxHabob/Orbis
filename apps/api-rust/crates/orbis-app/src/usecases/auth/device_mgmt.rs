use std::sync::Arc;
use uuid::Uuid;
use orbis_domain::entities::device::UserDevice;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::UserDeviceRepository;

pub struct DeviceManagementUseCase {
    pub device_repo: Arc<dyn UserDeviceRepository>,
}

impl DeviceManagementUseCase {
    pub async fn list_devices(&self, ctx: &RequestContext) -> Result<Vec<UserDevice>, DomainError> {
        let user_id = ctx.actor.id().ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        self.device_repo.find_by_user(user_id).await
    }

    pub async fn revoke_device(&self, ctx: &RequestContext, device_id: Uuid) -> Result<(), DomainError> {
        let _user_id = ctx.actor.id().ok_or_else(|| DomainError::Unauthorized("Not authenticated".into()))?;
        self.device_repo.delete(device_id).await
    }
}
