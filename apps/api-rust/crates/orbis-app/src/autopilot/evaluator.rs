use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::context::RequestContext;
use crate::ports::repositories::{
    AcademicCalendarRepository, AcademicPeriodRepository, ExamSessionRepository,
    StudentFeeRepository,
};
use orbis_domain::autopilot::task::{AutopilotTask, RiskLevel, TaskType};
use orbis_domain::entities::academic_calendar::AcademicCalendarStatus;
use orbis_domain::entities::academic_period::PeriodKind;
use orbis_domain::entities::settings::OrganizationSettings;
use orbis_domain::entities::student_fee::FeeStatus;
use orbis_domain::errors::DomainError;
use rust_decimal::Decimal;

pub struct AutopilotEvaluator {
    pub year_repo: Arc<dyn AcademicCalendarRepository>,
    pub period_repo: Arc<dyn AcademicPeriodRepository>,
    pub fee_repo: Option<Arc<dyn StudentFeeRepository>>,
    pub exam_session_repo: Option<Arc<dyn ExamSessionRepository>>,
}

impl AutopilotEvaluator {
    pub fn new(
        year_repo: Arc<dyn AcademicCalendarRepository>,
        period_repo: Arc<dyn AcademicPeriodRepository>,
    ) -> Self {
        Self {
            year_repo,
            period_repo,
            fee_repo: None,
            exam_session_repo: None,
        }
    }

    pub fn with_fee_repo(mut self, fee_repo: Arc<dyn StudentFeeRepository>) -> Self {
        self.fee_repo = Some(fee_repo);
        self
    }

