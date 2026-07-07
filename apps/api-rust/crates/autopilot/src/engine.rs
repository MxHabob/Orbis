//! Engine — main autopilot tick loop that orchestrates Evaluator → Planner → Executor.
//!
//! The engine runs on a configurable interval and:
//!   1. Loads config for the org
//!   2. Evaluates system state (Evaluator)
//!   3. Plans new tasks from calendar events (Planner)
//!   4. Executes ready/approved tasks (Executor)
//!   5. Runs diagnostics (Diagnostics)
//!   6. Records tick result
//!
//! The engine is designed to be called from a tokio::spawn loop or a Tauri-managed interval.

use std::sync::Arc;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use serde::Serialize;
use tracing::{info, error, warn};

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::{EventBus, EventEnvelope};

use crate::config::ResolvedConfig;
use crate::diagnostics::{Diagnostics, HealthReport, HealthStatus};
use crate::entities::autopilot_config;
use crate::evaluator::{Evaluator, SystemSnapshot};
use crate::executor::{Executor, ExecutionResult};
use crate::planner::{Planner, PlanResult};
use crate::resumption::{ResumptionService, CatchUpReport};

/// Result of a single tick.
#[derive(Debug, Clone, Serialize)]
pub struct TickResult {
    pub org_id: String,
    pub tick_at: String,
    pub snapshot_summary: SnapshotSummary,
    pub plan: PlanSummary,
    pub execution: ExecutionSummary,
    pub health: HealthReport,
    pub status: TickStatus,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnapshotSummary {
    pub active_events: usize,
    pub missed_events: usize,
    pub upcoming_events: usize,
    pub pending_tasks: usize,
    pub failed_tasks: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlanSummary {
    pub tasks_created: usize,
    pub already_planned: usize,
    pub missed_event_count: usize,
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

/// The main Autopilot Engine.
pub struct AutopilotEngine {
    event_bus: Arc<EventBus>,
}

impl AutopilotEngine {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self { event_bus }
    }

    /// Run catch-up on startup (non-blocking, fire-and-forget).
    pub async fn startup_catch_up(
        &self,
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<CatchUpReport, AppError> {
        let config = self.load_config(registry_db, org_id).await?;

        if !config.is_enabled {
            info!(org_id, "Autopilot disabled — skipping catch-up");
            return Ok(CatchUpReport {
                missed: vec![],
                activated: 0,
                already_planned: 0,
            });
        }

        ResumptionService::catch_up(registry_db, year_db, &config).await
    }

    /// Execute a single tick of the autopilot for a given org.
    ///
    /// This is the main entry point, called periodically from a timer or manually.
    pub async fn tick(
        &self,
        registry_db: &DatabaseConnection,
        year_db: &DatabaseConnection,
        org_id: &str,
    ) -> TickResult {
        let tick_at = Utc::now().to_rfc3339();
        info!(org_id, "Autopilot tick started");

        // ── 1. Load config ───────────────────────────────────────
        let config = match self.load_config(registry_db, org_id).await {
            Ok(c) => c,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        if !config.is_enabled {
            return TickResult {
                org_id: org_id.to_string(),
                tick_at,
                snapshot_summary: SnapshotSummary::empty(),
                plan: PlanSummary::empty(),
                execution: ExecutionSummary::empty(),
                health: HealthReport::healthy(org_id),
                status: TickStatus::Disabled,
                error: None,
            };
        }

        // Check if paused
        if let Ok(Some(cfg_model)) = autopilot_config::Entity::find()
            .filter(autopilot_config::Column::OrgId.eq(org_id))
            .one(registry_db)
            .await
        {
            if cfg_model.paused_at.is_some() {
                return TickResult {
                    org_id: org_id.to_string(),
                    tick_at,
                    snapshot_summary: SnapshotSummary::empty(),
                    plan: PlanSummary::empty(),
                    execution: ExecutionSummary::empty(),
                    health: HealthReport::healthy(org_id),
                    status: TickStatus::Paused,
                    error: None,
                };
            }
        }

        // ── 2. Evaluate + Plan + Execute + Diagnose (parallel where possible) ──
        let snapshot = match Evaluator::evaluate(registry_db, year_db, org_id).await {
            Ok(s) => s,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        let snapshot_summary = SnapshotSummary {
            active_events: snapshot.active_events.len(),
            missed_events: snapshot.missed_events.len(),
            upcoming_events: snapshot.upcoming_events.len(),
            pending_tasks: snapshot.pending_tasks.len(),
            failed_tasks: snapshot.failed_tasks.len(),
        };

        // Plan
        let plan_result = match Planner::plan(registry_db, &snapshot, &config).await {
            Ok(p) => p,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        let plan_summary = PlanSummary {
            tasks_created: plan_result.tasks_created.len(),
            already_planned: plan_result.already_planned,
            missed_event_count: plan_result.missed_event_count,
        };

        // Execute
        let executor = Executor::new(self.event_bus.clone());
        let exec_result = match executor.execute_ready_tasks(registry_db, &config).await {
            Ok(r) => r,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        let exec_summary = ExecutionSummary {
            executed: exec_result.executed,
            succeeded: exec_result.succeeded,
            failed: exec_result.failed,
            skipped: exec_result.skipped,
        };

        // Diagnostics
        let health = match Diagnostics::health_check(registry_db, org_id).await {
            Ok(h) => h,
            Err(e) => return Self::error_result(org_id, &tick_at, &e.to_string()),
        };

        // ── 3. Determine overall tick status ─────────────────────
        let status = if exec_result.failed > 0 {
            TickStatus::Warning
        } else if health.status == HealthStatus::Critical {
            TickStatus::Error
        } else if health.status == HealthStatus::Warning {
            TickStatus::Warning
        } else {
            TickStatus::Ok
        };

        // ── 4. Record tick result ────────────────────────────────
        let tick_result = TickResult {
            org_id: org_id.to_string(),
            tick_at: tick_at.clone(),
            snapshot_summary,
            plan: plan_summary,
            execution: exec_summary,
            health,
            status,
            error: None,
        };

        // Update last_tick in config (fire-and-forget)
        let result_json = serde_json::to_string(&tick_result).unwrap_or_default();
        let _ = Self::update_last_tick(registry_db, org_id, &tick_at, &result_json).await;

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
                "tasks_created": tick_result.plan.tasks_created,
                "executed": tick_result.execution.executed,
                "succeeded": tick_result.execution.succeeded,
                "failed": tick_result.execution.failed,
            }),
            chain_depth: 0,
        });

        info!(
            org_id,
            status = ?tick_result.status,
            tasks_created = tick_result.plan.tasks_created,
            executed = tick_result.execution.executed,
            succeeded = tick_result.execution.succeeded,
            failed = tick_result.execution.failed,
            "Autopilot tick completed"
        );

        tick_result
    }

    // ── Helpers ──────────────────────────────────────────────────

    async fn load_config(
        &self,
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<ResolvedConfig, AppError> {
        let config = autopilot_config::Entity::find()
            .filter(autopilot_config::Column::OrgId.eq(org_id))
            .one(db)
            .await?;

        Ok(match config {
            Some(model) => ResolvedConfig::from_model(&model),
            None => ResolvedConfig::default_for_org(org_id),
        })
    }

    async fn update_last_tick(
        db: &DatabaseConnection,
        org_id: &str,
        tick_at: &str,
        result: &str,
    ) -> Result<(), AppError> {
        if let Some(model) = autopilot_config::Entity::find()
            .filter(autopilot_config::Column::OrgId.eq(org_id))
            .one(db)
            .await?
        {
            let mut active: autopilot_config::ActiveModel = model.into();
            active.last_tick_at = Set(Some(tick_at.to_string()));
            active.last_tick_result = Set(Some(result.to_string()));
            active.updated_at = Set(Utc::now().to_rfc3339());
            active.update(db).await?;
        }
        Ok(())
    }

    fn error_result(org_id: &str, tick_at: &str, error: &str) -> TickResult {
        error!(org_id, error, "Autopilot tick failed");
        TickResult {
            org_id: org_id.to_string(),
            tick_at: tick_at.to_string(),
            snapshot_summary: SnapshotSummary::empty(),
            plan: PlanSummary::empty(),
            execution: ExecutionSummary::empty(),
            health: HealthReport::healthy(org_id),
            status: TickStatus::Error,
            error: Some(error.to_string()),
        }
    }
}

// ── Summary helpers ─────────────────────────────────────────────

impl SnapshotSummary {
    fn empty() -> Self {
        Self {
            active_events: 0,
            missed_events: 0,
            upcoming_events: 0,
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
            missed_event_count: 0,
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
