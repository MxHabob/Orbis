use orbis_domain::entities::academic_year::AcademicYear;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use crate::context::RequestContext;
use crate::ports::repositories::AcademicYearRepository;
use crate::ports::event_bus::EventBus;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct CreateAcademicYearUseCase {
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub event_bus: Arc<dyn EventBus>,
}

impl CreateAcademicYearUseCase {
    pub async fn execute(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AcademicYear, DomainError> {
        let now = Utc::now();
        let year_id = Uuid::new_v4();

        let year = AcademicYear {
            id: year_id,
            org_id,
            name,
            name_ar: None,
            start_date,
            end_date,
            hijri_start: None,
            hijri_end: None,
            status: "planning".to_string(),
            is_locked: false,
            locked_at: None,
            locked_by: None,
            lock_reason: None,
            current_period_id: None,
            archived_at: None,
            archived_by: None,
            notes: None,
            created_at: now,
            updated_at: now,
        };

        self.year_repo.save(&year).await?;

        let event = DomainEvent::AcademicYearCreated {
            year_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(year)
    }
}
