use orbis_domain::entities::subject::Subject;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::SubjectRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

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
        credit_hours: Option<i32>,
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
            short_description: None,
            is_core,
            is_elective: !is_core,
            credit_hours,
            requires_lab: false,
            requires_practical: false,
            max_students_per_class: None,
            default_grade_groups: None,
            prerequisites: None,
            learning_outcomes: None,
            color: None,
            icon: None,
            cover_image: None,
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
