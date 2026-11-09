import { Pencil } from "@orbis/icons";
import * as m from "$lib/paraglide/messages";
/**
 * Orbis IAM Permissions System (Single Source of Truth)
 *
 * Maps directly to backend `orbis_domain::iam::permission::Permission`.
 * Provides Paraglide-localized metadata, domain groupings, resources,
 * actions, system role presets, and utility helpers.
 */

export interface IamActionDef {
  key: string;
  labelKey: IamMessageKey;
  descriptionKey?: IamMessageKey;
}

export interface IamResourceDef {
  key: string;
  labelKey: IamMessageKey;
  descriptionKey: IamMessageKey;
  actions: IamActionDef[];
}

export interface IamDomainDef {
  id: string;
  nameKey: IamMessageKey;
  descriptionKey: IamMessageKey;
  iconName: string;
  resources: IamResourceDef[];
}

export interface SystemRoleDef {
  id: string;
  nameKey: IamMessageKey;
  descriptionKey: IamMessageKey;
  iconName: string;
  color: string;
  isSystem: boolean;
  defaultPermissions: Record<string, string[]>;
}

export type IamMessageKey = `iam_${string}`;

export function translateIamMessage(
  key: IamMessageKey,
  locale?: "ar" | "en",
): string {
  const message = m[key as keyof typeof m] as
    | ((
        inputs?: Record<string, never>,
        options?: { locale?: "ar" | "en" },
      ) => string)
    | undefined;
  return message?.({}, locale ? { locale } : undefined) ?? key;
}

// ============================================================================
// Canonical Backend Permissions Map (Single Source of Truth)
// ============================================================================

