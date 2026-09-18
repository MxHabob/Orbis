//! Bootstrap port — Application-layer contract for bulk organization bootstrap persistence.
//!
//! The Application layer (BootstrapService) owns orchestration, validation, and business rules.
//! The Infrastructure layer implements this port for efficient bulk persistence.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use orbis_domain::errors::DomainError;

// ── Validated Bootstrap Data ────────────────────────────────────────────────
// Produced by BootstrapService after validation. Consumed by BulkBootstrapPort.

#[derive(Debug, Clone)]
pub struct ValidatedBootstrapData {
    pub organization: ValidatedOrg,
    pub admin_person: ValidatedPerson,
    pub admin_user: ValidatedUser,
    pub admin_role_assignment: ValidatedRoleAssignment,
    pub roles: Vec<ValidatedRole>,
    pub role_permissions: Vec<ValidatedRolePermission>,
    pub period_templates: Vec<ValidatedPeriodTemplate>,
    pub assessment_scheme: ValidatedAssessmentScheme,
    pub accounts: Vec<ValidatedAccount>,
    pub currency: ValidatedCurrency,
    pub org_settings: serde_json::Value,
    pub report_definitions: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct ValidatedOrg {
    pub id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub slug: String,
    pub org_type: String,
    pub timezone: String,
    pub locale: String,
    pub base_currency_code: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedPerson {
    pub id: Uuid,
    pub org_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedUser {
    pub id: Uuid,
    pub person_id: Uuid,
    pub org_id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedRoleAssignment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role_id: String,
    pub org_id: Uuid,
    pub scope_type: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedRole {
    pub id: String,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub is_system: bool,
}

#[derive(Debug, Clone)]
pub struct ValidatedRolePermission {
    pub id: Uuid,
    pub role_id: String,
    pub org_id: Uuid,
    pub resource: String,
    pub action: String,
    pub effect: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedPeriodTemplate {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub institution_type: String,
    pub structure: serde_json::Value,
    pub is_default: bool,
}

#[derive(Debug, Clone)]
pub struct ValidatedAssessmentScheme {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub description: String,
    pub max_score: f64,
    pub formula: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct ValidatedAccount {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub currency_code: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedCurrency {
    pub id: String,
    pub name: String,
    pub name_ar: String,
    pub symbol: String,
    pub decimal_places: i32,
}

/// Result of a bootstrap persistence operation.
#[derive(Debug, Serialize, Deserialize)]
pub struct BootstrapPersistenceResult {
    pub org_id: String,
    pub admin_user_id: String,
    pub rows_inserted: u64,
}

// ── Initial Setup Data ──────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ValidatedInitialSetupData {
    pub org_id: Uuid,
    pub academic_year: ValidatedAcademicYear,
    pub periods: Vec<ValidatedPeriod>,
    pub classes: Vec<ValidatedClass>,
    pub subjects: Vec<ValidatedSubject>,
}

#[derive(Debug, Clone)]
pub struct ValidatedAcademicYear {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedPeriod {
    pub id: Uuid,
    pub org_id: Uuid,
    pub year_id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub period_type: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedClass {
    pub id: Uuid,
    pub org_id: Uuid,
    pub name: String,
    pub name_ar: String,
    pub level: String,
    pub division: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidatedSubject {
    pub id: Uuid,
    pub org_id: Uuid,
    pub code: String,
    pub name: String,
    pub name_ar: String,
    pub is_core: bool,
}

// ── Port Trait ──────────────────────────────────────────────────────────────

/// Infrastructure port for efficient bulk bootstrap persistence.
///
/// Application = WHAT + WHY (validation, orchestration, business rules)
/// Infrastructure = HOW efficiently (batch SQL, bulk inserts, transactions)
#[async_trait]
pub trait BulkBootstrapPort: Send + Sync {
    /// Persist all bootstrap data in a single atomic transaction.
    /// Must be idempotent — safe to retry on partial failure.
    async fn persist_bootstrap(
        &self,
        data: ValidatedBootstrapData,
    ) -> Result<BootstrapPersistenceResult, DomainError>;

    /// Persist initial setup data (academic year, periods, classes, subjects)
    /// in a single atomic transaction.
    async fn persist_initial_setup(
        &self,
        data: ValidatedInitialSetupData,
    ) -> Result<(), DomainError>;
}
