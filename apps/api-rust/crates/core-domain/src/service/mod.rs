use std::sync::Arc;
use crate::config::Config;
use crate::repository::Repository;
use infra_scheduler::{NoOpScheduleSolver, RemoteTimefoldSolver, ScheduleSolver};

pub mod auth;
pub mod authz;
pub mod class;
pub mod behavior;
pub mod notification;
pub mod organization;
pub mod user;
pub mod academic_cycle;
pub mod org_unit;
pub mod enrollment;
pub mod grade;
pub mod attendance_svc;
pub mod dashboard;
pub mod import_export;
pub mod bulk_import;
pub mod finance;
pub mod schedule;
pub mod schedule_common;
pub mod schedule_solver;
pub mod room;
pub mod subject;
pub mod assessment;
pub mod guardian;
pub mod document;
pub mod account;
pub mod seed;
pub mod permission;

// ── Examination Management System ──
pub mod exam_session;

// ── Academic Year Management & Promotion ──
pub mod academic_year;
pub mod academic_context;
pub mod calendar;
pub mod promotion;

pub mod student_ordering;
pub mod exam_numbers;
pub mod exam_logistics;
pub mod exam_auto_schedule;
pub mod exam_reports;
pub mod exam_analytics;

#[derive(Clone)]
pub struct Service {
    pub repo: Arc<Repository>,
    pub cfg: Arc<Config>,
    pub authz: Arc<authz::AuthzService>,
    pub(crate) scheduler: Arc<dyn ScheduleSolver>,
}

use chrono::Utc;
use serde_json::json;

impl Service {
    pub fn new(repo: Repository, cfg: Config) -> Self {
        let scheduler: Arc<dyn ScheduleSolver> = match &cfg.solver_service_url {
            Some(url) if !url.is_empty() => Arc::new(RemoteTimefoldSolver::new(url.clone())),
            _ => Arc::new(NoOpScheduleSolver),
        };

        Self {
            repo: Arc::new(repo),
            cfg: Arc::new(cfg),
            authz: Arc::new(authz::AuthzService::new()),
            scheduler,
        }
    }

    /// Centralized security helper: verifies permission and records audit logs on failure.
    pub async fn check_permission_and_audit(
        &self,
        user: &crate::entities::user::Model,
        permission: authz::Permission,
        ctx: Option<&authz::AuthzContext>,
    ) -> Result<(), String> {
        match self.authz.check_permission(&self.repo.db, user, permission, ctx).await {
            Ok(_) => Ok(()),
            Err(err_msg) => {
                // Log warning alert to tracing log system
                tracing::warn!(
                    "🔒 Security Warning: User '{}' ({}) from organization '{}' failed authorization check for '{:?}'. Context: {:?}. Reason: {}",
                    user.full_name,
                    user.id,
                    user.org_id,
                    permission,
                    ctx,
                    err_msg
                );

                // Persist the security failure into the database audit logs
                let audit_record = crate::entities::audit_log::Model {
                    id: uuid::Uuid::new_v4().to_string(),
                    org_id: user.org_id.clone(),
                    user_id: Some(user.id.clone()),
                    action: "security.unauthorized_attempt".to_string(),
                    entity_type: "permission".to_string(),
                    entity_id: format!("{:?}", permission),
                    old_data: None,
                    new_data: Some(json!({
                        "permission": format!("{:?}", permission),
                        "context": ctx.map(|c| json!({
                            "target_user_id": c.target_user_id,
                            "target_class_id": c.target_class_id,
                            "target_org_id": c.target_org_id,
                        })),
                        "error": err_msg.clone()
                    }).to_string()),
                    ip_address: None,
                    user_agent: None,
                    session_id: None,
                    metadata: json!({
                        "severity": "high",
                        "incident": true,
                        "client_notified": false
                    }).to_string(),
                    created_at: Utc::now().to_rfc3339(),
                };

                let _ = self.repo.create_audit_log(audit_record).await;

                Err(err_msg)
            }
        }
    }
}
