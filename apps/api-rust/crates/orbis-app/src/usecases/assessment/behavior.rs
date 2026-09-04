use orbis_domain::entities::behavior_record::BehaviorRecord;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::BehaviorRecordRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct LogBehaviorUseCase {
    pub behavior_repo: Arc<dyn BehaviorRecordRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl LogBehaviorUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        org_id: Uuid,
        reporter_id: Uuid,
        record_type: String,
        description: String,
        points: i32,
    ) -> Result<BehaviorRecord, DomainError> {
        let now = Utc::now();
        let record_id = Uuid::new_v4();

        let record = BehaviorRecord {
            id: record_id,
            student_id,
            org_id,
            reporter_id,
            record_type: record_type.clone(),
            description,
            points,
            created_at: now,
            updated_at: now,
        };

        self.behavior_repo.save(&record).await?;

        let event = DomainEvent::BehaviorLogged {
            record_id,
            student_id,
            record_type,
            points,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(record)
    }
}
