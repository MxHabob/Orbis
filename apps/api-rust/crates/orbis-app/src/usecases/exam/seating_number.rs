use orbis_domain::entities::seating_number::SeatingNumber;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::SeatingNumberRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct AssignSeatingNumberUseCase {
    pub seating_repo: Arc<dyn SeatingNumberRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl AssignSeatingNumberUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        exam_session_id: Uuid,
        student_id: Uuid,
        seating_number: String,
        room_id: Option<Uuid>,
    ) -> Result<SeatingNumber, DomainError> {
        let now = Utc::now();
        let seating_id = Uuid::new_v4();

        let seating = SeatingNumber {
            id: seating_id,
            exam_session_id,
            student_id,
            seating_number: seating_number.clone(),
            room_id,
            created_at: now,
            updated_at: now,
        };

        self.seating_repo.save(&seating).await?;

        let event = DomainEvent::SeatingNumberAssigned {
            student_id,
            session_id: exam_session_id,
            seating_number,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(seating)
    }
}
