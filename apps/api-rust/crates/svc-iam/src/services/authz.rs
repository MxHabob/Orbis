//! Authorization service — permission checking against role_permissions table.
//!
//! Uses `AppError` instead of `String` for structured errors.
//! The Permission enum is a system-wide permission registry.

use serde::{Deserialize, Serialize};
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use shared_kernel::errors::AppError;
use crate::entities::{role_permission, user};

/// System-wide permission variants.
///
/// Each variant maps to a (document_type, action) pair in the
/// `role_permissions` table. Admin/SuperAdmin bypass all checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Organization
    OrganizationRead, OrganizationWrite,
    // Users
    UserRead, UserWrite, UserDelete, UserRoleWrite, UserPasswordWrite,
    // Academic Period
    PeriodRead, PeriodWrite, PeriodActivate, PeriodLock, PeriodUnlock, GradeEntryManage,
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
}

impl Permission {
    /// Map a Permission variant to (document_type, action) for database lookup.
    pub fn to_doc_action(&self) -> (&'static str, &'static str) {
        match self {
            Self::OrganizationRead => ("Organization", "read"),
            Self::OrganizationWrite => ("Organization", "write"),
            Self::UserRead => ("User", "read"),
            Self::UserWrite => ("User", "write"),
            Self::UserDelete => ("User", "delete"),
            Self::UserRoleWrite => ("User", "role_write"),
            Self::UserPasswordWrite => ("User", "password_write"),
            Self::PeriodRead => ("Period", "read"),
            Self::PeriodWrite => ("Period", "write"),
            Self::PeriodActivate => ("Period", "activate"),
            Self::PeriodLock => ("Period", "lock"),
            Self::PeriodUnlock => ("Period", "unlock"),
            Self::GradeEntryManage => ("GradeEntry", "manage"),
            Self::OrgUnitRead => ("OrgUnit", "read"),
            Self::OrgUnitWrite => ("OrgUnit", "write"),
            Self::OrgUnitDelete => ("OrgUnit", "delete"),
            Self::ClassRead => ("Class", "read"),
            Self::ClassWrite => ("Class", "write"),
            Self::ClassDelete => ("Class", "delete"),
            Self::SubjectRead => ("Subject", "read"),
            Self::SubjectWrite => ("Subject", "write"),
            Self::SubjectDelete => ("Subject", "delete"),
            Self::ScheduleRead => ("Schedule", "read"),
            Self::ScheduleWrite => ("Schedule", "write"),
            Self::ScheduleDelete => ("Schedule", "delete"),
            Self::AssessmentRead => ("Assessment", "read"),
            Self::AssessmentWrite => ("Assessment", "write"),
            Self::AssessmentDelete => ("Assessment", "delete"),
            Self::GuardianRead => ("Guardian", "read"),
            Self::GuardianWrite => ("Guardian", "write"),
            Self::GuardianDelete => ("Guardian", "delete"),
            Self::DocumentRead => ("Document", "read"),
            Self::DocumentWrite => ("Document", "write"),
            Self::DocumentDelete => ("Document", "delete"),
            Self::EnrollmentRead => ("Enrollment", "read"),
            Self::EnrollmentWrite => ("Enrollment", "write"),
            Self::EnrollmentPromote => ("Enrollment", "promote"),
            Self::EnrollmentTransfer => ("Enrollment", "transfer"),
            Self::GradeRead => ("Grade", "read"),
            Self::GradeWrite => ("Grade", "write"),
            Self::GradeAnalytics => ("Grade", "analytics"),
            Self::GradeReport => ("Grade", "report"),
            Self::AttendanceRead => ("Attendance", "read"),
            Self::AttendanceWrite => ("Attendance", "write"),
            Self::AttendanceAnalytics => ("Attendance", "analytics"),
            Self::AttendanceReport => ("Attendance", "report"),
            Self::BehaviorRead => ("Behavior", "read"),
            Self::BehaviorWrite => ("Behavior", "write"),
            Self::BehaviorDelete => ("Behavior", "delete"),
            Self::BehaviorAI => ("Behavior", "ai"),
            Self::NotificationRead => ("Notification", "read"),
            Self::NotificationWrite => ("Notification", "write"),
            Self::DashboardRead => ("Dashboard", "read"),
            Self::AuditRead => ("Audit", "read"),
            Self::FinanceRead => ("Finance", "read"),
            Self::FinanceWrite => ("Finance", "write"),
            Self::ImportData => ("ImportExport", "import"),
            Self::ExportData => ("ImportExport", "export"),
            Self::ExamSessionRead => ("Exam", "read"),
            Self::ExamSessionWrite => ("Exam", "write"),
            Self::ExamSessionActivate => ("Exam", "activate"),
            Self::ExamSessionDelete => ("Exam", "delete"),
            Self::SeatingNumberGenerate => ("Exam", "generate_seating"),
            Self::SeatingNumberRead => ("Exam", "read_seating"),
            Self::SecretNumberGenerate => ("Exam", "generate_secret"),
            Self::SecretNumberRead => ("Exam", "read_secret"),
            Self::SecretNumberReveal => ("Exam", "reveal_secret"),
            Self::ExamHallRead => ("Exam", "read_hall"),
            Self::ExamHallWrite => ("Exam", "write_hall"),
            Self::ExamHallAllocate => ("Exam", "allocate_hall"),
            Self::ProctorRead => ("Exam", "read_proctor"),
            Self::ProctorAssign => ("Exam", "assign_proctor"),
            Self::StudentOrderRead => ("Exam", "read_order"),
            Self::StudentOrderRebuild => ("Exam", "rebuild_order"),
            Self::StudentOrderFreeze => ("Exam", "freeze_order"),
            Self::ExamScheduleRead => ("Exam", "read_schedule"),
            Self::ExamScheduleWrite => ("Exam", "write_schedule"),
            Self::TemplateRead => ("Template", "read"),
            Self::TemplateWrite => ("Template", "write"),
            Self::TemplatePublish => ("Template", "publish"),
            Self::TemplateDelete => ("Template", "delete"),
            Self::TemplateImport => ("Template", "import"),
            Self::TemplateExport => ("Template", "export"),
            Self::TemplateGenerateSingle => ("Template", "generate_single"),
            Self::TemplateGenerateBatch => ("Template", "generate_batch"),
            Self::TemplateGenerateDownload => ("Template", "generate_download"),
            Self::TemplateAssetUpload => ("Template", "asset_upload"),
            Self::TemplateAssetDelete => ("Template", "asset_delete"),
            Self::TemplateComponentWrite => ("Template", "component_write"),
            Self::TemplateComponentDelete => ("Template", "component_delete"),
        }
    }
}

