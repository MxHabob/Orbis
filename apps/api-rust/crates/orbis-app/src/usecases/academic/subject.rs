use orbis_domain::entities::subject::Subject;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::SubjectRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct CreateSubjectUseCase {
    pub subject_repo: Arc<dyn SubjectRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateSubjectUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Option<Uuid>,
        code: String,
        name: String,
        credit_hours: Option<Decimal>,
        is_core: bool,
    ) -> Result<Subject, DomainError> {
        let now = Utc::now();
        let subject_id = Uuid::new_v4();

        let subject = Subject {
            id: subject_id,
            org_id,
            unit_id,
            code,
            name,
            name_ar: None,
            name_en: None,
            description: None,
            is_core,
            is_elective: !is_core,
            credit_hours,
            theory_hours: None,
            practical_hours: None,
            requires_lab: false,
            max_students_per_class: None,
            color: None,
            icon: None,
            sort_order: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.subject_repo.save(&subject).await?;

        let event = DomainEvent::SubjectCreated {
            subject_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(subject)
    }
}
