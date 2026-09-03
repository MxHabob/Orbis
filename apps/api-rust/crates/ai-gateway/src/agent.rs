//! Agent Session — AI agent lifecycle management.
//!
//! Each AI interaction happens within a session that tracks:
//!   - Who the AI is acting as (user_id + role)
//!   - What risk level is allowed
//!   - How many actions were executed/suggested
//!   - Session status (active → completed/suspended/revoked)

use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, ActiveModelTrait, Set};
use uuid::Uuid;
use std::sync::Arc;
use tracing::info;

use shared_kernel::errors::AppError;
use shared_kernel::types::RiskLevel;

use crate::entities::ai_agent_session::{self, session_status};

/// Request to create a new AI agent session.
pub struct CreateSessionRequest {
    pub org_id: String,
    pub agent_type: String,
    pub agent_version: Option<String>,
    pub acting_as_user_id: String,
    pub acting_as_role: String,
    pub conversation_id: Option<String>,
    pub max_risk_level: RiskLevel,
    pub requires_approval_above: RiskLevel,
}

pub struct AgentSessionManager;

impl AgentSessionManager {
    /// Create a new AI agent session.
    pub async fn create_session(
        db: &DatabaseConnection,
        req: CreateSessionRequest,
    ) -> Result<ai_agent_session::Model, AppError> {
        let now = Utc::now().to_rfc3339();

        let session = ai_agent_session::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            org_id: Set(req.org_id),
            agent_type: Set(req.agent_type),
            agent_version: Set(req.agent_version),
            acting_as_user_id: Set(req.acting_as_user_id),
            acting_as_role: Set(req.acting_as_role),
            conversation_id: Set(req.conversation_id),
            status: Set(session_status::ACTIVE.to_string()),
            max_risk_level: Set(req.max_risk_level.as_str().to_string()),
            requires_approval_above: Set(req.requires_approval_above.as_str().to_string()),
            actions_executed: Set(0),
            actions_suggested: Set(0),
            started_at: Set(now.clone()),
            last_active_at: Set(now.clone()),
            ended_at: Set(None),
            created_at: Set(now),
        };

        let created = session.insert(db).await?;

        info!(
            session_id = %created.id,
            agent_type = %created.agent_type,
            "AI agent session created"
        );

