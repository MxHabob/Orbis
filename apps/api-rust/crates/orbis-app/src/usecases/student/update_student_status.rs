use orbis_domain::entities::student::{Student, StudentStatus};
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::StudentRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct UpdateStudentStatusUseCase {
    pub student_repo: Arc<dyn StudentRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl UpdateStudentStatusUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        new_status: StudentStatus,
        reason: Option<String>,
    ) -> Result<Student, DomainError> {
        self.permission_checker.check(ctx, Permission::StudentUpdate).await?;

        let mut student = self.student_repo.find_by_id(student_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("Student {} not found", student_id)))?;

        let old_status = student.status;
        student.status = new_status;
        student.updated_at = Utc::now();

        if let Some(r) = reason {
            if let Some(obj) = student.meta.as_object_mut() {
                obj.insert("status_change_reason".into(), serde_json::Value::String(r));
                obj.insert("previous_status".into(), serde_json::Value::String(old_status.to_string()));
            }
        }

        self.student_repo.save(&student).await?;

        let event = DomainEvent::StudentStatusChanged {
            student_id,
            old_status: old_status.to_string(),
            new_status: new_status.to_string(),
            timestamp: Utc::now(),
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(student)
    }
}
