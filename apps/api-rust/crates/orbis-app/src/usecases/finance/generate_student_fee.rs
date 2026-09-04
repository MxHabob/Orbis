use crate::ports::repositories::{StudentFeeRepository, FeeTemplateRepository};
use crate::ports::event_bus::EventBus;
use crate::context::RequestContext;
use orbis_domain::entities::student_fee::StudentFee;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use std::sync::Arc;

pub struct GenerateStudentFeeUseCase {
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
    pub fee_template_repo: Arc<dyn FeeTemplateRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl GenerateStudentFeeUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        enrollment_id: Uuid,
        fee_template_id: Uuid,
        discount_amount: f64,
        due_date: NaiveDate,
    ) -> Result<StudentFee, DomainError> {
        let template = self.fee_template_repo.find_by_id(fee_template_id).await?
            .ok_or(DomainError::NotFound("Fee Template not found".to_string()))?;

        let net_amount = template.default_amount - discount_amount;
        let fee = StudentFee {
            id: Uuid::new_v4(),
            org_id,
            enrollment_id,
            fee_template_id,
            original_amount: template.default_amount,
            discount_amount,
            net_amount,
            status: "Pending".to_string(),
            due_date,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.student_fee_repo.save(&fee).await?;

        let _ = self.event_bus.publish(_ctx, &DomainEvent::StudentFeeGenerated {
            fee_id: fee.id,
            enrollment_id,
            net_amount,
            timestamp: Utc::now(),
        }).await;

        Ok(fee)
    }
}
