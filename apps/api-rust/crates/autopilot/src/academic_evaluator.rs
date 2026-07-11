//! AcademicEvaluator — reads academic period dates and proposes lifecycle tasks.
//!
//! Unlike the original Evaluator (which relied on calendar_events as intermediary),
//! this evaluator reads directly from `academic_periods` and `academic_years`
//! to determine what actions Autopilot should take.
//!
//! It produces a `Vec<ProposedTask>` consumed by the Planner.

use chrono::{NaiveDate, Utc};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use tracing::{debug, warn};

use shared_kernel::errors::AppError;
use svc_organization::entities::{academic_year, academic_period, academic_policy};

/// A task proposed by the evaluator for the planner to create.
#[derive(Debug, Clone)]
pub struct ProposedTask {
    pub task_type: String,
    pub target_id: String,
    pub target_type: String,
    pub reason: String,
    pub risk: Risk,
    /// Additional context as JSON
    pub params: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Risk {
    Low,
    Medium,
    High,
    Critical,
}

impl Risk {
    pub fn as_str(&self) -> &'static str {
        match self {
            Risk::Low => "low",
            Risk::Medium => "medium",
            Risk::High => "high",
            Risk::Critical => "critical",
        }
    }
}

impl ProposedTask {
    pub fn new(
        task_type: &str,
        target_id: &str,
        target_type: &str,
        reason: String,
        risk: Risk,
    ) -> Self {
        Self {
            task_type: task_type.to_string(),
            target_id: target_id.to_string(),
            target_type: target_type.to_string(),
            reason,
            risk,
            params: None,
        }
    }
}

pub struct AcademicEvaluator;

