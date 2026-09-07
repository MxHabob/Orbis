use orbis_domain::entities::grade::Grade;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::GradeRepository;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;

pub struct RecordGradeUseCase {
    pub grade_repo: Arc<dyn GradeRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl RecordGradeUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        period_id: Uuid,
        student_id: Uuid,
        enrollment_id: Option<Uuid>,
        class_subject_id: Uuid,
        component_id: Option<Uuid>,
        score: Decimal,
        max_score: Decimal,
        is_excused: bool,
        is_exempt: bool,
        notes: Option<String>,
    ) -> Result<Grade, DomainError> {
        self.permission_checker.check(ctx, Permission::GradeWrite).await?;

        let now = Utc::now();
        let grade_id = Uuid::new_v4();

        let grade = Grade {
            id: grade_id,
            org_id,
            period_id,
            student_id,
            enrollment_id,
            class_subject_id,
            component_id,
            score,
            max_score,
            weight: None,
            letter_grade: None,
            is_final: false,
            is_excused,
            is_exempt,
            change_reason: None,
            recorded_by: ctx.user_id(),
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

