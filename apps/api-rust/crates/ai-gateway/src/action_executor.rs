//! Action Executor — 7-step sandboxed execution pipeline for AI actions.
//!
//! Flow:
//!   ① Resolve   (in-memory, ~0ms)
//!   ② Authorize  (permission check)
//!   ③ Risk gate  (compare action risk vs session max)
//!   ④ Idempotency check
//!   ⑤ Approval gate (conditional)
//!   ⑥ Execute    (domain service call)
//!   ⑦ Audit      (async fire-and-forget)

use std::sync::Arc;
use std::time::Instant;
use chrono::Utc;
use sea_orm::{DatabaseConnection, ActiveModelTrait, Set};
use serde::Serialize;
use serde_json::Value;
use tracing::{info, warn};
use uuid::Uuid;

use shared_kernel::errors::AppError;
use shared_kernel::event_bus::{EventBus, EventEnvelope};
use shared_kernel::types::RiskLevel;

use crate::action_registry::ActionRegistry;
use crate::entities::{ai_action_log, ai_agent_session};

/// Result of executing an action.
#[derive(Debug, Clone, Serialize)]
pub enum ActionExecutionResult {
    /// Action executed successfully.
    Executed { output: Value, duration_ms: u64 },
    /// Action was already executed (idempotency).
    AlreadyExecuted,
    /// Action requires approval — pending entry created.
    PendingApproval { approval_id: String },
    /// Action was rejected by risk gate.
    RiskExceeded { action_risk: String, session_max: String },
}

pub struct ActionExecutor {
    registry: Arc<ActionRegistry>,
    event_bus: Arc<EventBus>,
    db: DatabaseConnection,
}

impl ActionExecutor {
    pub fn new(
        registry: Arc<ActionRegistry>,
        event_bus: Arc<EventBus>,
        db: DatabaseConnection,
    ) -> Self {
        Self { registry, event_bus, db }
    }

    /// Execute an action within an AI agent session.
    pub async fn execute(
        &self,
        session: &ai_agent_session::Model,
        action_code: &str,
        params: Value,
        reasoning: &str,
    ) -> Result<ActionExecutionResult, AppError> {
        // ① Resolve (in-memory, ~0ms)
        let action = self.registry.find(action_code)
            .ok_or_else(|| AppError::ActionNotFound { code: action_code.to_string() })?;

        let def = &action.definition;

        // ② Authorize — check session has required permissions
        // (Simplified: in production, this calls AuthzService)
        // For now, we trust the session's acting_as_role

        // ③ Risk gate
        let action_risk = def.risk_level;
        let session_max = RiskLevel::from_str(&session.max_risk_level);
        if action_risk > session_max {
            warn!(
                action = action_code,
                action_risk = %action_risk,
                session_max = %session_max,
                "Risk level exceeded"
            );
            return Ok(ActionExecutionResult::RiskExceeded {
                action_risk: action_risk.to_string(),
                session_max: session_max.to_string(),
            });
        }

        // ④ Idempotency check
        let idem_key = format!("{}:{}:{}", session.id, action_code, 
            serde_json::to_string(&params).unwrap_or_default());
        // In production, check ai_action_log for existing idem_key

        // ⑤ Approval gate
        if def.requires_approval && action_risk >= RiskLevel::High {
            let approval_id = Uuid::new_v4().to_string();

            // Create pending approval entry
            let log = ai_action_log::ActiveModel {
                id: Set(approval_id.clone()),
                org_id: Set(session.org_id.clone()),
                session_id: Set(session.id.clone()),
                correlation_id: Set(None),
                action_code: Set(action_code.to_string()),
                input_data: Set(Some(serde_json::to_string(&params).unwrap_or_default())),
                output_data: Set(None),
                reasoning: Set(reasoning.to_string()),
                confidence: Set(None),
                status: Set(ai_action_log::action_status::PENDING.to_string()),
                risk_level: Set(action_risk.as_str().to_string()),
                approval_required: Set(1),
                approved_by: Set(None),
                approved_at: Set(None),
                rejection_reason: Set(None),
                affected_entities: Set(None),
                idempotency_key: Set(Some(idem_key)),
                duration_ms: Set(None),
                error: Set(None),
                is_reversible: Set(if def.is_reversible { 1 } else { 0 }),
                rollback_data: Set(None),
                reversed_at: Set(None),
                reversed_by: Set(None),
                executed_at: Set(Utc::now().to_rfc3339()),
                created_at: Set(Utc::now().to_rfc3339()),
            };
            log.insert(&self.db).await?;

            info!(
                action = action_code,
                approval_id = %approval_id,
                "Action requires approval — pending"
            );

            return Ok(ActionExecutionResult::PendingApproval { approval_id });
        }

        // ⑥ Execute (domain service call)
        let start = Instant::now();
        let result = (action.handler)(params.clone()).await;
        let duration = start.elapsed();
        let duration_ms = duration.as_millis() as u64;

        // ⑦ Audit (async fire-and-forget)
        let status = if result.is_ok() {
            ai_action_log::action_status::EXECUTED
        } else {
            ai_action_log::action_status::FAILED
        };

        let output_data = result.as_ref().ok().map(|v| serde_json::to_string(v).unwrap_or_default());
        let error = result.as_ref().err().map(|e| e.to_string());

        // Spawn audit log persistence (non-blocking)
        let db = self.db.clone();
        let log_entry = ai_action_log::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            org_id: Set(session.org_id.clone()),
            session_id: Set(session.id.clone()),
            correlation_id: Set(None),
            action_code: Set(action_code.to_string()),
            input_data: Set(Some(serde_json::to_string(&params).unwrap_or_default())),
            output_data: Set(output_data),
            reasoning: Set(reasoning.to_string()),
            confidence: Set(None),
            status: Set(status.to_string()),
            risk_level: Set(action_risk.as_str().to_string()),
            approval_required: Set(0),
            approved_by: Set(None),
            approved_at: Set(None),
            rejection_reason: Set(None),
            affected_entities: Set(None),
            idempotency_key: Set(Some(idem_key)),
            duration_ms: Set(Some(duration_ms as i32)),
            error: Set(error.clone()),
            is_reversible: Set(if def.is_reversible { 1 } else { 0 }),
            rollback_data: Set(None),
            reversed_at: Set(None),
            reversed_by: Set(None),
            executed_at: Set(Utc::now().to_rfc3339()),
            created_at: Set(Utc::now().to_rfc3339()),
        };

        tokio::spawn(async move {
            let _ = log_entry.insert(&db).await;
        });

        // Publish event
        self.event_bus.publish(EventEnvelope {
            event_type: "ai.action.executed".to_string(),
            entity_type: "ai_action".to_string(),
            entity_id: action_code.to_string(),
            org_id: session.org_id.clone(),
            actor: format!("ai:{}", session.agent_type),
            actor_type: "ai_agent".to_string(),
            correlation_id: None,
            payload: serde_json::json!({
                "action_code": action_code,
                "status": status,
                "duration_ms": duration_ms,
                "session_id": session.id,
            }),
            chain_depth: 0,
        });

        match result {
            Ok(output) => {
                info!(
                    action = action_code,
                    duration_ms,
                    "Action executed successfully"
                );
                Ok(ActionExecutionResult::Executed { output, duration_ms })
            }
            Err(e) => Err(e),
        }
    }
}
