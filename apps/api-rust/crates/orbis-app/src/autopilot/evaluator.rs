use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::repositories::{AcademicCalendarRepository, AcademicPeriodRepository};
use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskType};
use orbis_domain::entities::academic_calendar::AcademicCalendarStatus;
use orbis_domain::entities::academic_period::PeriodKind;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;

pub struct AutopilotEvaluator {
    pub year_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
}

impl AutopilotEvaluator {
    pub fn new(
        year_repo: Arc<dyn AcademicCalendarRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
    ) -> Self {
        Self {
            year_repo,
            period_repo,
        }
    }

    /// Evaluates organization policies and returns proposed Autopilot tasks
    pub async fn evaluate(
        &self,
        _ctx: &RequestContext,
        org_id: Uuid,
        settings: &OrganizationSettings,
    ) -> Result<Vec<AutopilotTask>, DomainError> {
        let now = Utc::now();
        let mut tasks = Vec::new();

        if !settings.autopilot.enabled {
            return Ok(tasks);
        }

        // ── 1. Academic Year Lifecycle ──────────────────────────────────
        let periods = self.period_repo.find_by_org(org_id).await?;
        let years = self.year_repo.find_by_org(org_id).await?;
        for year in &years {
            let starts_at = year.starts_at.unwrap_or(year.start_date);
            if year.status == AcademicCalendarStatus::Planning && starts_at <= now {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::YearActivate,
                    Some(year.id),
                    Some("academic_calendar".to_string()),
                    RiskLevel::Medium,
                    Some(format!(
                        "Academic year '{}' start date has arrived",
                        year.name
                    )),
                    starts_at,
                ));
            }

            if year.status == AcademicCalendarStatus::Active {
                let year_periods: Vec<_> = periods
                    .iter()
                    .filter(|period| period.year_id == year.id)
                    .collect();
                if !year_periods.is_empty()
                    && year_periods
                        .iter()
                        .all(|p| p.state.to_string() == "closed" || p.is_locked)
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::YearClose,
                        Some(year.id),
                        Some("academic_calendar".to_string()),
                        RiskLevel::High,
                        Some(format!(
                            "All periods in year '{}' are closed. Ready for year closing",
                            year.name
                        )),
                        year.start_date,
                    ));
                }
            }

            if year.status == AcademicCalendarStatus::Closed && settings.autopilot.auto_promotion {
                if let Some(period) = periods
                    .iter()
                    .filter(|period| {
                        period.year_id == year.id && period.period_kind == PeriodKind::Teaching
                    })
                    .max_by_key(|period| (period.end_date, period.id))
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::PromotionDryRun,
                        Some(period.id),
                        Some("academic_period".into()),
                        RiskLevel::Medium,
                        Some(format!(
                            "Year '{}' is closed; progression preview recommended",
                            year.name
                        )),
                        period.end_date,
                    ));
                }
            }
        }

        // ── 2. Academic Period Lifecycle Gates ─────────────────────────
        for period in &periods {
            let period_state_str = period.state.to_string();

            // Period Activation
            if period.period_kind == PeriodKind::Teaching
                && period_state_str == "planning"
                && period.start_date <= now
                && now <= period.end_date
            {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::PeriodActivate,
                    Some(period.id),
                    Some("academic_period".to_string()),
                    RiskLevel::Medium,
                    Some(format!("Period '{}' start date has arrived", period.name)),
                    period.start_date,
                ));
            }

            // Enrollment Window: Open
            if let Some(enroll_start) = period.enrollment_starts_at {
                if !period.is_enrollment_open
                    && enroll_start <= now
                    && period.enrollment_ends_at.is_none_or(|end| now <= end)
                    && period_state_str == "active_teaching"
                    && settings.autopilot.auto_open_enrollment
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::EnrollmentOpen,
                        Some(period.id),
                        Some("academic_period".to_string()),
                        RiskLevel::Low,
                        Some(format!(
                            "Enrollment start date reached for period '{}'",
                            period.name
                        )),
                        enroll_start,
                    ));
                }
            }

            // Enrollment Window: Close
            if let Some(enroll_end) = period.enrollment_ends_at {
                if period.is_enrollment_open
                    && enroll_end < now
                    && settings.autopilot.auto_close_enrollment
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::EnrollmentClose,
                        Some(period.id),
                        Some("academic_period".to_string()),
                        RiskLevel::Low,
                        Some(format!(
                            "Enrollment deadline passed for period '{}'",
                            period.name
                        )),
                        enroll_end,
                    ));
                }
            }

            // Grade Entry Window: Open
            if let Some(grade_start) = period.grade_entry_starts_at {
                if !period.is_grade_entry_open
                    && grade_start <= now
                    && period.grade_entry_ends_at.is_none_or(|end| now <= end)
                    && (period_state_str == "active_teaching"
                        || period_state_str == "grade_submission")
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::GradeEntryOpen,
                        Some(period.id),
                        Some("academic_period".to_string()),
                        RiskLevel::Low,
                        Some(format!(
                            "Grade entry window started for period '{}'",
                            period.name
                        )),
                        grade_start,
                    ));
                }
            }

            // Grade Entry Window: Close
            if let Some(grade_end) = period.grade_entry_ends_at {
                if period.is_grade_entry_open && grade_end < now {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::GradeEntryClose,
                        Some(period.id),
                        Some("academic_period".to_string()),
                        RiskLevel::Medium,
                        Some(format!(
                            "Grade entry deadline passed for period '{}'",
                            period.name
                        )),
                        grade_end,
                    ));
                }
            }

            // Auto-lock period after end date + grace days
            let grace_duration = chrono::Duration::days(settings.autopilot.grace_days as i64);
            let lock_at = period.end_date + grace_duration;
            if !period.is_locked && lock_at < now && settings.autopilot.auto_period_lock {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::PeriodLock,
                    Some(period.id),
                    Some("academic_period".to_string()),
                    RiskLevel::High,
                    Some(format!(
                        "Period '{}' ended and grace period expired — auto-locking",
                        period.name
                    )),
                    lock_at,
                ));
            }
        }

        Ok(tasks)
    }
}
