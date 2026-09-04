use crate::ports::repositories::IntegrationRepository;
use crate::context::RequestContext;
use orbis_domain::entities::integration::IntegrationConfig;
use orbis_domain::errors::DomainError;
use std::sync::Arc;

pub struct GetConnectedAppsUseCase {
    pub integration_repo: Arc<dyn IntegrationRepository>,
}

impl GetConnectedAppsUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
    ) -> Result<Vec<IntegrationConfig>, DomainError> {
        // Fetch all configs for the organization
        self.integration_repo.find_configs_by_org(_ctx.tenant_id).await
    }
}
