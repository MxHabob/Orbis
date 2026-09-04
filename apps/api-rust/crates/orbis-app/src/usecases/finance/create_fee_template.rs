use crate::ports::repositories::FeeTemplateRepository;
use crate::context::RequestContext;
use orbis_domain::entities::fee_template::FeeTemplate;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use std::sync::Arc;

pub struct CreateFeeTemplateUseCase {
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
}

impl CreateFeeTemplateUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        default_amount: f64,
        revenue_account_id: Uuid,
        receivable_account_id: Uuid,
        is_mandatory: bool,
    ) -> Result<FeeTemplate, DomainError> {
        let template = FeeTemplate {
            id: Uuid::new_v4(),
            org_id,
            name,
            default_amount,
            revenue_account_id,
            receivable_account_id,
            is_mandatory,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.fee_template_repo.save(&template).await?;

        Ok(template)
    }
}
