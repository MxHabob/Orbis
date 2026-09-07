use orbis_domain::entities::enrollment::{Enrollment, StudyMode};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::EnrollmentRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct ChangeStudyModeUseCase {
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl ChangeStudyModeUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        enrollment_id: Uuid,
        new_mode: StudyMode,
        reason: Option<String>,
    ) -> Result<Enrollment, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let mut enrollment = self.enrollment_repo.find_by_id(enrollment_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Enrollment {} not found", enrollment_id)))?;

        let old_mode = enrollment.study_mode;
        enrollment.study_mode = new_mode;
        enrollment.is_attendance_exempt = new_mode.default_attendance_exempt();
        enrollment.is_grade_exempt = new_mode.default_grade_exempt();
        enrollment.updated_at = Utc::now();

        if let Some(r) = reason {
            if let Some(obj) = enrollment.meta.as_object_mut() {
                obj.insert("study_mode_change_reason".into(), serde_json::Value::String(r));
                obj.insert("previous_study_mode".into(), serde_json::Value::String(old_mode.to_string()));
            }
        }

        self.enrollment_repo.save(&enrollment).await?;

        let event = DomainEvent::StudentStudyModeChanged {
            student_id: enrollment.student_id,
            old_mode: old_mode.to_string(),
            new_mode: new_mode.to_string(),
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(enrollment)
    }
}
