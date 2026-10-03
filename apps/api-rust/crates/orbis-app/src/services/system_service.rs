//! System Service — health checks, system status, and organization readiness.
//!
//! Provides a unified interface for:
//! - Health/liveness checks (is the database connected?)
//! - Organization existence checks (does an org exist?)
//! - Organization readiness (what setup items are pending?)
//!
//! Used by Desktop startup orchestrator, CLI doctor, and server health endpoints.

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::ports::repositories::SystemRepository;
use orbis_domain::errors::DomainError;

/// System health response — returned by health checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub api_version: String,
    pub database_connected: bool,
    pub has_organization: bool,
    pub schema_version: String,
}

/// Organization lifecycle state — derived from what data exists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OrgLifecycleState {
    /// No organization exists in the database.
    Empty,
    /// Organization + admin exist, but no academic configuration.
    CoreReady,
    /// Academic year, classes, and subjects exist — fully operational.
    Operational,
}

/// A pending setup item — something the user can configure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetupItem {
    pub key: String,
    pub label: String,
    pub label_ar: String,
    pub is_required: bool,
    pub is_completed: bool,
}

/// Organization readiness — computed by querying what exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationReadiness {
    pub state: OrgLifecycleState,
    pub org_id: Option<String>,
    pub org_name: Option<String>,
    pub has_organization: bool,
    pub has_admin: bool,
    pub has_academic_year: bool,
    pub has_classes: bool,
    pub has_subjects: bool,
    pub pending_setup_items: Vec<SetupItem>,
}

pub struct SystemService {
    pub system_repo: Arc<dyn SystemRepository>,
}

impl SystemService {
    pub fn new(system_repo: Arc<dyn SystemRepository>) -> Self {
        Self { system_repo }
    }

    /// Health check — is the system reachable and queryable?
    pub async fn health(&self) -> HealthResponse {
        let has_org = self.system_repo.has_organization().await.unwrap_or(false);

        HealthResponse {
            status: "ok".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            api_version: "v1".to_string(),
            database_connected: true,
            has_organization: has_org,
            schema_version: "m20240101_000001".to_string(),
        }
    }

    /// Does an organization exist in the database?
    pub async fn has_organization(&self) -> Result<bool, DomainError> {
        self.system_repo.has_organization().await
    }

    /// Get organization readiness — what setup items are pending.
    pub async fn get_readiness(&self) -> Result<OrganizationReadiness, DomainError> {
        let has_org = self.system_repo.has_organization().await?;
        if !has_org {
            return Ok(OrganizationReadiness {
                state: OrgLifecycleState::Empty,
                org_id: None,
                org_name: None,
                has_organization: false,
                has_admin: false,
                has_academic_year: false,
                has_classes: false,
                has_subjects: false,
                pending_setup_items: vec![],
            });
        }

        let org_id = self.system_repo.get_first_org_id().await?;
        let org_name = self.system_repo.get_org_name().await?;
        let has_admin = self.system_repo.has_admin().await?;

        let (has_academic_year, has_classes, has_subjects) = if let Some(oid) = org_id {
            let year = self.system_repo.has_academic_year(oid).await?;
            let cls = self.system_repo.has_classes(oid).await?;
            let subj = self.system_repo.has_subjects(oid).await?;
            (year, cls, subj)
        } else {
            (false, false, false)
        };

        // Determine lifecycle state
        let state = if has_academic_year && has_classes && has_subjects {
            OrgLifecycleState::Operational
        } else if has_org && has_admin {
            OrgLifecycleState::CoreReady
        } else {
            OrgLifecycleState::Empty
        };

        // Build pending setup items
        let mut items = vec![];

        items.push(SetupItem {
            key: "academic_year".into(),
            label: "Academic Year".into(),
            label_ar: "العام الدراسي".into(),
            is_required: true,
            is_completed: has_academic_year,
        });
        items.push(SetupItem {
            key: "classes".into(),
            label: "Classes".into(),
            label_ar: "الفصول".into(),
            is_required: true,
            is_completed: has_classes,
        });
        items.push(SetupItem {
            key: "subjects".into(),
            label: "Subjects".into(),
            label_ar: "المواد".into(),
            is_required: true,
            is_completed: has_subjects,
        });

        Ok(OrganizationReadiness {
            state,
            org_id: org_id.map(|id| id.to_string()),
            org_name,
            has_organization: has_org,
            has_admin,
            has_academic_year,
            has_classes,
            has_subjects,
            pending_setup_items: items,
        })
    }
}
