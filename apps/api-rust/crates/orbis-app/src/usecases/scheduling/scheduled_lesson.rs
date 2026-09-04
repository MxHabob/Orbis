use orbis_domain::entities::scheduled_lesson::ScheduledLesson;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ScheduledLessonRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ScheduleLessonUseCase {
    pub lesson_repo: Arc<dyn ScheduledLessonRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl ScheduleLessonUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        class_subject_id: Uuid,
        room_id: Option<Uuid>,
        time_slot_id: Uuid,
        teacher_id: Option<Uuid>,
    ) -> Result<ScheduledLesson, DomainError> {
        let now = Utc::now();
        let lesson_id = Uuid::new_v4();

        let lesson = ScheduledLesson {
            id: lesson_id,
            class_subject_id,
            room_id,
            time_slot_id,
            teacher_id,
            created_at: now,
            updated_at: now,
        };

        self.lesson_repo.save(&lesson).await?;

        let event = DomainEvent::LessonScheduled {
            lesson_id,
            class_subject_id,
            time_slot_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(lesson)
    }
}
