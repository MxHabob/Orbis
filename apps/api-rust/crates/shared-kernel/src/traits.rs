//! Platform Service Contracts — ServiceContext dependencies.
//!
//! These traits define the cross-cutting concerns that every microservice
//! can depend on without coupling to concrete implementations.
//!
//! Implementations:
//!   - `PermissionChecker`  → svc-iam
//!   - `AuditLogger`        → svc-iam
//!   - `SettingsProvider`   → svc-organization
//!   - `PolicyEngine`       → svc-organization
//!   - `NotificationSender` → infra-integration
//!   - `FeeAssigner`        → svc-finance

use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

use crate::errors::AppError;
use crate::settings::{RegistrationSettings, FinanceSettings, GradingSettings, OrgSettings};

// ── Re-export existing types ────────────────────────────────────

// Actor and DomainEvent are kept here from the original traits.rs

// ── Permission Checking ─────────────────────────────────────────

/// System-wide permission variants.
///
/// Each variant maps to a (document_type, action) pair in the
/// `role_permissions` table. Admin/SuperAdmin bypass all checks.
///
/// NOTE: This is a copy of domain-iam's Permission enum, living here
/// so that all services can reference it without depending on svc-iam.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
pub enum Permission {
    // Organization
    OrganizationRead, OrganizationWrite,
    // Users
    UserRead, UserWrite, UserDelete, UserRoleWrite, UserPasswordWrite,
    // Academic Cycle
    CycleRead, CycleWrite, CycleActivate, CycleLock, CycleUnlock, GradeEntryManage,
    // Org Units
    OrgUnitRead, OrgUnitWrite, OrgUnitDelete,
    // Classes
    ClassRead, ClassWrite, ClassDelete,
    // Subjects
    SubjectRead, SubjectWrite, SubjectDelete,
    // Schedule
    ScheduleRead, ScheduleWrite, ScheduleDelete,
    // Assessments
    AssessmentRead, AssessmentWrite, AssessmentDelete,
    // Guardians
    GuardianRead, GuardianWrite, GuardianDelete,
    // Documents
    DocumentRead, DocumentWrite, DocumentDelete,
    // Enrollments
    EnrollmentRead, EnrollmentWrite, EnrollmentPromote, EnrollmentTransfer,
    // Grades
    GradeRead, GradeWrite, GradeAnalytics, GradeReport,
    // Attendance
    AttendanceRead, AttendanceWrite, AttendanceAnalytics, AttendanceReport,
    // Behavior
    BehaviorRead, BehaviorWrite, BehaviorDelete, BehaviorAI,
    // Notifications
    NotificationRead, NotificationWrite,
    // Dashboard & Audit
    DashboardRead, AuditRead,
    // Finance
    FinanceRead, FinanceWrite,
    // Import/Export
    ImportData, ExportData,
    // Examination
    ExamSessionRead, ExamSessionWrite, ExamSessionActivate, ExamSessionDelete,
    SeatingNumberGenerate, SeatingNumberRead,
    SecretNumberGenerate, SecretNumberRead, SecretNumberReveal,
    ExamHallRead, ExamHallWrite, ExamHallAllocate,
    ProctorRead, ProctorAssign,
    StudentOrderRead, StudentOrderRebuild, StudentOrderFreeze,
    ExamScheduleRead, ExamScheduleWrite,
    // Templates
    TemplateRead, TemplateWrite, TemplatePublish, TemplateDelete,
    TemplateImport, TemplateExport,
    TemplateGenerateSingle, TemplateGenerateBatch, TemplateGenerateDownload,
    TemplateAssetUpload, TemplateAssetDelete,
    TemplateComponentWrite, TemplateComponentDelete,
    // Promotion
    PromotionRead, PromotionWrite, PromotionExecute,
}

/// Additional authorization context for fine-grained checks.
#[derive(Debug, Clone, Default)]
pub struct AuthzContext {
    /// Restrict check to a specific class.
    pub class_id: Option<String>,
    /// Restrict check to a specific cycle.
    pub cycle_id: Option<String>,
    /// Restrict check to a specific org unit.
    pub org_unit_id: Option<String>,
}

/// Checks whether an actor has a specific permission.
///
/// Implemented by `svc-iam` — wraps the existing `AuthzService`.
#[async_trait]
pub trait PermissionChecker: Send + Sync {
    /// Check if actor has the given permission. Returns `Err(Unauthorized)` if denied.
    async fn check(&self, actor: &Actor, permission: Permission) -> Result<(), AppError>;

    /// Check with additional context (e.g. class-level permissions).
    async fn check_with_context(
        &self,
        actor: &Actor,
        permission: Permission,
        ctx: &AuthzContext,
    ) -> Result<(), AppError> {
        // Default: ignore context, delegate to simple check.
        let _ = ctx;
        self.check(actor, permission).await
    }
}

// ── Audit Logging ───────────────────────────────────────────────

/// An audit log entry — records who did what and when.
#[derive(Debug, Clone, Serialize)]
pub struct AuditEntry {
    pub org_id: String,
    pub actor: Actor,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub old_data: Option<Value>,
    pub new_data: Option<Value>,
    pub correlation_id: Option<String>,
}

