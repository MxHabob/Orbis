/**
 * Orbis IAM Permissions System (Single Source of Truth)
 * 
 * Maps directly to backend `orbis_domain::iam::permission::Permission`.
 * Provides full bilingual metadata (Arabic/English), domain groupings,
 * resources, actions, system role presets, and utility helpers.
 */

export interface IamActionDef {
  key: string;
  labelEn: string;
  labelAr: string;
  descEn?: string;
  descAr?: string;
}

export interface IamResourceDef {
  key: string;
  labelEn: string;
  labelAr: string;
  descEn: string;
  descAr: string;
  actions: IamActionDef[];
}

export interface IamDomainDef {
  id: string;
  nameEn: string;
  nameAr: string;
  descEn: string;
  descAr: string;
  iconName: string;
  resources: IamResourceDef[];
}

export interface SystemRoleDef {
  id: string;
  nameEn: string;
  nameAr: string;
  descEn: string;
  descAr: string;
  iconName: string;
  color: string;
  isSystem: boolean;
  defaultPermissions: Record<string, string[]>;
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

  // --- Promotions ---
  PromotionRead: "promotion:read",
  PromotionWrite: "promotion:write",
  PromotionExecute: "promotion:execute",

  // --- Special System Roles ---
  AutopilotBypass: "system:autopilot",
} as const;

export type Permission = typeof Permissions[keyof typeof Permissions];

// ============================================================================
// 1. Unified IAM Domain & Resource Catalog
// ============================================================================

