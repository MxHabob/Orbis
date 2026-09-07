use orbis_domain::entities::academic_period::AcademicPeriod;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::{AcademicPeriodRepository, AcademicYearRepository};
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct CreateAcademicPeriodUseCase {
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateAcademicPeriodUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        year_id: Uuid,
        parent_id: Option<Uuid>,
        name: String,
        period_type: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AcademicPeriod, DomainError> {
        let year = self.year_repo.find_by_id(year_id).await?
            .ok_or_else(|| DomainError::NotFound("Academic year not found".into()))?;

        if year.org_id != org_id {
            return Err(DomainError::Validation("Year does not belong to org".into()));
        }

        let now = Utc::now();
        let period_id = Uuid::new_v4();

        let period = AcademicPeriod {
            id: period_id,
            org_id,
            year_id,
            parent_id,
            name,
            name_ar: None,
            code: None,
            period_type,
            start_date,
            end_date,
            hijri_start: None,
            hijri_end: None,
            enrollment_start: None,
            enrollment_end: None,
            is_enrollment_open: false,
            grade_entry_start: None,
            grade_entry_end: None,
            is_grade_entry_open: false,
            exam_start: None,
            exam_end: None,
            state: orbis_domain::entities::academic_period::AcademicPeriodState::Planning,
            is_locked: false,
            locked_at: None,
            locked_by: None,
            sort_order: None,
            metadata: None,
            created_at: now,
            updated_at: now,
        };

        self.period_repo.save(&period).await?;

        let event = DomainEvent::AcademicPeriodCreated {
            period_id,
            year_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(period)
    }
}
