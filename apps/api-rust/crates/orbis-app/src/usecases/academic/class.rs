use orbis_domain::entities::class::Class;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ClassRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct CreateClassUseCase {
    pub class_repo: Arc<dyn ClassRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateClassUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        unit_id: Uuid,
        period_id: Uuid,
        name: String,
        level: String,
    ) -> Result<Class, DomainError> {
        let now = Utc::now();
        let class_id = Uuid::new_v4();

        let class = Class {
            id: class_id,
            org_id,
            unit_id,
            period_id,
            name,
            name_ar: None,
            level,
            division: None,
            homeroom_teacher_id: None,
            capacity: None,
            current_students: Some(0),
            room_id: None,
            color: None,
            is_active: true,
            is_deleted: false,
            deleted_at: None,
            created_at: now,
            updated_at: now,
        };

        self.class_repo.save(&class).await?;

        let event = DomainEvent::ClassCreated {
            class_id,
            org_id,
            period_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(class)
    }
}
