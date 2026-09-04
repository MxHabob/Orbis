use orbis_domain::entities::exam_proctor::ExamProctor;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::ExamProctorRepository;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct AssignProctorUseCase {
    pub proctor_repo: Arc<dyn ExamProctorRepository>,
}

impl AssignProctorUseCase {
    pub async fn execute(
        &self,
        _ctx: &RequestContext,
        exam_schedule_id: Uuid,
        room_id: Uuid,
        teacher_id: Uuid,
        is_chief: bool,
    ) -> Result<ExamProctor, DomainError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let proctor = ExamProctor {
            id,
            exam_schedule_id,
            room_id,
            teacher_id,
            is_chief,
            created_at: now,
            updated_at: now,
        };

        self.proctor_repo.save(&proctor).await?;

        Ok(proctor)
    }
}
