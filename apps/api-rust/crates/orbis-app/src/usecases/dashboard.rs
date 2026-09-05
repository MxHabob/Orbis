use std::sync::Arc;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::DashboardCounts;
use crate::ports::repositories::DashboardRepository;

pub struct GetDashboardUseCase {
    pub dashboard_repo: Arc<dyn DashboardRepository>,
}

impl GetDashboardUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
    ) -> Result<DashboardCounts, DomainError> {
        // Always scoped to the authenticated tenant
        self.dashboard_repo.get_counts(ctx.tenant_id).await
    }
}
