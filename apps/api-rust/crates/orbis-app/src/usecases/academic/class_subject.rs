use orbis_domain::entities::class_subject::ClassSubject;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ClassSubjectRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct AssignClassSubjectUseCase {
    pub class_subject_repo: Arc<dyn ClassSubjectRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl AssignClassSubjectUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        class_id: Uuid,
        subject_id: Uuid,
        period_id: Uuid,
        primary_teacher_id: Uuid,
        weekly_hours: Option<i32>,
    ) -> Result<ClassSubject, DomainError> {
        let now = Utc::now();
        let class_subject_id = Uuid::new_v4();

        let class_subject = ClassSubject {
            id: class_subject_id,
            class_id,
            subject_id,
            period_id,
            primary_teacher_id,
            assistant_teacher_id: None,
            substitute_teacher_id: None,
            grade_groups: None,
            weekly_hours,
            theory_hours: None,
            practical_hours: None,
            assessment_config: None,
            scheme_id: None,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        self.class_subject_repo.save(&class_subject).await?;

        let event = DomainEvent::ClassSubjectAssigned {
            class_subject_id,
            class_id,
            subject_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(class_subject)
    }
}
