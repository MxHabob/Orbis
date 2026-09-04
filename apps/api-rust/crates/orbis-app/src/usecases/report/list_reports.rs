//! List reports use case — lists available report definitions for the current user.

use crate::context::RequestContext;
use crate::ports::reporting::ReportDefinitionRepository;
use orbis_domain::entities::report::ReportDefinition;
use orbis_domain::errors::DomainError;
use std::sync::Arc;

pub struct ListReportsUseCase {
    pub definition_repo: Arc<dyn ReportDefinitionRepository>,
}

impl ListReportsUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        domain: Option<&str>,
        category: Option<&str>,
    ) -> Result<Vec<ReportDefinition>, DomainError> {
        let definitions = self
            .definition_repo
            .list_by_org(ctx.tenant_id, domain, category)
            .await?;

        // Filter to active only
        let active: Vec<ReportDefinition> = definitions
            .into_iter()
            .filter(|d| d.is_active)
            .collect();

        Ok(active)
    }
}
