use orbis_domain::entities::exam_session::ExamSession;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ExamSessionRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

pub struct CreateExamSessionUseCase {
    pub session_repo: Arc<dyn ExamSessionRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateExamSessionUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        academic_period_id: Uuid,
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<ExamSession, DomainError> {
        let now = Utc::now();
        let session_id = Uuid::new_v4();

        let session = ExamSession {
            id: session_id,
            org_id,
            academic_period_id,
            name,
            start_date,
            end_date,
            status: "Scheduled".to_string(),
            created_at: now,
            updated_at: now,
        };

        self.session_repo.save(&session).await?;

        let event = DomainEvent::ExamSessionCreated {
            session_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(session)
    }
}
