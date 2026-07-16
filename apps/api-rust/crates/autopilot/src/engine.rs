//! Engine ط£آ¢أ¢â€ڑآ¬أ¢â‚¬â€Œ main autopilot tick loop (Unified Single-DB Architecture).
//!
//! The engine runs on a configurable interval and:
//!   1. Loads policy from `academic_policies`
//!   2. Evaluates academic state via AcademicEvaluator (reads periods directly)
//!   3. Plans new tasks from ProposedTask list
//!   4. Executes ready/approved tasks via AcademicExecutor (real DB mutations)
//!   5. Runs diagnostics
//!   6. Records tick result
//!
//! Key change from v1: Single `db` parameter replaces `registry_db` + `year_db`.

use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set, IntoActiveModel};
use serde::Serialize;
use tracing::{info, error, warn};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::{EventBus, EventEnvelope};
use shared_kernel::types::ExecutionMode;
use shared_kernel::period_gate::PeriodGate;

use crate::academic_evaluator::{AcademicEvaluator, ProposedTask, Risk};
use crate::academic_executor::AcademicExecutor;


use crate::diagnostics::{Diagnostics, HealthReport, HealthStatus};
use crate::entities::autopilot_task;
use platform_policy::entities::academic_policy;

/// Result of a single tick.
#[derive(Debug, Clone, Serialize)]
pub struct TickResult {
    pub org_id: String,
    pub tick_at: String,
    pub evaluation: EvaluationSummary,
    pub plan: PlanSummary,
    pub execution: ExecutionSummary,
    pub health: HealthReport,
    pub status: TickStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EvaluationSummary {
    pub proposed_tasks: usize,
    pub pending_tasks: usize,
    pub failed_tasks: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanSummary {
    pub tasks_created: usize,
    pub already_planned: usize,
    pub skipped_manual: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionSummary {
    pub executed: usize,
    pub succeeded: usize,
    pub failed: usize,
    pub skipped: usize,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TickStatus {
    Ok,
    Warning,
    Error,
    Paused,
    Disabled,
}

/// The main Autopilot Engine (Unified Architecture).
pub struct AutopilotEngine {
    event_bus: Arc<EventBus>,
    period_gate: Option<Arc<PeriodGate>>,
    executors: Vec<Arc<dyn crate::executor::AutopilotExecutor>>,
}

impl AutopilotEngine {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { 
            event_bus, 
            period_gate: None,
            executors: Vec::new(),
        }
    }

    pub fn with_period_gate(mut self, gate: Arc<PeriodGate>) -> Self {
        self.period_gate = Some(gate);
        self
    }

    pub fn with_executor(mut self, executor: Arc<dyn crate::executor::AutopilotExecutor>) -> Self {
        self.executors.push(executor);
        self
    }

    /// Execute a single tick of the autopilot for a given org.
    ///
    /// Single `db` parameter ط£آ¢أ¢â€ڑآ¬أ¢â‚¬â€Œ no more registry_db + year_db split.
    /// `org_unit_id` ط£آ¢أ¢â€ڑآ¬أ¢â‚¬â€Œ optional branch context for policy resolution.
    pub async fn tick(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        org_unit_id: Option<&str>,
    ) -> TickResult {
        let tick_at = Utc::now().to_rfc3339();
        info!(org_id, ?org_unit_id, "Autopilot tick started");

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 1. Load effective policy (branch override ط£آ¢أ¢â‚¬آ أ¢â‚¬â„¢ org default) ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let policy = match Self::load_effective_policy(db, org_id, org_unit_id).await {
            Ok(p) => p,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        if policy.autopilot_enabled == 0 {
            return TickResult {
                org_id: org_id.to_string(),
                tick_at,
                evaluation: EvaluationSummary::empty(),
                plan: PlanSummary::empty(),
                execution: ExecutionSummary::empty(),
                health: HealthReport::healthy(org_id),
                status: TickStatus::Disabled,
                error: None,
            };
        }

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 2. Evaluate academic state ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let proposed_tasks = match AcademicEvaluator::evaluate(db, org_id, &policy).await {
            Ok(tasks) => tasks,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        // Count existing pending/failed tasks
        let (pending_count, failed_count) = Self::count_task_statuses(db, org_id).await;

        let eval_summary = EvaluationSummary {
            proposed_tasks: proposed_tasks.len(),
            pending_tasks: pending_count,
            failed_tasks: failed_count,
        };

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 3. Plan ط£آ¢أ¢â€ڑآ¬أ¢â‚¬â€Œ create autopilot_tasks from proposals ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let plan_result = self.plan_tasks(db, org_id, &proposed_tasks, &policy).await;
        let plan_summary = PlanSummary {
            tasks_created: plan_result.0,
            already_planned: plan_result.1,
            skipped_manual: plan_result.2,
        };

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 4. Execute ready/approved tasks ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let exec_result = self.execute_ready_tasks(db, org_id, &policy).await;
        let exec_summary = ExecutionSummary {
            executed: exec_result.0,
            succeeded: exec_result.1,
            failed: exec_result.2,
            skipped: exec_result.3,
        };

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 5. Diagnostics ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let health = match Diagnostics::health_check(db, org_id).await {
            Ok(h) => h,
            Err(_) => HealthReport::healthy(org_id),
        };

        // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ 6. Determine status ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬
        let status = if exec_result.2 > 0 {
            TickStatus::Warning
        } else if health.status == HealthStatus::Critical {
            TickStatus::Error
        } else if health.status == HealthStatus::Warning {
            TickStatus::Warning
        } else {
            TickStatus::Ok
        };

        let tick_result = TickResult {
            org_id: org_id.to_string(),
            tick_at: tick_at.clone(),
            evaluation: eval_summary,
            plan: plan_summary,
            execution: exec_summary,
            health,
            status,
            error: None,
        };

        // Publish tick event
        self.event_bus.publish(EventEnvelope {
            event_type: "autopilot.tick.completed".to_string(),
            entity_type: "autopilot".to_string(),
            entity_id: org_id.to_string(),
            org_id: org_id.to_string(),
            actor: "autopilot".to_string(),
            actor_type: "system".to_string(),
            correlation_id: None,
            payload: serde_json::json!({
                "status": tick_result.status,
                "proposed": tick_result.evaluation.proposed_tasks,
                "created": tick_result.plan.tasks_created,
                "executed": tick_result.execution.executed,
                "succeeded": tick_result.execution.succeeded,
                "failed": tick_result.execution.failed,
            }),
            chain_depth: 0,
        });

        info!(
            org_id,
            status = ?tick_result.status,
            proposed = tick_result.evaluation.proposed_tasks,
            created = tick_result.plan.tasks_created,
            executed = tick_result.execution.executed,
            "Autopilot tick completed"
        );

        tick_result
    }

    // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ Planning ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬

    /// Create autopilot_task rows from ProposedTask list.
    /// Returns (created, already_planned, skipped_manual).
    async fn plan_tasks(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        proposals: &[ProposedTask],
        policy: &academic_policy::Model,
    ) -> (usize, usize, usize) {
        let mut created = 0usize;
        let mut already_planned = 0usize;
        let mut skipped_manual = 0usize;

        let mode = match policy.autopilot_mode.as_str() {
            "automatic" => ExecutionMode::Automatic,
            "suggested" => ExecutionMode::Suggested,
            _ => ExecutionMode::Manual,
        };

        for proposal in proposals {
            // Skip if manual mode
            if mode == ExecutionMode::Manual {
                skipped_manual += 1;
                continue;
            }

            // Check if task already exists for this target + type (today)
            let today = Utc::now().date_naive().to_string();
            let idempotency_key = format!("{}:{}:{}", proposal.task_type, proposal.target_id, today);

            let existing = autopilot_task::Entity::find()
                .filter(autopilot_task::Column::IdempotencyKey.eq(&idempotency_key))
                .one(db)
                .await;

            if let Ok(Some(_)) = existing {
                already_planned += 1;
                continue;
            }

            // Determine initial status based on mode + risk
            let initial_status = match (&mode, &proposal.risk) {
                (ExecutionMode::Automatic, Risk::Critical) => autopilot_task::status::REQUIRES_APPROVAL,
                (ExecutionMode::Automatic, Risk::High) => autopilot_task::status::REQUIRES_APPROVAL,
                (ExecutionMode::Automatic, _) => autopilot_task::status::READY,
                (ExecutionMode::Suggested, _) => autopilot_task::status::REQUIRES_APPROVAL,
                (ExecutionMode::Manual, _) => autopilot_task::status::PENDING,
            };

            let now = Utc::now().to_rfc3339();
            let task = autopilot_task::ActiveModel {
                id: Set(Uuid::new_v4().to_string()),
                org_id: Set(org_id.to_string()),
                calendar_event_id: Set(None),
                task_type: Set(proposal.task_type.clone()),
                scheduled_at: Set(now.clone()),
                deadline_at: Set(None),
                target_entity_type: Set(Some(proposal.target_type.clone())),
                target_entity_id: Set(Some(proposal.target_id.clone())),
                params: Set(proposal.params.clone()),
                prerequisites: Set(None),
                status: Set(initial_status.to_string()),
                execution_mode: Set(mode.as_str().to_string()),
                attempts: Set(0),
                last_attempt_at: Set(None),
                last_error: Set(None),
                suggested_at: Set(if mode == ExecutionMode::Suggested {
                    Some(now.clone())
                } else {
                    None
                }),
                approved_by: Set(None),
                approved_at: Set(None),
                rejection_reason: Set(None),
                completed_at: Set(None),
                result: Set(None),
                idempotency_key: Set(idempotency_key),
                created_at: Set(now.clone()),
                updated_at: Set(now),
            };

            match task.insert(db).await {
                Ok(inserted) => {
                    info!(
                        task_id = %inserted.id,
                        task_type = %proposal.task_type,
                        status = initial_status,
                        "Autopilot task created"
                    );
                    created += 1;
                }
                Err(e) => {
                    warn!(
                        task_type = %proposal.task_type,
                        error = %e,
                        "Failed to create autopilot task"
                    );
                }
            }
        }

        (created, already_planned, skipped_manual)
    }

    // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ Execution ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬

    /// Execute all tasks that are READY or APPROVED.
    /// Returns (executed, succeeded, failed, skipped).
    async fn execute_ready_tasks(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
        _policy: &academic_policy::Model,
    ) -> (usize, usize, usize, usize) {
        let tasks = match autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.is_in([
                autopilot_task::status::READY,
                autopilot_task::status::APPROVED,
            ]))
            .all(db)
            .await
        {
            Ok(t) => t,
            Err(e) => {
                error!(error = %e, "Failed to load ready tasks");
                return (0, 0, 0, 0);
            }
        };

        let mut executed = 0usize;
        let mut succeeded = 0usize;
        let mut failed = 0usize;
        let mut skipped = 0usize;

        for task in tasks {
            if task.attempts >= 3 {
                skipped += 1;
                let _ = Self::update_task_status(db, &task.id, autopilot_task::status::FAILED, None).await;
                continue;
            }

            executed += 1;
            let start = Instant::now();

            let mut exec_result = Err(AppError::internal("No executor found for task type"));
            
            for executor in &self.executors {
                if executor.can_handle(&task.task_type) {
                    exec_result = executor.execute(db, &task, &self.event_bus, self.period_gate.as_deref()).await;
                    break;
                }
            }

            // Fallback for academic tasks (legacy hardcoding fallback)
            if exec_result.is_err() && task.task_type.starts_with("academic.") {
                exec_result = AcademicExecutor::execute(db, &task, &self.event_bus, self.period_gate.as_ref()).await;
            }

            match exec_result {
                Ok(output) => {
                    succeeded += 1;

                    if output == "REQUIRES_APPROVAL" {
                        // Update task as requires_approval
                        let mut active = task.clone().into_active_model();
                        active.status = Set(autopilot_task::status::REQUIRES_APPROVAL.to_string());
                        active.updated_at = Set(Utc::now().to_rfc3339());
                        let _ = active.update(db).await;
                    } else {
                        // Update task as completed
                        let _ = Self::complete_task(db, &task.id, &output).await;
                    }

                    // Log execution
                    let _ = Self::log_execution(
                        db, org_id, &task.id, &task.task_type,
                        "completed", Some(&output), None,
                        start.elapsed().as_millis() as i32,
                    ).await;

                    info!(
                        task_id = %task.id,
                        task_type = %task.task_type,
                        duration_ms = start.elapsed().as_millis() as u64,
                        "Task completed"
                    );
                }
                Err(err) => {
                    failed += 1;
                    let err_msg = err.to_string();

                    let _ = Self::fail_task(db, &task.id, &err_msg, task.attempts + 1).await;
                    let _ = Self::log_execution(
                        db, org_id, &task.id, &task.task_type,
                        "failed", None, Some(&err_msg),
                        start.elapsed().as_millis() as i32,
                    ).await;

                    error!(
                        task_id = %task.id,
                        error = %err_msg,
                        "Task failed"
                    );
                }
            }
        }

        (executed, succeeded, failed, skipped)
    }

    // ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ Helpers ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬

    /// Load the effective policy for an org + optional org_unit.
    ///
    /// 1. If `org_unit_id` is provided, try to find a branch-specific override.
    /// 2. If no branch override, fall back to the org-wide default (org_unit_id IS NULL).
    async fn load_effective_policy(
        db: &DatabaseConnection,
        org_id: &str,
        org_unit_id: Option<&str>,
    ) -> Result<academic_policy::Model, AppError> {
        // Try branch-specific override first
        if let Some(unit_id) = org_unit_id {
            let branch_policy = academic_policy::Entity::find()
                .filter(academic_policy::Column::OrgId.eq(org_id))
                .filter(academic_policy::Column::OrgUnitId.eq(unit_id))
                .one(db)
                .await?;
            if let Some(p) = branch_policy {
                return Ok(p);
            }
        }

        // Fall back to org-wide default
        academic_policy::Entity::find()
            .filter(academic_policy::Column::OrgId.eq(org_id))
            .filter(academic_policy::Column::OrgUnitId.is_null())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("academic_policies", org_id))
    }

    async fn count_task_statuses(db: &DatabaseConnection, org_id: &str) -> (usize, usize) {
        let all = autopilot_task::Entity::find()
            .filter(autopilot_task::Column::OrgId.eq(org_id))
            .filter(autopilot_task::Column::Status.is_in([
                autopilot_task::status::PENDING,
                autopilot_task::status::READY,
                autopilot_task::status::REQUIRES_APPROVAL,
                autopilot_task::status::FAILED,
            ]))
            .all(db)
            .await
            .unwrap_or_default();

        let pending = all.iter().filter(|t| {
            t.status == autopilot_task::status::PENDING
                || t.status == autopilot_task::status::READY
                || t.status == autopilot_task::status::REQUIRES_APPROVAL
        }).count();
        let failed = all.iter().filter(|t| t.status == autopilot_task::status::FAILED).count();
        (pending, failed)
    }

    async fn update_task_status(
        db: &DatabaseConnection,
        task_id: &str,
        status: &str,
        error: Option<&str>,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active = task.into_active_model();
        active.status = Set(status.to_string());
        active.last_error = Set(error.map(String::from));
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn complete_task(
        db: &DatabaseConnection,
        task_id: &str,
        result: &str,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active = task.into_active_model();
        active.status = Set(autopilot_task::status::COMPLETED.to_string());
        active.completed_at = Set(Some(Utc::now().to_rfc3339()));
        active.result = Set(Some(result.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn fail_task(
        db: &DatabaseConnection,
        task_id: &str,
        error: &str,
        attempts: i32,
    ) -> Result<(), AppError> {
        let task = autopilot_task::Entity::find_by_id(task_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("autopilot_task", task_id))?;

        let mut active = task.into_active_model();
        active.status = Set(if attempts >= 3 {
            autopilot_task::status::FAILED.to_string()
        } else {
            autopilot_task::status::PENDING.to_string()
        });
        active.attempts = Set(attempts);
        active.last_attempt_at = Set(Some(Utc::now().to_rfc3339()));
        active.last_error = Set(Some(error.to_string()));
        active.updated_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    async fn log_execution(
        db: &DatabaseConnection,
        org_id: &str,
        task_id: &str,
        action: &str,
        status: &str,
        details: Option<&str>,
        error: Option<&str>,
        duration_ms: i32,
    ) -> Result<(), AppError> {
        use sea_orm::{Statement, ConnectionTrait, DbBackend};
        let now_str = Utc::now().to_rfc3339();
        let id = Uuid::new_v4().to_string();
        let session_id = format!("autopilot_sys_{}", org_id);

        let sql = r#"
            INSERT INTO ai_action_log
                (id, org_id, session_id, correlation_id, action_code, output_data, reasoning, confidence, status, risk_level, approval_required, duration_ms, error, is_reversible, executed_at, created_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
        "#;

        db.execute(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            sql,
            vec![
                id.into(),
                org_id.to_string().into(),
                session_id.into(),
                task_id.to_string().into(),
                action.to_string().into(),
                details.map(String::from).into(),
                "Autopilot Engine Execution".to_string().into(),
                1.0.into(), // confidence
                status.to_string().into(),
                "low".to_string().into(), // risk_level
                0.into(), // approval_required
                duration_ms.into(),
                error.map(String::from).into(),
                0.into(), // is_reversible
                now_str.clone().into(),
                now_str.into(),
            ],
        )).await?;

        Ok(())
    }

    fn error_result(org_id: &str, tick_at: &str, error: &str) -> TickResult {
        error!(org_id, error, "Autopilot tick failed");
        TickResult {
            org_id: org_id.to_string(),
            tick_at: tick_at.to_string(),
            evaluation: EvaluationSummary::empty(),
            plan: PlanSummary::empty(),
            execution: ExecutionSummary::empty(),
            health: HealthReport::healthy(org_id),
            status: TickStatus::Error,
            error: Some(error.to_string()),
        }
    }
}

// ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ Summary helpers ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬ط£آ¢أ¢â‚¬â€Œأ¢â€ڑآ¬

impl EvaluationSummary {
    fn empty() -> Self {
        Self {
            proposed_tasks: 0,
            pending_tasks: 0,
            failed_tasks: 0,
        }
    }
}

impl PlanSummary {
    fn empty() -> Self {
        Self {
            tasks_created: 0,
            already_planned: 0,
            skipped_manual: 0,
        }
    }
}

impl ExecutionSummary {
    fn empty() -> Self {
        Self {
            executed: 0,
            succeeded: 0,
            failed: 0,
            skipped: 0,
        }
    }
}

impl HealthReport {
    fn healthy(org_id: &str) -> Self {
        Self {
            org_id: org_id.to_string(),
            checked_at: Utc::now().to_rfc3339(),
            stuck_tasks: vec![],
            overdue_tasks: vec![],
            status: HealthStatus::Healthy,
        }
    }
}


