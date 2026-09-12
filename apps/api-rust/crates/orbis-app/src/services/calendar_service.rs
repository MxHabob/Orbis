use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use orbis_domain::entities::academic_year::AcademicYear;
use orbis_domain::entities::academic_period::{AcademicPeriod, AcademicPeriodState};
use orbis_domain::entities::period_template::PeriodTemplate;
use orbis_domain::events::DomainEvent;
use orbis_domain::errors::DomainError;
use orbis_domain::iam::permission::Permission;

use crate::context::RequestContext;
use crate::ports::repositories::{
    AcademicYearRepository, AcademicPeriodRepository, PeriodTemplateRepository,
};
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;

pub struct CalendarService {
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub template_repo: Arc<dyn PeriodTemplateRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl CalendarService {
    pub fn new(
        year_repo: Arc<dyn AcademicYearRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        template_repo: Arc<dyn PeriodTemplateRepository>,
        event_bus: Arc<dyn EventBus>,
        permission_checker: Arc<dyn PermissionCheckerPort>,
    ) -> Self {
        Self {
            year_repo,
            period_repo,
            template_repo,
            event_bus,
            permission_checker,
        }
    }

    /// Create a new academic year
    pub async fn create_year(
        &self,
        ctx: &RequestContext,
        org_id: Uuid,
        name: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker.check(ctx, Permission::PeriodWrite).await?;

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
            status: "active".to_string(),
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

    /// List academic years for an organization
    pub async fn list_years(&self, org_id: Uuid) -> Result<Vec<AcademicYear>, DomainError> {
        self.year_repo.find_by_org(org_id).await
    }

    /// Get current active academic year
    pub async fn get_current_year(&self, org_id: Uuid) -> Result<Option<AcademicYear>, DomainError> {
        self.year_repo.find_current(org_id).await
    }

    /// Get academic year by ID
    pub async fn get_year(&self, id: Uuid) -> Result<Option<AcademicYear>, DomainError> {
        self.year_repo.find_by_id(id).await
    }

    /// Create an academic period within a year
    pub async fn create_period(
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
        self.permission_checker.check(ctx, Permission::PeriodWrite).await?;

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
            state: AcademicPeriodState::ActiveTeaching,
            start_date,
            end_date,
            hijri_start: None,
            hijri_end: None,
            enrollment_starts_at: None,
            enrollment_ends_at: None,
            is_enrollment_open: true,
            grade_entry_starts_at: None,
            grade_entry_ends_at: None,
            is_grade_entry_open: true,
            exam_starts_at: None,
            exam_ends_at: None,
            is_locked: false,
            locked_at: None,
            locked_by: None,
            sort_order: Some(0),
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

    /// List academic periods by year
    pub async fn list_periods_by_year(&self, year_id: Uuid) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_year(year_id).await
    }

    /// List all academic periods for an organization
    pub async fn list_periods_by_org(&self, org_id: Uuid) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_org(org_id).await
    }

    /// Get academic period by ID
    pub async fn get_period(&self, id: Uuid) -> Result<Option<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_id(id).await
    }

    /// List period templates
    pub async fn list_period_templates(&self, org_id: Uuid) -> Result<Vec<PeriodTemplate>, DomainError> {
        self.template_repo.find_by_org(org_id).await
    }
}