impl AcademicEvaluator {
    /// Evaluate the academic state and return proposed tasks.
    ///
    /// Reads directly from `academic_years` and `academic_periods` —
    /// no calendar_events intermediary needed.
    pub async fn evaluate(
        db: &DatabaseConnection,
        org_id: &str,
        policy: &academic_policy::Model,
    ) -> Result<Vec<ProposedTask>, AppError> {
        let today = Utc::now().date_naive();
        let mut tasks = Vec::new();

        // ── 1. Year lifecycle ──────────────────────────────────────────
        let years = academic_year::Entity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .filter(academic_year::Column::Status.is_not_in(["archived"]))
            .all(db)
            .await?;

        for year in &years {
            // Year should be activated?
            if year.status == "planning" {
                if let Some(start) = parse_date(&year.start_date) {
                    if start <= today {
                        tasks.push(ProposedTask::new(
                            "year.activate",
                            &year.id,
                            "academic_year",
                            format!("Year start date {} has arrived", year.start_date),
                            Risk::Medium,
                        ));
                    }
                }
            }
        }

        // ── 2. Period lifecycle ─────────────────────────────────────────
        let periods = academic_period::Entity::find()
            .filter(academic_period::Column::OrgId.eq(org_id))
            .filter(academic_period::Column::Status.is_not_in(["closed"]))
            .all(db)
            .await?;

        for period in &periods {
            let period_start = parse_date(&period.start_date);
            let period_end = parse_date(&period.end_date);

            // Period should be activated?
            if period.status == "planned" {
                if let Some(start) = period_start {
                    if start <= today {
                        tasks.push(ProposedTask::new(
                            "period.activate",
                            &period.id,
                            "academic_period",
                            format!("Period '{}' start date has arrived", period.name),
                            Risk::Medium,
                        ));
                    }
                }
            }

            // ── Enrollment window ──
            if policy.auto_open_enrollment == 1 {
                if let Some(ref enr_start) = period.enrollment_start {
                    if period.is_enrollment_open == 0 {
                        if let Some(d) = parse_date(enr_start) {
                            if d <= today && period.status == "active" {
                                tasks.push(ProposedTask::new(
                                    "enrollment.open",
                                    &period.id,
                                    "academic_period",
                                    format!("Enrollment start date {} reached", enr_start),
                                    Risk::Low,
                                ));
                            }
                        }
                    }
                }
            }

            if policy.auto_close_enrollment == 1 {
                if let Some(ref enr_end) = period.enrollment_end {
                    if period.is_enrollment_open == 1 {
                        if let Some(d) = parse_date(enr_end) {
                            if d < today {
                                tasks.push(ProposedTask::new(
                                    "enrollment.close",
                                    &period.id,
                                    "academic_period",
                                    format!("Enrollment end date {} has passed", enr_end),
                                    Risk::Medium,
                                ));
                            }
                        }
                    }
                }
            }

            // ── Grade entry window ──
            if policy.auto_open_grade_entry == 1 {
                if let Some(ref ge_start) = period.grade_entry_start {
                    if period.is_grade_entry_open == 0 && period.status == "active" {
                        if let Some(d) = parse_date(ge_start) {
                            if d <= today {
                                tasks.push(ProposedTask::new(
                                    "grade_entry.open",
                                    &period.id,
                                    "academic_period",
                                    format!("Grade entry start date {} reached", ge_start),
                                    Risk::Low,
                                ));
                            }
                        }
                    }
                }
            }

            if policy.auto_close_grade_entry == 1 {
                if let Some(ref ge_end) = period.grade_entry_end {
                    if period.is_grade_entry_open == 1 {
                        if let Some(d) = parse_date(ge_end) {
                            let deadline = d + chrono::Duration::days(
                                policy.grace_days_after_deadline as i64,
                            );
                            if deadline < today {
                                tasks.push(ProposedTask::new(
                                    "grade_entry.close",
                                    &period.id,
                                    "academic_period",
                                    format!(
                                        "Grade entry deadline {} + {} grace days exceeded",
                                        ge_end, policy.grace_days_after_deadline
                                    ),
                                    Risk::Medium,
                                ));
                            }
                        }
                    }
                }
            }

            // ── Exam → grading transition ──
            if period.status == "active" {
                if let Some(ref exam_end) = period.exam_end {
                    if let Some(d) = parse_date(exam_end) {
                        if d <= today {
                            tasks.push(ProposedTask::new(
                                "period.start_grading",
                                &period.id,
                                "academic_period",
                                format!("Exam period ended on {}", exam_end),
                                Risk::Medium,
                            ));
                        }
                    }
                }
            }

            // ── Period lock ──
            if policy.auto_lock_periods == 1 && period.is_locked == 0 {
                if let Some(end) = period_end {
                    let lock_date = end + chrono::Duration::days(
                        policy.grace_days_after_deadline as i64,
                    );
                    if lock_date <= today {
                        tasks.push(ProposedTask::new(
                            "period.lock",
                            &period.id,
                            "academic_period",
                            format!(
                                "Period end date {} + {} grace days exceeded",
                                period.end_date, policy.grace_days_after_deadline
                            ),
                            Risk::High,
                        ));
                    }
                }
            }

            // ── Period close ──
            if period.status != "closed" && period.is_locked == 1 {
                // If locked and all grade entries are closed, suggest closing
                if period.is_grade_entry_open == 0 && period.is_enrollment_open == 0 {
                    tasks.push(ProposedTask::new(
                        "period.close",
                        &period.id,
                        "academic_period",
                        format!("Period '{}' is locked with no open windows", period.name),
                        Risk::High,
                    ));
                }
            }
        }

        // ── 3. Year closing detection ──────────────────────────────────
        for year in &years {
            if year.status == "active" {
                let year_periods = academic_period::Entity::find()
                    .filter(academic_period::Column::YearId.eq(&year.id))
                    .filter(academic_period::Column::ParentId.is_null()) // top-level only
                    .all(db)
                    .await?;

                if !year_periods.is_empty() {
                    let all_closed = year_periods.iter().all(|p| p.status == "closed");
                    if all_closed {
                        tasks.push(ProposedTask::new(
                            "year.start_closing",
                            &year.id,
                            "academic_year",
                            "All top-level periods are closed".to_string(),
                            Risk::High,
                        ));
                    }
                }
            }
        }

        // ── 4. Promotion suggestion ────────────────────────────────────
        for year in &years {
            if year.status == "closing" {
                tasks.push(ProposedTask::new(
                    "promotion.suggest",
                    &year.id,
                    "academic_year",
                    format!("Year '{}' is in closing state — consider promotion", year.name),
                    Risk::Low,
                ));
            }
        }

        debug!(
            org_id,
            proposed_count = tasks.len(),
            "Academic evaluation complete"
        );

        Ok(tasks)
    }
}

/// Parse a date string (YYYY-MM-DD) into NaiveDate, returning None on failure.
fn parse_date(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.fZ"))
        .ok()
}
