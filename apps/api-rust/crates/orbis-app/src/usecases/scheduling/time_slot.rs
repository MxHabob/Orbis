use orbis_domain::entities::time_slot::TimeSlot;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::TimeSlotRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveTime};

pub struct CreateTimeSlotUseCase {
    pub time_slot_repo: Arc<dyn TimeSlotRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateTimeSlotUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        day_of_week: i16,
        start_time: NaiveTime,
        end_time: NaiveTime,
        slot_index: i32,
    ) -> Result<TimeSlot, DomainError> {
        let now = Utc::now();
        let time_slot_id = Uuid::new_v4();

        let time_slot = TimeSlot {
            id: time_slot_id,
            period_id,
            day_of_week,
            start_time,
            end_time,
            slot_index,
            created_at: now,
            updated_at: now,
        };

        self.time_slot_repo.save(&time_slot).await?;

        let event = DomainEvent::TimeSlotCreated {
            time_slot_id,
            period_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(time_slot)
    }
}