        Ok(created)
    }

    /// Get an active session by ID.
    pub async fn get_session(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> Result<ai_agent_session::Model, AppError> {
        ai_agent_session::Entity::find_by_id(session_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("ai_agent_session", session_id))
    }

    /// End a session (mark as completed).
    pub async fn end_session(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> Result<(), AppError> {
        let session = Self::get_session(db, session_id).await?;

        let mut active: ai_agent_session::ActiveModel = session.into();
        active.status = Set(session_status::COMPLETED.to_string());
        active.ended_at = Set(Some(Utc::now().to_rfc3339()));
        active.last_active_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;

        info!(session_id, "AI agent session completed");
        Ok(())
    }

    /// Revoke a session (force stop).
    pub async fn revoke_session(
        db: &DatabaseConnection,
        session_id: &str,
    ) -> Result<(), AppError> {
        let session = Self::get_session(db, session_id).await?;

        let mut active: ai_agent_session::ActiveModel = session.into();
        active.status = Set(session_status::REVOKED.to_string());
        active.ended_at = Set(Some(Utc::now().to_rfc3339()));
        active.last_active_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;

        info!(session_id, "AI agent session revoked");
        Ok(())
    }

    /// Increment action counters on a session.
    pub async fn record_action(
        db: &DatabaseConnection,
        session_id: &str,
        executed: bool,
    ) -> Result<(), AppError> {
        let session = Self::get_session(db, session_id).await?;

        let mut active: ai_agent_session::ActiveModel = session.clone().into();
        if executed {
            active.actions_executed = Set(session.actions_executed + 1);
        } else {
            active.actions_suggested = Set(session.actions_suggested + 1);
        }
        active.last_active_at = Set(Utc::now().to_rfc3339());
        active.update(db).await?;
        Ok(())
    }

    /// List active sessions for an org.
    pub async fn list_active_sessions(
        db: &DatabaseConnection,
        org_id: &str,
    ) -> Result<Vec<ai_agent_session::Model>, AppError> {
        Ok(ai_agent_session::Entity::find()
            .filter(ai_agent_session::Column::OrgId.eq(org_id))
            .filter(ai_agent_session::Column::Status.eq(session_status::ACTIVE))
            .all(db)
            .await?)
    }

    /// Query the agent (simple placeholder for Agent Runtime).
    pub async fn query(
        db: &DatabaseConnection,
        event_bus: Arc<shared_kernel::event_bus::EventBus>,
        org_id: &str,
        session_id: &str,
        text: &str,
        context: Option<serde_json::Value>,
    ) -> Result<String, AppError> {
        let session = Self::get_session(db, session_id).await?;
        
        info!(
            session_id,
            org_id,
            text_len = text.len(),
            "Processing AI query"
        );

        use crate::llm_gateway::LlmGateway;
        use infra_integration::adapters::gemini::GeminiAdapter;
        use crate::action_registry::ActionRegistry;
        use std::sync::Arc;

        let config = shared_kernel::config::Config::load();
        let adapter = Box::new(GeminiAdapter::new(config.gemini_api_key));
        let registry = Arc::new(ActionRegistry::build());
        let executor = crate::action_executor::ActionExecutor::new(registry.clone(), event_bus, db.clone());
        let gateway = LlmGateway::new(adapter, registry, executor);

        let response = gateway.dispatch_agent_query(
            &session,
            text,
            context
        ).await?;

        // Record that an action was "suggested" or "executed" based on the query.
        Self::record_action(db, session_id, false).await?;

        Ok(response)
    }

    // ── Logs & Action Approval ────────────────────────────────────────

    pub async fn get_session_logs(
        db: &DatabaseConnection,
        org_id: &str,
        session_id: &str,
    ) -> Result<Vec<crate::entities::ai_action_log::Model>, AppError> {
        use crate::entities::ai_action_log;
        use sea_orm::QueryOrder;
        
        Ok(ai_action_log::Entity::find()
            .filter(ai_action_log::Column::OrgId.eq(org_id))
            .filter(ai_action_log::Column::SessionId.eq(session_id))
            .order_by_desc(ai_action_log::Column::ExecutedAt)
            .all(db)
            .await?)
    }

    pub async fn approve_action(
        db: &DatabaseConnection,
        _org_id: &str,
        action_log_id: &str,
        approved_by: &str,
    ) -> Result<(), AppError> {
        use crate::entities::ai_action_log;
        
        let log = ai_action_log::Entity::find_by_id(action_log_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("ai_action_log", action_log_id))?;
            
        let mut active: ai_action_log::ActiveModel = log.into();
        active.status = Set(ai_action_log::action_status::APPROVED.to_string());
        active.approved_by = Set(Some(approved_by.to_string()));
        active.approved_at = Set(Some(Utc::now().to_rfc3339()));
        active.update(db).await?;
        
        Ok(())
    }

    pub async fn reject_action(
        db: &DatabaseConnection,
        _org_id: &str,
        action_log_id: &str,
        _rejected_by: &str,
        reason: &str,
    ) -> Result<(), AppError> {
        use crate::entities::ai_action_log;
        
        let log = ai_action_log::Entity::find_by_id(action_log_id.to_string())
            .one(db)
            .await?
            .ok_or_else(|| AppError::not_found("ai_action_log", action_log_id))?;
            
        let mut active: ai_action_log::ActiveModel = log.into();
        active.status = Set(ai_action_log::action_status::REJECTED.to_string());
        active.rejection_reason = Set(Some(reason.to_string()));
        active.update(db).await?;
        
        Ok(())
    }
}
