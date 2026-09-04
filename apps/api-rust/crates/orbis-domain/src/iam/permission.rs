use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    // --- Organization ---
    OrganizationRead,
    OrganizationWrite,

    // --- Users & IAM ---
    UserRead,
    UserWrite,
    UserDelete,
    UserRoleWrite,
    UserPasswordWrite,

    // --- Academic Period & Calendar ---
    PeriodRead,
    PeriodWrite,
    PeriodActivate,
    PeriodLock,
    PeriodUnlock,

    // --- Org Units (Branches/Departments) ---
    OrgUnitRead,
    OrgUnitWrite,
    OrgUnitDelete,

    // --- Classes & Subjects ---
    ClassRead,
    ClassWrite,
    ClassDelete,
    SubjectRead,
    SubjectWrite,
    SubjectDelete,

    // --- Scheduling ---
    ScheduleRead,
    ScheduleWrite,
    ScheduleDelete,

    // --- Assessments & Grades ---
    AssessmentRead,
    AssessmentWrite,
    AssessmentDelete,
    GradeRead,
    GradeWrite,
    GradeEntryManage,
    GradeAnalytics,
    GradeReport,

    // --- Students & Guardians ---
    GuardianRead,
    GuardianWrite,
    GuardianDelete,
    EnrollmentRead,
    EnrollmentWrite,
    EnrollmentPromote,
    EnrollmentTransfer,

    // --- Attendance & Behavior ---
    AttendanceRead,
    AttendanceWrite,
    AttendanceAnalytics,
    AttendanceReport,
    BehaviorRead,
    BehaviorWrite,
    BehaviorDelete,
    BehaviorAI,

    // --- Documents & Templates ---
    DocumentRead,
    DocumentWrite,
    DocumentDelete,
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

    // --- Examinations ---
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

    // --- System & Audit ---
    NotificationRead,
    NotificationWrite,
    DashboardRead,
    AuditRead,
    ImportData,
    ExportData,

    // --- Finance ---
    FinanceRead,
    FinanceWrite,
    ProcessPayment,

    // --- Promotions ---
    PromotionRead,
    PromotionWrite,
    PromotionExecute,

    // --- Special System Roles ---
    AutopilotBypass, // System-level permission
}