export const IAM_DOMAINS: IamDomainDef[] = [
  {
    id: "iam",
    nameEn: "Identity & Organization",
    nameAr: "الهوية والهيكل الإداري",
    descEn: "Organization configuration, user accounts, roles, and administrative branches.",
    descAr: "تهيئة المنظمة، حسابات المستخدمين، الأدوار، والفروع الإدارية.",
    iconName: "Shield",
    resources: [
      {
        key: "organization",
        labelEn: "Organization Settings",
        labelAr: "إعدادات المنظمة",
        descEn: "Core organization profile, metadata, and branding",
        descAr: "بيانات المنظمة الأساسية، الهوية، والتهيئة العامة",
        actions: [
          { key: "read", labelEn: "View Organization", labelAr: "عرض المنظمة" },
          { key: "write", labelEn: "Update Settings", labelAr: "تعديل الإعدادات" },
        ],
      },
      {
        key: "org_unit",
        labelEn: "Org Units & Branches",
        labelAr: "الوحدات والفروع",
        descEn: "Campuses, branches, faculties, and departmental units",
        descAr: "الفروع، المباني، والكليات والوحدات الإدارية",
        actions: [
          { key: "read", labelEn: "View Units", labelAr: "عرض الفروع" },
          { key: "write", labelEn: "Create / Edit Units", labelAr: "إنشاء وتعديل الفروع" },
          { key: "delete", labelEn: "Delete Units", labelAr: "حذف الفروع" },
        ],
      },
      {
        key: "user",
        labelEn: "Users & Staff Accounts",
        labelAr: "المستخدمين والموظفين",
        descEn: "Staff, teachers, admins, and student user accounts",
        descAr: "حسابات الكادر الإداري، المعلمين، والطلاب",
        actions: [
          { key: "read", labelEn: "View Users", labelAr: "عرض المستخدمين" },
          { key: "write", labelEn: "Create / Edit Users", labelAr: "إنشاء وتعديل الحسابات" },
          { key: "delete", labelEn: "Suspend / Delete", labelAr: "تعطيل وحذف الحسابات" },
          { key: "role_write", labelEn: "Assign Roles", labelAr: "تعيين وتعديل الأدوار" },
          { key: "password_write", labelEn: "Reset Password", labelAr: "إعادة ضبط كلمة المرور" },
        ],
      },
    ],
  },
  {
    id: "academic",
    nameEn: "Academics & Scheduling",
    nameAr: "الشؤون الأكاديمية والجدول",
    descEn: "Academic calendars, periods, classes, subjects, and timetables.",
    descAr: "التقويم الأكاديمي، الفصول، المواد، والشعب والجداول الدراسية.",
    iconName: "GraduationCap",
    resources: [
      {
        key: "period",
        labelEn: "Academic Periods",
        labelAr: "الفصول الأكاديمية",
        descEn: "Semesters, terms, enrollment windows, and period locking",
        descAr: "الفصول الدراسية، نوافذ القيد، وقفل الفترات",
        actions: [
          { key: "read", labelEn: "View Periods", labelAr: "عرض الفصول الأكاديمية" },
          { key: "write", labelEn: "Create / Edit Periods", labelAr: "إنشاء وتعديل الفصول" },
          { key: "activate", labelEn: "Activate / Open Enrollment", labelAr: "تفعيل وفتح القيد" },
          { key: "lock", labelEn: "Lock Grades & Period", labelAr: "قفل الدرجات والفترة" },
          { key: "unlock", labelEn: "Unlock Period", labelAr: "إلغاء قفل الفترة" },
        ],
      },
      {
        key: "class",
        labelEn: "Classes & Sections",
        labelAr: "الفصول والشعب",
        descEn: "Grade levels, classes, section capacity, and homeroom assignments",
        descAr: "الصفوف الدراسية، الشعب، السعة، وتعيين رواد الفصول",
        actions: [
          { key: "read", labelEn: "View Classes", labelAr: "عرض الصفوف والشعب" },
          { key: "write", labelEn: "Create / Edit Classes", labelAr: "إنشاء وتعديل الشعب" },
          { key: "delete", labelEn: "Delete Classes", labelAr: "حذف الصفوف والشعب" },
        ],
      },
      {
        key: "subject",
        labelEn: "Subjects & Curriculum",
        labelAr: "المواد والمناهج",
        descEn: "Courses, subjects, credit weights, and syllabus",
        descAr: "المواد الدراسية، الأوزان النسبية، والخطط الدراسية",
        actions: [
          { key: "read", labelEn: "View Subjects", labelAr: "عرض المواد" },
          { key: "write", labelEn: "Create / Edit Subjects", labelAr: "إنشاء وتعديل المواد" },
          { key: "delete", labelEn: "Delete Subjects", labelAr: "حذف المواد" },
        ],
      },
      {
        key: "schedule",
        labelEn: "Timetables & Slots",
        labelAr: "الجداول والحصص",
        descEn: "Lesson slots, teacher timetable allocations, and room bookings",
        descAr: "الحصص اليومية، توزيع جداول المعلمين، والقاعات",
        actions: [
          { key: "read", labelEn: "View Schedules", labelAr: "عرض الجداول الدراسية" },
          { key: "write", labelEn: "Build / Edit Schedule", labelAr: "بناء وتعديل الجداول" },
          { key: "delete", labelEn: "Clear Schedule", labelAr: "مسح وتفريغ الجداول" },
        ],
      },
    ],
  },
  {
    id: "students",
    nameEn: "Students & Admissions",
    nameAr: "شؤون الطلاب والقبول",
    descEn: "Student profiles, guardians, enrollment lifecycle, and cohort promotions.",
    descAr: "سجلات الطلاب، أولياء الأمور، القيد والتسجيل، والترقية والنقل.",
    iconName: "Users",
    resources: [
      {
        key: "student",
        labelEn: "Student Profiles",
        labelAr: "سجلات الطلاب",
        descEn: "Demographics, personal files, admission records, and identification",
        descAr: "البيانات الشخصية، الملف الأكاديمي، وسجلات القبول",
        actions: [
          { key: "read", labelEn: "View Profiles", labelAr: "عرض ملفات الطلاب" },
          { key: "create", labelEn: "Admit New Student", labelAr: "تسجيل طالب جديد" },
          { key: "update", labelEn: "Update Profile", labelAr: "تحديث بيانات الطالب" },
          { key: "delete", labelEn: "Archive / Remove", labelAr: "أرشفة وحذف الطالب" },
          { key: "write", labelEn: "Full Student Write", labelAr: "إدارة شاملة لملف الطالب" },
        ],
      },
      {
        key: "guardian",
        labelEn: "Guardians & Contacts",
        labelAr: "أولياء الأمور",
        descEn: "Parents, legal guardians, emergency contacts, and portal links",
        descAr: "أولياء الأمور، جهات الاتصال للطوارئ، والربط بالبوابة",
        actions: [
          { key: "read", labelEn: "View Guardians", labelAr: "عرض أولياء الأمور" },
          { key: "write", labelEn: "Link / Edit Guardians", labelAr: "ربط وتعديل أولياء الأمور" },
          { key: "delete", labelEn: "Unlink Guardian", labelAr: "فك ارتباط ولي الأمر" },
        ],
      },
      {
        key: "enrollment",
        labelEn: "Enrollments & Status",
        labelAr: "القيد وحالات الدراسة",
        descEn: "Active enrollments, section transfers, and study status",
        descAr: "حالات القيد، النقل بين الشعب، وإيقاف القيد",
        actions: [
          { key: "read", labelEn: "View Enrollments", labelAr: "عرض قيود الطلاب" },
          { key: "write", labelEn: "Update Status", labelAr: "تعديل حالة القيد" },
          { key: "promote", labelEn: "Promote Students", labelAr: "ترقية الطلاب للفصل التالي" },
          { key: "transfer", labelEn: "Transfer Section", labelAr: "نقل الطالب بين الفصول" },
        ],
      },
      {
        key: "promotion",
        labelEn: "Year-End Promotions",
        labelAr: "الترحيل السنوي العام",
        descEn: "Cohort batch progression, graduation, and retention rules",
        descAr: "ترحيل الدفعات السنوي، التخرج، وقواعد الإعادة",
        actions: [
          { key: "read", labelEn: "View Progression Plans", labelAr: "عرض خطط الترحيل" },
          { key: "write", labelEn: "Configure Promotion Rules", labelAr: "ضبط قواعد الترحيل" },
          { key: "execute", labelEn: "Execute Batch Promotion", labelAr: "تنفيذ الترحيل النهائي" },
        ],
      },
    ],
  },
  {
    id: "assessment",
    nameEn: "Assessments & Attendance",
    nameAr: "التقييم والدرجات والحضور",
    descEn: "Gradebook, assessment components, attendance tracking, and behavior notes.",
    descAr: "دفتر الدرجات، الاختبارات الدورية، تسجيل الحضور، وملاحظات السلوك.",
    iconName: "FileCheck",
    resources: [
      {
        key: "assessment",
        labelEn: "Assessment Schemes",
        labelAr: "مخططات التقييم",
        descEn: "Grading scales, rubrics, weightings, and exam schemes",
        descAr: "سلالم الدرجات، الأوزان النسبية، وأنظمة التقييم",
        actions: [
          { key: "read", labelEn: "View Schemes", labelAr: "عرض مخططات التقييم" },
          { key: "write", labelEn: "Create / Edit Schemes", labelAr: "إنشاء وتعديل المخططات" },
          { key: "delete", labelEn: "Delete Schemes", labelAr: "حذف المخططات" },
        ],
      },
      {
        key: "grade",
        labelEn: "Grades & GPA",
        labelAr: "رصد الدرجات والمعدلات",
        descEn: "Gradebook entry, GPA calculations, rankings, and transcripts",
        descAr: "رصد الدرجات، احتساب المعدلات التراكمية، وكشوف العلامات",
        actions: [
          { key: "read", labelEn: "View Grades", labelAr: "عرض الدرجات" },
          { key: "write", labelEn: "Record / Edit Grades", labelAr: "رصد وتعديل الدرجات" },
          { key: "entry_manage", labelEn: "Manage Grade Entry Deadlines", labelAr: "إدارة فترات الرصد" },
          { key: "analytics", labelEn: "Compute GPA & Stats", labelAr: "احتساب المعدلات والتحليلات" },
          { key: "report", labelEn: "Generate Grade Transcripts", labelAr: "إصدار الشهادات وكشوف الدرجات" },
        ],
      },
      {
        key: "attendance",
        labelEn: "Daily & Lesson Attendance",
        labelAr: "الحضور والغياب",
        descEn: "Daily absence tracking, lesson attendance, and absence excuses",
        descAr: "تسجيل الغياب اليومي، حضور الحصص، وأعذار الغياب",
        actions: [
          { key: "read", labelEn: "View Attendance", labelAr: "عرض سجلات الحضور" },
          { key: "write", labelEn: "Take Attendance", labelAr: "تسجيل وتعديل الحضور" },
          { key: "analytics", labelEn: "Attendance Analytics", labelAr: "تحليلات ونسب الحضور" },
          { key: "report", labelEn: "Attendance Reports", labelAr: "إصدار تقارير الغياب" },
        ],
      },
      {
        key: "behavior",
        labelEn: "Behavior & Conduct",
        labelAr: "السلوك والمخالفات",
        descEn: "Positive merits, disciplinary infractions, and AI conduct insights",
        descAr: "النقاط الإيجابية، المخالفات السلوكية، وتحليلات السلوك الذكية",
        actions: [
          { key: "read", labelEn: "View Behavior Logs", labelAr: "عرض سجلات السلوك" },
          { key: "write", labelEn: "Record Incident / Merit", labelAr: "تسجيل مخالفة أو تميز" },
          { key: "delete", labelEn: "Remove Behavior Entry", labelAr: "حذف قيد سلوكي" },
          { key: "ai", labelEn: "AI Behavioral Recommendations", labelAr: "تحليلات الذكاء الاصطناعي للسلوك" },
        ],
      },
    ],
  },
  {
    id: "examinations",
    nameEn: "Examinations & Control",
    nameAr: "كنترول الامتحانات واللجان",
    descEn: "Exam sessions, seating arrangements, secret numbering, halls, and proctors.",
    descAr: "جلسات الامتحانات، أرقام الجلوس، الأرقام السرية، القاعات، والمراقبين.",
    iconName: "Lock",
    resources: [
      {
        key: "exam_session",
        labelEn: "Exam Sessions",
        labelAr: "جلسات الامتحانات",
        descEn: "Midterm and final exam period configurations",
        descAr: "إعدادات فترات وجلسات الامتحانات النصفية والنهائية",
        actions: [
          { key: "read", labelEn: "View Exam Sessions", labelAr: "عرض جلسات الامتحانات" },
          { key: "write", labelEn: "Create / Edit Session", labelAr: "إنشاء وتعديل الجلسات" },
          { key: "activate", labelEn: "Activate Session", labelAr: "بدء وتفعيل الجلسة" },
          { key: "delete", labelEn: "Delete Session", labelAr: "حذف الجلسة" },
        ],
      },
      {
        key: "seating_number",
        labelEn: "Seating Numbers",
        labelAr: "أرقام الجلوس",
        descEn: "Automated student seating number generation and distribution",
        descAr: "توليد وتوزيع أرقام جلوس الطلاب آلياً",
        actions: [
          { key: "read", labelEn: "View Seating Numbers", labelAr: "عرض أرقام الجلوس" },
          { key: "generate", labelEn: "Generate Seating Numbers", labelAr: "توليد أرقام الجلوس" },
        ],
      },
      {
        key: "secret_number",
        labelEn: "Secret Numbers",
        labelAr: "الأرقام السرية (الكنترول)",
        descEn: "Blind grading anonymous barcode/secret number mapping",
        descAr: "نظام التصحيح السري وتوليد الأكواد المشفرة",
        actions: [
          { key: "read", labelEn: "View Secret Numbers", labelAr: "عرض قائمة الأرقام السرية" },
          { key: "generate", labelEn: "Generate Control Numbers", labelAr: "توليد الأرقام السرية" },
          { key: "reveal", labelEn: "Reveal Secret Mappings (Supervisors)", labelAr: "كشف المطابقة السرية (للكنترول)" },
        ],
      },
      {
        key: "exam_hall",
        labelEn: "Exam Halls & Seating Grid",
        labelAr: "لجان وقاعات الامتحانات",
        descEn: "Hall capacities, desk matrix, and seat allocation algorithms",
        descAr: "سعة القاعات، شبكة المقاعد، وتوزيع الطلاب آلياً",
        actions: [
          { key: "read", labelEn: "View Exam Halls", labelAr: "عرض لجان الامتحانات" },
          { key: "write", labelEn: "Configure Halls & Desks", labelAr: "تهيئة القاعات والمقاعد" },
          { key: "allocate", labelEn: "Run Auto-Allocation", labelAr: "تنفيذ التوزيع الآلي للطلاب" },
        ],
      },
      {
        key: "proctor",
        labelEn: "Proctors & Supervisors",
        labelAr: "المراقبين والمشرفين",
        descEn: "Invigilator duty rosters, hall supervision, and swap management",
        descAr: "جداول مراقبة الامتحانات، مشرفي اللجان، وتبديل النوبات",
        actions: [
          { key: "read", labelEn: "View Proctors", labelAr: "عرض جدول المراقبة" },
          { key: "assign", labelEn: "Assign Proctors to Halls", labelAr: "توزيع المراقبين على اللجان" },
        ],
      },
      {
        key: "student_order",
        labelEn: "Student Roster Ordering",
        labelAr: "ترتيب الطلاب وأبجدية الكنترول",
        descEn: "Alphabetical sorting, seating serialization, and roster freezing",
        descAr: "الفرز الأبجدي، تسلسل الكنترول، وتجميد القوائم",
        actions: [
          { key: "read", labelEn: "View Ordering", labelAr: "عرض ترتيب الطلاب" },
          { key: "rebuild", labelEn: "Rebuild Ordering Algorithm", labelAr: "إعادة بناء الترتيب" },
          { key: "freeze", labelEn: "Freeze Final Roster", labelAr: "تجميد القائمة النهائية" },
        ],
      },
      {
        key: "exam_schedule",
        labelEn: "Exam Timetable",
        labelAr: "جدول مواعيد الامتحانات",
        descEn: "Exam dates, time slots, and room allocations",
        descAr: "مواعيد الاختبارات، الفترات الزمنية، وتخصيص القاعات",
        actions: [
          { key: "read", labelEn: "View Exam Timetable", labelAr: "عرض جدول الامتحانات" },
          { key: "write", labelEn: "Build / Edit Exam Schedule", labelAr: "إنشاء وتعديل جدول الامتحانات" },
        ],
      },
    ],
  },
  {
    id: "finance",
    nameEn: "Finance & Accounts",
    nameAr: "المالية والرسوم",
    descEn: "Tuition fee structures, payments, invoices, discounts, and transactions.",
    descAr: "هياكل الرسوم الدراسية، سندات القبض، الفواتير، الخصومات، والعمليات المالية.",
    iconName: "Wallet",
    resources: [
      {
        key: "finance",
        labelEn: "Finance & Billing Ledger",
        labelAr: "النظام المالي والرسوم",
        descEn: "General ledger, fee templates, student fee balances, and payment processing",
        descAr: "شجرة الحسابات، قوالب الرسوم، حسابات الطلاب، ومعالجة الدفعات",
        actions: [
          { key: "read", labelEn: "View Financial Records", labelAr: "عرض السجلات المالية والرسوم" },
          { key: "write", labelEn: "Create / Edit Fees & Invoices", labelAr: "إنشاء وتعديل الرسوم والفواتير" },
          { key: "payment", labelEn: "Process Payments & Receipts", labelAr: "تحصيل الدفعات وإصدار السندات" },
        ],
      },
    ],
  },
  {
    id: "documents",
    nameEn: "Documents & Templates",
    nameAr: "الوثائق وقوالب التقارير",
    descEn: "Document generation, certificate templates, assets, and export tools.",
    descAr: "توليد الشهادات، تصميم قوالب التقارير، رفع الأصول، والتصدير.",
    iconName: "Layers",
    resources: [
      {
        key: "document",
        labelEn: "Official Documents",
        labelAr: "الوثائق والشهادات",
        descEn: "Generated certificates, letters of enrollment, transcripts",
        descAr: "الشهادات المعتمدة، خطابات القيد، والمستندات الرسمية",
        actions: [
          { key: "read", labelEn: "View Documents", labelAr: "عرض الوثائق والشهادات" },
          { key: "write", labelEn: "Generate / Issue Documents", labelAr: "إصدار وتعديل الوثائق" },
          { key: "delete", labelEn: "Revoke / Delete Document", labelAr: "إلغاء وحذف الوثائق" },
        ],
      },
      {
        key: "template",
        labelEn: "DocForge Templates",
        labelAr: "قوالب DocForge",
        descEn: "Visual template editor, report schemas, component library, and asset store",
        descAr: "محرر القوالب المرئي، تصاميم الشهادات، المكونات، ومخزن الأصول",
        actions: [
          { key: "read", labelEn: "View Templates", labelAr: "عرض القوالب" },
          { key: "write", labelEn: "Design / Edit Templates", labelAr: "تصميم وتعديل القوالب" },
          { key: "publish", labelEn: "Publish Template", labelAr: "نشر واعتماد القالب" },
          { key: "delete", labelEn: "Delete Template", labelAr: "حذف القالب" },
          { key: "import", labelEn: "Import Template Schema", labelAr: "استيراد قالب" },
          { key: "export", labelEn: "Export Template Schema", labelAr: "تصدير قالب" },
          { key: "generate_single", labelEn: "Generate Single Preview", labelAr: "توليد معاينة فردية" },
          { key: "generate_batch", labelEn: "Run Batch Generation", labelAr: "توليد دفعات جماعية" },
          { key: "generate_download", labelEn: "Download Generated PDFs", labelAr: "تحميل ملفات PDF" },
          { key: "asset_upload", labelEn: "Upload Assets & Logos", labelAr: "رفع الصور والشعارات" },
          { key: "asset_delete", labelEn: "Delete Assets", labelAr: "حذف الأصول" },
          { key: "component_write", labelEn: "Create Custom Components", labelAr: "إنشاء وتعديل المكونات" },
          { key: "component_delete", labelEn: "Delete Custom Components", labelAr: "حذف المكونات" },
        ],
      },
    ],
  },
  {
    id: "system",
    nameEn: "Workflows & Operations",
    nameAr: "سير العمليات والتشغيل",
    descEn: "Multi-level approval workflows, audit trails, data import/export, and autopilot.",
    descAr: "مسارات الاعتماد والموافقات، سجل التدقيق، الاستيراد والتصدير، والأتمتة.",
    iconName: "Sparkles",
    resources: [
      {
        key: "workflow",
        labelEn: "Approval Workflows",
        labelAr: "مسارات الاعتماد والموافقات",
        descEn: "Multi-stage requests, sign-offs, and administrative routing",
        descAr: "طلبات الاعتماد متعددة المستويات والتوقيعات الإدارية",
        actions: [
          { key: "read", labelEn: "View Workflows", labelAr: "عرض مسارات الاعتماد" },
          { key: "write", labelEn: "Configure Workflow Steps", labelAr: "إنشاء وضبط مسار الاعتماد" },
          { key: "approve", labelEn: "Approve / Reject Requests", labelAr: "الموافقة والاعتماد أو الرفض" },
          { key: "execute", labelEn: "Trigger / Execute Step", labelAr: "تنفيذ خطوات المسار" },
        ],
      },
      {
        key: "notification",
        labelEn: "System Notifications",
        labelAr: "الإشعارات والتنبيهات",
        descEn: "Broadcast alerts, SMS/Email dispatches, and event triggers",
        descAr: "إرسال التعاميم، رسائل SMS، وتنبيهات النظام",
        actions: [
          { key: "read", labelEn: "View Notifications", labelAr: "عرض الإشعارات" },
          { key: "write", labelEn: "Send / Broadcast Notice", labelAr: "إرسال وبث التعاميم" },
        ],
      },
      {
        key: "dashboard",
        labelEn: "Executive Dashboard",
        labelAr: "لوحة المؤشرات التنفيذية",
        descEn: "High-level KPI widgets, live attendance rates, and financial graphs",
        descAr: "المؤشرات العامة، إحصائيات الحضور، والرسوم البيانية",
        actions: [
          { key: "read", labelEn: "Access Executive Dashboard", labelAr: "الاطلاع على لوحة المؤشرات" },
        ],
      },
      {
        key: "audit",
        labelEn: "Audit Trail & Security Logs",
        labelAr: "سجل التدقيق والأمان",
        descEn: "Immutable security event logs, change history, and login records",
        descAr: "سجل العمليات الإدارية، تاريخ التعديلات، وجلسات الدخول",
        actions: [
          { key: "read", labelEn: "View Audit Logs", labelAr: "الاطلاع على سجل التدقيق" },
        ],
      },
      {
        key: "data",
        labelEn: "Data Migration & Sync",
        labelAr: "استيراد وتصدير البيانات",
        descEn: "Bulk Excel/CSV data importing, migration wizards, and backups",
        descAr: "استيراد وتصدير ملفات Excel/CSV والنسخ الاحتياطي",
        actions: [
          { key: "import", labelEn: "Import Bulk Data", labelAr: "استيراد البيانات الشاملة" },
          { key: "export", labelEn: "Export Full Data", labelAr: "تصدير البيانات الشاملة" },
        ],
      },
      {
        key: "system",
        labelEn: "System Autopilot Engine",
        labelAr: "محرك الأتمتة الذكي (Autopilot)",
        descEn: "Automated background tasks, constraints solver, and system overrides",
        descAr: "مهام الخلفية المؤتمتة، محرك الجداول، وتجاوز النظام",
        actions: [
          { key: "autopilot", labelEn: "Autopilot Bypass / Overrides", labelAr: "تشغيل وصلاحيات Autopilot" },
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
    nameEn: "Super Administrator",
    nameAr: "مدير النظام العام",
    descEn: "Unrestricted master access across all organizations, settings, security policies, and databases.",
    descAr: "صلاحيات وتحكم مطلق وشامل بجميع إعدادات المنظمات، والسياسات الأمنية، وقواعد البيانات.",
    iconName: "Crown",
    color: "amber",
    isSystem: true,
    defaultPermissions: {
      "*": ["*"],
    },
  },
  {
    id: "admin",
    nameEn: "Organization Admin",
    nameAr: "مدير المنظمة",
    descEn: "Manages all institutional operations, staff accounts, permissions, academic setups, and billing.",
    descAr: "إدارة كافة العمليات المؤسسية، حسابات الكادر، الصلاحيات، التهيئة الأكاديمية، والمالية.",
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
      finance: ["read", "write", "payment"],
      document: ["read", "write", "delete"],
      template: ["read", "write", "publish", "delete", "import", "export", "generate_single", "generate_batch", "generate_download", "asset_upload", "asset_delete", "component_write", "component_delete"],
      workflow: ["read", "write", "approve", "execute"],
      notification: ["read", "write"],
      dashboard: ["read"],
      audit: ["read"],
      data: ["import", "export"],
    },
  },
  {
    id: "principal",
    nameEn: "School Principal",
    nameAr: "مدير المدرسة / المنشأة",
    descEn: "Academic leadership, staff supervision, attendance and grade approvals, workflows, and school reports.",
    descAr: "القيادة الأكاديمية، الإشراف على المعلمين، اعتماد الدرجات والغياب، الموافقات الإدارية، والتقارير.",
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
      template: ["read", "generate_single", "generate_batch", "generate_download"],
      workflow: ["read", "approve", "execute"],
      notification: ["read", "write"],
      dashboard: ["read"],
      audit: ["read"],
    },
  },
  {
    id: "teacher",
    nameEn: "Teacher / Faculty",
    nameAr: "معلم / عضو هيئة تدريس",
    descEn: "Classroom instruction, daily attendance, gradebook entry, student conduct logging, and schedules.",
    descAr: "إدارة الحصص، رصد الحضور والغياب، إدخال الدرجات، تسجيل الملاحظات السلوكية، ومتابعة الجدول.",
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
    nameEn: "Academic Registrar",
    nameAr: "مسجل شؤون الطلاب",
    descEn: "Student admissions, document verifications, enrollments, section transfers, and cohort promotions.",
    descAr: "قبول وتسجيل الطلاب، تدقيق الوثائق، القيد والشعب، وتسكين ونقل الطلاب، وإصدار الشهادات.",
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
      template: ["read", "generate_single", "generate_batch", "generate_download"],
      notification: ["read", "write"],
    },
  },
  {
    id: "accountant",
    nameEn: "Financial Accountant",
    nameAr: "محاسب مالي",
    descEn: "Tuition billing, receipt vouchers, student ledgers, discount policies, and payment tracking.",
    descAr: "إدارة الرسوم الدراسية، سندات القبض والصرف، فواتير الطلاب، الخصومات، والتقارير المالية.",
    iconName: "Landmark",
    color: "emerald",
    isSystem: true,
    defaultPermissions: {
      student: ["read"],
      guardian: ["read"],
      enrollment: ["read"],
      finance: ["read", "write", "payment"],
      document: ["read", "write"],
      template: ["read", "generate_single", "generate_batch", "generate_download"],
      notification: ["read"],
    },
  },
  {
    id: "staff",
    nameEn: "Administrative Staff",
    nameAr: "كادر إداري",
    descEn: "Operational support, correspondence, student inquiries, and daily activity coordination.",
    descAr: "المساندة الإدارية والتشغيلية اليومية، المراسلات، الاستقبال، ومتابعة الأنشطة المدرسية.",
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
    nameEn: "Student",
    nameAr: "طالب",
    descEn: "Access to personal timetable, assessment scores, attendance history, and learning certificates.",
    descAr: "الاطلاع على الجدول الدراسي، نتائج التقييمات، سجل الحضور والغياب، والشهادات.",
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
    nameEn: "Parent / Guardian",
    nameAr: "ولي أمر",
    descEn: "Real-time monitoring of dependent children's academic performance, attendance, fees, and alerts.",
    descAr: "متابعة أداء الأبناء أكاديمياً، كشوف الدرجات، الحضور والغياب، الرسوم والمستحقات، والإشعارات.",
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
    { key: "read", labelEn: "View", labelAr: "عرض" },
    { key: "write", labelEn: "Edit", labelAr: "تعديل" },
  ];
}

