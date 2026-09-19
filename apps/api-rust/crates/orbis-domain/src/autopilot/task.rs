use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Executing,
    Completed,
    Failed,
    Suggested,
    Skipped,
    Cancelled,
}

impl TaskStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "pending",
            TaskStatus::Executing => "executing",
            TaskStatus::Completed => "completed",
            TaskStatus::Failed => "failed",
            TaskStatus::Suggested => "suggested",
            TaskStatus::Skipped => "skipped",
            TaskStatus::Cancelled => "cancelled",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pending" => TaskStatus::Pending,
            "executing" => TaskStatus::Executing,
            "completed" => TaskStatus::Completed,
            "failed" => TaskStatus::Failed,
            "suggested" => TaskStatus::Suggested,
            "skipped" => TaskStatus::Skipped,
            "cancelled" => TaskStatus::Cancelled,
            _ => TaskStatus::Pending,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl RiskLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskLevel::Low => "low",
            RiskLevel::Medium => "medium",
            RiskLevel::High => "high",
            RiskLevel::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "low" => RiskLevel::Low,
            "medium" => RiskLevel::Medium,
            "high" => RiskLevel::High,
            "critical" => RiskLevel::Critical,
            _ => RiskLevel::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    EnrollmentOpen,
    EnrollmentClose,
    GradeEntryOpen,
    GradeEntryClose,
    PeriodActivate,
    PeriodLock,
    YearActivate,
    YearClose,
    PromotionDryRun,
    LateFeeReminder,
    GenerateReport,
    BackupDatabase,
    GenericWorkflow,
    SendDelayNotification,
    TransitionPeriod,
}

impl TaskType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TaskType::EnrollmentOpen => "enrollment_open",
            TaskType::EnrollmentClose => "enrollment_close",
            TaskType::GradeEntryOpen => "grade_entry_open",
            TaskType::GradeEntryClose => "grade_entry_close",
            TaskType::PeriodActivate => "period_activate",
            TaskType::PeriodLock => "period_lock",
            TaskType::YearActivate => "year_activate",
            TaskType::YearClose => "year_close",
            TaskType::PromotionDryRun => "promotion_dry_run",
            TaskType::LateFeeReminder => "late_fee_reminder",
            TaskType::GenerateReport => "generate_report",
            TaskType::BackupDatabase => "backup_database",
            TaskType::GenericWorkflow => "generic_workflow",
            TaskType::SendDelayNotification => "send_delay_notification",
            TaskType::TransitionPeriod => "transition_period",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().replace('.', "_").as_str() {
            "enrollment_open" | "enrollment.open" => TaskType::EnrollmentOpen,
            "enrollment_close" | "enrollment.close" => TaskType::EnrollmentClose,
            "grade_entry_open" | "grade_entry.open" => TaskType::GradeEntryOpen,
            "grade_entry_close" | "grade_entry.close" => TaskType::GradeEntryClose,
            "period_activate" | "period.activate" => TaskType::PeriodActivate,
            "period_lock" | "period.lock" => TaskType::PeriodLock,
            "year_activate" | "year.activate" => TaskType::YearActivate,
            "year_close" | "year.close" | "year.start_closing" => TaskType::YearClose,
            "promotion_dry_run" | "promotion.dry_run" => TaskType::PromotionDryRun,
            "late_fee_reminder" | "fee.overdue_notification" | "send_delay_notification" => TaskType::LateFeeReminder,
            "generate_report" | "report_generation" => TaskType::GenerateReport,
            "backup_database" | "backup" => TaskType::BackupDatabase,
            "transition_period" => TaskType::TransitionPeriod,
            _ => TaskType::GenericWorkflow,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutopilotTask {
    pub id: Uuid,
    pub org_id: Uuid,
    pub task_type: TaskType,
    pub status: TaskStatus,
    pub target_id: Option<Uuid>,
    pub target_type: Option<String>,
    pub risk_level: RiskLevel,
    pub reason: Option<String>,
    pub parameters: Option<Value>,
    pub scheduled_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub attempts: i32,
    pub max_retries: i32,
    pub error_log: Option<String>,
    pub idempotency_key: Option<String>,
    pub triggered_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AutopilotTask {
    pub fn new(
        org_id: Uuid,
        task_type: TaskType,
        target_id: Option<Uuid>,
        target_type: Option<String>,
        risk_level: RiskLevel,
        reason: Option<String>,
        scheduled_at: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            org_id,
            task_type,
            status: TaskStatus::Pending,
            target_id,
            target_type,
            risk_level,
            reason,
            parameters: None,
            scheduled_at,
            executed_at: None,
            attempts: 0,
            max_retries: 3,
            error_log: None,
            idempotency_key: None,
            triggered_by: "policy".to_string(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_type_conversions() {
        assert_eq!(TaskType::from_str("enrollment_open"), TaskType::EnrollmentOpen);
        assert_eq!(TaskType::from_str("enrollment.open"), TaskType::EnrollmentOpen);
        assert_eq!(TaskType::from_str("period.lock"), TaskType::PeriodLock);
        assert_eq!(TaskType::from_str("promotion_dry_run"), TaskType::PromotionDryRun);
        assert_eq!(TaskType::EnrollmentOpen.as_str(), "enrollment_open");
        assert_eq!(TaskType::PeriodLock.as_str(), "period_lock");
    }

    #[test]
    fn test_task_status_conversions() {
        assert_eq!(TaskStatus::Pending.as_str(), "pending");
        assert_eq!(TaskStatus::Executing.as_str(), "executing");
        assert_eq!(TaskStatus::Completed.as_str(), "completed");
        assert_eq!(TaskStatus::Failed.as_str(), "failed");
        assert_eq!(TaskStatus::Suggested.as_str(), "suggested");
        assert_eq!(TaskStatus::Cancelled.as_str(), "cancelled");
    }

    #[test]
    fn test_autopilot_task_creation() {
        let org_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let now = Utc::now();
        let task = AutopilotTask::new(
            org_id,
            TaskType::PeriodLock,
            Some(target_id),
            Some("academic_period".to_string()),
            RiskLevel::High,
            Some("Locking period after grade closing window".to_string()),
            now,
        );

        assert_eq!(task.org_id, org_id);
        assert_eq!(task.task_type, TaskType::PeriodLock);
        assert_eq!(task.status, TaskStatus::Pending);
        assert_eq!(task.target_id, Some(target_id));
        assert_eq!(task.risk_level, RiskLevel::High);
        assert_eq!(task.attempts, 0);
    }
}