impl Permission {
    pub fn as_str(&self) -> &'static str {
        match self {
            Permission::OrganizationRead => "organization:read",
            Permission::OrganizationWrite => "organization:write",
            Permission::UserRead => "user:read",
            Permission::UserWrite => "user:write",
            Permission::UserDelete => "user:delete",
            Permission::UserRoleWrite => "user:role_write",
            Permission::UserPasswordWrite => "user:password_write",
            Permission::PeriodRead => "period:read",
            Permission::PeriodWrite => "period:write",
            Permission::PeriodActivate => "period:activate",
            Permission::PeriodLock => "period:lock",
            Permission::PeriodUnlock => "period:unlock",
            Permission::OrgUnitRead => "org_unit:read",
            Permission::OrgUnitWrite => "org_unit:write",
            Permission::OrgUnitDelete => "org_unit:delete",
            Permission::ClassRead => "class:read",
            Permission::ClassWrite => "class:write",
            Permission::ClassDelete => "class:delete",
            Permission::SubjectRead => "subject:read",
            Permission::SubjectWrite => "subject:write",
            Permission::SubjectDelete => "subject:delete",
            Permission::ScheduleRead => "schedule:read",
            Permission::ScheduleWrite => "schedule:write",
            Permission::ScheduleDelete => "schedule:delete",
            Permission::AssessmentRead => "assessment:read",
            Permission::AssessmentWrite => "assessment:write",
            Permission::AssessmentDelete => "assessment:delete",
            Permission::GradeRead => "grade:read",
            Permission::GradeWrite => "grade:write",
            Permission::GradeEntryManage => "grade:entry_manage",
            Permission::GradeAnalytics => "grade:analytics",
            Permission::GradeReport => "grade:report",
            Permission::GuardianRead => "guardian:read",
            Permission::GuardianWrite => "guardian:write",
            Permission::GuardianDelete => "guardian:delete",
            Permission::EnrollmentRead => "enrollment:read",
            Permission::EnrollmentWrite => "enrollment:write",
            Permission::EnrollmentPromote => "enrollment:promote",
            Permission::EnrollmentTransfer => "enrollment:transfer",
            Permission::AttendanceRead => "attendance:read",
            Permission::AttendanceWrite => "attendance:write",
            Permission::AttendanceAnalytics => "attendance:analytics",
            Permission::AttendanceReport => "attendance:report",
            Permission::BehaviorRead => "behavior:read",
            Permission::BehaviorWrite => "behavior:write",
            Permission::BehaviorDelete => "behavior:delete",
            Permission::BehaviorAI => "behavior:ai",
            Permission::DocumentRead => "document:read",
            Permission::DocumentWrite => "document:write",
            Permission::DocumentDelete => "document:delete",
            Permission::TemplateRead => "template:read",
            Permission::TemplateWrite => "template:write",
            Permission::TemplatePublish => "template:publish",
            Permission::TemplateDelete => "template:delete",
            Permission::TemplateImport => "template:import",
            Permission::TemplateExport => "template:export",
            Permission::TemplateGenerateSingle => "template:generate_single",
            Permission::TemplateGenerateBatch => "template:generate_batch",
            Permission::TemplateGenerateDownload => "template:generate_download",
            Permission::TemplateAssetUpload => "template:asset_upload",
            Permission::TemplateAssetDelete => "template:asset_delete",
            Permission::TemplateComponentWrite => "template:component_write",
            Permission::TemplateComponentDelete => "template:component_delete",
            Permission::ExamSessionRead => "exam_session:read",
            Permission::ExamSessionWrite => "exam_session:write",
            Permission::ExamSessionActivate => "exam_session:activate",
            Permission::ExamSessionDelete => "exam_session:delete",
            Permission::SeatingNumberGenerate => "seating_number:generate",
            Permission::SeatingNumberRead => "seating_number:read",
            Permission::SecretNumberGenerate => "secret_number:generate",
            Permission::SecretNumberRead => "secret_number:read",
            Permission::SecretNumberReveal => "secret_number:reveal",
            Permission::ExamHallRead => "exam_hall:read",
            Permission::ExamHallWrite => "exam_hall:write",
            Permission::ExamHallAllocate => "exam_hall:allocate",
            Permission::ProctorRead => "proctor:read",
            Permission::ProctorAssign => "proctor:assign",
            Permission::StudentOrderRead => "student_order:read",
            Permission::StudentOrderRebuild => "student_order:rebuild",
            Permission::StudentOrderFreeze => "student_order:freeze",
            Permission::ExamScheduleRead => "exam_schedule:read",
            Permission::ExamScheduleWrite => "exam_schedule:write",
            Permission::NotificationRead => "notification:read",
            Permission::NotificationWrite => "notification:write",
            Permission::DashboardRead => "dashboard:read",
            Permission::AuditRead => "audit:read",
            Permission::ImportData => "data:import",
            Permission::ExportData => "data:export",
            Permission::FinanceRead => "finance:read",
            Permission::FinanceWrite => "finance:write",
            Permission::ProcessPayment => "finance:payment",
            Permission::PromotionRead => "promotion:read",
            Permission::PromotionWrite => "promotion:write",
            Permission::PromotionExecute => "promotion:execute",
            Permission::AutopilotBypass => "system:autopilot",
        }
    }

    pub fn from_str(val: &str) -> Option<Self> {
        // Simple iteration to find the matching enum (in production we'd use a macro or static map)
        let all_perms = [
            Permission::OrganizationRead, Permission::OrganizationWrite, Permission::UserRead, Permission::UserWrite, Permission::UserDelete, Permission::UserRoleWrite, Permission::UserPasswordWrite,
            Permission::PeriodRead, Permission::PeriodWrite, Permission::PeriodActivate, Permission::PeriodLock, Permission::PeriodUnlock,
            Permission::OrgUnitRead, Permission::OrgUnitWrite, Permission::OrgUnitDelete,
            Permission::ClassRead, Permission::ClassWrite, Permission::ClassDelete, Permission::SubjectRead, Permission::SubjectWrite, Permission::SubjectDelete,
            Permission::ScheduleRead, Permission::ScheduleWrite, Permission::ScheduleDelete,
            Permission::AssessmentRead, Permission::AssessmentWrite, Permission::AssessmentDelete, Permission::GradeRead, Permission::GradeWrite, Permission::GradeEntryManage, Permission::GradeAnalytics, Permission::GradeReport,
            Permission::GuardianRead, Permission::GuardianWrite, Permission::GuardianDelete, Permission::EnrollmentRead, Permission::EnrollmentWrite, Permission::EnrollmentPromote, Permission::EnrollmentTransfer,
            Permission::AttendanceRead, Permission::AttendanceWrite, Permission::AttendanceAnalytics, Permission::AttendanceReport, Permission::BehaviorRead, Permission::BehaviorWrite, Permission::BehaviorDelete, Permission::BehaviorAI,
            Permission::DocumentRead, Permission::DocumentWrite, Permission::DocumentDelete, Permission::TemplateRead, Permission::TemplateWrite, Permission::TemplatePublish, Permission::TemplateDelete, Permission::TemplateImport, Permission::TemplateExport, Permission::TemplateGenerateSingle, Permission::TemplateGenerateBatch, Permission::TemplateGenerateDownload, Permission::TemplateAssetUpload, Permission::TemplateAssetDelete, Permission::TemplateComponentWrite, Permission::TemplateComponentDelete,
            Permission::ExamSessionRead, Permission::ExamSessionWrite, Permission::ExamSessionActivate, Permission::ExamSessionDelete, Permission::SeatingNumberGenerate, Permission::SeatingNumberRead, Permission::SecretNumberGenerate, Permission::SecretNumberRead, Permission::SecretNumberReveal, Permission::ExamHallRead, Permission::ExamHallWrite, Permission::ExamHallAllocate, Permission::ProctorRead, Permission::ProctorAssign, Permission::StudentOrderRead, Permission::StudentOrderRebuild, Permission::StudentOrderFreeze, Permission::ExamScheduleRead, Permission::ExamScheduleWrite,
            Permission::NotificationRead, Permission::NotificationWrite, Permission::DashboardRead, Permission::AuditRead, Permission::ImportData, Permission::ExportData,
            Permission::FinanceRead, Permission::FinanceWrite, Permission::ProcessPayment,
            Permission::PromotionRead, Permission::PromotionWrite, Permission::PromotionExecute,
            Permission::AutopilotBypass,
        ];

        all_perms.into_iter().find(|p| p.as_str() == val)
    }
}
