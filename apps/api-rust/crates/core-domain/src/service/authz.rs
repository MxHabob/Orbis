use serde::{Deserialize, Serialize};

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

    pub fn check_permission(
        &self,
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

        // 2. Principal check (School Principal)
        if role == "principal" {
            match permission {
                // Principal cannot perform system-wide organization modification or hard deletions of users
                Permission::UserDelete | Permission::OrganizationWrite | Permission::AuditRead => {
                    return Err("Principal is not authorized for system administration actions".to_string());
                }
                _ => return Ok(()),
            }
        }

        // 3. Teacher check
        if role == "teacher" {
            match permission {
                Permission::GradeRead | Permission::GradeWrite | Permission::GradeAnalytics | Permission::GradeReport |
                Permission::AttendanceRead | Permission::AttendanceWrite | Permission::AttendanceAnalytics | Permission::AttendanceReport |
                Permission::BehaviorRead | Permission::BehaviorWrite | Permission::BehaviorAI |
                Permission::ClassRead | Permission::SubjectRead | Permission::ScheduleRead |
                Permission::AssessmentRead | Permission::DocumentRead |
                Permission::NotificationRead | Permission::NotificationWrite |
                Permission::UserRead | Permission::CycleRead | Permission::OrgUnitRead | Permission::EnrollmentRead |
                Permission::OrganizationRead | Permission::DashboardRead |
                Permission::ExamSessionRead | Permission::ExamScheduleRead |
                Permission::SeatingNumberRead | Permission::ProctorRead |
                Permission::StudentOrderRead | Permission::ExamHallRead => {
                    return Ok(());
                }
                _ => return Err("Teacher is not authorized for this operation".to_string()),
            }
        }

        // 4. Student and Parent check
        if role == "student" || role == "parent" {
            match permission {
                Permission::GradeRead | Permission::AttendanceRead | Permission::BehaviorRead |
                Permission::ScheduleRead | Permission::AssessmentRead | Permission::DocumentRead |
                Permission::NotificationRead | Permission::NotificationWrite | Permission::ClassRead |
                Permission::UserRead | Permission::OrganizationRead => {
                    // ABAC: Ensure they can only access their own records
                    if let Some(ctx) = ctx {
                        if let Some(ref target_id) = ctx.target_user_id {
                            if target_id == &user.id {
                                return Ok(());
                            }
                        }
                    }
                    // If it is OrganizationRead, the multi-tenancy organization check above is sufficient
                    if permission == Permission::OrganizationRead {
                        return Ok(());
                    }
                    // For safety, deny if context is missing or doesn't match the user's own ID
                    return Err("Access denied: You are only authorized to access your own records".to_string());
                }
                _ => return Err("Student/Parent is not authorized for this operation".to_string()),
            }
        }

        Err("Access denied: Unauthorized role or permission".to_string())
    }
}