/**
 * Finds a resource definition by key.
 */
export function getResourceDef(resourceKey: string): IamResourceDef | undefined {
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
export function getDomainByResource(resourceKey: string): IamDomainDef | undefined {
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
export function getResourceLabel(resourceKey: string, lang: "ar" | "en" = "ar"): string {
  const res = getResourceDef(resourceKey);
  if (!res) return resourceKey;
  return lang === "ar" ? res.labelAr : res.labelEn;
}

/**
 * Returns a localized label for an action within a resource.
 */
export function getActionLabel(resourceKey: string, actionKey: string, lang: "ar" | "en" = "ar"): string {
  const actions = getActionsForResource(resourceKey);
  const action = actions.find((a) => a.key === actionKey);
  if (!action) {
    return actionKey;
  }
  return lang === "ar" ? action.labelAr : action.labelEn;
}

/**
 * Returns a badge color variant based on the action key.
 */
export function getActionBadgeVariant(actionKey: string): "default" | "secondary" | "destructive" | "outline" | "lime" {
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
export function countRolePermissions(perms?: Record<string, string[]> | null, roleId?: string): number {
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
export function getRoleDomainStats(perms?: Record<string, string[]> | null, roleId?: string): Array<{
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
      percentage: domainTotal > 0 ? Math.round((domainSelected / domainTotal) * 100) : 0,
    };
  });
}
