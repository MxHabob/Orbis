use chrono::{Datelike, DateTime, Utc};
use orbis_domain::entities::academic_period::{AcademicPeriod, AcademicPeriodState};
use orbis_domain::entities::academic_calendar::{AcademicCalendar, AcademicCalendarStatus};
use orbis_domain::entities::assessment_scheme::AssessmentScheme;
use orbis_domain::entities::period_type::PeriodType;
use orbis_domain::errors::DomainError;
use orbis_domain::events::DomainEvent;
use orbis_domain::iam::permission::Permission;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::event_bus::EventBus;
use crate::ports::iam::PermissionCheckerPort;
use crate::ports::repositories::{
    AcademicPeriodRepository, AcademicCalendarRepository, PeriodTypeRepository,
};

pub struct CalendarService {
    pub year_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub template_repo: Arc<dyn PeriodTypeRepository>,
    pub event_bus: Arc<dyn EventBus>,
    pub permission_checker: Arc<dyn PermissionCheckerPort>,
}

impl CalendarService {
    pub fn new(
        year_repo: Arc<dyn AcademicCalendarRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        template_repo: Arc<dyn PeriodTypeRepository>,
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
    ) -> Result<AcademicCalendar, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Academic year name cannot be empty.".into(),
            ));
        }
        if end_date <= start_date {
            return Err(DomainError::Validation(
                "Academic year end date must be after its start date.".into(),
            ));
        }

        let now = Utc::now();
        let year_id = Uuid::new_v4();

        let year = AcademicCalendar {
            id: year_id,
            org_id,
            code: None,
            name,
            name_ar,
            start_date,
            end_date,
            starts_at: Some(start_date),
            ends_at: Some(end_date),
            hijri_start: None,
            hijri_end: None,
            status: AcademicCalendarStatus::Active,
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

        let event = DomainEvent::AcademicCalendarCreated {
            year_id,
            org_id,
            timestamp: now,
        };
        self.event_bus.publish(ctx, &event).await?;

        Ok(year)
    }

    /// List academic years for an organization
    pub async fn list_years(&self, org_id: Uuid) -> Result<Vec<AcademicCalendar>, DomainError> {
        self.year_repo.find_by_org(org_id).await
    }

    /// Get current active academic year
    pub async fn get_current_year(
        &self,
        org_id: Uuid,
    ) -> Result<Option<AcademicCalendar>, DomainError> {
        self.year_repo.find_current(org_id).await
    }

    /// Get academic year by ID
    pub async fn get_year(&self, id: Uuid) -> Result<Option<AcademicCalendar>, DomainError> {
        self.year_repo.find_by_id(id).await
    }

    /// Delete an academic year only when it has no academic periods.
    pub async fn delete_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<(), DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let year = self
            .year_repo
            .find_by_id(year_id)
            .await?
            .filter(|year| year.org_id == ctx.tenant_id)
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        if !self.period_repo.find_by_year(year.id).await?.is_empty() {
            return Err(DomainError::Validation(
                "Cannot delete an academic year that has periods. Delete its periods first."
                    .into(),
            ));
        }

        self.year_repo.delete(year_id).await
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

        if name.trim().is_empty() {
            return Err(DomainError::Validation(
                "Academic period name cannot be empty.".into(),
            ));
        }
        if end_date <= start_date {
            return Err(DomainError::Validation(
                "Academic period end date must be after its start date.".into(),
            ));
        }

        let year = self
            .year_repo
            .find_by_id(year_id)
            .await?
            .filter(|year| year.org_id == org_id)
            .ok_or_else(|| DomainError::NotFound("Academic year not found.".into()))?;
        if start_date < year.start_date || end_date > year.end_date {
            return Err(DomainError::Validation(
                "Academic period dates must fall within its academic year.".into(),
            ));
        }

        if let Some(parent_id) = parent_id {
            let parent = self
                .period_repo
                .find_by_id(parent_id)
                .await?
                .filter(|parent| parent.org_id == org_id && parent.year_id == year_id)
                .ok_or_else(|| DomainError::Validation(
                    "Academic period parent must belong to the same organization and year."
                        .into(),
                ))?;
            if start_date < parent.start_date || end_date > parent.end_date {
                return Err(DomainError::Validation(
                    "Academic period dates must fall within its parent period.".into(),
                ));
            }
        }

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
    pub async fn list_period_types(
        &self,
        org_id: Uuid,
    ) -> Result<Vec<PeriodType>, DomainError> {
        self.template_repo.find_by_org(org_id).await
    }

    /// Activate an academic year
    pub async fn activate_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicCalendar, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = AcademicCalendarStatus::Active;
        year.updated_at = Utc::now();
        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Mark academic year as closing (initiates promotion dry-run)
    pub async fn start_year_closing(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicCalendar, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = AcademicCalendarStatus::Closing;
        year.updated_at = Utc::now();
        self.year_repo.save(&year).await?;
        Ok(year)
    }

    /// Close an academic year
    pub async fn close_year(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
    ) -> Result<AcademicCalendar, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;
        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        year.status = AcademicCalendarStatus::Closed;
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
    ) -> Result<AcademicCalendar, DomainError> {
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
    ) -> Result<AcademicCalendar, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        let mut year =
            self.year_repo.find_by_id(year_id).await?.ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        let now = Utc::now();
        year.status = AcademicCalendarStatus::Archived;
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

    // ── Official Calendar Templates ─────────────────────────────

    /// Returns the list of available official calendar templates.
    pub fn list_official_templates(
        &self,
    ) -> Vec<super::calendar_templates::CalendarTemplateInfo> {
        super::calendar_templates::list_all_templates()
    }

    /// Apply an official calendar template to an academic year.
    ///
    /// This creates the supplementary periods defined by the template
    /// (prep, breaks, public exams) alongside the core teaching structure.
    /// Periods with matching codes that already exist are skipped (idempotent).
    ///
    /// The core 8-month Hijri teaching structure should be provisioned
    /// separately via `OnboardingService::provision_hijri_school_calendar`.
    /// This method adds the supplementary periods that complete the
    /// official calendar.
    pub async fn apply_official_calendar_template(
        &self,
        ctx: &RequestContext,
        year_id: Uuid,
        template_code: &str,
    ) -> Result<Vec<AcademicPeriod>, DomainError> {
        self.permission_checker
            .check(ctx, Permission::PeriodWrite)
            .await?;

        // Validate template exists
        let _template =
            super::calendar_templates::find_template(template_code).ok_or_else(|| {
                DomainError::NotFound(format!(
                    "Official calendar template '{}' not found",
                    template_code
                ))
            })?;

        // Load the year
        let year = self
            .year_repo
            .find_by_id(year_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Academic year {} not found", year_id))
            })?;

        // Load existing periods for idempotency check
        let existing_periods = self.period_repo.find_by_year(year_id).await?;
        let existing_codes: Vec<String> = existing_periods
            .iter()
            .filter_map(|p| p.code.clone())
            .collect();

        // Find term boundaries for placing mid-year break and public exams
        let term1_end = existing_periods
            .iter()
            .find(|p| p.code.as_deref() == Some("T1") && p.parent_id.is_none())
            .map(|p| p.end_date.date_naive());

        let term2_end = existing_periods
            .iter()
            .find(|p| p.code.as_deref() == Some("T2") && p.parent_id.is_none())
            .map(|p| p.end_date.date_naive());

        let now = Utc::now();
        let year_start = year.start_date.date_naive();

        // Build supplementary periods using the template
        let new_periods =
            super::calendar_templates::yemen_1447_1448::build_supplementary_periods(
                year.org_id,
                year_id,
                year_start,
                term1_end,
                term2_end,
                &existing_codes,
                now,
            );

        // Persist the new periods
        let mut created = Vec::new();
        for period in new_periods {
            self.period_repo.save(&period).await?;
            created.push(period);
        }

        // Update year metadata with provenance
        let mut updated_year = year;
        updated_year.notes = Some(format!(
            "Official calendar: Ministerial Decision No. {} — {}",
            super::calendar_templates::yemen_1447_1448::DECISION_NUMBER,
            super::calendar_templates::yemen_1447_1448::DECISION_AUTHORITY,
        ));
        updated_year.updated_at = now;
        self.year_repo.save(&updated_year).await?;

        Ok(created)
    }

    /// Authoritative source of truth for calendar defaults, structured periods,
    /// statutory assessment schemes, and timetable slots based on institution type and country profile.
    pub fn get_calendar_defaults(
        &self,
        org_id: Uuid,
        inst_type: &str,
        country_code: &str,
    ) -> CalendarDefaultsDto {
        let is_yemen_school =
            inst_type == "school" && (country_code == "YE" || country_code.is_empty());

        if is_yemen_school {
            let supp_defs = super::calendar_templates::yemen_1447_1448::supplementary_period_defs();

            // 1. Authoritative Assessment Schemes directly from domain factories
            let monthly = AssessmentScheme::default_school_monthly(org_id);
            let term_exam = AssessmentScheme::default_periodic_term_exam(org_id);
            let term_agg = AssessmentScheme::default_yemen_school_term_aggregation(org_id);
            let thanawiya = AssessmentScheme::yemen_secondary_certificate(org_id);
            let basic = AssessmentScheme::yemen_basic_certificate(org_id);

            let assessment_schemes = vec![
                AssessmentSchemeSummaryDto {
                    name: monthly.name,
                    name_ar: monthly.name_ar.unwrap_or_default(),
                    scheme_type: "periodic_monthly".to_string(),
                    max_score: monthly.max_score.to_f64().unwrap_or(100.0),
                    passing_score: 50.0,
                    description_ar: monthly.description.unwrap_or_default(),
                },
                AssessmentSchemeSummaryDto {
                    name: term_exam.name,
                    name_ar: term_exam.name_ar.unwrap_or_default(),
                    scheme_type: "periodic_term_exam".to_string(),
                    max_score: term_exam.max_score.to_f64().unwrap_or(30.0),
                    passing_score: 10.0,
                    description_ar: term_exam.description.unwrap_or_default(),
                },
                AssessmentSchemeSummaryDto {
                    name: term_agg.name,
                    name_ar: term_agg.name_ar.unwrap_or_default(),
                    scheme_type: "term_aggregation".to_string(),
                    max_score: term_agg.max_score.to_f64().unwrap_or(50.0),
                    passing_score: 25.0,
                    description_ar: term_agg.description.unwrap_or_default(),
                },
                AssessmentSchemeSummaryDto {
                    name: "Annual General Total (100 pts)".to_string(),
                    name_ar: "المجموع السنوي العام (100 درجة)".to_string(),
                    scheme_type: "annual_aggregation".to_string(),
                    max_score: 100.0,
                    passing_score: 50.0,
                    description_ar: "محصلة الفصل الأول (50) + محصلة الفصل الثاني (50) — عتبة النجاح 50%".to_string(),
                },
                AssessmentSchemeSummaryDto {
                    name: thanawiya.name,
                    name_ar: thanawiya.name_ar.unwrap_or_default(),
                    scheme_type: "secondary_certificate".to_string(),
                    max_score: thanawiya.max_score.to_f64().unwrap_or(100.0),
                    passing_score: 50.0,
                    description_ar: thanawiya.description.unwrap_or_default(),
                },
                AssessmentSchemeSummaryDto {
                    name: basic.name,
                    name_ar: basic.name_ar.unwrap_or_default(),
                    scheme_type: "basic_certificate".to_string(),
                    max_score: basic.max_score.to_f64().unwrap_or(100.0),
                    passing_score: 50.0,
                    description_ar: basic.description.unwrap_or_default(),
                },
            ];

            // 2. Authoritative Period Hierarchy (2 Terms, 8 Teaching Months, Monthly Exams, Finals, Supplementary Breaks/Exams)
            let month_names = [
                ("Muharram", "محرم"),
                ("Safar", "صفر"),
                ("Rabi' al-Awwal", "ربيع الأول"),
                ("Rabi' al-Thani", "ربيع الآخر"),
                ("Jumada al-Ula", "جمادى الأولى"),
                ("Jumada al-Thani", "جمادى الآخرة"),
                ("Rajab", "رجب"),
                ("Sha'ban", "شعبان"),
            ];

            let mut periods_summary = Vec::new();
            // Term 1
            periods_summary.push(PeriodSummaryDto {
                code: "T1".to_string(),
                name: "First Term".to_string(),
                name_ar: "الفصل الدراسي الأول".to_string(),
                period_type: "semester".to_string(),
                period_kind: "teaching".to_string(),
                sort_order: 1,
                parent_code: None,
            });

            for i in 0..4 {
                let m_num = i + 1;
                let (m_en, m_ar) = month_names[i];
                periods_summary.push(PeriodSummaryDto {
                    code: format!("H{}-M{:02}", super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR, m_num),
                    name: format!("{} {}", m_en, super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR),
                    name_ar: format!("{} {}", m_ar, super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR),
                    period_type: "month".to_string(),
                    period_kind: "teaching".to_string(),
                    sort_order: m_num as i32,
                    parent_code: Some("T1".to_string()),
                });
                if i < 3 {
                    periods_summary.push(PeriodSummaryDto {
                        code: format!("H{}-M{:02}-EXAM", super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR, m_num),
                        name: format!("{} Exam", m_en),
                        name_ar: format!("اختبار {} الشهري", m_ar),
                        period_type: "month".to_string(),
                        period_kind: "examination".to_string(),
                        sort_order: 100 + m_num as i32,
                        parent_code: Some("T1".to_string()),
                    });
                } else {
                    periods_summary.push(PeriodSummaryDto {
                        code: "T1-FINAL".to_string(),
                        name: "Term 1 Final Examination".to_string(),
                        name_ar: "اختبار نهاية الفصل الدراسي الأول".to_string(),
                        period_type: "month".to_string(),
                        period_kind: "examination".to_string(),
                        sort_order: 104,
                        parent_code: Some("T1".to_string()),
                    });
                }
            }

            // Mid-year vacation from template
            if let Some(midyear) = supp_defs.iter().find(|d| d.code == "YE-MIDYEAR-BREAK") {
                periods_summary.push(PeriodSummaryDto {
                    code: midyear.code.to_string(),
                    name: midyear.name.to_string(),
                    name_ar: midyear.name_ar.to_string(),
                    period_type: format!("{:?}", midyear.period_type).to_lowercase(),
                    period_kind: format!("{:?}", midyear.period_kind).to_lowercase(),
                    sort_order: midyear.sort_order,
                    parent_code: None,
                });
            }

            // Term 2
            periods_summary.push(PeriodSummaryDto {
                code: "T2".to_string(),
                name: "Second Term".to_string(),
                name_ar: "الفصل الدراسي الثاني".to_string(),
                period_type: "semester".to_string(),
                period_kind: "teaching".to_string(),
                sort_order: 6,
                parent_code: None,
            });

            for i in 4..8 {
                let m_num = i + 1;
                let (m_en, m_ar) = month_names[i];
                periods_summary.push(PeriodSummaryDto {
                    code: format!("H{}-M{:02}", super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR, m_num),
                    name: format!("{} {}", m_en, super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR),
                    name_ar: format!("{} {}", m_ar, super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR),
                    period_type: "month".to_string(),
                    period_kind: "teaching".to_string(),
                    sort_order: m_num as i32,
                    parent_code: Some("T2".to_string()),
                });
                if i < 7 {
                    periods_summary.push(PeriodSummaryDto {
                        code: format!("H{}-M{:02}-EXAM", super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR, m_num),
                        name: format!("{} Exam", m_en),
                        name_ar: format!("اختبار {} الشهري", m_ar),
                        period_type: "month".to_string(),
                        period_kind: "examination".to_string(),
                        sort_order: 100 + m_num as i32,
                        parent_code: Some("T2".to_string()),
                    });
                } else {
                    periods_summary.push(PeriodSummaryDto {
                        code: "T2-FINAL".to_string(),
                        name: "Term 2 Final Examination".to_string(),
                        name_ar: "اختبار نهاية الفصل الدراسي الثاني".to_string(),
                        period_type: "month".to_string(),
                        period_kind: "examination".to_string(),
                        sort_order: 108,
                        parent_code: Some("T2".to_string()),
                    });
                }
            }

            // Public examination windows
            for def in &supp_defs {
                if def.code != "YE-MIDYEAR-BREAK" && def.code != "YE-ADMIN-OPEN" && def.code != "YE-TEACHER-PREP" {
                    periods_summary.push(PeriodSummaryDto {
                        code: def.code.to_string(),
                        name: def.name.to_string(),
                        name_ar: def.name_ar.to_string(),
                        period_type: format!("{:?}", def.period_type).to_lowercase(),
                        period_kind: format!("{:?}", def.period_kind).to_lowercase(),
                        sort_order: def.sort_order,
                        parent_code: None,
                    });
                }
            }

            // 3. Timetable slots (7 lesson periods: 07:30 to 13:30 + 2 breaks)
            let timetable_slots = vec![
                TimeSlotSummaryDto {
                    slot_index: 1,
                    name: "Lesson 1".to_string(),
                    name_ar: "الحصة الأولى".to_string(),
                    start_time: "07:30".to_string(),
                    end_time: "08:15".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 2,
                    name: "Lesson 2".to_string(),
                    name_ar: "الحصة الثانية".to_string(),
                    start_time: "08:15".to_string(),
                    end_time: "09:00".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 3,
                    name: "Morning Break".to_string(),
                    name_ar: "الفسحة الصباحية".to_string(),
                    start_time: "09:00".to_string(),
                    end_time: "09:15".to_string(),
                    is_break: true,
                    break_type: Some("short".to_string()),
                },
                TimeSlotSummaryDto {
                    slot_index: 4,
                    name: "Lesson 3".to_string(),
                    name_ar: "الحصة الثالثة".to_string(),
                    start_time: "09:15".to_string(),
                    end_time: "10:00".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 5,
                    name: "Lesson 4".to_string(),
                    name_ar: "الحصة الرابعة".to_string(),
                    start_time: "10:00".to_string(),
                    end_time: "10:45".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 6,
                    name: "Lunch Break".to_string(),
                    name_ar: "استراحة الغداء".to_string(),
                    start_time: "10:45".to_string(),
                    end_time: "11:15".to_string(),
                    is_break: true,
                    break_type: Some("lunch".to_string()),
                },
                TimeSlotSummaryDto {
                    slot_index: 7,
                    name: "Lesson 5".to_string(),
                    name_ar: "الحصة الخامسة".to_string(),
                    start_time: "11:15".to_string(),
                    end_time: "12:00".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 8,
                    name: "Lesson 6".to_string(),
                    name_ar: "الحصة السادسة".to_string(),
                    start_time: "12:00".to_string(),
                    end_time: "12:45".to_string(),
                    is_break: false,
                    break_type: None,
                },
                TimeSlotSummaryDto {
                    slot_index: 9,
                    name: "Lesson 7".to_string(),
                    name_ar: "الحصة السابعة".to_string(),
                    start_time: "12:45".to_string(),
                    end_time: "13:30".to_string(),
                    is_break: false,
                    break_type: None,
                },
            ];

            CalendarDefaultsDto {
                suggested_name: super::calendar_templates::yemen_1447_1448::TEMPLATE_NAME.to_string(),
                suggested_name_ar: super::calendar_templates::yemen_1447_1448::TEMPLATE_NAME_AR.to_string(),
                start_date: "2026-06-27T07:30".to_string(),
                end_date: "2027-02-07T13:30".to_string(),
                hijri_start_year: super::calendar_templates::yemen_1447_1448::HIJRI_START_YEAR,
                hijri_start_month: super::calendar_templates::yemen_1447_1448::HIJRI_START_MONTH,
                hijri_start_day: super::calendar_templates::yemen_1447_1448::HIJRI_START_DAY,
                hijri_month_lengths: vec![30, 29, 30, 29, 30, 29, 30, 29],
                selected_study_days: vec![7, 1, 2, 3, 4],
                periods_summary,
                assessment_schemes,
                timetable_slots,
            }
        } else {
            let current_year = chrono::Utc::now().year();
            CalendarDefaultsDto {
                suggested_name: format!("{}-{}", current_year, current_year + 1),
                suggested_name_ar: format!("{}-{} م", current_year, current_year + 1),
                start_date: format!("{}-09-01T08:00", current_year),
                end_date: format!("{}-06-30T14:00", current_year + 1),
                hijri_start_year: 1448,
                hijri_start_month: 1,
                hijri_start_day: 1,
                hijri_month_lengths: vec![30, 29, 30, 29, 30, 29, 30, 29],
                selected_study_days: vec![1, 2, 3, 4, 5],
                periods_summary: vec![
                    PeriodSummaryDto {
                        code: "FALL".to_string(),
                        name: "Fall Semester".to_string(),
                        name_ar: "فصل الخريف".to_string(),
                        period_type: "semester".to_string(),
                        period_kind: "teaching".to_string(),
                        sort_order: 1,
                        parent_code: None,
                    },
                    PeriodSummaryDto {
                        code: "SPRING".to_string(),
                        name: "Spring Semester".to_string(),
                        name_ar: "فصل الربيع".to_string(),
                        period_type: "semester".to_string(),
                        period_kind: "teaching".to_string(),
                        sort_order: 2,
                        parent_code: None,
                    },
                ],
                assessment_schemes: vec![
                    AssessmentSchemeSummaryDto {
                        name: "University Credit-Hour Assessment".to_string(),
                        name_ar: "نظام الساعات المعتمدة الجامعي".to_string(),
                        scheme_type: "credit_hour".to_string(),
                        max_score: 100.0,
                        passing_score: 60.0,
                        description_ar: "نظام تقييم الساعات المعتمدة (100 درجة)".to_string(),
                    },
                ],
                timetable_slots: vec![],
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarDefaultsDto {
    pub suggested_name: String,
    pub suggested_name_ar: String,
    pub start_date: String,
    pub end_date: String,
    pub hijri_start_year: i32,
    pub hijri_start_month: u8,
    pub hijri_start_day: u8,
    pub hijri_month_lengths: Vec<u8>,
    pub selected_study_days: Vec<i16>,
    pub periods_summary: Vec<PeriodSummaryDto>,
    pub assessment_schemes: Vec<AssessmentSchemeSummaryDto>,
    pub timetable_slots: Vec<TimeSlotSummaryDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodSummaryDto {
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub period_type: String,
    pub period_kind: String,
    pub sort_order: i32,
    pub parent_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentSchemeSummaryDto {
    pub name: String,
    pub name_ar: String,
    pub scheme_type: String,
    pub max_score: f64,
    pub passing_score: f64,
    pub description_ar: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlotSummaryDto {
    pub slot_index: i32,
    pub name: String,
    pub name_ar: String,
    pub start_time: String,
    pub end_time: String,
    pub is_break: bool,
    pub break_type: Option<String>,
}

