//! Authorization service — permission checking against role_permissions table.
//!
//! Uses `AppError` instead of `String` for structured errors.
//! The Permission enum is a system-wide permission registry.


use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
use shared_kernel::errors::AppError;
use crate::entities::{role_permission, user};

use shared_kernel::traits::Permission;

/// Map a Permission variant to (document_type, action) for database lookup.
pub fn to_doc_action(permission: &Permission) -> (&'static str, &'static str) {
    match permission {
        Permission::OrganizationRead => ("Organization", "read"),
        Permission::OrganizationWrite => ("Organization", "write"),
            Permission::UserRead => ("User", "read"),
            Permission::UserWrite => ("User", "write"),
            Permission::UserDelete => ("User", "delete"),
            Permission::UserRoleWrite => ("User", "role_write"),
            Permission::UserPasswordWrite => ("User", "password_write"),
            Permission::PeriodRead => ("Period", "read"),
            Permission::PeriodWrite => ("Period", "write"),
            Permission::PeriodActivate => ("Period", "activate"),
            Permission::PeriodLock => ("Period", "lock"),
            Permission::PeriodUnlock => ("Period", "unlock"),
            Permission::GradeEntryManage => ("GradeEntry", "manage"),
            Permission::OrgUnitRead => ("OrgUnit", "read"),
            Permission::OrgUnitWrite => ("OrgUnit", "write"),
            Permission::OrgUnitDelete => ("OrgUnit", "delete"),
            Permission::ClassRead => ("Class", "read"),
            Permission::ClassWrite => ("Class", "write"),
            Permission::ClassDelete => ("Class", "delete"),
            Permission::SubjectRead => ("Subject", "read"),
            Permission::SubjectWrite => ("Subject", "write"),
            Permission::SubjectDelete => ("Subject", "delete"),
            Permission::ScheduleRead => ("Schedule", "read"),
            Permission::ScheduleWrite => ("Schedule", "write"),
            Permission::ScheduleDelete => ("Schedule", "delete"),
            Permission::AssessmentRead => ("Assessment", "read"),
            Permission::AssessmentWrite => ("Assessment", "write"),
            Permission::AssessmentDelete => ("Assessment", "delete"),
            Permission::GuardianRead => ("Guardian", "read"),
            Permission::GuardianWrite => ("Guardian", "write"),
            Permission::GuardianDelete => ("Guardian", "delete"),
            Permission::DocumentRead => ("Document", "read"),
            Permission::DocumentWrite => ("Document", "write"),
            Permission::DocumentDelete => ("Document", "delete"),
            Permission::EnrollmentRead => ("Enrollment", "read"),
            Permission::EnrollmentWrite => ("Enrollment", "write"),
            Permission::EnrollmentPromote => ("Enrollment", "promote"),
            Permission::EnrollmentTransfer => ("Enrollment", "transfer"),
            Permission::GradeRead => ("Grade", "read"),
            Permission::GradeWrite => ("Grade", "write"),
            Permission::GradeAnalytics => ("Grade", "analytics"),
            Permission::GradeReport => ("Grade", "report"),
            Permission::AttendanceRead => ("Attendance", "read"),
            Permission::AttendanceWrite => ("Attendance", "write"),
            Permission::AttendanceAnalytics => ("Attendance", "analytics"),
            Permission::AttendanceReport => ("Attendance", "report"),
            Permission::BehaviorRead => ("Behavior", "read"),
            Permission::BehaviorWrite => ("Behavior", "write"),
            Permission::BehaviorDelete => ("Behavior", "delete"),
            Permission::BehaviorAI => ("Behavior", "ai"),
            Permission::NotificationRead => ("Notification", "read"),
            Permission::NotificationWrite => ("Notification", "write"),
            Permission::DashboardRead => ("Dashboard", "read"),
            Permission::AuditRead => ("Audit", "read"),
            Permission::FinanceRead => ("Finance", "read"),
            Permission::FinanceWrite => ("Finance", "write"),
            Permission::ImportData => ("ImportExport", "import"),
            Permission::ExportData => ("ImportExport", "export"),
            Permission::ExamSessionRead => ("Exam", "read"),
            Permission::ExamSessionWrite => ("Exam", "write"),
            Permission::ExamSessionActivate => ("Exam", "activate"),
            Permission::ExamSessionDelete => ("Exam", "delete"),
            Permission::SeatingNumberGenerate => ("Exam", "generate_seating"),
            Permission::SeatingNumberRead => ("Exam", "read_seating"),
            Permission::SecretNumberGenerate => ("Exam", "generate_secret"),
            Permission::SecretNumberRead => ("Exam", "read_secret"),
            Permission::SecretNumberReveal => ("Exam", "reveal_secret"),
            Permission::ExamHallRead => ("Exam", "read_hall"),
            Permission::ExamHallWrite => ("Exam", "write_hall"),
            Permission::ExamHallAllocate => ("Exam", "allocate_hall"),
            Permission::ProctorRead => ("Exam", "read_proctor"),
            Permission::ProctorAssign => ("Exam", "assign_proctor"),
            Permission::StudentOrderRead => ("Exam", "read_order"),
            Permission::StudentOrderRebuild => ("Exam", "rebuild_order"),
            Permission::StudentOrderFreeze => ("Exam", "freeze_order"),
            Permission::ExamScheduleRead => ("Exam", "read_schedule"),
            Permission::ExamScheduleWrite => ("Exam", "write_schedule"),
            Permission::TemplateRead => ("Template", "read"),
            Permission::TemplateWrite => ("Template", "write"),
            Permission::TemplatePublish => ("Template", "publish"),
            Permission::TemplateDelete => ("Template", "delete"),
            Permission::TemplateImport => ("Template", "import"),
            Permission::TemplateExport => ("Template", "export"),
            Permission::TemplateGenerateSingle => ("Template", "generate_single"),
            Permission::TemplateGenerateBatch => ("Template", "generate_batch"),
            Permission::TemplateGenerateDownload => ("Template", "generate_download"),
            Permission::TemplateAssetUpload => ("Template", "asset_upload"),
        Permission::TemplateAssetDelete => ("Template", "asset_delete"),
        Permission::TemplateComponentWrite => ("Template", "component_write"),
        Permission::TemplateComponentDelete => ("Template", "component_delete"),
        Permission::PromotionRead => ("Promotion", "read"),
        Permission::PromotionWrite => ("Promotion", "write"),
        Permission::PromotionExecute => ("Promotion", "execute"),
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

        // Tenant boundaries apply to administrators as well.
        if let Some(ctx) = ctx {
            if let Some(ref target_org_id) = ctx.target_org_id {
                if target_org_id != &user.org_id {
                    return Err(AppError::unauthorized("Cannot access resources outside of your organization"));
                }
            }
        }

        // Administrators bypass permission rules, but not tenant boundaries.
        if role == "super_admin" || role == "admin" {
            return Ok(());
        }

        // 3. Database rule lookup
        let (doc_type, action) = to_doc_action(&permission);

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

        // 4. ABAC (Attribute-Based Access Control) Integration
        if let Some(ctx) = ctx {
            let mut abac_ctx = platform_policy::abac::PolicyContext::new();
            abac_ctx.subject_attributes.insert("role".to_string(), role.clone());
            abac_ctx.subject_attributes.insert("user_id".to_string(), user.id.clone());
            
            if let Some(ref target_class_id) = ctx.target_class_id {
                abac_ctx.resource_attributes.insert("class_id".to_string(), target_class_id.clone());
            }
            if let Some(ref target_user_id) = ctx.target_user_id {
                abac_ctx.resource_attributes.insert("target_user_id".to_string(), target_user_id.clone());
            }
            abac_ctx.resource_attributes.insert("doc_type".to_string(), doc_type.to_string());

            // Instantiate a lightweight ABAC engine (in a real app, rules are loaded from DB/Cache)
            let abac_engine = platform_policy::abac::AbacEngine::new(vec![]);
            if abac_engine.evaluate(&abac_ctx) == platform_policy::abac::Effect::Deny {
                tracing::warn!("ABAC denied access to {} for {}", doc_type, user.id);
                return Err(AppError::unauthorized("Access denied by policy"));
            }
        }

        // 5. Creator-only check
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