export const Permissions = {
  // --- Organization ---
  OrganizationRead: "organization:read",
  OrganizationWrite: "organization:write",

  // --- Users & IAM ---
  UserRead: "user:read",
  UserWrite: "user:write",
  UserDelete: "user:delete",
  UserRoleWrite: "user:role_write",
  UserPasswordWrite: "user:password_write",

  // --- User Groups ---
  GroupRead: "group:read",
  GroupWrite: "group:write",
  GroupDelete: "group:delete",
  GroupAssign: "group:assign",

  // --- Employees & Staff ---
  EmployeeRead: "employee:read",
  EmployeeWrite: "employee:write",
  EmployeeCreate: "employee:create",
  EmployeeUpdate: "employee:update",
  EmployeeDelete: "employee:delete",

  // --- Academic Period & Calendar ---
  PeriodRead: "period:read",
  PeriodWrite: "period:write",
  PeriodActivate: "period:activate",
  PeriodLock: "period:lock",
  PeriodUnlock: "period:unlock",

  // --- Org Units (Branches/Departments) ---
  OrgUnitRead: "org_unit:read",
  OrgUnitWrite: "org_unit:write",
  OrgUnitDelete: "org_unit:delete",

  // --- Classes & Subjects ---
  ClassRead: "class:read",
  ClassWrite: "class:write",
  ClassDelete: "class:delete",
  SubjectRead: "subject:read",
  SubjectWrite: "subject:write",
  SubjectDelete: "subject:delete",

  // --- Scheduling ---
  ScheduleRead: "schedule:read",
  ScheduleWrite: "schedule:write",
  ScheduleDelete: "schedule:delete",

  // --- Assessments & Grades ---
  AssessmentRead: "assessment:read",
  AssessmentWrite: "assessment:write",
  AssessmentDelete: "assessment:delete",
  GradeRead: "grade:read",
  GradeWrite: "grade:write",
  GradeEntryManage: "grade:entry_manage",
  GradeAnalytics: "grade:analytics",
  GradeReport: "grade:report",

  // --- Students & Guardians ---
  StudentRead: "student:read",
  StudentWrite: "student:write",
  StudentCreate: "student:create",
  StudentUpdate: "student:update",
  StudentDelete: "student:delete",
  GuardianRead: "guardian:read",
  GuardianWrite: "guardian:write",
  GuardianDelete: "guardian:delete",
  EnrollmentRead: "enrollment:read",
  EnrollmentWrite: "enrollment:write",
  EnrollmentPromote: "enrollment:promote",
  EnrollmentTransfer: "enrollment:transfer",

  // --- Workflows & Approvals ---
  WorkflowRead: "workflow:read",
  WorkflowWrite: "workflow:write",
  WorkflowApprove: "workflow:approve",
  WorkflowExecute: "workflow:execute",

  // --- Attendance & Behavior ---
  AttendanceRead: "attendance:read",
  AttendanceWrite: "attendance:write",
  AttendanceAnalytics: "attendance:analytics",
  AttendanceReport: "attendance:report",
  BehaviorRead: "behavior:read",
  BehaviorWrite: "behavior:write",
  BehaviorDelete: "behavior:delete",
  BehaviorAI: "behavior:ai",

  // --- Documents & Templates ---
  DocumentRead: "document:read",
  DocumentWrite: "document:write",
  DocumentDelete: "document:delete",
  TemplateRead: "template:read",
  TemplateWrite: "template:write",
  TemplatePublish: "template:publish",
  TemplateDelete: "template:delete",
  TemplateImport: "template:import",
  TemplateExport: "template:export",
  TemplateGenerateSingle: "template:generate_single",
  TemplateGenerateBatch: "template:generate_batch",
  TemplateGenerateDownload: "template:generate_download",
  TemplateAssetUpload: "template:asset_upload",
  TemplateAssetDelete: "template:asset_delete",
  TemplateComponentWrite: "template:component_write",
  TemplateComponentDelete: "template:component_delete",

  // --- Examinations ---
  ExamSessionRead: "exam_session:read",
  ExamSessionWrite: "exam_session:write",
  ExamSessionActivate: "exam_session:activate",
  ExamSessionDelete: "exam_session:delete",
  SeatingNumberGenerate: "seating_number:generate",
  SeatingNumberRead: "seating_number:read",
  SecretNumberGenerate: "secret_number:generate",
  SecretNumberRead: "secret_number:read",
  SecretNumberReveal: "secret_number:reveal",
  ExamHallRead: "exam_hall:read",
  ExamHallWrite: "exam_hall:write",
  ExamHallAllocate: "exam_hall:allocate",
  ProctorRead: "proctor:read",
  ProctorAssign: "proctor:assign",
  StudentOrderRead: "student_order:read",
  StudentOrderRebuild: "student_order:rebuild",
  StudentOrderFreeze: "student_order:freeze",
  ExamScheduleRead: "exam_schedule:read",
  ExamScheduleWrite: "exam_schedule:write",

  // --- System & Audit ---
  NotificationRead: "notification:read",
  NotificationWrite: "notification:write",
  DashboardRead: "dashboard:read",
  AuditRead: "audit:read",
  ImportData: "data:import",
  ExportData: "data:export",

  // --- Finance ---
  FinanceRead: "finance:read",
  FinanceWrite: "finance:write",
  ProcessPayment: "finance:payment",
  FinanceBackdate: "finance:backdate",
  FinanceFutureDate: "finance:future_date",

  // --- Promotions ---
  PromotionRead: "promotion:read",
  PromotionWrite: "promotion:write",
  PromotionExecute: "promotion:execute",

  // --- Special System Roles ---
  AutopilotBypass: "system:autopilot",
} as const;

export type Permission = (typeof Permissions)[keyof typeof Permissions];

// ============================================================================
// 1. Unified IAM Domain & Resource Catalog
// ============================================================================

