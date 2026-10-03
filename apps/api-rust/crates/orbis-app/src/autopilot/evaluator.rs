use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::repositories::{
    AcademicPeriodRepository, AcademicYearRepository, EnrollmentRepository, PolicyRepository,
    StudentFeeRepository,
};
use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskType};
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::errors::DomainError;
use orbis_domain::queries::filters::StudentFeeFilter;

pub struct AutopilotEvaluator {
    pub policy_repo: Arc<dyn PolicyRepository>,
    pub year_repo: Arc<dyn AcademicYearRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub enrollment_repo: Arc<dyn EnrollmentRepository>,
    pub student_fee_repo: Arc<dyn StudentFeeRepository>,
}

impl AutopilotEvaluator {
    pub fn new(
        policy_repo: Arc<dyn PolicyRepository>,
        year_repo: Arc<dyn AcademicYearRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
        enrollment_repo: Arc<dyn EnrollmentRepository>,
        student_fee_repo: Arc<dyn StudentFeeRepository>,
    ) -> Self {
        Self {
            policy_repo,
            year_repo,
            period_repo,
            enrollment_repo,
            student_fee_repo,
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
        let years = self.year_repo.find_by_org(org_id).await?;
        for year in &years {
            if year.status == "planning" && year.start_date <= now {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::YearActivate,
                    Some(year.id),
                    Some("academic_year".to_string()),
                    RiskLevel::Medium,
                    Some(format!(
                        "Academic year '{}' start date has arrived",
                        year.name
                    )),
                    now,
                ));
            }

            if year.status == "active" {
                let periods = self.period_repo.find_by_year(year.id).await?;
                if !periods.is_empty()
                    && periods
                        .iter()
                        .all(|p| p.state.to_string() == "closed" || p.is_locked)
                {
                    tasks.push(AutopilotTask::new(
                        org_id,
                        TaskType::YearClose,
                        Some(year.id),
                        Some("academic_year".to_string()),
                        RiskLevel::High,
                        Some(format!(
                            "All periods in year '{}' are closed. Ready for year closing",
                            year.name
                        )),
                        now,
                    ));
                }
            }

            if year.status == "closing" && settings.autopilot.auto_promotion {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::PromotionDryRun,
                    Some(year.id),
                    Some("academic_year".to_string()),
                    RiskLevel::Medium,
                    Some(format!(
                        "Year '{}' is closing — progression dry-run recommended",
                        year.name
                    )),
                    now,
                ));
            }
        }

        // ── 2. Academic Period Lifecycle Gates ─────────────────────────
        let periods = self.period_repo.find_by_org(org_id).await?;
        for period in &periods {
            let period_state_str = period.state.to_string();

            // Period Activation
            if period_state_str == "planning" && period.start_date <= now {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::PeriodActivate,
                    Some(period.id),
                    Some("academic_period".to_string()),
                    RiskLevel::Medium,
                    Some(format!("Period '{}' start date has arrived", period.name)),
                    now,
                ));
            }

            // Enrollment Window: Open
            if let Some(enroll_start) = period.enrollment_starts_at {
                if !period.is_enrollment_open
                    && enroll_start <= now
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
                        now,
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
                        now,
                    ));
                }
            }

            // Grade Entry Window: Open
            if let Some(grade_start) = period.grade_entry_starts_at {
                if !period.is_grade_entry_open
                    && grade_start <= now
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
                        now,
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
                        now,
                    ));
                }
            }

            // Auto-lock period after end date + grace days
            let grace_duration = chrono::Duration::days(settings.autopilot.grace_days as i64);
            if !period.is_locked
                && (period.end_date + grace_duration) < now
                && settings.autopilot.auto_period_lock
            {
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
                    now,
                ));
            }
        }

        // ── 3. Overdue Financial Fees ─────────────────────────────────
        if settings.autopilot.auto_fee_assignment {
            let mut filter = StudentFeeFilter::default();
            filter.is_overdue = Some(true);
            filter.status = Some("pending".to_string());
            let overdue_result = self.student_fee_repo.list(org_id, filter).await?;
            for fee in overdue_result.items {
                tasks.push(AutopilotTask::new(
                    org_id,
                    TaskType::LateFeeReminder,
                    Some(fee.id),
                    Some("student_fee".to_string()),
                    RiskLevel::Low,
                    Some(format!(
                        "Student fee {} is overdue since {:?}",
                        fee.id, fee.due_date
                    )),
                    now,
                ));
            }
        }

        Ok(tasks)
    }
}
