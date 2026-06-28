<script lang="ts">
import { onMount } from "svelte";
import { fade, slide } from "svelte/transition";
import { Button } from "@orbis/ui-svelte/button";
import { Input } from "@orbis/ui-svelte/input";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { commands } from "$lib/services/ipc";

let { data }: { data: { orgId: string } } = $props();
import {
  Shield,
  Search,
  Plus,
  Users,
  Check,
  HelpCircle,
  FileText,
  RotateCcw,
  Sparkles,
  Bookmark,
  ChevronRight,
  ArrowLeft,
  BookOpen,
  DollarSign,
  UserCheck,
  Building,
  GraduationCap,
} from "lucide-svelte";

interface Permission {
  id: string;
  name: string;
  nameAr: string;
  description: string;
  descriptionAr: string;
  category: "academics" | "finance" | "users" | "students" | "reports";
  scope: "global" | "local" | "none";
}

interface Role {
  id: string;
  name: string;
  nameAr: string;
  description: string;
  descriptionAr: string;
  userCount: number;
  permissions: Record<string, "global" | "local" | "none">;
  isCustom?: boolean;
}

// Default Permissions Definition
const defaultPermissions: Permission[] = [
  // Organization
  {
    id: "OrganizationRead",
    name: "Read Organization Info",
    nameAr: "عرض معلومات المنشأة",
    description: "Read details of the organization.",
    descriptionAr: "الاطلاع على الهوية والشعار وبيانات المنشأة.",
    category: "users",
    scope: "global",
  },
  {
    id: "OrganizationWrite",
    name: "Write Organization Info",
    nameAr: "تعديل معلومات المنشأة",
    description: "Write details of the organization.",
    descriptionAr: "تحديث الهوية والشعار والعملة وإعدادات المنشأة العامة.",
    category: "users",
    scope: "global",
  },
  // Users
  {
    id: "UserRead",
    name: "Read Users Directory",
    nameAr: "استعراض دليل المستخدمين",
    description: "Browse and view user accounts in the directory.",
    descriptionAr: "الاطلاع على قائمة الموظفين والمعلمين والإداريين بالمنشأة.",
    category: "users",
    scope: "global",
  },
  {
    id: "UserWrite",
    name: "Create and Edit Users",
    nameAr: "إنشاء وتعديل حسابات المستخدمين",
    description: "Add new users or edit profile information.",
    descriptionAr: "صلاحية إضافة موظفين ومعلمين جدد وتحديث ملفاتهم وبياناتهم.",
    category: "users",
    scope: "global",
  },
  {
    id: "UserDelete",
    name: "Delete Users",
    nameAr: "حذف حسابات المستخدمين",
    description: "Permanently delete or archive user accounts.",
    descriptionAr: "حذف حسابات الموظفين السابقين لمنع دخولهم للنظام.",
    category: "users",
    scope: "global",
  },
  {
    id: "UserRoleWrite",
    name: "Modify User Roles",
    nameAr: "تعديل أدوار ومجموعات المستخدمين",
    description: "Assign security roles to users.",
    descriptionAr: "تغيير المسمى الوظيفي ودور المستخدم ومستوى صلاحياته.",
    category: "users",
    scope: "global",
  },
  {
    id: "UserPasswordWrite",
    name: "Reset User Passwords",
    nameAr: "إعادة تعيين كلمات مرور المستخدمين",
    description: "Change user passwords.",
    descriptionAr: "القدرة على إعادة تعيين كلمات سر الحسابات في حال نسيانها.",
    category: "users",
    scope: "global",
  },
  // Academic Cycle
  {
    id: "CycleRead",
    name: "Read Academic Cycles",
    nameAr: "عرض الفترات والأعوام الدراسية",
    description: "View terms, semesters, and years.",
    descriptionAr:
      "الاطلاع على الأعوام الدراسية والفصول والفترات الزمنية المجدولة.",
    category: "academics",
    scope: "global",
  },
  {
    id: "CycleWrite",
    name: "Create and Edit Cycles",
    nameAr: "إنشاء وتعديل الفترات والأعوام",
    description: "Modify structure of semesters and terms.",
    descriptionAr: "صلاحية إضافة أو تعديل الفصول والأعوام الدراسية وتواريخها.",
    category: "academics",
    scope: "global",
  },
  {
    id: "CycleActivate",
    name: "Activate Academic Cycle",
    nameAr: "تفعيل الفترة الدراسية الحالية",
    description: "Set the currently active academic term.",
    descriptionAr:
      "تحديد العام أو الفصل الدراسي الحالي ليكون هو المعتمد بالعمليات.",
    category: "academics",
    scope: "global",
  },
  {
    id: "CycleLock",
    name: "Lock Academic Cycles",
    nameAr: "إقفال الفترة الدراسية",
    description: "Prevent grading modifications by locking cycle.",
    descriptionAr:
      "إقفال الفصل الدراسي لمنع تعديل أو رصد درجات إضافية بعد انتهاء الرصد.",
    category: "academics",
    scope: "global",
  },
  {
    id: "CycleUnlock",
    name: "Unlock Academic Cycles",
    nameAr: "إعادة فتح الفترة الدراسية",
    description: "Re-open locked cycle for changes.",
    descriptionAr:
      "إعادة فتح فصل مقفل للسماح بتعديل درجات استثنائية لبعض الطلاب.",
    category: "academics",
    scope: "global",
  },
  {
    id: "GradeEntryManage",
    name: "Manage Grade Entry Windows",
    nameAr: "إدارة فترات رصد الدرجات",
    description: "Set start and end dates for grade entries.",
    descriptionAr:
      "تحديد الفترات الزمنية المسموح فيها للمعلمين برصد درجات الطلاب.",
    category: "academics",
    scope: "global",
  },
  // Org Units
  {
    id: "OrgUnitRead",
    name: "View Organization Units",
    nameAr: "عرض التقسيمات الإدارية والمباني",
    description: "Browse school departments, buildings, and rooms.",
    descriptionAr:
      "الاطلاع على أقسام المدرسة الإدارية، المباني، القاعات والمختبرات.",
    category: "academics",
    scope: "global",
  },
  {
    id: "OrgUnitWrite",
    name: "Manage Org Units",
    nameAr: "إدارة وتعديل التقسيمات الإدارية والمباني",
    description: "Create or edit buildings and rooms.",
    descriptionAr: "إضافة أو تعديل المباني المدرسية وتوزيع القاعات الدراسية.",
    category: "academics",
    scope: "global",
  },
  {
    id: "OrgUnitDelete",
    name: "Delete Org Units",
    nameAr: "حذف التقسيمات الإدارية والمباني",
    description: "Remove buildings or departments.",
    descriptionAr: "حذف مبنى أو مختبر أو قسم إداري من هيكل المنظمة.",
    category: "academics",
    scope: "global",
  },
  // Classes
  {
    id: "ClassRead",
    name: "View Classes and Divisions",
    nameAr: "عرض الفصول والشعب الدراسية",
    description: "View classroom divisions and levels.",
    descriptionAr:
      "الاطلاع على الفصول الدراسية والشعب ومستوياتها وأعداد طلابها.",
    category: "academics",
    scope: "global",
  },
  {
    id: "ClassWrite",
    name: "Manage Classes and Divisions",
    nameAr: "إدارة وتعديل الفصول والشعب",
    description: "Create or modify classes and divisions.",
    descriptionAr: "إضافة فصول جديدة أو توزيع الطلاب على شعب دراسية مختلفة.",
    category: "academics",
    scope: "global",
  },
  {
    id: "ClassDelete",
    name: "Delete Classes",
    nameAr: "حذف الفصول والشعب",
    description: "Remove classes or sections.",
    descriptionAr: "إزالة فصل دراسي أو شعبة من النظام.",
    category: "academics",
    scope: "global",
  },
  // Subjects
  {
    id: "SubjectRead",
    name: "View Subjects Curriculum",
    nameAr: "عرض المناهج والمواد الدراسية",
    description: "Browse active subjects list.",
    descriptionAr: "الاطلاع على المواد الدراسية المقررة والمناهج.",
    category: "academics",
    scope: "global",
  },
  {
    id: "SubjectWrite",
    name: "Manage Subjects Curriculum",
    nameAr: "إدارة وتعديل المواد الدراسية",
    description: "Create or edit school subjects.",
    descriptionAr:
      "إضافة مادة دراسية جديدة أو تعديل توزيع درجات المواد والرموز الخاصة بها.",
    category: "academics",
    scope: "global",
  },
  {
    id: "SubjectDelete",
    name: "Delete Subjects",
    nameAr: "حذف المواد الدراسية",
    description: "Remove subjects from curriculum.",
    descriptionAr: "حذف مادة دراسية من قائمة المناهج المعتمدة.",
    category: "academics",
    scope: "global",
  },
  // Schedule
  {
    id: "ScheduleRead",
    name: "View Timetable Schedules",
    nameAr: "عرض الجدول المدرسي واليومي",
    description: "View daily schedules for classes and teachers.",
    descriptionAr:
      "الاطلاع على توزيع الحصص اليومي والجدول الدراسي للمعلمين والطلاب.",
    category: "academics",
    scope: "global",
  },
  {
    id: "ScheduleWrite",
    name: "Manage Timetable Schedules",
    nameAr: "إدارة وتعديل الجدول المدرسي",
    description: "Create or modify school schedules.",
    descriptionAr:
      "توزيع وإعداد الحصص اليومية وتعيين حصص المعلمين وقاعات الحصص.",
    category: "academics",
    scope: "global",
  },
  {
    id: "ScheduleDelete",
    name: "Delete Schedules",
    nameAr: "حذف وتصفية الجداول",
    description: "Clear or reset academic schedule.",
    descriptionAr: "مسح الجدول الدراسي بالكامل أو إعادة تصفيره للبدء من جديد.",
    category: "academics",
    scope: "global",
  },
  // Assessments
  {
    id: "AssessmentRead",
    name: "View Student Assessments",
    nameAr: "عرض التقييمات والاختبارات القصيرة",
    description: "View homeworks, quizzes, and projects.",
    descriptionAr:
      "الاطلاع على الاختبارات الدورية، الواجبات والأنشطة الأكاديمية.",
    category: "academics",
    scope: "global",
  },
  {
    id: "AssessmentWrite",
    name: "Manage Student Assessments",
    nameAr: "إدارة وتعديل التقييمات والواجبات",
    description: "Create or edit quizzes and assignment rules.",
    descriptionAr:
      "إضافة واجب شهري أو إعداد اختبارات قصيرة وتحديد أوزانها النسبية.",
    category: "academics",
    scope: "global",
  },
  {
    id: "AssessmentDelete",
    name: "Delete Assessments",
    nameAr: "حذف التقييمات والواجبات",
    description: "Remove homeworks or quizzes.",
    descriptionAr: "إزالة واجب أو نشاط تم التقييم عليه من كشوف الدرجات.",
    category: "academics",
    scope: "global",
  },
  // Guardians
  {
    id: "GuardianRead",
    name: "View Guardians Directory",
    nameAr: "عرض بيانات أولياء الأمور",
    description: "Browse student parents and contacts.",
    descriptionAr:
      "الاطلاع على معلومات الاتصال الخاصة بأولياء الأمور وبياناتهم الشخصية.",
    category: "students",
    scope: "global",
  },
  {
    id: "GuardianWrite",
    name: "Manage Guardians Details",
    nameAr: "إدارة وتعديل بيانات أولياء الأمور",
    description: "Add or modify parent account details.",
    descriptionAr: "تحديث أرقام هواتف أولياء الأمور أو ربطهم بحسابات طلاب جدد.",
    category: "students",
    scope: "global",
  },
  {
    id: "GuardianDelete",
    name: "Delete Guardians Records",
    nameAr: "حذف سجلات أولياء الأمور",
    description: "Remove parent contact records.",
    descriptionAr: "إزالة سجل ولي أمر من قاعدة بيانات المنشأة.",
    category: "students",
    scope: "global",
  },
  // Documents
  {
    id: "DocumentRead",
    name: "View Student Documents",
    nameAr: "عرض الوثائق والمرفقات للطلاب",
    description: "Read certificates and registration files.",
    descriptionAr:
      "الاطلاع على الشهادات المرفوعة، وصور الهويات وشهادات الميلاد للطلاب.",
    category: "students",
    scope: "global",
  },
  {
    id: "DocumentWrite",
    name: "Manage Student Documents",
    nameAr: "إضافة وتعديل الوثائق والملفات",
    description: "Upload and modify document files.",
    descriptionAr: "رفع شهادات ميلاد، بطاقات عائلية، أو شهادات نقل للطلاب.",
    category: "students",
    scope: "global",
  },
  {
    id: "DocumentDelete",
    name: "Delete Documents",
    nameAr: "حذف وثائق الطلاب المرفقة",
    description: "Permanently delete uploaded documents.",
    descriptionAr: "إزالة ملفات ووثائق مرفوعة بالخطأ من سجلات الطلاب.",
    category: "students",
    scope: "global",
  },
  // Enrollments
  {
    id: "EnrollmentRead",
    name: "View Student Enrollments",
    nameAr: "عرض عمليات تسجيل الطلاب",
    description: "Browse registered and active students list.",
    descriptionAr: "الاطلاع على سجلات قبول الطلاب ووضعهم الدراسي الحالي.",
    category: "students",
    scope: "global",
  },
  {
    id: "EnrollmentWrite",
    name: "Manage Student Enrollments",
    nameAr: "تسجيل وقبول الطلاب الجدد",
    description: "Enroll new students to academic terms.",
    descriptionAr: "قبول وتوثيق الطلاب الجدد وتوليد أرقامهم الأكاديمية.",
    category: "students",
    scope: "global",
  },
  {
    id: "EnrollmentPromote",
    name: "Promote Students to Next Level",
    nameAr: "ترقية الطلاب وتوزيعهم السنوي",
    description: "Advance students to next class level.",
    descriptionAr:
      "إجراء ترحيل الطلاب الناجحين وتوزيعهم على صفوف العام القادم.",
    category: "students",
    scope: "global",
  },
  {
    id: "EnrollmentTransfer",
    name: "Transfer Students Between Classes",
    nameAr: "نقل وترحيل الطلاب بين الشعب",
    description: "Move students between classes or levels.",
    descriptionAr: "نقل الطالب من شعبة دراسية إلى أخرى خلال الفصل الدراسي.",
    category: "students",
    scope: "global",
  },
  // Grades
  {
    id: "GradeRead",
    name: "Read Grade Reports",
    nameAr: "عرض كشوف درجات الطلاب",
    description: "View assessment results and averages.",
    descriptionAr: "الاطلاع على درجات أعمال السنة والدرجات النهائية للطلاب.",
    category: "academics",
    scope: "global",
  },
  {
    id: "GradeWrite",
    name: "Submit and Edit Grades",
    nameAr: "رصد وتعديل درجات الطلاب",
    description: "Input quiz scores and final marks.",
    descriptionAr: "إمكانية إدخال وتعديل درجات الاختبارات الشهرية والنهائية.",
    category: "academics",
    scope: "global",
  },
  {
    id: "GradeAnalytics",
    name: "View Grade Analytics",
    nameAr: "عرض تحليلات ومؤشرات الدرجات",
    description: "Access class averages and distribution metrics.",
    descriptionAr:
      "الاطلاع على منحنيات الأداء الأكاديمي، ونسب الرسوب والنجاح والـ GPA.",
    category: "academics",
    scope: "global",
  },
  {
    id: "GradeReport",
    name: "Generate Report Cards",
    nameAr: "استخراج الشهادات الرسمية للطلاب",
    description: "Build performance certificates for print.",
    descriptionAr: "إصدار وتجهيز شهادات النتائج الشهرية والفصلية لجميع الفصول.",
    category: "academics",
    scope: "global",
  },
  // Attendance
  {
    id: "AttendanceRead",
    name: "View Attendance Registry",
    nameAr: "عرض سجل الحضور والغياب",
    description: "View student delay and absence statistics.",
    descriptionAr: "الاطلاع على إحصائيات حضور وغياب وتأخر الطلاب اليومية.",
    category: "students",
    scope: "global",
  },
  {
    id: "AttendanceWrite",
    name: "Record Student Attendance",
    nameAr: "رصد الحضور والغياب اليومي",
    description: "Mark students as absent, late or present.",
    descriptionAr: "أخذ غياب وحضور الطلاب اليومي وتوثيق الأعذار الطبية.",
    category: "students",
    scope: "global",
  },
  {
    id: "AttendanceAnalytics",
    name: "View Attendance Analytics",
    nameAr: "عرض إحصائيات ونسب الحضور",
    description: "Access charts of absence rates.",
    descriptionAr: "تحليل معدلات الحضور والغياب على مستوى المدرسة والفصول.",
    category: "students",
    scope: "global",
  },
  {
    id: "AttendanceReport",
    name: "Generate Attendance Sheets",
    nameAr: "تصدير كشوف حضور وغياب الفصول",
    description: "Print lists of monthly attendance.",
    descriptionAr:
      "تصدير وطباعة تقارير الغياب الشهرية لأولياء الأمور والإدارة.",
    category: "students",
    scope: "global",
  },
  // Behavior
  {
    id: "BehaviorRead",
    name: "View Behavior Logs",
    nameAr: "عرض المخالفات وسلوكيات الطلاب",
    description: "View student behavioral points and infractions.",
    descriptionAr:
      "الاطلاع على تقارير الانضباط السلوكي والمخالفات المرصودة للطلاب.",
    category: "students",
    scope: "global",
  },
  {
    id: "BehaviorWrite",
    name: "Record Behavior Incidents",
    nameAr: "رصد وتوثيق المخالفات السلوكية",
    description: "Submit notes on behavioral issues.",
    descriptionAr: "تسجيل حالات عدم الانضباط أو توثيق سلوك إيجابي مميز للطلاب.",
    category: "students",
    scope: "global",
  },
  {
    id: "BehaviorDelete",
    name: "Delete Behavior Records",
    nameAr: "حذف سجلات المخالفات السلوكية",
    description: "Remove behavior infractions.",
    descriptionAr: "صلاحية إلغاء أو حذف إنذار سلوكي تم رصده لطالب.",
    category: "students",
    scope: "global",
  },
  {
    id: "BehaviorAI",
    name: "Access AI Behavior Insights",
    nameAr: "عرض تحليلات السلوك الذكية (AI)",
    description: "Review automated behavior analysis alerts.",
    descriptionAr:
      "عرض اقتراحات سلوكية وتحليلات آلية مدعومة بالذكاء الاصطناعي.",
    category: "students",
    scope: "global",
  },
  // Notifications
  {
    id: "NotificationRead",
    name: "Read Notifications Log",
    nameAr: "استعراض الإشعارات المرسلة والمستلمة",
    description: "View notification history.",
    descriptionAr: "الاطلاع على الرسائل الجماعية والتنبيهات المستلمة.",
    category: "users",
    scope: "global",
  },
  {
    id: "NotificationWrite",
    name: "Send Broad Notification Alerts",
    nameAr: "إرسال وتعميم الإشعارات الجماعية",
    description: "Broadcast alerts to students or staff.",
    descriptionAr:
      "إرسال رسائل جماعية وتنبيهات فورية لجميع الموظفين أو أولياء الأمور.",
    category: "users",
    scope: "global",
  },
  // Dashboard
  {
    id: "DashboardRead",
    name: "View Dashboard Overview",
    nameAr: "عرض لوحة القيادة والملخصات العامة",
    description: "Access statistics overview on dashboard.",
    descriptionAr:
      "الاطلاع على أعداد الطلاب والغياب والرسوم في شاشة الملخص الرئيسي.",
    category: "users",
    scope: "global",
  },
  // Audit Log
  {
    id: "AuditRead",
    name: "View System Audit Logs",
    nameAr: "عرض سجل التدقيق والرقابة الأمني",
    description: "Analyze actions done by users.",
    descriptionAr:
      "مراقبة سجل عمليات المستخدمين وتتبع الإضافات والتعديلات الأمنية.",
    category: "reports",
    scope: "global",
  },
  // Finance
  {
    id: "FinanceRead",
    name: "View Financial Records",
    nameAr: "عرض الحسابات والتقارير المالية",
    description: "Analyze accounting general ledger.",
    descriptionAr:
      "الاطلاع على كشوفات المقبوضات اليومية وميزان الحسابات للمنشأة.",
    category: "finance",
    scope: "global",
  },
  {
    id: "FinanceWrite",
    name: "Manage Finance Operations",
    nameAr: "إدارة الحسابات والمدفوعات والرسوم",
    description: "Record income and modify fee rules.",
    descriptionAr: "تسجيل تحصيل رسوم الطلاب، تحديد فئات الرسوم وإصدار السندات.",
    category: "finance",
    scope: "global",
  },
  // Import/Export
  {
    id: "ImportData",
    name: "Import Data",
    nameAr: "استيراد كشوفات الطلاب من إكسل",
    description: "Upload Excel sheets of users.",
    descriptionAr:
      "رفع قواعد البيانات الخارجية للطلاب وتسكينهم بالفصول تلقائياً.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExportData",
    name: "Export Raw Data",
    nameAr: "تصدير كشوفات البيانات الخام",
    description: "Download CSV files of database records.",
    descriptionAr:
      "تصدير وتنزيل كافة بيانات المدرسة المالية والأكاديمية بصيغة CSV.",
    category: "reports",
    scope: "global",
  },
  // Examination Management
  {
    id: "ExamSessionRead",
    name: "View Exam Sessions",
    nameAr: "عرض دورات الامتحانات المعتمدة",
    description: "Browse official exam periods.",
    descriptionAr: "الاطلاع على فترات الامتحانات المجدولة وجداول اللجان.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamSessionWrite",
    name: "Manage Exam Sessions",
    nameAr: "إدارة وتوزيع دورات الامتحانات",
    description: "Create and structure exam periods.",
    descriptionAr:
      "إضافة دورة امتحانات وتحديد المواد المشمولة وتواريخ انطلاقها.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamSessionActivate",
    name: "Activate Exam Sessions",
    nameAr: "تفعيل فترات الامتحانات رسمياً",
    description: "Commit exam sessions to execution.",
    descriptionAr: "إطلاق الامتحانات ووضعها قيد التنفيذ النشط بالمدرسة.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamSessionDelete",
    name: "Delete Exam Sessions",
    nameAr: "إلغاء وحذف دورات الامتحانات",
    description: "Cancel and remove exam sessions.",
    descriptionAr: "إلغاء فترة اختبار مقررة وتوزيعات اللجان الخاصة بها.",
    category: "reports",
    scope: "global",
  },
  {
    id: "SeatingNumberGenerate",
    name: "Generate Student Seating Numbers",
    nameAr: "توليد أرقام جلوس الطلاب",
    description: "Auto-generate seating IDs.",
    descriptionAr:
      "توليد أرقام جلوس متسلسلة للطلاب حسب الترتيب الأبجدي والأكاديمي.",
    category: "reports",
    scope: "global",
  },
  {
    id: "SeatingNumberRead",
    name: "View Seating Number Lists",
    nameAr: "الاطلاع على كشوفات أرقام الجلوس",
    description: "Browse seating assignments.",
    descriptionAr: "عرض قائمة أرقام جلوس الطلاب وتوزيع قاعات لجان الامتحانات.",
    category: "reports",
    scope: "global",
  },
  {
    id: "SecretNumberGenerate",
    name: "Generate Secret Exam Codes",
    nameAr: "توليد الأرقام السرية للامتحانات",
    description: "Generate secret numbers for grading anonymity.",
    descriptionAr:
      "توليد الأرقام الكودية السرية للامتحانات لضمان نزاهة وسرية التصحيح.",
    category: "reports",
    scope: "global",
  },
  {
    id: "SecretNumberRead",
    name: "Read Secret Number Maps",
    nameAr: "استعراض كشوفات الأرقام السرية المربوطة",
    description: "Browse secret and seating number correlations.",
    descriptionAr:
      "الاطلاع على قائمة الأرقام السرية المربوطة لمراجعة الدرجات الكنترولية.",
    category: "reports",
    scope: "global",
  },
  {
    id: "SecretNumberReveal",
    name: "Reveal Secret Numbers on Cards",
    nameAr: "كشف وفك تشفير الأرقام السرية",
    description: "Expose anonymous student names for final control entry.",
    descriptionAr:
      "فك تشفير الأرقام السرية وربطها بالأسماء النهائية لإعلان النتائج.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamHallRead",
    name: "View Examination Halls",
    nameAr: "عرض لجان وقاعات الامتحانات",
    description: "Browse allocated rooms for exams.",
    descriptionAr: "الاطلاع على مباني وقاعات لجان الامتحانات وأعداد مقاعدها.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamHallWrite",
    name: "Manage Examination Halls",
    nameAr: "إدارة وتخصيص قاعات الاختبارات",
    description: "Set configurations of halls.",
    descriptionAr:
      "إعداد وتوزيع أعداد المقاعد والمسافات البينية لقاعات لجان الامتحانات.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamHallAllocate",
    name: "Allocate Student Seats in Halls",
    nameAr: "توزيع الطلاب على قاعات اللجان",
    description: "Auto-assign seating arrangements.",
    descriptionAr:
      "إجراء التوزيع التلقائي العشوائي أو الأبجدي للطلاب داخل قاعات الامتحانات.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ProctorRead",
    name: "View Proctors Schedule",
    nameAr: "عرض جدول توزيع المراقبين",
    description: "Browse assigned staff schedules for exams.",
    descriptionAr:
      "الاطلاع على جدول فترات المراقبة للمعلمين بقاعات الامتحانات.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ProctorAssign",
    name: "Assign Proctors to Halls",
    nameAr: "توزيع وإسناد المراقبين على اللجان",
    description: "Manage teacher duty assignments.",
    descriptionAr:
      "توزيع المعلمين إلكترونياً لمراقبة قاعات الامتحانات وتجنب التعارضات.",
    category: "reports",
    scope: "global",
  },
  {
    id: "StudentOrderRead",
    name: "View Student Ordering Index",
    nameAr: "عرض مؤشر ترتيب وفرز الطلاب",
    description: "Access centralized ranking lists.",
    descriptionAr:
      "عرض كشوفات ترتيب الطلاب حسب الأداء الأكاديمي أو الترتيب الأبجدي.",
    category: "reports",
    scope: "global",
  },
  {
    id: "StudentOrderRebuild",
    name: "Rebuild Ordering Index",
    nameAr: "إعادة بناء مؤشر ترتيب الطلاب",
    description: "Recalculate class rankings.",
    descriptionAr: "إعادة حساب الترتيب والمراكز الطلابية للفصل الدراسي الحالي.",
    category: "reports",
    scope: "global",
  },
  {
    id: "StudentOrderFreeze",
    name: "Freeze Ordering Rank Numbers",
    nameAr: "تجميد ترتيب المراكز الأكاديمية",
    description: "Lock rankings to prevent shift changes.",
    descriptionAr:
      "تجميد الترتيب النهائي لأوائل الطلبة تمهيداً لطباعة النتائج وتكريمهم.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamScheduleRead",
    name: "View Exam Timetable Schedules",
    nameAr: "عرض جدول فترات الاختبارات المعتمد",
    description: "View exam calendar timetables.",
    descriptionAr: "الاطلاع على مواعيد اختبارات المواد والصفوف الأكاديمية.",
    category: "reports",
    scope: "global",
  },
  {
    id: "ExamScheduleWrite",
    name: "Manage Exam Schedules",
    nameAr: "إعداد وتعديل جدول الامتحانات",
    description: "Auto-schedule or plan exam events.",
    descriptionAr: "تعديل وإعداد جدول تواريخ امتحانات المواد واللجان الموزعة.",
    category: "reports",
    scope: "global",
  },
];