    pub fn with_exam_session_repo(
        mut self,
        exam_session_repo: Arc<dyn ExamSessionRepository>,
    ) -> Self {
        self.exam_session_repo = Some(exam_session_repo);
        self
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

        let max_retries = settings.autopilot.max_retries.max(1);
        let push_task = |tasks: &mut Vec<AutopilotTask>, mut task: AutopilotTask| {
            task.max_retries = max_retries;
            tasks.push(task);
        };

        // ── 1. Academic Year Lifecycle ──────────────────────────────────
        let periods = self.period_repo.find_by_org(org_id).await?;
        let years = self.year_repo.find_by_org(org_id).await?;
        for year in &years {
            let starts_at = year.starts_at.unwrap_or(year.start_date);
            if year.status == AcademicCalendarStatus::Planning && starts_at <= now {
                push_task(
                    &mut tasks,
                    AutopilotTask::new(
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
                    ),
                );
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
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }

            if year.status == AcademicCalendarStatus::Closing {
                push_task(
                    &mut tasks,
                    AutopilotTask::new(
                        org_id,
                        TaskType::YearClose,
                        Some(year.id),
                        Some("academic_calendar".to_string()),
                        RiskLevel::High,
                        Some(format!(
                            "Year '{}' is closing — ready to finalize closure",
                            year.name
                        )),
                        year.end_date,
                    ),
                );
            }

            if year.status == AcademicCalendarStatus::Closed && settings.autopilot.auto_promotion {
                if let Some(period) = periods
                    .iter()
                    .filter(|period| {
                        period.year_id == year.id && period.period_kind == PeriodKind::Teaching
                    })
                    .max_by_key(|period| (period.end_date, period.id))
                {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }
        }

        // ── 2. Academic Period Lifecycle Gates ─────────────────────────
        for period in &periods {
            let period_state_str = period.state.to_string();

            // Period Activation (both Teaching and Examination periods)
            if matches!(
                period.period_kind,
                PeriodKind::Teaching | PeriodKind::Examination
            ) && period_state_str == "planning"
                && period.start_date <= now
                && now <= period.end_date
            {
                push_task(
                    &mut tasks,
                    AutopilotTask::new(
                        org_id,
                        TaskType::PeriodActivate,
                        Some(period.id),
                        Some("academic_period".to_string()),
                        RiskLevel::Medium,
                        Some(format!("Period '{}' start date has arrived", period.name)),
                        period.start_date,
                    ),
                );
            }

            // Enrollment Window: Open
            if let Some(enroll_start) = period.enrollment_starts_at {
                if !period.is_enrollment_open
                    && enroll_start <= now
                    && period.enrollment_ends_at.is_none_or(|end| now <= end)
                    && period_state_str == "active_teaching"
                    && settings.autopilot.auto_open_enrollment
                {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }

            // Enrollment Window: Close
            if let Some(enroll_end) = period.enrollment_ends_at {
                if period.is_enrollment_open
                    && enroll_end < now
                    && settings.autopilot.auto_close_enrollment
                {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }

            // Grade Entry Window: Open
            if let Some(grade_start) = period.grade_entry_starts_at {
                if !period.is_grade_entry_open
                    && grade_start <= now
                    && period.grade_entry_ends_at.is_none_or(|end| now <= end)
                    && matches!(
                        period_state_str.as_str(),
                        "active_teaching" | "final_exams" | "grade_submission"
                    )
                {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }

            // Grade Entry Window: Close (explicit deadline or end_date + grade_entry_auto_close_days)
            let grade_close_at = period.grade_entry_ends_at.or_else(|| {
                settings.academic.grade_entry_auto_close.then(|| {
                    period.end_date
                        + chrono::Duration::days(
                            settings.autopilot.grade_entry_auto_close_days as i64,
                        )
                })
            });

            // Transition to GradeSubmission after end_date while grade entry is still open
            if matches!(
                period.period_kind,
                PeriodKind::Teaching | PeriodKind::Examination
            ) && matches!(period_state_str.as_str(), "active_teaching" | "final_exams")
                && period.end_date < now
                && period.is_grade_entry_open
            {
                let mut task = AutopilotTask::new(
                    org_id,
                    TaskType::TransitionPeriod,
                    Some(period.id),
                    Some("academic_period".to_string()),
                    RiskLevel::Low,
                    Some(format!(
                        "Period '{}' ended — transitioning to grade submission phase",
                        period.name
                    )),
                    period.end_date,
                );
                task.parameters = Some(serde_json::json!({ "target_state": "grade_submission" }));
                push_task(&mut tasks, task);
            }

            // Pre-deadline Grade Entry Reminder
            if let Some(grade_end) = grade_close_at {
                if settings.autopilot.auto_grade_entry_reminder
                    && period.is_grade_entry_open
                    && now < grade_end
                {
                    let lead = chrono::Duration::hours(
                        settings.autopilot.notify_before_deadline_hours.max(1) as i64,
                    );
                    let remind_at = grade_end - lead;
                    if now >= remind_at {
                        let mut task = AutopilotTask::new(
                            org_id,
                            TaskType::SendDelayNotification,
                            Some(period.id),
                            Some("academic_period".to_string()),
                            RiskLevel::Low,
                            Some(format!(
                                "Grade entry deadline approaching for period '{}'",
                                period.name
                            )),
                            remind_at,
                        );
                        task.parameters =
                            Some(serde_json::json!({ "kind": "grade_entry_deadline" }));
                        push_task(&mut tasks, task);
                    }
                }
            }

            if let Some(grade_end) = grade_close_at {
                if period.is_grade_entry_open && grade_end < now {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
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
                        ),
                    );
                }
            }

            // Auto-create draft ExamSession for upcoming/active Examination periods
            if settings.autopilot.auto_exam_scheduling
                && period.period_kind == PeriodKind::Examination
                && period.start_date - chrono::Duration::days(14) <= now
                && now <= period.end_date
            {
                let has_session = if let Some(repo) = &self.exam_session_repo {
                    !repo.find_by_period(org_id, period.id).await?.is_empty()
                } else {
                    false
                };
                if !has_session {
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
                            org_id,
                            TaskType::CreateExamSession,
                            Some(period.id),
                            Some("academic_period".to_string()),
                            RiskLevel::Low,
                            Some(format!(
                                "Prepare exam session for examination period '{}'",
                                period.name
                            )),
                            period.start_date - chrono::Duration::days(14),
                        ),
                    );
                }
            }

            // Auto-lock period after end date + grace days (respecting manual unlocks via updated_at)
            let grace_duration = chrono::Duration::days(settings.autopilot.grace_days as i64);
            let lock_base = period.end_date.max(period.updated_at);
            let lock_at = lock_base + grace_duration;
            if !period.is_locked && lock_at < now && settings.autopilot.auto_period_lock {
                push_task(
                    &mut tasks,
                    AutopilotTask::new(
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
                    ),
                );
            }
        }

        // ── 3. Overdue Financial Fees Reminders ─────────────────────────
        if let Some(fee_repo) = &self.fee_repo {
            let today = now.date_naive();
            let fees = fee_repo.find_by_org(org_id).await?;
            for fee in fees {
                let is_unpaid = matches!(
                    fee.status,
                    FeeStatus::Pending | FeeStatus::PartiallyPaid | FeeStatus::Overdue
                );
                if is_unpaid && fee.balance_due > Decimal::ZERO && fee.due_date < today {
                    let due_utc = fee
                        .due_date
                        .and_hms_opt(0, 0, 0)
                        .map(|dt| dt.and_utc())
                        .unwrap_or(now);
                    push_task(
                        &mut tasks,
                        AutopilotTask::new(
                            org_id,
                            TaskType::LateFeeReminder,
                            Some(fee.id),
                            Some("student_fee".to_string()),
                            RiskLevel::Low,
                            Some(format!("Student fee {} is overdue", fee.id)),
                            due_utc,
                        ),
                    );
                }
            }
        }

        Ok(tasks)
    }
}
