use orbis_domain::entities::assessment_scheme::AssessmentScheme;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::AssessmentSchemeRepository;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct CreateAssessmentSchemeUseCase {
    pub scheme_repo: Arc<dyn AssessmentSchemeRepository>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl CreateAssessmentSchemeUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        description: Option<String>,
        max_score: f64,
        formula: Option<serde_json::Value>,
    ) -> Result<AssessmentScheme, DomainError> {
        self.permission_checker.check(ctx, Permission::AssessmentWrite).await?;

        let now = Utc::now();
        let scheme_id = Uuid::new_v4();

        let scheme = AssessmentScheme {
            id: scheme_id,
            org_id,
            name,
            description,
            max_score,
            formula,
            created_at: now,
            updated_at: now,
        };

        self.scheme_repo.save(&scheme).await?;

        Ok(scheme)
    }
}
