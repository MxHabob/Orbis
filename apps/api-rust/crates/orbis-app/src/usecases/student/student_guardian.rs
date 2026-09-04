use orbis_domain::entities::student_guardian::StudentGuardian;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::StudentGuardianRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct AssignGuardianUseCase {
    pub student_guardian_repo: Arc<dyn StudentGuardianRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl AssignGuardianUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        guardian_id: Uuid,
        relationship: String,
        is_primary: bool,
    ) -> Result<StudentGuardian, DomainError> {
        let now = Utc::now();
        let student_guardian_id = Uuid::new_v4();

        let guardian = StudentGuardian {
            id: student_guardian_id,
            student_id,
            guardian_id,
            relationship,
            relationship_ar: None,
            is_primary,
            is_emergency_contact: false,
            is_authorized_pickup: false,
            can_view_grades: false,
            can_view_attendance: false,
            can_view_behavior: false,
            can_view_finance: false,
            can_view_schedule: false,
            can_receive_notifications: true,
            is_payer: false,
            payment_share_percent: None,
            occupation: None,
            employer: None,
            work_phone: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.student_guardian_repo.save(&guardian).await?;

        let event = DomainEvent::GuardianAssigned {
            student_guardian_id,
            student_id,
            guardian_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(guardian)
    }
}