export const IAM_DOMAINS: IamDomainDef[] = [
  {
    id: "iam",
    nameKey: "iam_domain_iam_name",
    descriptionKey: "iam_domain_iam_description",
    iconName: "Shield",
    resources: [
      {
        key: "organization",
        labelKey: "iam_resource_organization_label",
        descriptionKey: "iam_resource_organization_description",
        actions: [
          { key: "read", labelKey: "iam_action_organization_read_label" },
          { key: "write", labelKey: "iam_action_organization_write_label" },
        ],
      },
      {
        key: "org_unit",
        labelKey: "iam_resource_org_unit_label",
        descriptionKey: "iam_resource_org_unit_description",
        actions: [
          { key: "read", labelKey: "iam_action_org_unit_read_label" },
          { key: "write", labelKey: "iam_action_org_unit_write_label" },
          { key: "delete", labelKey: "iam_action_org_unit_delete_label" },
        ],
      },
      {
        key: "user",
        labelKey: "iam_resource_user_label",
        descriptionKey: "iam_resource_user_description",
        actions: [
          { key: "read", labelKey: "iam_action_user_read_label" },
          { key: "write", labelKey: "iam_action_user_write_label" },
          { key: "delete", labelKey: "iam_action_user_delete_label" },
          { key: "role_write", labelKey: "iam_action_user_role_write_label" },
          {
            key: "password_write",
            labelKey: "iam_action_user_password_write_label",
          },
        ],
      },
      {
        key: "group",
        labelKey: "iam_resource_group_label",
        descriptionKey: "iam_resource_group_description",
        actions: [
          { key: "read", labelKey: "iam_action_group_read_label" },
          { key: "write", labelKey: "iam_action_group_write_label" },
          { key: "delete", labelKey: "iam_action_group_delete_label" },
          { key: "assign", labelKey: "iam_action_group_assign_label" },
        ],
      },
    ],
  },
  {
    id: "employees",
    nameKey: "iam_domain_employees_name",
    descriptionKey: "iam_domain_employees_description",
    iconName: "UserGroup",
    resources: [
      {
        key: "employee",
        labelKey: "iam_resource_employee_label",
        descriptionKey: "iam_resource_employee_description",
        actions: [
          { key: "read", labelKey: "iam_action_employee_read_label" },
          { key: "write", labelKey: "iam_action_employee_write_label" },
          { key: "create", labelKey: "iam_action_employee_create_label" },
          { key: "update", labelKey: "iam_action_employee_update_label" },
          { key: "delete", labelKey: "iam_action_employee_delete_label" },
        ],
      },
    ],
  },
  {
    id: "academic",
    nameKey: "iam_domain_academic_name",
    descriptionKey: "iam_domain_academic_description",
    iconName: "GraduationCap",
    resources: [
      {
        key: "period",
        labelKey: "iam_resource_period_label",
        descriptionKey: "iam_resource_period_description",
        actions: [
          { key: "read", labelKey: "iam_action_period_read_label" },
          { key: "write", labelKey: "iam_action_period_write_label" },
          { key: "activate", labelKey: "iam_action_period_activate_label" },
          { key: "lock", labelKey: "iam_action_period_lock_label" },
          { key: "unlock", labelKey: "iam_action_period_unlock_label" },
        ],
      },
      {
        key: "class",
        labelKey: "iam_resource_class_label",
        descriptionKey: "iam_resource_class_description",
        actions: [
          { key: "read", labelKey: "iam_action_class_read_label" },
          { key: "write", labelKey: "iam_action_class_write_label" },
          { key: "delete", labelKey: "iam_action_class_delete_label" },
        ],
      },
      {
        key: "subject",
        labelKey: "iam_resource_subject_label",
        descriptionKey: "iam_resource_subject_description",
        actions: [
          { key: "read", labelKey: "iam_action_subject_read_label" },
          { key: "write", labelKey: "iam_action_subject_write_label" },
          { key: "delete", labelKey: "iam_action_subject_delete_label" },
        ],
      },
      {
        key: "schedule",
        labelKey: "iam_resource_schedule_label",
        descriptionKey: "iam_resource_schedule_description",
        actions: [
          { key: "read", labelKey: "iam_action_schedule_read_label" },
          { key: "write", labelKey: "iam_action_schedule_write_label" },
          { key: "delete", labelKey: "iam_action_schedule_delete_label" },
        ],
      },
    ],
  },
  {
    id: "students",
    nameKey: "iam_domain_students_name",
    descriptionKey: "iam_domain_students_description",
    iconName: "Users",
    resources: [
      {
        key: "student",
        labelKey: "iam_resource_student_label",
        descriptionKey: "iam_resource_student_description",
        actions: [
          { key: "read", labelKey: "iam_action_student_read_label" },
          { key: "create", labelKey: "iam_action_student_create_label" },
          { key: "update", labelKey: "iam_action_student_update_label" },
          { key: "delete", labelKey: "iam_action_student_delete_label" },
          { key: "write", labelKey: "iam_action_student_write_label" },
        ],
      },
      {
        key: "guardian",
        labelKey: "iam_resource_guardian_label",
        descriptionKey: "iam_resource_guardian_description",
        actions: [
          { key: "read", labelKey: "iam_action_guardian_read_label" },
          { key: "write", labelKey: "iam_action_guardian_write_label" },
          { key: "delete", labelKey: "iam_action_guardian_delete_label" },
        ],
      },
      {
        key: "enrollment",
        labelKey: "iam_resource_enrollment_label",
        descriptionKey: "iam_resource_enrollment_description",
        actions: [
          { key: "read", labelKey: "iam_action_enrollment_read_label" },
          { key: "write", labelKey: "iam_action_enrollment_write_label" },
          { key: "promote", labelKey: "iam_action_enrollment_promote_label" },
          { key: "transfer", labelKey: "iam_action_enrollment_transfer_label" },
        ],
      },
      {
        key: "promotion",
        labelKey: "iam_resource_promotion_label",
        descriptionKey: "iam_resource_promotion_description",
        actions: [
          { key: "read", labelKey: "iam_action_promotion_read_label" },
          { key: "write", labelKey: "iam_action_promotion_write_label" },
          { key: "execute", labelKey: "iam_action_promotion_execute_label" },
        ],
      },
    ],
  },
  {
    id: "assessment",
    nameKey: "iam_domain_assessment_name",
    descriptionKey: "iam_domain_assessment_description",
    iconName: "FileCheck",
    resources: [
      {
        key: "assessment",
        labelKey: "iam_resource_assessment_label",
        descriptionKey: "iam_resource_assessment_description",
        actions: [
          { key: "read", labelKey: "iam_action_assessment_read_label" },
          { key: "write", labelKey: "iam_action_assessment_write_label" },
          { key: "delete", labelKey: "iam_action_assessment_delete_label" },
        ],
      },
      {
        key: "grade",
        labelKey: "iam_resource_grade_label",
        descriptionKey: "iam_resource_grade_description",
        actions: [
          { key: "read", labelKey: "iam_action_grade_read_label" },
          { key: "write", labelKey: "iam_action_grade_write_label" },
          {
            key: "entry_manage",
            labelKey: "iam_action_grade_entry_manage_label",
          },
          { key: "analytics", labelKey: "iam_action_grade_analytics_label" },
          { key: "report", labelKey: "iam_action_grade_report_label" },
        ],
      },
      {
        key: "attendance",
        labelKey: "iam_resource_attendance_label",
        descriptionKey: "iam_resource_attendance_description",
        actions: [
          { key: "read", labelKey: "iam_action_attendance_read_label" },
          { key: "write", labelKey: "iam_action_attendance_write_label" },
          {
            key: "analytics",
            labelKey: "iam_action_attendance_analytics_label",
          },
          { key: "report", labelKey: "iam_action_attendance_report_label" },
        ],
      },
      {
        key: "behavior",
        labelKey: "iam_resource_behavior_label",
        descriptionKey: "iam_resource_behavior_description",
        actions: [
          { key: "read", labelKey: "iam_action_behavior_read_label" },
          { key: "write", labelKey: "iam_action_behavior_write_label" },
          { key: "delete", labelKey: "iam_action_behavior_delete_label" },
          { key: "ai", labelKey: "iam_action_behavior_ai_label" },
        ],
      },
    ],
  },
  {
    id: "examinations",
    nameKey: "iam_domain_examinations_name",
    descriptionKey: "iam_domain_examinations_description",
    iconName: "Lock",
    resources: [
      {
        key: "exam_session",
        labelKey: "iam_resource_exam_session_label",
        descriptionKey: "iam_resource_exam_session_description",
        actions: [
          { key: "read", labelKey: "iam_action_exam_session_read_label" },
          { key: "write", labelKey: "iam_action_exam_session_write_label" },
          {
            key: "activate",
            labelKey: "iam_action_exam_session_activate_label",
          },
          { key: "delete", labelKey: "iam_action_exam_session_delete_label" },
        ],
      },
      {
        key: "seating_number",
        labelKey: "iam_resource_seating_number_label",
        descriptionKey: "iam_resource_seating_number_description",
        actions: [
          { key: "read", labelKey: "iam_action_seating_number_read_label" },
          {
            key: "generate",
            labelKey: "iam_action_seating_number_generate_label",
          },
        ],
      },
      {
        key: "secret_number",
        labelKey: "iam_resource_secret_number_label",
        descriptionKey: "iam_resource_secret_number_description",
        actions: [
          { key: "read", labelKey: "iam_action_secret_number_read_label" },
          {
            key: "generate",
            labelKey: "iam_action_secret_number_generate_label",
          },
          { key: "reveal", labelKey: "iam_action_secret_number_reveal_label" },
        ],
      },
      {
        key: "exam_hall",
        labelKey: "iam_resource_exam_hall_label",
        descriptionKey: "iam_resource_exam_hall_description",
        actions: [
          { key: "read", labelKey: "iam_action_exam_hall_read_label" },
          { key: "write", labelKey: "iam_action_exam_hall_write_label" },
          { key: "allocate", labelKey: "iam_action_exam_hall_allocate_label" },
        ],
      },
      {
        key: "proctor",
        labelKey: "iam_resource_proctor_label",
        descriptionKey: "iam_resource_proctor_description",
        actions: [
          { key: "read", labelKey: "iam_action_proctor_read_label" },
          { key: "assign", labelKey: "iam_action_proctor_assign_label" },
        ],
      },
      {
        key: "student_order",
        labelKey: "iam_resource_student_order_label",
        descriptionKey: "iam_resource_student_order_description",
        actions: [
          { key: "read", labelKey: "iam_action_student_order_read_label" },
          {
            key: "rebuild",
            labelKey: "iam_action_student_order_rebuild_label",
          },
          { key: "freeze", labelKey: "iam_action_student_order_freeze_label" },
        ],
      },
      {
        key: "exam_schedule",
        labelKey: "iam_resource_exam_schedule_label",
        descriptionKey: "iam_resource_exam_schedule_description",
        actions: [
          { key: "read", labelKey: "iam_action_exam_schedule_read_label" },
          { key: "write", labelKey: "iam_action_exam_schedule_write_label" },
        ],
      },
    ],
  },
  {
    id: "finance",
    nameKey: "iam_domain_finance_name",
    descriptionKey: "iam_domain_finance_description",
    iconName: "Wallet",
    resources: [
      {
        key: "finance",
        labelKey: "iam_resource_finance_label",
        descriptionKey: "iam_resource_finance_description",
        actions: [
          { key: "read", labelKey: "iam_action_finance_read_label" },
          { key: "write", labelKey: "iam_action_finance_write_label" },
          { key: "payment", labelKey: "iam_action_finance_payment_label" },
          { key: "backdate", labelKey: "iam_action_finance_backdate_label" },
          {
            key: "future_date",
            labelKey: "iam_action_finance_future_date_label",
          },
        ],
      },
    ],
  },
  {
    id: "documents",
    nameKey: "iam_domain_documents_name",
    descriptionKey: "iam_domain_documents_description",
    iconName: "Layers",
    resources: [
      {
        key: "document",
        labelKey: "iam_resource_document_label",
        descriptionKey: "iam_resource_document_description",
        actions: [
          { key: "read", labelKey: "iam_action_document_read_label" },
          { key: "write", labelKey: "iam_action_document_write_label" },
          { key: "delete", labelKey: "iam_action_document_delete_label" },
        ],
      },
      {
        key: "template",
        labelKey: "iam_resource_template_label",
        descriptionKey: "iam_resource_template_description",
        actions: [
          { key: "read", labelKey: "iam_action_template_read_label" },
          { key: "write", labelKey: "iam_action_template_write_label" },
          { key: "publish", labelKey: "iam_action_template_publish_label" },
          { key: "delete", labelKey: "iam_action_template_delete_label" },
          { key: "import", labelKey: "iam_action_template_import_label" },
          { key: "export", labelKey: "iam_action_template_export_label" },
          {
            key: "generate_single",
            labelKey: "iam_action_template_generate_single_label",
          },
          {
            key: "generate_batch",
            labelKey: "iam_action_template_generate_batch_label",
          },
          {
            key: "generate_download",
            labelKey: "iam_action_template_generate_download_label",
          },
          {
            key: "asset_upload",
            labelKey: "iam_action_template_asset_upload_label",
          },
          {
            key: "asset_delete",
            labelKey: "iam_action_template_asset_delete_label",
          },
          {
            key: "component_write",
            labelKey: "iam_action_template_component_write_label",
          },
          {
            key: "component_delete",
            labelKey: "iam_action_template_component_delete_label",
          },
        ],
      },
    ],
  },
  {
    id: "system",
    nameKey: "iam_domain_system_name",
    descriptionKey: "iam_domain_system_description",
    iconName: "Sparkles",
    resources: [
      {
        key: "workflow",
        labelKey: "iam_resource_workflow_label",
        descriptionKey: "iam_resource_workflow_description",
        actions: [
          { key: "read", labelKey: "iam_action_workflow_read_label" },
          { key: "write", labelKey: "iam_action_workflow_write_label" },
          { key: "approve", labelKey: "iam_action_workflow_approve_label" },
          { key: "execute", labelKey: "iam_action_workflow_execute_label" },
        ],
      },
      {
        key: "notification",
        labelKey: "iam_resource_notification_label",
        descriptionKey: "iam_resource_notification_description",
        actions: [
          { key: "read", labelKey: "iam_action_notification_read_label" },
          { key: "write", labelKey: "iam_action_notification_write_label" },
        ],
      },
      {
        key: "dashboard",
        labelKey: "iam_resource_dashboard_label",
        descriptionKey: "iam_resource_dashboard_description",
        actions: [{ key: "read", labelKey: "iam_action_dashboard_read_label" }],
      },
      {
        key: "audit",
        labelKey: "iam_resource_audit_label",
        descriptionKey: "iam_resource_audit_description",
        actions: [{ key: "read", labelKey: "iam_action_audit_read_label" }],
      },
      {
        key: "data",
        labelKey: "iam_resource_data_label",
        descriptionKey: "iam_resource_data_description",
        actions: [
          { key: "import", labelKey: "iam_action_data_import_label" },
          { key: "export", labelKey: "iam_action_data_export_label" },
        ],
      },
      {
        key: "system",
        labelKey: "iam_resource_system_label",
        descriptionKey: "iam_resource_system_description",
        actions: [
          { key: "autopilot", labelKey: "iam_action_system_autopilot_label" },
        ],
      },
    ],
  },
];

