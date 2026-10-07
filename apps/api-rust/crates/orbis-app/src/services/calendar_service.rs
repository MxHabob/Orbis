use chrono::{DateTime, Utc};
use orbis_domain::entities::academic_period::{AcademicPeriod, AcademicPeriodState};
use orbis_domain::entities::academic_year::AcademicYear;
use orbis_domain::entities::period_template::PeriodTemplate;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicPeriodRepository, AcademicYearRepository, PeriodTemplateRepository,
};

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
        name_ar: Option<String>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let now = Utc::now();
        let year_id = Uuid::new_v4();

        let year = AcademicYear {
            id: year_id,
            org_id,
            name,
            name_ar,
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
    pub async fn get_current_year(
        &self,
        org_id: Uuid,
    ) -> Result<Option<AcademicYear>, DomainError> {
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
        name_ar: Option<String>,
        code: Option<String>,
        period_type: String,
        period_kind: Option<String>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        use orbis_domain::entities::academic_period::{PeriodKind, PeriodType};

        let now = Utc::now();
        let period_id = Uuid::new_v4();
        let parsed_type = PeriodType::from_str(&period_type);
        let parsed_kind = match period_kind {
            Some(ref k) => PeriodKind::from_str(k),
            None => {
                if parsed_type.is_term() {
                    PeriodKind::Teaching
                } else {
                    PeriodKind::Teaching
                }
            }
        };

        let period = AcademicPeriod {
            id: period_id,
            org_id,
            year_id,
            parent_id,
            name,
            name_ar,
            code,
            period_type: parsed_type,
            period_kind: parsed_kind,
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
    pub async fn list_periods_by_year(
        &self,
        year_id: Uuid,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_year(year_id).await
    }

    /// List all academic periods for an organization
    pub async fn list_periods_by_org(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_org(org_id).await
    }

    /// Get academic period by ID
    pub async fn get_period(&self, id: Uuid) -> Result<Option<AcademicPeriod>, DomainError> {
        self.period_repo.find_by_id(id).await
    }

    /// List period templates
    pub async fn list_period_templates(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<PeriodTemplate>, DomainError> {
        self.template_repo.find_by_org(org_id).await
    }

    /// Activate an academic year
    pub async fn activate_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = "active".to_string();
        year.updated_at = Utc::now();
        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Mark academic year as closing (initiates promotion dry-run)
    pub async fn start_year_closing(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = "closing".to_string();
        year.updated_at = Utc::now();
        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Close an academic year
    pub async fn close_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = "closed".to_string();
        year.is_locked = true;
        year.locked_at = Some(Utc::now());
        year.updated_at = Utc::now();
        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Transition academic period to a new state
    pub async fn transition_period_state(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        new_state: AcademicPeriodState,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        let now = Utc::now();
        period.state = new_state;
        period.updated_at = now;

        match new_state {
            AcademicPeriodState::EnrollmentOpen => {
                period.is_enrollment_open = true;
            }
            AcademicPeriodState::ActiveTeaching => {
                period.is_enrollment_open = true;
                period.is_grade_entry_open = true;
            }
            AcademicPeriodState::GradeSubmission => {
                period.is_enrollment_open = false;
                period.is_grade_entry_open = true;
            }
            AcademicPeriodState::Closed | AcademicPeriodState::Archived => {
                period.is_enrollment_open = false;
                period.is_grade_entry_open = false;
                period.is_locked = true;
                period.locked_at = Some(now);
            }
            _ => {}
        }

        self.period_repo.save(&period).await?;

        let event = DomainEvent::AcademicPeriodStateChanged {
            period_id,
            org_id: period.org_id,
            new_state: new_state.to_string(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(period)
    }

    /// Open enrollment window for a period
    pub async fn open_enrollment_window(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        period.is_enrollment_open = true;
        period.updated_at = Utc::now();
        self.period_repo.save(&period).await?;
        Ok(period)
    }

    /// Close enrollment window for a period
    pub async fn close_enrollment_window(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        period.is_enrollment_open = false;
        period.updated_at = Utc::now();
        self.period_repo.save(&period).await?;
        Ok(period)
    }

    /// Open grade entry window for a period
    pub async fn open_grade_entry_window(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        period.is_grade_entry_open = true;
        period.updated_at = Utc::now();
        self.period_repo.save(&period).await?;
        Ok(period)
    }

    /// Close grade entry window for a period
    pub async fn close_grade_entry_window(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        period.is_grade_entry_open = false;
        period.updated_at = Utc::now();
        self.period_repo.save(&period).await?;
        Ok(period)
    }

    /// Auto-lock an academic period (grades, attendance, enrollment locked)
    pub async fn lock_period(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodLock)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        let now = Utc::now();
        period.is_locked = true;
        period.locked_at = Some(now);
        period.locked_by = ctx.actor.id();
        period.is_enrollment_open = false;
        period.is_grade_entry_open = false;
        period.state = AcademicPeriodState::Closed;
        period.updated_at = now;

        self.period_repo.save(&period).await?;

        let event = DomainEvent::AcademicPeriodStateChanged {
            period_id,
            org_id: period.org_id,
            new_state: "closed".to_string(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(period)
    }

    /// Unlock an academic period
    pub async fn unlock_period(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodUnlock)
            .await?;
        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        let now = Utc::now();
        period.is_locked = false;
        period.locked_at = None;
        period.locked_by = None;
        period.state = AcademicPeriodState::ActiveTeaching;
        period.updated_at = now;

        self.period_repo.save(&period).await?;

        let event = DomainEvent::AcademicPeriodStateChanged {
            period_id,
            org_id: period.org_id,
            new_state: "active_teaching".to_string(),
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(period)
    }

    /// Authoritatively close an academic period
    pub async fn close_period(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<AcademicPeriod, DomainError> {
        self.lock_period(ctx, period_id).await
    }

    /// Update an academic year's mutable fields
    pub async fn update_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        notes: Option<String>,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        if year.is_locked {
            return Err(DomainError::Validation(
                "Cannot update a locked academic year".into(),
            ));
        }

        if let Some(n) = name {
            year.name = n;
        }
        if let Some(n) = name_ar {
            year.name_ar = Some(n);
        }
        if let Some(d) = start_date {
            year.start_date = d;
        }
        if let Some(d) = end_date {
            year.end_date = d;
        }
        if let Some(n) = notes {
            year.notes = Some(n);
        }
        year.updated_at = Utc::now();

        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Archive an academic year
    pub async fn archive_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicYear, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        let now = Utc::now();
        year.status = "archived".to_string();
        year.archived_at = Some(now);
        year.archived_by = ctx.actor.id();
        year.is_locked = true;
        year.locked_at = Some(now);
        year.locked_by = ctx.actor.id();
        year.updated_at = now;

        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Update an academic period's mutable fields
    pub async fn update_period(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
        name: Option<String>,
        name_ar: Option<String>,
        code: Option<String>,
        start_date: Option<DateTime<Utc>>,
        end_date: Option<DateTime<Utc>>,
        period_kind: Option<String>,
    ) -> Result<AcademicPeriod, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let mut period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        if period.is_locked {
            return Err(DomainError::Validation(
                "Cannot update a locked period".into(),
            ));
        }

        if let Some(n) = name {
            period.name = n;
        }
        if let Some(n) = name_ar {
            period.name_ar = Some(n);
        }
        if let Some(c) = code {
            period.code = Some(c);
        }
        if let Some(s) = start_date {
            period.start_date = s;
        }
        if let Some(e) = end_date {
            period.end_date = e;
        }
        if let Some(k) = period_kind {
            period.period_kind = orbis_domain::entities::academic_period::PeriodKind::from_str(&k);
        }
        period.updated_at = Utc::now();

        self.period_repo.save(&period).await?;
        Ok(period)
    }

    /// Delete an academic period (if unlocked and has no child periods)
    pub async fn delete_period(
        &self,
        ctx: &RequestContext,
        period_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let period = self
            .period_repo
            .find_by_id(period_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic period {} not found", period_id))
            })?;

        if period.is_locked {
            return Err(DomainError::Validation(
                "Cannot delete a locked period".into(),
            ));
        }

        // Check no child periods exist
        let children = self.period_repo.find_by_year(period.year_id).await?;
        if children.iter().any(|c| c.parent_id == Some(period_id)) {
            return Err(DomainError::Validation(
                "Cannot delete a period with child periods. Please delete child periods first."
                    .into(),
            ));
        }

        self.period_repo.delete(period_id).await?;
        Ok(())
    }
}
