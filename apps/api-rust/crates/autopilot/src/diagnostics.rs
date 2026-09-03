//! Diagnostics â€” system health checks and problem detection.
//!
//! Runs as part of each tick to detect:
//!   â€¢ Tasks stuck in executing state
//!   â€¢ Tasks that have exceeded their deadline
//!   â€¢ Stale autopilot (no tick for too long)

use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter};
use serde::Serialize;
use tracing::warn;

use shared_kernel::errors::AppError;
use crate::entities::autopilot_task;

/// Health check result.
#[derive(Debug, Clone, Serialize)]
pub struct HealthReport {
    pub org_id: String,
    pub checked_at: String,
    /// Tasks stuck in "executing" state (possible zombie).
    pub stuck_tasks: Vec<String>,
    /// Tasks past their deadline.
    pub overdue_tasks: Vec<String>,
    /// Overall health status.
    pub status: HealthStatus,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
}

pub struct Diagnostics;

impl Diagnostics {
    /// Run a health check for the given org.
    pub async fn health_check(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<HealthReport, AppError> {
        let now = Utc::now();
        let now_str = now.to_rfc3339();

        // Find tasks stuck in "executing" (started >10 minutes ago)
        let executing_tasks = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.eq(autopilot_task::status::EXECUTING))
            .all(db)
            .await?;

        let stuck_tasks: Vec<String> = executing_tasks.iter()
            .filter(|t| {
                t.last_attempt_at.as_ref()
                    .and_then(|ts| chrono::DateTime::parse_from_rfc3339(ts).ok())
                    .map(|started| (now - started.with_timezone(&Utc)).num_minutes() > 10)
                    .unwrap_or(true) // No timestamp = definitely stuck
            })
            .map(|t| t.id.clone())
            .collect();

        // Find tasks past their deadline
        let pending_tasks = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.is_in([
                autopilot_task::status::PENDING,
                autopilot_task::status::READY,
                autopilot_task::status::REQUIRES_APPROVAL,
            ]))
            .all(db)
            .await?;

        let overdue_tasks: Vec<String> = pending_tasks.iter()
            .filter(|t| {
                t.deadline_at.as_ref()
                    .map(|d| d.as_str() < now_str.as_str())
                    .unwrap_or(false)
            })
            .map(|t| t.id.clone())
            .collect();

        // Determine health status
        let status = if !stuck_tasks.is_empty() {
            warn!(
                org_id,
                count = stuck_tasks.len(),
                "Stuck tasks detected (executing >10 min)"
            );
            HealthStatus::Critical
        } else if !overdue_tasks.is_empty() {
            warn!(
                org_id,
                count = overdue_tasks.len(),
                "Overdue tasks detected (past deadline)"
            );
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        };

        Ok(HealthReport {
            org_id: org_id.to_string(),
            checked_at: now_str,
            stuck_tasks,
            overdue_tasks,
            status,
        })
    }
}
