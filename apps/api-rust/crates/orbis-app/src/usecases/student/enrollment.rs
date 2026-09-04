use orbis_domain::entities::enrollment::Enrollment;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::EnrollmentRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct EnrollStudentUseCase {
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl EnrollStudentUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        class_id: Uuid,
        student_number: String,
        enrollment_type: String,
    ) -> Result<Enrollment, DomainError> {
        let now = Utc::now();
        let enrollment_id = Uuid::new_v4();

        let enrollment = Enrollment {
            id: enrollment_id,
            org_id,
            period_id,
            student_id,
            class_id,
            student_number,
            status: "Active".to_string(),
            enrollment_date: now,
            enrollment_type,
            withdrawal_date: None,
            withdrawal_reason: None,
            meta: None,
            created_at: now,
            updated_at: now,
        };

        self.enrollment_repo.save(&enrollment).await?;

        let event = DomainEvent::StudentEnrolled {
            enrollment_id,
            student_id,
            class_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(enrollment)
    }
}
