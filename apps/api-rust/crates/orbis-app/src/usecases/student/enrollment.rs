use orbis_domain::entities::enrollment::{Enrollment, StudyMode, EnrollmentStatus};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{EnrollmentRepository, StudentRepository};
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct EnrollStudentUseCase {
    pub student_repo: Arc<dyn StudentRepository>,
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
        class_id: Option<Uuid>,
        student_number: String,
        study_mode: StudyMode,
    ) -> Result<Enrollment, DomainError> {
        let student = self.student_repo.find_by_id(student_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        if !student.can_enroll() {
            return Err(DomainError::Validation(format!(
                "Cannot enroll student with status {:?}", student.status
            )));
        }

        let now = Utc::now();
        let enrollment_id = Uuid::new_v4();

        let enrollment = Enrollment {
            id: enrollment_id,
            org_id,
            period_id,
            student_id,
            class_id,
            student_number,
            study_mode,
            status: EnrollmentStatus::Active,
            is_attendance_exempt: study_mode.default_attendance_exempt(),
            is_grade_exempt: study_mode.default_grade_exempt(),
            enrollment_date: now,
            withdrawal_date: None,
            withdrawal_reason: None,
            meta: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        };

        self.enrollment_repo.save(&enrollment).await?;

        let event = DomainEvent::StudentEnrolled {
            enrollment_id,
            student_id,
            class_id,
            org_id,
            study_mode: study_mode.to_string(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(enrollment)
    }
}