// Initial Roles Definitions for School/University Settings
const initialRoles: Role[] = [
  {
    id: "super_admin",
    name: "Super Admin",
    nameAr: "مدير النظام الرئيسي",
    description:
      "Full administrative access with control over organization configs, billing, and logs.",
    descriptionAr:
      "يمتلك كامل الصلاحيات الإدارية المطلقة على النظام بما في ذلك التعديل الجوهري للبيانات والمسؤولين.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("super_admin"),
  },
  {
    id: "principal",
    name: "School Principal (Admin)",
    nameAr: "مدير المدرسة (إداري)",
    description:
      "Manages academic structures, scheduling, and staff directory. No financial backend modification.",
    descriptionAr:
      "يدير الهيكل التعليمي للطلاب والمدرسين وتوزيع الحصص والامتحانات، ولا يمتلك التحكم المالي العميق.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("principal"),
  },
  {
    id: "academic_supervisor",
    name: "Academic Supervisor (Vice Principal)",
    nameAr: "المشرف الأكاديمي (وكيل المدرسة)",
    description:
      "Manages academic operations, schedules, teachers, grade entries, exam sessions, assessments, student behavior, and attendance.",
    descriptionAr:
      "الإشراف الأكاديمي الكامل وإدارة المناهج والخطط الدراسية وتوزيع الحصص والفترات الدراسية ورصد الدرجات.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("academic_supervisor"),
  },
  {
    id: "registrar",
    name: "Registrar (Admissions)",
    nameAr: "مسؤول التسجيل والقبول",
    description:
      "Handles student enrollment, admissions, transfers, class promotions, and parent-student file directories.",
    descriptionAr:
      "إدارة قبول وتسجيل الطلاب الجدد وتوزيعهم على الصفوف وتحديث وثائقهم وملفاتهم ونقل الطلاب بين الشعب.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("registrar"),
  },
  {
    id: "finance_officer",
    name: "Financial Officer",
    nameAr: "المسؤول المالي / الحسابات",
    description:
      "Manages student billing, tuition plans, payments, ledger, and cash control reports.",
    descriptionAr:
      "إدارة المدفوعات والرسوم المدرسية وسندات القبض وحسابات الصندوق وتعديل العملات والتقارير المالية.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("finance_officer"),
  },
  {
    id: "teacher",
    name: "Teacher",
    nameAr: "المعلم / المدرس",
    description:
      "Limited to viewing and inputting grades and academic schedules for their assigned classes.",
    descriptionAr:
      "صلاحيات محدودة برصد الدرجات والغياب اليومي للطلاب والاطلاع على الفصول والجدول الدراسي الخاص به فقط.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("teacher"),
  },
  {
    id: "student",
    name: "Student",
    nameAr: "الطالب",
    description:
      "Read-only access to view personal grades, attendance sheets, exam seating numbers, and class timetables.",
    descriptionAr:
      "حساب للعرض فقط للطلاب لمتابعة جدول الحصص اليومي ورصد الدرجات والغياب وأرقام الجلوس.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("student"),
  },
  {
    id: "parent",
    name: "Parent / Guardian",
    nameAr: "ولي الأمر",
    description:
      "Read-only access to academic GPA reports, exam cards, billing dues, and attendance status of their children.",
    descriptionAr:
      "حساب لأولياء الأمور لمتابعة الأداء التعليمي والمستحقات والرسوم وجدول الامتحانات لأبنائهم.",
    userCount: 0,
    permissions: getDefaultPermissionsForRole("parent"),
  },
];