/// Context for permission checks (optional scope constraints).
#[derive(Debug, Clone, Default)]
pub struct AuthzContext {
    pub target_user_id: Option<String>,
    pub target_class_id: Option<String>,
    pub target_org_id: Option<String>,
}

/// Authorization service — stateless, uses database for rule lookups.
#[derive(Clone, Debug)]
pub struct AuthzService;

impl AuthzService {
    pub fn new() -> Self {
        Self
    }

    /// Check if a user has a specific permission.
    ///
    /// Admin and SuperAdmin roles bypass all checks.
    /// Returns `AppError::Forbidden` on denial.
    pub async fn check_permission(
        &self,
        db: &sea_orm::DatabaseConnection,
        user: &user::Model,
        permission: Permission,
        ctx: Option<&AuthzContext>,
    ) -> Result<(), AppError> {
        let role = user.primary_role.to_lowercase();

        // 1. Admin bypass
        if role == "super_admin" || role == "admin" {
            return Ok(());
        }

        // 2. Cross-tenant boundary check
        if let Some(ctx) = ctx {
            if let Some(ref target_org_id) = ctx.target_org_id {
                if target_org_id != &user.org_id {
                    return Err(AppError::unauthorized("Cannot access resources outside of your organization"));
                }
            }
        }

        // 3. Database rule lookup
        let (doc_type, action) = permission.to_doc_action();

        let rule = role_permission::Entity::find()
            .filter(role_permission::Column::Role.eq(&role))
            .filter(role_permission::Column::DocumentType.eq(doc_type))
            .one(db)
            .await?
            .ok_or_else(|| AppError::unauthorized(format!("No permission rule for document type '{}'", doc_type)))?;

        let actions: Vec<String> = serde_json::from_str(&rule.actions)
            .map_err(|e| AppError::internal(format!("Failed to parse actions: {}", e)))?;

        if !actions.iter().any(|a| a == action) {
            return Err(AppError::unauthorized(format!("Action '{}' not permitted for '{}'", action, doc_type)));
        }

        // 4. Creator-only check
        if rule.only_if_creator == 1 {
            let is_creator = ctx
                .and_then(|c| c.target_user_id.as_ref())
                .map(|id| id == &user.id)
                .unwrap_or(false);

            if !is_creator {
                return Err(AppError::unauthorized("Operation only allowed for the creator"));
            }
        }

        Ok(())
    }
}