// ============================================================================
// 2. Standard System Roles with Default Domain Permissions
// ============================================================================

export const SYSTEM_ROLE_DEFINITIONS: SystemRoleDef[] = [
  {
    id: "super_admin",
    nameKey: "iam_role_super_admin_name",
    descriptionKey: "iam_role_super_admin_description",
    iconName: "Crown",
    color: "amber",
    isSystem: true,
    defaultPermissions: {
      "*": ["*"],
    },
  },
  {
    id: "admin",
    nameKey: "iam_role_admin_name",
    descriptionKey: "iam_role_admin_description",
    iconName: "Shield",
    color: "emerald",
    isSystem: true,
    defaultPermissions: {
      organization: ["read", "write"],
      org_unit: ["read", "write", "delete"],
      user: ["read", "write", "delete", "role_write", "password_write"],
      period: ["read", "write", "activate", "lock", "unlock"],
      class: ["read", "write", "delete"],
      subject: ["read", "write", "delete"],
      schedule: ["read", "write", "delete"],
      student: ["read", "create", "update", "delete", "write"],
      guardian: ["read", "write", "delete"],
      enrollment: ["read", "write", "promote", "transfer"],
      promotion: ["read", "write", "execute"],
      assessment: ["read", "write", "delete"],
      grade: ["read", "write", "entry_manage", "analytics", "report"],
      attendance: ["read", "write", "analytics", "report"],
      behavior: ["read", "write", "delete", "ai"],
      exam_session: ["read", "write", "activate", "delete"],
      seating_number: ["read", "generate"],
      secret_number: ["read", "generate", "reveal"],
      exam_hall: ["read", "write", "allocate"],
      proctor: ["read", "assign"],
      student_order: ["read", "rebuild", "freeze"],
      exam_schedule: ["read", "write"],
      finance: ["read", "write", "payment", "backdate", "future_date"],
      document: ["read", "write", "delete"],
      template: [
        "read",
        "write",
        "publish",
        "delete",
        "import",
        "export",
        "generate_single",
        "generate_batch",
        "generate_download",
        "asset_upload",
        "asset_delete",
        "component_write",
        "component_delete",
      ],
      workflow: ["read", "write", "approve", "execute"],
      notification: ["read", "write"],
      dashboard: ["read"],
      audit: ["read"],
      data: ["import", "export"],
    },
  },
  {
    id: "principal",
    nameKey: "iam_role_principal_name",
    descriptionKey: "iam_role_principal_description",
    iconName: "GraduationCap",
    color: "blue",
    isSystem: true,
    defaultPermissions: {
      organization: ["read"],
      org_unit: ["read"],
      user: ["read"],
      period: ["read", "activate", "lock"],
      class: ["read", "write"],
      subject: ["read"],
      schedule: ["read", "write"],
      student: ["read", "update"],
      guardian: ["read"],
      enrollment: ["read", "promote", "transfer"],
      promotion: ["read", "execute"],
      assessment: ["read", "write"],
      grade: ["read", "analytics", "report"],
      attendance: ["read", "analytics", "report"],
      behavior: ["read", "write", "ai"],
      exam_session: ["read", "activate"],
      seating_number: ["read", "generate"],
      secret_number: ["read", "reveal"],
      exam_hall: ["read", "allocate"],
      proctor: ["read", "assign"],
      student_order: ["read", "freeze"],
      exam_schedule: ["read", "write"],
      finance: ["read"],
      document: ["read", "write"],
      template: [
        "read",
        "generate_single",
        "generate_batch",
        "generate_download",
      ],
      workflow: ["read", "approve", "execute"],
      notification: ["read", "write"],
      dashboard: ["read"],
      audit: ["read"],
    },
  },
  {
    id: "teacher",
    nameKey: "iam_role_teacher_name",
    descriptionKey: "iam_role_teacher_description",
    iconName: "BookOpen",
    color: "indigo",
    isSystem: true,
    defaultPermissions: {
      class: ["read"],
      subject: ["read"],
      schedule: ["read"],
      student: ["read"],
      guardian: ["read"],
      assessment: ["read", "write"],
      grade: ["read", "write", "report"],
      attendance: ["read", "write"],
      behavior: ["read", "write"],
      exam_schedule: ["read"],
      document: ["read"],
      template: ["read", "generate_single"],
      notification: ["read"],
    },
  },
  {
    id: "registrar",
    nameKey: "iam_role_registrar_name",
    descriptionKey: "iam_role_registrar_description",
    iconName: "ClipboardCheck",
    color: "cyan",
    isSystem: true,
    defaultPermissions: {
      period: ["read"],
      class: ["read", "write"],
      subject: ["read"],
      student: ["read", "create", "update", "write"],
      guardian: ["read", "write"],
      enrollment: ["read", "write", "promote", "transfer"],
      promotion: ["read", "write"],
      attendance: ["read"],
      document: ["read", "write"],
      template: [
        "read",
        "generate_single",
        "generate_batch",
        "generate_download",
      ],
      notification: ["read", "write"],
    },
  },
  {
    id: "accountant",
    nameKey: "iam_role_accountant_name",
    descriptionKey: "iam_role_accountant_description",
    iconName: "BuildingLibrary",
    color: "emerald",
    isSystem: true,
    defaultPermissions: {
      student: ["read"],
      guardian: ["read"],
      enrollment: ["read"],
      finance: ["read", "write", "payment"],
      document: ["read", "write"],
      template: [
        "read",
        "generate_single",
        "generate_batch",
        "generate_download",
      ],
      notification: ["read"],
    },
  },
  {
    id: "staff",
    nameKey: "iam_role_staff_name",
    descriptionKey: "iam_role_staff_description",
    iconName: "UserCog",
    color: "slate",
    isSystem: true,
    defaultPermissions: {
      student: ["read"],
      guardian: ["read"],
      attendance: ["read"],
      document: ["read"],
      notification: ["read"],
    },
  },
  {
    id: "student",
    nameKey: "iam_role_student_name",
    descriptionKey: "iam_role_student_description",
    iconName: "User",
    color: "teal",
    isSystem: true,
    defaultPermissions: {
      schedule: ["read"],
      grade: ["read"],
      attendance: ["read"],
      document: ["read"],
      notification: ["read"],
    },
  },
  {
    id: "parent",
    nameKey: "iam_role_parent_name",
    descriptionKey: "iam_role_parent_description",
    iconName: "Users",
    color: "violet",
    isSystem: true,
    defaultPermissions: {
      student: ["read"],
      schedule: ["read"],
      grade: ["read"],
      attendance: ["read"],
      behavior: ["read"],
      finance: ["read"],
      document: ["read"],
      notification: ["read"],
    },
  },
];

