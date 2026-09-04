use orbis_domain::entities::exam_schedule::ExamSchedule;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ExamScheduleRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, NaiveTime};

pub struct ScheduleExamUseCase {
    pub schedule_repo: Arc<dyn ExamScheduleRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl ScheduleExamUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        exam_session_id: Uuid,
        class_subject_id: Uuid,
        exam_date: NaiveDate,
        start_time: NaiveTime,
        end_time: NaiveTime,
    ) -> Result<ExamSchedule, DomainError> {
        let now = Utc::now();
        let schedule_id = Uuid::new_v4();

        let schedule = ExamSchedule {
            id: schedule_id,
            exam_session_id,
            class_subject_id,
            exam_date,
            start_time,
            end_time,
            created_at: now,
            updated_at: now,
        };

        self.schedule_repo.save(&schedule).await?;

        let event = DomainEvent::ExamScheduled {
            schedule_id,
            session_id: exam_session_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(schedule)
    }
}
