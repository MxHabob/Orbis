use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // Organization
    OrganizationRead,
    OrganizationWrite,

    // Users
    UserRead,
    UserWrite,
    UserDelete,
    UserRoleWrite,
    UserPasswordWrite,

    // Academic Cycle
    CycleRead,
    CycleWrite,
    CycleActivate,
    CycleLock,
    CycleUnlock,
    GradeEntryManage,

    // Org Units
    OrgUnitRead,
    OrgUnitWrite,
    OrgUnitDelete,

    // Classes
    ClassRead,
    ClassWrite,
    ClassDelete,

    // Subjects
    SubjectRead,
    SubjectWrite,
    SubjectDelete,

    // Schedule
    ScheduleRead,
    ScheduleWrite,
    ScheduleDelete,

    // Assessments
    AssessmentRead,
    AssessmentWrite,
    AssessmentDelete,

    // Guardians
    GuardianRead,
    GuardianWrite,
    GuardianDelete,

    // Documents
    DocumentRead,
    DocumentWrite,
    DocumentDelete,

    // Enrollments
    EnrollmentRead,
    EnrollmentWrite,
    EnrollmentPromote,
    EnrollmentTransfer,

    // Grades
    GradeRead,
    GradeWrite,
    GradeAnalytics,
    GradeReport,

    // Attendance
    AttendanceRead,
    AttendanceWrite,
    AttendanceAnalytics,
    AttendanceReport,

    // Behavior
    BehaviorRead,
    BehaviorWrite,
    BehaviorDelete,
    BehaviorAI,

    // Notifications
    NotificationRead,
    NotificationWrite,

    // Dashboard
    DashboardRead,

    // Audit Log
    AuditRead,

    // Finance
    FinanceRead,
    FinanceWrite,

    // Import/Export
    ImportData,
    ExportData,

    // Examination Management
    ExamSessionRead,
    ExamSessionWrite,
    ExamSessionActivate,
    ExamSessionDelete,
    SeatingNumberGenerate,
    SeatingNumberRead,
    SecretNumberGenerate,
    SecretNumberRead,
    SecretNumberReveal,
    ExamHallRead,
    ExamHallWrite,
    ExamHallAllocate,
    ProctorRead,
    ProctorAssign,
    StudentOrderRead,
    StudentOrderRebuild,
    StudentOrderFreeze,
    ExamScheduleRead,
    ExamScheduleWrite,

    // Template Management
    TemplateRead,
    TemplateWrite,
    TemplatePublish,
    TemplateDelete,
    TemplateImport,
    TemplateExport,
    TemplateGenerateSingle,
    TemplateGenerateBatch,
    TemplateGenerateDownload,
    TemplateAssetUpload,
    TemplateAssetDelete,
    TemplateComponentWrite,
    TemplateComponentDelete,
}

#[derive(Debug, Clone, Default)]
pub struct AuthzContext {
    pub target_user_id: Option<String>,
    pub target_class_id: Option<String>,
    pub target_org_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AuthzService;

impl AuthzService {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_permission(
        &self,
        db: &sea_orm::DatabaseConnection,
        user: &crate::entities::user::Model,
        permission: Permission,
        ctx: Option<&AuthzContext>,
    ) -> Result<(), String> {
        let role = user.primary_role.to_lowercase();

        // 1. Super Admin and Admin bypass all checks
        if role == "super_admin" || role == "admin" {
            return Ok(());
        }

        // Cross-tenant data boundary protection (Multi-tenancy check)
        if let Some(ctx) = ctx {
            if let Some(ref target_org_id) = ctx.target_org_id {
                if target_org_id != &user.org_id {
                    return Err("Access denied: Cannot access resources outside of your organization".to_string());
                }
            }
        }

        // 2. Map legacy Permission variants to (document_type, action)
        let (doc_type, action) = match permission {
            Permission::OrganizationRead => ("Organization", "read"),
            Permission::OrganizationWrite => ("Organization", "write"),
            Permission::UserRead => ("User", "read"),
            Permission::UserWrite => ("User", "write"),
            Permission::UserDelete => ("User", "delete"),
            Permission::UserRoleWrite => ("User", "role_write"),
            Permission::UserPasswordWrite => ("User", "password_write"),
            Permission::CycleRead => ("Cycle", "read"),
            Permission::CycleWrite => ("Cycle", "write"),
            Permission::CycleActivate => ("Cycle", "activate"),
            Permission::CycleLock => ("Cycle", "lock"),
            Permission::CycleUnlock => ("Cycle", "unlock"),
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
        };

        // 3. Query role permissions rule from the SQLite database
        use sea_orm::{EntityTrait, QueryFilter, ColumnTrait};
        let rule_opt = crate::entities::role_permission::Entity::find()
            .filter(crate::entities::role_permission::Column::Role.eq(role))
            .filter(crate::entities::role_permission::Column::DocumentType.eq(doc_type.to_string()))
            .one(db)
            .await
            .map_err(|e| e.to_string())?;

        let rule = match rule_opt {
            Some(r) => r,
            None => return Err(format!("Access denied: No permission rule defined for document type '{}'", doc_type)),
        };

        let actions: Vec<String> = serde_json::from_str(&rule.actions)
            .map_err(|e| format!("Failed to parse actions JSON: {}", e))?;

        if !actions.iter().any(|a| a == action) {
            return Err(format!("Access denied: Action '{}' not permitted for document type '{}'", action, doc_type));
        }

        // 4. Check "Only If Creator" condition
        if rule.only_if_creator == 1 {
            let mut is_creator = false;
            if let Some(ctx) = ctx {
                if let Some(ref target_user_id) = ctx.target_user_id {
                    if target_user_id == &user.id {
                        is_creator = true;
                    }
                }
            }
            if !is_creator {
                return Err("Access denied: Operation only allowed for creator".to_string());
            }
        }

        Ok(())
    }
}

