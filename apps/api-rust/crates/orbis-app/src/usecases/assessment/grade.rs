use orbis_domain::entities::grade::Grade;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::GradeRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

pub struct RecordGradeUseCase {
    pub grade_repo: Arc<dyn GradeRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl RecordGradeUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        student_id: Uuid,
        class_subject_id: Uuid,
        scheme_id: Uuid,
        score: f64,
        is_excused: bool,
        notes: Option<String>,
    ) -> Result<Grade, DomainError> {
        let now = Utc::now();
        let grade_id = Uuid::new_v4();

        let grade = Grade {
            id: grade_id,
            student_id,
            class_subject_id,
            scheme_id,
            score,
            is_excused,
            notes,
            created_at: now,
            updated_at: now,
        };

        self.grade_repo.save(&grade).await?;

        let event = DomainEvent::GradeRecorded {
            grade_id,
            student_id,
            class_subject_id,
            score,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(grade)
    }
}
