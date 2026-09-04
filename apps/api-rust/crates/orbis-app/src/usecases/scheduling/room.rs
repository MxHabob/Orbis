use orbis_domain::entities::room::Room;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::RoomRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct CreateRoomUseCase {
    pub room_repo: Arc<dyn RoomRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateRoomUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        capacity: i32,
        room_type: String,
    ) -> Result<Room, DomainError> {
        let now = Utc::now();
        let room_id = Uuid::new_v4();

        let room = Room {
            id: room_id,
            org_id,
            name,
            capacity,
            room_type,
            created_at: now,
            updated_at: now,
        };

        self.room_repo.save(&room).await?;

        let event = DomainEvent::RoomCreated {
            room_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(room)
    }
}