// ============================================================================
// 3. Helper Functions & Utilities
// ============================================================================

/**
 * Returns all unique resource keys in the system catalog.
 */
export function getAllResourceKeys(): string[] {
  const keys: string[] = [];
  for (const domain of IAM_DOMAINS) {
    for (const res of domain.resources) {
      if (!keys.includes(res.key)) {
        keys.push(res.key);
      }
    }
  }
  return keys;
}

/**
 * Returns all actions available for a specific resource key.
 */
export function getActionsForResource(resourceKey: string): IamActionDef[] {
  for (const domain of IAM_DOMAINS) {
    for (const res of domain.resources) {
      if (res.key === resourceKey) {
        return res.actions;
      }
    }
  }
  return [
    { key: "read", labelKey: "iam_action_default_read_label" },
    { key: "write", labelKey: "iam_action_default_write_label" },
  ];
}

/**
 * Finds a resource definition by key.
 */
export function getResourceDef(
  resourceKey: string,
): IamResourceDef | undefined {
  for (const domain of IAM_DOMAINS) {
    for (const res of domain.resources) {
      if (res.key === resourceKey) return res;
    }
  }
  return undefined;
}

/**
 * Finds the domain containing a specific resource key.
 */
export function getDomainByResource(
  resourceKey: string,
): IamDomainDef | undefined {
  for (const domain of IAM_DOMAINS) {
    if (domain.resources.some((r) => r.key === resourceKey)) {
      return domain;
    }
  }
  return undefined;
}

