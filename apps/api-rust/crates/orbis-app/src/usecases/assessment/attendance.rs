use orbis_domain::entities::attendance::Attendance;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::AttendanceRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};

pub struct MarkAttendanceUseCase {
    pub attendance_repo: Arc<dyn AttendanceRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl MarkAttendanceUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        class_id: Uuid,
        date: NaiveDate,
        status: String,
        notes: Option<String>,
    ) -> Result<Attendance, DomainError> {
        let now = Utc::now();
        let attendance_id = Uuid::new_v4();

        let attendance = Attendance {
            id: attendance_id,
            student_id,
            class_id: Some(class_id),
            class_subject_id: None,
            scheduled_lesson_id: None,
            date,
            status: status.clone(),
            notes,
            created_at: now,
            updated_at: now,
        };

        self.attendance_repo.save(&attendance).await?;

        let event = DomainEvent::AttendanceMarked {
            attendance_id,
            student_id,
            class_id: Some(class_id),
            class_subject_id: None,
            scheduled_lesson_id: None,
            status,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(attendance)
    }
}