function getDefaultPermissionsForRole(
  roleId: string,
): Record<string, "global" | "local" | "none"> {
  const defaults: Record<string, "global" | "local" | "none"> = {};
  defaultPermissions.forEach((p) => {
    if (roleId === "super_admin") {
      defaults[p.id] = "global";
    } else if (roleId === "principal") {
      if (
        p.id === "UserDelete" ||
        p.id === "OrganizationWrite" ||
        p.id === "AuditRead"
      ) {
        defaults[p.id] = "none";
      } else {
        defaults[p.id] = "global";
      }
    } else if (roleId === "academic_supervisor") {
      if (
        p.category === "academics" ||
        p.category === "reports" ||
        p.category === "students" ||
        p.id === "UserRead" ||
        p.id === "OrganizationRead" ||
        p.id === "DashboardRead" ||
        p.id === "NotificationRead" ||
        p.id === "NotificationWrite"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else if (roleId === "registrar") {
      if (p.category === "students") {
        defaults[p.id] = "global";
      } else if (
        p.id === "ClassRead" ||
        p.id === "SubjectRead" ||
        p.id === "ScheduleRead" ||
        p.id === "UserRead" ||
        p.id === "OrganizationRead" ||
        p.id === "DashboardRead" ||
        p.id === "NotificationRead" ||
        p.id === "ExportData" ||
        p.id === "ImportData"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else if (roleId === "finance_officer") {
      if (p.category === "finance") {
        defaults[p.id] = "global";
      } else if (
        p.id === "UserRead" ||
        p.id === "OrganizationRead" ||
        p.id === "DashboardRead" ||
        p.id === "NotificationRead" ||
        p.id === "ExportData" ||
        p.id === "EnrollmentRead" ||
        p.id === "GuardianRead"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else if (roleId === "teacher") {
      if (
        p.id === "GradeRead" ||
        p.id === "GradeWrite" ||
        p.id === "GradeAnalytics" ||
        p.id === "GradeReport" ||
        p.id === "AttendanceRead" ||
        p.id === "AttendanceWrite" ||
        p.id === "AttendanceAnalytics" ||
        p.id === "AttendanceReport" ||
        p.id === "BehaviorRead" ||
        p.id === "BehaviorWrite" ||
        p.id === "BehaviorAI"
      ) {
        defaults[p.id] = "local";
      } else if (
        p.id === "ClassRead" ||
        p.id === "SubjectRead" ||
        p.id === "ScheduleRead" ||
        p.id === "AssessmentRead" ||
        p.id === "DocumentRead" ||
        p.id === "NotificationRead" ||
        p.id === "NotificationWrite" ||
        p.id === "UserRead" ||
        p.id === "CycleRead" ||
        p.id === "OrgUnitRead" ||
        p.id === "EnrollmentRead" ||
        p.id === "OrganizationRead" ||
        p.id === "DashboardRead" ||
        p.id === "ExamSessionRead" ||
        p.id === "ExamScheduleRead" ||
        p.id === "SeatingNumberRead" ||
        p.id === "ProctorRead" ||
        p.id === "StudentOrderRead" ||
        p.id === "ExamHallRead"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else if (roleId === "student") {
      if (
        p.id === "GradeRead" ||
        p.id === "ScheduleRead" ||
        p.id === "AssessmentRead" ||
        p.id === "AttendanceRead" ||
        p.id === "BehaviorRead" ||
        p.id === "DocumentRead" ||
        p.id === "SeatingNumberRead" ||
        p.id === "ExamScheduleRead"
      ) {
        defaults[p.id] = "local";
      } else if (
        p.id === "CycleRead" ||
        p.id === "ClassRead" ||
        p.id === "SubjectRead" ||
        p.id === "OrganizationRead" ||
        p.id === "DashboardRead" ||
        p.id === "NotificationRead"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else if (roleId === "parent") {
      if (
        p.id === "GradeRead" ||
        p.id === "ScheduleRead" ||
        p.id === "AssessmentRead" ||
        p.id === "AttendanceRead" ||
        p.id === "BehaviorRead" ||
        p.id === "DocumentRead" ||
        p.id === "SeatingNumberRead" ||
        p.id === "ExamScheduleRead" ||
        p.id === "FinanceRead"
      ) {
        defaults[p.id] = "local";
      } else if (
        p.id === "CycleRead" ||
        p.id === "ClassRead" ||
        p.id === "OrganizationRead" ||
        p.id === "NotificationRead"
      ) {
        defaults[p.id] = "global";
      } else {
        defaults[p.id] = "none";
      }
    } else {
      defaults[p.id] = "none";
    }
  });
  return defaults;
}

// UI States
let roles = $state<Role[]>(initialRoles);
let selectedRoleId = $state("super_admin");
let searchQuery = $state("");
let isLoading = $state(false);

// Create custom role state
let showCreateDialog = $state(false);
let newRoleName = $state("");
let newRoleDesc = $state("");

// Selected role object
const selectedRoleObj = $derived(
  roles.find((r) => r.id === selectedRoleId) || roles[0],
);

// Unsaved changes checking
let initialPermissions = $state<
  Record<string, Record<string, "global" | "local" | "none">>
>({});

function saveInitialStateSnapshot() {
  const snapshot: Record<
    string,
    Record<string, "global" | "local" | "none">
  > = {};
  roles.forEach((role) => {
    snapshot[role.id] = { ...role.permissions };
  });
  initialPermissions = snapshot;
}

async function loadRealUserCounts() {
  try {
    const usersList = await commands.users.getUsers(data.orgId);
    const counts: Record<string, number> = {};

    usersList.forEach((user) => {
      const r = String(user.primary_role || user.role || "").toLowerCase();
      const mappedRole =
        r === "admin" || r === "super_admin" ? "super_admin" : r;
      counts[mappedRole] = (counts[mappedRole] || 0) + 1;
    });

    roles = roles.map((role) => {
      let count = counts[role.id] || 0;
      return {
        ...role,
        userCount: count,
      };
    });
  } catch (e) {
    console.error("Failed to load real user counts", e);
  }
}

async function loadPermissionsFromDb() {
  isLoading = true;
  try {
    const dbRolePermissions = await commands.permissions.getRolePermissions();

    let savedCustomRoles: Role[] = [];
    try {
      const customSaved = localStorage.getItem(
        `orbis_custom_roles_${data.orgId || "default"}`,
      );
      if (customSaved) {
        savedCustomRoles = JSON.parse(customSaved);
      }
    } catch (e) {}

    let allRoles = [...initialRoles].map((ir) => ({
      ...ir,
      permissions: { ...ir.permissions },
    }));

    for (const cr of savedCustomRoles) {
      if (!allRoles.some((r) => r.id === cr.id)) {
        allRoles.push({
          ...cr,
          permissions: defaultPermissions.reduce(
            (acc, p) => ({ ...acc, [p.id]: "none" }),
            {},
          ),
        });
      }
    }

    roles = allRoles.map((role) => {
      const dbPerms = dbRolePermissions[role.id];
      if (dbPerms) {
        return {
          ...role,
          permissions: { ...role.permissions, ...dbPerms },
        };
      }
      return role;
    });

    saveInitialStateSnapshot();
  } catch (e) {
    console.error("Failed to load permissions from database", e);
    saveInitialStateSnapshot();
  } finally {
    isLoading = false;
  }
}

// Keep deep copy of initial states to show floating banner
onMount(async () => {
  await loadPermissionsFromDb();

  if (data.orgId) {
    await loadRealUserCounts();
  }
});

const hasUnsavedChanges = $derived.by(() => {
  if (!selectedRoleObj || !initialPermissions[selectedRoleObj.id]) return false;
  const current = selectedRoleObj.permissions;
  const initial = initialPermissions[selectedRoleObj.id];
  return Object.keys(current).some((key) => current[key] !== initial[key]);
});

// Category Translation Helper
function getCategoryLabel(cat: string): string {
  const isAr = getLocale() === "ar";
  switch (cat) {
    case "academics":
      return isAr ? "إدارة الشؤون الأكاديمية" : "Academic Management";
    case "finance":
      return isAr ? "الرقابة المالية" : "Financial Control";
    case "users":
      return isAr ? "المستخدمين والإعدادات" : "Users & Settings";
    case "students":
      return isAr ? "إدارة الطلاب" : "Student Management";
    case "reports":
      return isAr ? "التقارير والتحليلات" : "Reports & Analytics";
    default:
      return cat;
  }
}

// Filtered Permissions based on search query
const filteredPermissions = $derived(
  defaultPermissions.filter((p) => {
    const isAr = getLocale() === "ar";
    const keyword = searchQuery.toLowerCase();
    const matchesName = (isAr ? p.nameAr : p.name)
      .toLowerCase()
      .includes(keyword);
    const matchesDesc = (isAr ? p.descriptionAr : p.description)
      .toLowerCase()
      .includes(keyword);
    const matchesCat = getCategoryLabel(p.category)
      .toLowerCase()
      .includes(keyword);
    return matchesName || matchesDesc || matchesCat;
  }),
);

// Toggle permission scope directly
function togglePermissionScope(
  permId: string,
  currentScope: "global" | "local" | "none",
) {
  if (selectedRoleObj.id === "super_admin") {
    toast.warning(
      getLocale() === "ar"
        ? "لا يمكن تعديل صلاحيات المدير العام الرئيسي لحماية استقرار النظام"
        : "Cannot modify Super Admin permissions to maintain system stability",
    );
    return;
  }

  let nextScope: "global" | "local" | "none" = "none";
  if (currentScope === "none") nextScope = "local";
  else if (currentScope === "local") nextScope = "global";
  else nextScope = "none";

  selectedRoleObj.permissions[permId] = nextScope;
}

// Batch edit actions
function selectAllForRole() {
  if (selectedRoleObj.id === "super_admin") return;
  filteredPermissions.forEach((p) => {
    selectedRoleObj.permissions[p.id] = "global";
  });
}

function deselectAllForRole() {
  if (selectedRoleObj.id === "super_admin") return;
  filteredPermissions.forEach((p) => {
    selectedRoleObj.permissions[p.id] = "none";
  });
}

function restoreDefaultsForRole() {
  if (selectedRoleObj.id === "super_admin") return;

  selectedRoleObj.permissions = getDefaultPermissionsForRole(
    selectedRoleObj.id,
  );
  toast.info(
    getLocale() === "ar"
      ? "تمت استعادة الإعدادات الافتراضية"
      : "Restored defaults successfully",
  );
}

function discardChanges() {
  if (!initialPermissions[selectedRoleObj.id]) return;
  selectedRoleObj.permissions = { ...initialPermissions[selectedRoleObj.id] };
  toast.success(
    getLocale() === "ar" ? "تم التراجع عن التغييرات" : "Changes discarded",
  );
}

async function saveChanges() {
  isLoading = true;
  try {
    await commands.permissions.saveRolePermissions(
      selectedRoleObj.id,
      selectedRoleObj.permissions,
    );

    const customRolesList = roles.filter((r) => r.isCustom);
    localStorage.setItem(
      `orbis_custom_roles_${data.orgId || "default"}`,
      JSON.stringify(customRolesList),
    );

    initialPermissions[selectedRoleObj.id] = { ...selectedRoleObj.permissions };

    toast.success(
      getLocale() === "ar"
        ? "تم حفظ تحديثات الصلاحيات بنجاح!"
        : "Permissions updated and saved successfully!",
    );
  } catch (e) {
    console.error("Failed to save role permissions to database", e);
    toast.error(
      getLocale() === "ar"
        ? "فشل في حفظ التحديثات في قاعدة البيانات"
        : "Failed to save permissions to the database",
    );
  } finally {
    isLoading = false;
  }
}

function createCustomRole() {
  if (!newRoleName.trim()) {
    toast.error(
      getLocale() === "ar" ? "يرجى كتابة اسم الدور" : "Please input role name",
    );
    return;
  }

  const newId = `role_${Date.now()}`;
  const newRole: Role = {
    id: newId,
    name: newRoleName,
    nameAr: newRoleName,
    description: newRoleDesc || "Custom created user authorization group.",
    descriptionAr:
      newRoleDesc || "مجموعة صلاحيات مخصصة تم إنشاؤها يدوياً للموظفين.",
    userCount: 0,
    permissions: defaultPermissions.reduce(
      (acc, p) => ({ ...acc, [p.id]: "none" }),
      {},
    ),
    isCustom: true,
  };

  roles = [...roles, newRole];
  initialPermissions[newId] = { ...newRole.permissions };
  selectedRoleId = newId;
  showCreateDialog = false;
  newRoleName = "";
  newRoleDesc = "";

  const customRolesList = roles.filter((r) => r.isCustom);
  localStorage.setItem(
    `orbis_custom_roles_${data.orgId || "default"}`,
    JSON.stringify(customRolesList),
  );

  toast.success(
    getLocale() === "ar" ? "تم إنشاء الدور بنجاح" : "Role created successfully",
  );
}
</script>

<svelte:head>
  <title>{getLocale() === "ar" ? "الصلاحيات والأدوار" : "Roles & Permissions"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden flex flex-col font-sans text-start bg-background/50 relative">
  <!-- Header Area -->
  <div class="p-6 border-b border-border/40 bg-card/10 flex items-center justify-between gap-4">
    <div class="flex flex-col gap-1 text-start">
      <h1 class="text-xl font-bold tracking-tight text-foreground flex items-center gap-2">
        <Shield size={20} class="text-primary" />
        <span>{getLocale() === "ar" ? "الأدوار وصلاحيات النظام" : "System Roles & Permissions"}</span>
      </h1>
      <p class="text-xs text-muted-foreground">
        {getLocale() === "ar" ? "تكوين أدوار الأمان وتخصيص مستويات الصلاحيات التفصيلية للمستخدمين" : "Configure security roles and assign granular feature access rights."}
      </p>
    </div>

    <Button variant="lime" class="text-xs h-9 px-4 font-semibold" onclick={() => showCreateDialog = true}>
      <Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
      {getLocale() === "ar" ? "إنشاء دور مخصص" : "Create Custom Role"}
    </Button>
  </div>

  <!-- Workspace Grid Layout -->
  <div class="flex-1 flex overflow-hidden">
    <!-- Left panel (Roles Sidebar List) -->
    <div class="w-80 border-r border-border/40 bg-card/5 flex flex-col">
      <div class="p-4 border-b border-border/40">
        <h3 class="text-xs font-bold text-muted-foreground uppercase tracking-wider">
          {getLocale() === "ar" ? "مجموعات الأدوار المتاحة" : "Available Role Groups"}
        </h3>
      </div>
      <div class="flex-1 overflow-y-auto p-3 flex flex-col gap-1.5 scrollbar-hide">
        {#each roles as r (r.id)}
          <button
            class="w-full text-start p-3.5 rounded-xl border transition-all duration-200 cursor-pointer flex flex-col gap-1.5 relative overflow-hidden group
              {selectedRoleId === r.id 
                ? 'bg-primary/10 border-primary/30 text-foreground ring-1 ring-primary/20' 
                : 'bg-card/30 border-border/40 hover:bg-card/80 text-muted-foreground hover:text-foreground hover:border-border'}"
            onclick={() => selectedRoleId = r.id}
          >
            <!-- Background glow for active role -->
            {#if selectedRoleId === r.id}
              <div class="absolute -right-6 -top-6 w-16 h-16 bg-primary/20 rounded-full blur-xl"></div>
            {/if}

            <div class="flex items-center justify-between w-full">
              <span class="text-xs font-bold transition-colors group-hover:text-primary {selectedRoleId === r.id ? 'text-primary' : 'text-foreground'}">
                {getLocale() === "ar" ? r.nameAr : r.name}
              </span>
              <span class="text-[9px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground font-bold flex items-center gap-1 border border-border/40">
                <Users size={8} />
                {r.userCount}
              </span>
            </div>
            
            <p class="text-[10px] text-muted-foreground leading-normal line-clamp-2">
              {getLocale() === "ar" ? r.descriptionAr : r.description}
            </p>

            {#if r.isCustom}
              <span class="text-[8px] self-start px-1 py-0.2 rounded border bg-amber-500/10 text-amber-500 border-amber-500/20 font-semibold tracking-wide uppercase font-mono scale-95 mt-1">
                Custom
              </span>
            {/if}
          </button>
        {/each}
      </div>
    </div>

    <!-- Right panel (Permissions list) -->
    <div class="flex-1 flex flex-col bg-background/20 overflow-hidden">
      <!-- Matrix Top Bar filters -->
      <div class="p-4 border-b border-border/40 flex flex-col sm:flex-row gap-3 items-center bg-card/5">
        <div class="relative flex-1 w-full">
          <Search size={14} class="absolute top-1/2 -translate-y-1/2 text-muted-foreground rtl:right-3 ltr:left-3" />
          <Input
            type="text"
            bind:value={searchQuery}
            placeholder={getLocale() === "ar" ? "البحث في الصلاحيات والمزايا..." : "Search permissions and scopes..."}
            class="w-full rtl:pr-9 ltr:pl-9 h-9 text-xs"
          />
        </div>

        <div class="flex items-center gap-2">
          <Button 
            variant="outline" 
            size="xs" 
            class="text-[10px] h-9" 
            disabled={selectedRoleId === 'super_admin'}
            onclick={selectAllForRole}
          >
            {getLocale() === "ar" ? "تفعيل الكل" : "Select All"}
          </Button>
          <Button 
            variant="outline" 
            size="xs" 
            class="text-[10px] h-9" 
            disabled={selectedRoleId === 'super_admin'}
            onclick={deselectAllForRole}
          >
            {getLocale() === "ar" ? "تعطيل الكل" : "Deselect All"}
          </Button>
          <Button 
            variant="ghost" 
            size="icon-xs" 
            class="h-9 w-9 text-muted-foreground hover:text-foreground" 
            disabled={selectedRoleId === 'super_admin'}
            onclick={restoreDefaultsForRole}
            title={getLocale() === "ar" ? "استعادة الافتراضي" : "Restore Defaults"}
          >
            <RotateCcw size={14} />
          </Button>
        </div>
      </div>

      <!-- Permission categories list -->
      <div class="flex-1 overflow-y-auto p-6 flex flex-col gap-6 scrollbar-hide pb-24">
        <!-- Selected Role details card -->
        <div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-2 relative overflow-hidden">
          <div class="absolute right-0 top-0 w-24 h-24 bg-gradient-to-br from-primary/10 to-transparent rounded-bl-full pointer-events-none"></div>
          <div class="flex items-center gap-2">
            <span class="p-1 rounded bg-primary/10 text-primary">
              <Shield size={14} />
            </span>
            <h2 class="text-sm font-bold text-foreground">
              {getLocale() === "ar" ? selectedRoleObj.nameAr : selectedRoleObj.name}
            </h2>
          </div>
          <p class="text-xs text-muted-foreground leading-relaxed max-w-2xl text-start">
            {getLocale() === "ar" ? selectedRoleObj.descriptionAr : selectedRoleObj.description}
          </p>
        </div>

        <!-- Grouping by categories -->
        {#each ["academics", "finance", "students", "users", "reports"] as category}
          {@const catPerms = filteredPermissions.filter(p => p.category === category)}
          {#if catPerms.length > 0}
            <div class="flex flex-col gap-3">
              <h3 class="text-xs font-bold text-foreground border-b border-border/40 pb-2 flex items-center gap-2">
                {#if category === "academics"}<BookOpen size={13} class="text-blue-500" />{/if}
                {#if category === "finance"}<DollarSign size={13} class="text-emerald-500" />{/if}
                {#if category === "students"}<GraduationCap size={13} class="text-purple-500" />{/if}
                {#if category === "users"}<UserCheck size={13} class="text-rose-500" />{/if}
                {#if category === "reports"}<FileText size={13} class="text-amber-500" />{/if}
                <span>{getCategoryLabel(category)}</span>
              </h3>

              <div class="grid grid-cols-1 gap-2.5">
                {#each catPerms as perm (perm.id)}
                  {@const currentScope = selectedRoleObj.permissions[perm.id] || "none"}
                  
                  <div class="flex items-center justify-between p-4 rounded-xl border border-border bg-card/20 hover:bg-card/45 transition-colors gap-4">
                    <div class="flex flex-col gap-1 text-start flex-1 min-w-0">
                      <div class="flex items-center gap-2">
                        <span class="text-xs font-bold text-foreground">
                          {getLocale() === "ar" ? perm.nameAr : perm.name}
                        </span>
                        {#if currentScope !== "none"}
                          <span class="text-[8px] px-1 rounded-full font-bold tracking-wide uppercase 
                            {currentScope === 'global' ? 'bg-emerald-500/10 text-emerald-500 border border-emerald-500/20' : 'bg-blue-500/10 text-blue-500 border border-blue-500/20'}">
                            {currentScope === "global" 
                              ? (getLocale() === "ar" ? "شامل" : "Global") 
                              : (getLocale() === "ar" ? "محلي" : "Local")}
                          </span>
                        {/if}
                      </div>
                      <p class="text-[10px] text-muted-foreground leading-normal max-w-3xl truncate sm:whitespace-normal">
                        {getLocale() === "ar" ? perm.descriptionAr : perm.description}
                      </p>
                    </div>

                    <!-- Custom Scope Selector Switch-button combo -->
                    <div class="flex items-center gap-2 select-none">
                      <!-- AWS/GCP style Granular Scope selector -->
                      <select
                        value={currentScope}
                        disabled={selectedRoleObj.id === 'super_admin'}
                        onchange={(e) => selectedRoleObj.permissions[perm.id] = (e.currentTarget.value as "global" | "local" | "none")}
                        class="bg-background text-foreground border border-border rounded-md px-2.5 py-1 text-[10px] font-semibold focus:ring-1 focus:ring-ring outline-none h-8 w-32 cursor-pointer disabled:opacity-50 disabled:cursor-not-allowed"
                      >
                        <option value="none">{getLocale() === "ar" ? "معطلة (تعطيل)" : "Disabled"}</option>
                        <option value="local">{getLocale() === "ar" ? "محلي (الصف فقط)" : "Class-only"}</option>
                        <option value="global">{getLocale() === "ar" ? "شامل (المنظمة)" : "Organization-wide"}</option>
                      </select>

                      <!-- Quick toggle switch -->
                      <button 
                        class="w-10 h-6 rounded-full p-0.5 transition-colors duration-200 cursor-pointer focus:outline-none flex items-center
                          {currentScope !== 'none' ? 'bg-primary' : 'bg-muted'}"
                        disabled={selectedRoleObj.id === 'super_admin'}
                        onclick={() => togglePermissionScope(perm.id, currentScope)}
                        aria-label="Toggle Permission"
                        title="Toggle Permission"
                      >
                        <div class="w-5 h-5 rounded-full bg-white shadow-md transform transition-transform duration-200
                          {currentScope !== 'none' ? (getLocale() === 'ar' ? '-translate-x-4' : 'translate-x-4') : 'translate-x-0'}">
                        </div>
                      </button>
                    </div>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>

  <!-- Floating Save Changes Glassmorphic Banner -->
  {#if hasUnsavedChanges}
    <div 
      in:slide={{ duration: 250 }} 
      out:slide={{ duration: 200 }} 
      class="absolute bottom-5 left-1/2 -translate-x-1/2 w-11/12 max-w-3xl z-50 rounded-2xl border border-primary/30 bg-background/80 backdrop-blur-xl shadow-2xl p-4 flex items-center justify-between gap-4"
    >
      <div class="flex items-center gap-3 text-start">
        <div class="w-8 h-8 rounded-lg bg-primary/20 flex items-center justify-center text-primary animate-pulse">
          <Sparkles size={16} />
        </div>
        <div class="flex flex-col">
          <span class="text-xs font-bold text-foreground">
            {getLocale() === "ar" ? "تغييرات غير محفوظة" : "Unsaved Security Configuration"}
          </span>
          <span class="text-[10px] text-muted-foreground leading-none mt-0.5">
            {getLocale() === "ar" 
              ? `تم تعديل مصفوفة صلاحيات دور: ${selectedRoleObj.nameAr}` 
              : `You modified permissions matrix for: ${selectedRoleObj.name}`}
          </span>
        </div>
      </div>

      <div class="flex items-center gap-2">
        <Button variant="ghost" class="text-xs h-8 px-4 text-muted-foreground hover:text-foreground" onclick={discardChanges} disabled={isLoading}>
          {getLocale() === "ar" ? "إلغاء التغييرات" : "Discard"}
        </Button>
        <Button variant="lime" class="text-xs h-8 px-4 font-bold" onclick={saveChanges} disabled={isLoading}>
          {isLoading ? (getLocale() === "ar" ? "جاري الحفظ..." : "Saving...") : (getLocale() === "ar" ? "حفظ التغييرات" : "Save Changes")}
        </Button>
      </div>
    </div>
  {/if}
</div>

<!-- Create Custom Role Modal Dialog -->
{#if showCreateDialog}
  <div 
    in:fade={{ duration: 150 }} 
    class="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4"
  >
    <div 
      in:slide={{ duration: 200 }} 
      class="bg-card border border-border rounded-2xl w-full max-w-md p-6 flex flex-col gap-5 text-start shadow-2xl"
    >
      <div class="flex flex-col gap-1">
        <h2 class="text-sm font-bold text-foreground flex items-center gap-1.5">
          <Plus size={16} class="text-primary" />
          <span>{getLocale() === "ar" ? "إنشاء دور صلاحيات مخصص" : "Create Custom Role"}</span>
        </h2>
        <p class="text-[10px] text-muted-foreground">
          {getLocale() === "ar" ? "قم بتسمية الدور الجديد لإسناد الصلاحيات المخصصة له" : "Define the custom role name and optional description."}
        </p>
      </div>

      <div class="flex flex-col gap-4">
        <div class="flex flex-col gap-1.5">
          <label for="new_role_name" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "اسم الدور الجديد" : "Role Title Name"}
          </label>
          <Input
            id="new_role_name"
            type="text"
            bind:value={newRoleName}
            placeholder={getLocale() === "ar" ? "مثال: المشرف التربوي" : "e.g., Academic Supervisor"}
            required
          />
        </div>

        <div class="flex flex-col gap-1.5">
          <label for="new_role_desc" class="text-xs font-semibold text-muted-foreground">
            {getLocale() === "ar" ? "الوصف" : "Description / Details"}
          </label>
          <Input
            id="new_role_desc"
            type="text"
            bind:value={newRoleDesc}
            placeholder={getLocale() === "ar" ? "تفاصيل إضافية عن الدور الجديد..." : "Describe what users in this role group do..."}
          />
        </div>
      </div>

      <div class="flex items-center justify-end gap-2 border-t border-border/40 pt-4 mt-2">
        <Button variant="ghost" class="text-xs h-8" onclick={() => showCreateDialog = false}>
          {getLocale() === "ar" ? "إلغاء" : "Cancel"}
        </Button>
        <Button variant="lime" class="text-xs h-8 font-bold" onclick={createCustomRole}>
          {getLocale() === "ar" ? "إنشاء الدور" : "Create Role"}
        </Button>
      </div>
    </div>
  </div>
{/if}