/**
 * Returns a localized label for a resource.
 */
export function getResourceLabel(
  resourceKey: string,
  locale?: "ar" | "en",
): string {
  const res = getResourceDef(resourceKey);
  if (!res) return resourceKey;
  return translateIamMessage(res.labelKey, locale);
}

/**
 * Returns a localized label for an action within a resource.
 */
export function getActionLabel(
  resourceKey: string,
  actionKey: string,
  locale?: "ar" | "en",
): string {
  const actions = getActionsForResource(resourceKey);
  const action = actions.find((a) => a.key === actionKey);
  if (!action) {
    return actionKey;
  }
  return translateIamMessage(action.labelKey, locale);
}

/**
 * Returns a badge color variant based on the action key.
 */
export function getActionBadgeVariant(
  actionKey: string,
): "default" | "secondary" | "destructive" | "outline" | "lime" {
  switch (actionKey) {
    case "delete":
    case "component_delete":
    case "asset_delete":
      return "destructive";
    case "write":
    case "create":
    case "update":
    case "payment":
    case "execute":
    case "publish":
      return "lime";
    case "activate":
    case "lock":
    case "unlock":
    case "approve":
    case "role_write":
    case "password_write":
    case "generate":
    case "reveal":
      return "default";
    case "read":
    default:
      return "secondary";
  }
}

