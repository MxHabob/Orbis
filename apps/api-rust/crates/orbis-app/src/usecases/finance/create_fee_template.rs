use crate::ports::repositories::FeeTemplateRepository;
use crate::context::RequestContext;
use orbis_domain::entities::fee_template::{FeeTemplate, FeeFrequency};
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use std::sync::Arc;

pub struct CreateFeeTemplateUseCase {
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
}

impl CreateFeeTemplateUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        code: String,
        name: String,
        name_ar: Option<String>,
        category: String,
        default_amount: Decimal,
        currency: String,
        revenue_account_id: Uuid,
        receivable_account_id: Uuid,
        applicable_levels: Option<Vec<String>>,
        applicable_study_modes: Option<Vec<String>>,
        frequency: FeeFrequency,
        is_mandatory: bool,
    ) -> Result<FeeTemplate, DomainError> {
        let template = FeeTemplate {
            id: Uuid::new_v4(),
            org_id,
            code,
            name,
            name_ar,
            category,
            default_amount,
            currency,
            revenue_account_id,
            receivable_account_id,
            applicable_levels,
            applicable_study_modes,
            frequency,
            is_mandatory,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.fee_template_repo.save(&template).await?;

        Ok(template)
    }
}
