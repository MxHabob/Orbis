use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use tracing::debug;
use uuid::Uuid;

use orbis_domain::errors::DomainError;
use crate::models::{
    academic_year::{self, Entity as YearEntity},
    academic_period::{self, Entity as PeriodEntity},
    enrollment::{self, Entity as EnrollmentEntity},
    student_fee::{self, Entity as StudentFeeEntity},
};

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

#[derive(Debug, Clone)]
pub struct ProposedTask {
    pub task_type: String,
    pub target_id: Uuid,
    pub target_type: String,
    pub reason: String,
    pub risk: Risk,
    pub params: Option<serde_json::Value>,
}

impl ProposedTask {
    pub fn new(
        task_type: &str,
        target_id: Uuid,
        target_type: &str,
        reason: String,
        risk: Risk,
    ) -> Self {
        Self {
            task_type: task_type.to_string(),
            target_id,
            target_type: target_type.to_string(),
            reason,
            risk,
            params: None,
        }
    }

    pub fn new_with_params(
        task_type: &str,
        target_id: Uuid,
        target_type: &str,
        reason: String,
        risk: Risk,
        params: Option<serde_json::Value>,
    ) -> Self {
        Self {
            task_type: task_type.to_string(),
            target_id,
            target_type: target_type.to_string(),
            reason,
            risk,
            params,
        }
    }
}

pub struct AcademicEvaluator;

impl AcademicEvaluator {
    /// Evaluates the complete academic state for an organization and produces proposed autopilot tasks.
    pub async fn evaluate(
        db: &DatabaseConnection,
        org_id: Uuid,
    ) -> Result<Vec<ProposedTask>, DomainError> {
        let now = Utc::now();
        let mut tasks = Vec::new();

        // ── 1. Academic Year Lifecycle ──────────────────────────────────
        let years = YearEntity::find()
            .filter(academic_year::Column::OrgId.eq(org_id))
            .filter(academic_year::Column::Status.ne("archived"))
            .all(db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        for year in &years {
            if year.status == "planning" && year.start_date <= now {
                tasks.push(ProposedTask::new(
                    "year.activate",
                    year.id,
                    "academic_year",
                    format!("Academic year '{}' start date has arrived", year.name),
                    Risk::Medium,
                ));
            }

            if year.status == "active" {
                // Check if all periods under this year are closed
                let periods = PeriodEntity::find()
                    .filter(academic_period::Column::YearId.eq(year.id))
                    .all(db)
                    .await
                    .map_err(|e| DomainError::Internal(e.to_string()))?;

                if !periods.is_empty() && periods.iter().all(|p| p.status == "closed") {
                    tasks.push(ProposedTask::new(
                        "year.start_closing",
                        year.id,
                        "academic_year",
                        format!("All periods in year '{}' are closed. Ready for year closing", year.name),
                        Risk::High,
                    ));
                }
            }

            if year.status == "closing" {
                tasks.push(ProposedTask::new(
                    "promotion.dry_run",
                    year.id,
                    "academic_year",
                    format!("Year '{}' is closing — initiate progression dry-run", year.name),
                    Risk::Medium,
                ));
            }
        }

        // ── 2. Academic Period Lifecycle Gates ─────────────────────────
        let periods = PeriodEntity::find()
            .filter(academic_period::Column::OrgId.eq(org_id))
            .filter(academic_period::Column::Status.ne("archived"))
            .all(db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        for period in &periods {
            // Period Activation
            if (period.status == "planning" || period.status == "planned") && period.start_date <= now {
                tasks.push(ProposedTask::new(
                    "period.activate",
                    period.id,
                    "academic_period",
                    format!("Period '{}' start date has arrived", period.name),
                    Risk::Medium,
                ));
            }

            // Enrollment Window
            if let Some(enroll_start) = period.enrollment_starts_at {
                if !period.is_enrollment_open && enroll_start <= now && period.status == "active_teaching" {
                    tasks.push(ProposedTask::new(
                        "enrollment.open",
                        period.id,
                        "academic_period",
                        format!("Enrollment start reached for period '{}'", period.name),
                        Risk::Low,
                    ));
                }
            }
            if let Some(enroll_end) = period.enrollment_ends_at {
                if period.is_enrollment_open && enroll_end < now {
                    tasks.push(ProposedTask::new(
                        "enrollment.close",
                        period.id,
                        "academic_period",
                        format!("Enrollment deadline exceeded for period '{}'", period.name),
                        Risk::Medium,
                    ));
                }
            }

            // Grade Entry Window
            if let Some(grade_start) = period.grade_entry_starts_at {
                if !period.is_grade_entry_open && grade_start <= now && (period.status == "active_teaching" || period.status == "grade_submission") {
                    tasks.push(ProposedTask::new(
                        "grade_entry.open",
                        period.id,
                        "academic_period",
                        format!("Grade entry window started for period '{}'", period.name),
                        Risk::Low,
                    ));
                }
            }
            if let Some(grade_end) = period.grade_entry_ends_at {
                if period.is_grade_entry_open && grade_end < now {
                    tasks.push(ProposedTask::new(
                        "grade_entry.close",
                        period.id,
                        "academic_period",
                        format!("Grade entry deadline exceeded for period '{}'", period.name),
                        Risk::Medium,
                    ));
                }
            }

            // Auto-lock after end date
            if !period.is_locked && period.end_date < now {
                tasks.push(ProposedTask::new(
                    "period.lock",
                    period.id,
                    "academic_period",
                    format!("Period '{}' end date passed — auto locking grades and enrollments", period.name),
                    Risk::High,
                ));
            }
        }

        // ── 3. Student Study Mode & Compliance Evaluations ─────────────
        let enrollments = EnrollmentEntity::find()
            .filter(enrollment::Column::OrgId.eq(org_id))
            .filter(enrollment::Column::Status.eq("active"))
            .all(db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        for enr in &enrollments {
            let study_mode_str = enr.study_mode.as_deref().unwrap_or("regular");
            let is_exempt_attendance = enr.is_attendance_exempt.unwrap_or(study_mode_str == "homeschooled");
            let is_exempt_grade = enr.is_grade_exempt.unwrap_or(study_mode_str == "auditor");

            // Verify homeschool exemption: Homeschooled students must not trigger attendance breach warnings
            if is_exempt_attendance {
                // Homeschooled / attendance exempt verified
                continue;
            }

            // Auditor students: exempt from GPA calculations
            if is_exempt_grade {
                continue;
            }
        }

        // ── 4. Overdue Financial Fees ─────────────────────────────────
        let overdue_fees = StudentFeeEntity::find()
            .filter(student_fee::Column::OrgId.eq(org_id))
            .filter(student_fee::Column::Status.eq("pending"))
            .filter(student_fee::Column::DueDate.lt(now))
            .all(db)
            .await
            .map_err(|e| DomainError::Internal(e.to_string()))?;

        for fee in overdue_fees {
            tasks.push(ProposedTask::new(
                "fee.overdue_notification",
                fee.id,
                "student_fee",
                format!("Fee {} is overdue since {:?}", fee.id, fee.due_date),
                Risk::Low,
            ));
        }

        debug!(
            org_id = %org_id,
            proposed_tasks_count = tasks.len(),
            "Autopilot academic evaluation finished"
        );

        Ok(tasks)
    }
}