/**
 * Checks if a given role ID is a pre-defined system role.
 */
export function isSystemRole(roleId: string): boolean {
  return SYSTEM_ROLE_DEFINITIONS.some((r) => r.id === roleId);
}

/**
 * Gets the definition for a system role.
 */
export function getSystemRoleDef(roleId: string): SystemRoleDef | undefined {
  return SYSTEM_ROLE_DEFINITIONS.find((r) => r.id === roleId);
}

/**
 * Computes total permissions granted across a role's permissions dictionary.
 */
export function countRolePermissions(
  perms?: Record<string, string[]> | null,
  roleId?: string,
): number {
  if (perms && Object.keys(perms).length > 0) {
    if (perms["*"] && perms["*"].includes("*")) {
      let total = 0;
      for (const d of IAM_DOMAINS) {
        for (const r of d.resources) {
          total += r.actions.length;
        }
      }
      return total;
    }
    let count = 0;
    for (const acts of Object.values(perms)) {
      count += (acts || []).length;
    }
    if (count > 0) return count;
  }

  // Fallback to system role preset if perms is empty
  if (roleId) {
    const sys = getSystemRoleDef(roleId);
    if (sys) {
      return countRolePermissions(sys.defaultPermissions);
    }
  }

  return 0;
}

/**
 * Calculates domain coverage statistics for a given role's permissions.
 */
export function getRoleDomainStats(
  perms?: Record<string, string[]> | null,
  roleId?: string,
): Array<{
  domain: IamDomainDef;
  selectedCount: number;
  totalCount: number;
  percentage: number;
}> {
  let effectivePerms = perms;
  if ((!effectivePerms || Object.keys(effectivePerms).length === 0) && roleId) {
    const sys = getSystemRoleDef(roleId);
    if (sys) {
      effectivePerms = sys.defaultPermissions;
    }
  }

  const isSuper = Boolean(effectivePerms?.["*"]?.includes("*"));

  return IAM_DOMAINS.map((domain) => {
    let domainTotal = 0;
    let domainSelected = 0;

    for (const res of domain.resources) {
      domainTotal += res.actions.length;
      if (isSuper) {
        domainSelected += res.actions.length;
      } else if (effectivePerms && effectivePerms[res.key]) {
        domainSelected += effectivePerms[res.key].length;
      }
    }

    return {
      domain,
      selectedCount: domainSelected,
      totalCount: domainTotal,
      percentage:
        domainTotal > 0 ? Math.round((domainSelected / domainTotal) * 100) : 0,
    };
  });
}