/// Logs audit entries for every write operation.
///
/// Implemented by `svc-iam` — writes to `audit_log` table.
#[async_trait]
pub trait AuditLogger: Send + Sync {
    async fn log(&self, entry: AuditEntry) -> Result<(), AppError>;
}

// ── Settings Provider ───────────────────────────────────────────

/// Provides organization settings to all services.
///
/// Implemented by `svc-organization` — reads from `organization.settings` JSON column.
#[async_trait]
pub trait SettingsProvider: Send + Sync {
    async fn get_org_settings(&self, org_id: &str) -> Result<OrgSettings, AppError>;

    async fn get_registration_settings(&self, org_id: &str) -> Result<RegistrationSettings, AppError> {
        Ok(self.get_org_settings(org_id).await?.registration)
    }

    async fn get_finance_settings(&self, org_id: &str) -> Result<FinanceSettings, AppError> {
        Ok(self.get_org_settings(org_id).await?.finance)
    }

    async fn get_grading_settings(&self, org_id: &str) -> Result<GradingSettings, AppError> {
        Ok(self.get_org_settings(org_id).await?.grading)
    }
}

// ── Policy Engine ───────────────────────────────────────────────

/// Evaluates organizational policies — approval workflows, overrides.
///
/// Implemented by `svc-organization`.
#[async_trait]
pub trait PolicyEngine: Send + Sync {
    /// Check if an action requires admin approval for this org.
    async fn requires_approval(&self, action: &str, org_id: &str) -> Result<bool, AppError>;

    /// Check if an override has been granted for this action.
    async fn check_override(
        &self,
        action: &str,
        org_id: &str,
        override_id: &str,
    ) -> Result<bool, AppError>;
}

// ── Notification Sender ─────────────────────────────────────────

/// A notification to send after important operations.
#[derive(Debug, Clone, Serialize)]
pub struct Notification {
    pub org_id: String,
    pub title: String,
    pub title_ar: Option<String>,
    pub body: String,
    pub body_ar: Option<String>,
    pub target_user_ids: Vec<String>,
    pub target_roles: Vec<String>,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationChannel {
    InApp,
    Push,
    Email,
    Sms,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

/// Sends notifications after important operations.
///
/// Implemented by `infra-integration`.
#[async_trait]
pub trait NotificationSender: Send + Sync {
    async fn send(&self, notification: Notification) -> Result<(), AppError>;
    async fn send_bulk(&self, notifications: Vec<Notification>) -> Result<(), AppError> {
        for n in notifications {
            self.send(n).await?;
        }
        Ok(())
    }
}

// ── Fee Assigner ────────────────────────────────────────────────

/// Assigns mandatory fees to a newly enrolled student.
///
/// Implemented by `svc-finance`.
#[async_trait]
pub trait FeeAssigner: Send + Sync {
    /// Assign mandatory fee templates to an enrollment. Returns assigned fee IDs.
    async fn assign_mandatory_fees(
        &self,
        org_id: &str,
        enrollment_id: &str,
        class_id: &str,
    ) -> Result<Vec<String>, AppError>;
}

// ═══════════════════════════════════════════════════════════════
// Preserved from original traits.rs
// ═══════════════════════════════════════════════════════════════

// ── Domain Event ────────────────────────────────────────────────

/// A domain event represents something that happened in the system.
///
/// Events are published to the `EventBus` after a successful command.
/// Consumers (Autopilot, AI Gateway, Notifications) subscribe to relevant events.
pub trait DomainEvent: std::any::Any + Send + Sync + 'static {
    fn event_type(&self) -> &'static str;
    fn entity_type(&self) -> &'static str;
    fn entity_id(&self) -> &str;
    fn org_id(&self) -> &str;
    fn correlation_id(&self) -> Option<&str> { None }
    fn actor(&self) -> &str;
    fn payload(&self) -> Value;
}

// ── Command / Query Contracts ───────────────────────────────────

/// A command handler processes a write operation and may emit domain events.
#[allow(async_fn_in_trait)]
pub trait CommandHandler<C, R> {
    async fn handle(&self, command: C) -> Result<R, AppError>;
}

/// A query handler processes a read operation (pure read — no mutations).
#[allow(async_fn_in_trait)]
pub trait QueryHandler<Q, R> {
    async fn handle(&self, query: Q) -> Result<R, AppError>;
}

// ── Actor Type ──────────────────────────────────────────────────

/// Identifies who performed an operation — used in audit logging.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "id")]
pub enum Actor {
    User(String),
    System,
    Agent(String),
    Autopilot,
}

impl Actor {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User(id) => id,
            Self::System => "system",
            Self::Agent(id) => id,
            Self::Autopilot => "autopilot",
        }
    }

    pub fn actor_type(&self) -> &'static str {
        match self {
            Self::User(_) => "user",
            Self::System => "system",
            Self::Agent(_) => "ai_agent",
            Self::Autopilot => "autopilot",
        }
    }
}
