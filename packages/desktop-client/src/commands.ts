/**
 * Typed Tauri IPC commands — maps 1:1 to #[tauri::command] in src-tauri.
 * Auto-generated bindings (specta) will replace manual types incrementally.
 */
import { invoke } from "./core";
import type {
  DocumentTemplate,
  Class,
  ClassSubject,
  DashboardStats,
  Document,
  Enrollment,
  Grade,
  GradeAnalytics,
  GuardianLink,
  JsonRecord,
  LoginResponse,
  ScheduleConflict,
  ScheduledLesson,
  Subject,
  TimeSlot,
  User,
} from "./types";

export type { CommandError, User, LoginResponse, JsonRecord } from "./types";

export const commands = {
  auth: {
    login: (email: string, password: string) =>
      invoke<LoginResponse>("login", { request: { email, password } }),

    register: (data: {
      email: string;
      password: string;
      full_name: string;
      org_slug: string;
    }) => invoke<User>("register", { request: data }),

    validateToken: (token: string) => invoke<User>("validate_token", { token }),

    logout: (token: string) => invoke<void>("logout", { token }),
  },

  users: {
    getOrganization: (orgId: string) =>
      invoke<JsonRecord>("get_organization", { orgId }),

    updateOrganization: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_organization", { orgId, request }),

    getUsers: (orgId: string, params?: JsonRecord) =>
      invoke<User[]>("get_users", { orgId, params: params ?? {} }),

    getUser: (userId: string) => invoke<User>("get_user", { userId }),

    createUser: (orgId: string, createdBy: string, request: JsonRecord) =>
      invoke<User>("create_user", { orgId, createdBy, request }),

    updateUser: (userId: string, request: JsonRecord) =>
      invoke<User>("update_user", { userId, request }),

    deleteUser: (userId: string, deletedBy: string) =>
      invoke<void>("delete_user", { userId, deletedBy }),

    changeUserRole: (userId: string, request: { role: string }) =>
      invoke<User>("change_user_role", { userId, request }),

    changeUserPassword: (userId: string, request: { new_password: string }) =>
      invoke<User>("change_user_password", { userId, request }),
  },

  academics: {
    getCycles: (orgId: string, params?: JsonRecord) =>
      invoke<JsonRecord[]>("get_cycles", { orgId, params: params ?? {} }),

    getCycle: (cycleId: string) => invoke<JsonRecord>("get_cycle", { cycleId }),

    createCycle: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_cycle", { orgId, request }),

    updateCycle: (cycleId: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_cycle", { cycleId, request }),

    activateCycle: (orgId: string, cycleId: string) =>
      invoke<JsonRecord>("activate_cycle", { orgId, cycleId }),

    lockCycle: (cycleId: string) =>
      invoke<JsonRecord>("lock_cycle", { cycleId }),

    getCurrentYear: (orgId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { is_current: 1 },
      }).then((cycles) => cycles[0] ?? null),

    getYears: (orgId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { cycle_type: "year" },
      }),

    getTerms: (orgId: string, yearId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { cycle_type: "term" },
      }).then((cycles) => cycles.filter((c) => c.parent_id === yearId)),
  },

  classes: {
    getAll: (orgId: string, params?: JsonRecord) =>
      invoke<Class[]>("get_classes", { orgId, params: params ?? {} }),

    create: (orgId: string, request: JsonRecord) =>
      invoke<Class>("create_class", { orgId, request }),

    update: (classId: string, request: JsonRecord) =>
      invoke<Class>("update_class", { classId, request }),

    delete: (classId: string) => invoke<void>("delete_class", { classId }),
  },

  enrollments: {
    getAll: (orgId: string, params?: JsonRecord) =>
      invoke<Enrollment[]>("get_enrollments", { orgId, params: params ?? {} }),

    getById: (enrollmentId: string) =>
      invoke<Enrollment>("get_enrollment", { enrollmentId }),

    create: (orgId: string, request: JsonRecord) =>
      invoke<Enrollment>("create_enrollment", { orgId, request }),

    withdraw: (enrollmentId: string, request: { reason: string }) =>
      invoke<Enrollment>("withdraw_enrollment", { enrollmentId, request }),

    transfer: (enrollmentId: string, request: { new_class_id: string }) =>
      invoke<Enrollment>("transfer_enrollment", { enrollmentId, request }),

    promote: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("promote_students", { orgId, request }),
  },

  grades: {
    getAll: (orgId: string, params?: JsonRecord) =>
      invoke<Grade[]>("get_grades", { orgId, params: params ?? {} }),

    submit: (orgId: string, gradedBy: string, request: JsonRecord) =>
      invoke<Grade>("submit_grade", { orgId, gradedBy, request }),

    submitBulk: (orgId: string, gradedBy: string, request: JsonRecord) =>
      invoke<Grade[]>("submit_grades_bulk", { orgId, gradedBy, request }),

    update: (gradeId: string, request: JsonRecord) =>
      invoke<Grade>("update_grade", { gradeId, request }),

    getReportCard: (enrollmentId: string) =>
      invoke<JsonRecord>("get_report_card", { enrollmentId }),

    getAnalytics: (orgId: string, params?: JsonRecord) =>
      invoke<GradeAnalytics>("get_grade_analytics", {
        orgId,
        params: params ?? {},
      }),
  },

  attendance: {
    getAll: (orgId: string, params?: JsonRecord) =>
      invoke<JsonRecord[]>("get_attendance", { orgId, params: params ?? {} }),

    record: (orgId: string, recordedBy: string, request: JsonRecord) =>
      invoke<JsonRecord>("record_attendance", {
        orgId,
        recordedBy,
        request,
      }),

    recordBulk: (orgId: string, recordedBy: string, request: JsonRecord) =>
      invoke<JsonRecord[]>("record_bulk_attendance", {
        orgId,
        recordedBy,
        request,
      }),

    getReport: (enrollmentId: string) =>
      invoke<JsonRecord>("get_attendance_report", { enrollmentId }),

    getAnalytics: (orgId: string, params?: JsonRecord) =>
      invoke<JsonRecord>("get_attendance_analytics", {
        orgId,
        params: params ?? {},
      }),
  },

  finance: {
    getLedgerAccounts: (orgId: string) =>
      invoke<JsonRecord[]>("get_ledger_accounts", { orgId }),

    createLedgerAccount: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_ledger_account", { orgId, request }),

    getTreasuries: (orgId: string) =>
      invoke<JsonRecord[]>("get_treasuries", { orgId }),

    createTreasury: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_treasury", { orgId, request }),

    getStudentFees: (orgId: string, enrollmentId?: string) =>
      invoke<JsonRecord[]>("get_student_fees", { orgId, enrollmentId }),

    assignStudentFee: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("assign_student_fee", { orgId, request }),

    getPayments: (orgId: string, enrollmentId?: string) =>
      invoke<JsonRecord[]>("get_payments", { orgId, enrollmentId }),

    recordPayment: (orgId: string, createdBy: string, request: JsonRecord) =>
      invoke<JsonRecord>("record_payment", { orgId, createdBy, request }),
  },

  behavior: {
    getRecords: (orgId: string, params?: JsonRecord) =>
      invoke<JsonRecord[]>("get_behavior_records", {
        orgId,
        params: params ?? {},
      }),

    submitAI: (request: JsonRecord) =>
      invoke<JsonRecord>("submit_ai_behavior", { request }),

    updateRecord: (recordId: string, reviewedBy: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_behavior_record", {
        recordId,
        reviewedBy,
        request,
      }),

    deleteRecord: (recordId: string) =>
      invoke<void>("delete_behavior_record", { recordId }),
  },

  notifications: {
    getAll: (userId: string, params?: JsonRecord) =>
      invoke<JsonRecord[]>("get_notifications", {
        userId,
        params: params ?? {},
      }),

    markRead: (notificationId: string) =>
      invoke<void>("mark_notification_read", { notificationId }),
  },

  orgUnits: {
    getAll: (orgId: string, params?: JsonRecord) =>
      invoke<JsonRecord[]>("get_org_units", { orgId, params: params ?? {} }),

    create: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_org_unit", { orgId, request }),

    update: (unitId: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_org_unit", { unitId, request }),

    delete: (unitId: string) => invoke<void>("delete_org_unit", { unitId }),
  },

  dashboard: {
    getStats: (orgId: string) =>
      invoke<DashboardStats>("get_dashboard_stats", { orgId }),
  },

  auditLogs: {
    getAll: (orgId: string) =>
      invoke<JsonRecord[]>("get_audit_logs", { orgId }),
  },

  subjects: {
    getAll: (orgId: string) => invoke<Subject[]>("get_subjects", { orgId }),

    getById: (id: string) => invoke<Subject>("get_subject", { id }),

    create: (orgId: string, request: JsonRecord) =>
      invoke<Subject>("create_subject", { orgId, request }),

    update: (id: string, request: JsonRecord) =>
      invoke<Subject>("update_subject", { id, request }),

    delete: (id: string) => invoke<void>("delete_subject", { id }),
  },

  assessments: {
    getByClass: (params: {
      classId?: string;
      subjectId?: string;
      termId?: string;
      month?: number;
    }) =>
      invoke<JsonRecord[]>("get_assessments", {
        classId: params.classId,
        subjectId: params.subjectId,
        termId: params.termId,
        month: params.month,
      }),

    create: (request: JsonRecord) =>
      invoke<JsonRecord>("create_assessment", { request }),

    update: (id: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_assessment", { id, request }),

    delete: (id: string) => invoke<void>("delete_assessment", { id }),
  },

  documents: {
    getAll: (params?: JsonRecord) =>
      invoke<Document[]>("get_documents", { params: params ?? {} }),

    getById: (id: string) => invoke<Document>("get_document", { id }),

    create: (request: JsonRecord) =>
      invoke<Document>("create_document", { request }),

    delete: (id: string) => invoke<void>("delete_document", { id }),
  },

  documentTemplates: {
    getAll: (entityType?: string) =>
      invoke<DocumentTemplate[]>("get_document_templates", { entityType }),

    getById: (id: string) =>
      invoke<DocumentTemplate>("get_document_template", { id }),

    create: (request: JsonRecord) =>
      invoke<DocumentTemplate>("create_document_template", { request }),

    update: (id: string, request: JsonRecord) =>
      invoke<DocumentTemplate>("update_document_template", { id, request }),

    delete: (id: string) => invoke<void>("delete_document_template", { id }),
  },

  widgets: {
    getWidgetPreferences: (userId: string) =>
      invoke<JsonRecord>("get_widget_preferences", { userId }),

    updateWidgetPreferences: (userId: string, preferences: unknown) =>
      invoke<JsonRecord>("update_widget_preferences", {
        userId,
        preferences,
      }),
  },

  schedule: {
    // ── Time Slots ──────────────────────────────────────────────────────────
    getTimeSlots: (orgId: string, cycleId: string) =>
      invoke<TimeSlot[]>("get_time_slots", { orgId, cycleId }),

    createTimeSlot: (orgId: string, request: JsonRecord) =>
      invoke<TimeSlot>("create_time_slot", { orgId, request }),

    updateTimeSlot: (id: string, request: JsonRecord) =>
      invoke<TimeSlot>("update_time_slot", { id, request }),

    deleteTimeSlot: (id: string) => invoke<void>("delete_time_slot", { id }),

    // ── Class Subjects ─────────────────────────────────────────────────────
    getClassSubjects: (classId?: string, cycleId?: string) =>
      invoke<ClassSubject[]>("get_class_subjects", { classId, cycleId }),

    createClassSubject: (request: JsonRecord) =>
      invoke<ClassSubject>("create_class_subject", { request }),

    updateClassSubject: (id: string, request: JsonRecord) =>
      invoke<ClassSubject>("update_class_subject", { id, request }),

    deleteClassSubject: (id: string) =>
      invoke<void>("delete_class_subject", { id }),

    // ── Lessons & Schedule Views ───────────────────────────────────────────
    getClassSchedule: (orgId: string, cycleId: string, classId: string) =>
      invoke<ScheduledLesson[]>("get_class_schedule", {
        orgId,
        cycleId,
        classId,
      }),

    getTeacherSchedule: (orgId: string, cycleId: string, teacherId: string) =>
      invoke<ScheduledLesson[]>("get_teacher_schedule", {
        orgId,
        cycleId,
        teacherId,
      }),

    getLessonById: (id: string) =>
      invoke<ScheduledLesson>("get_lesson_by_id", { id }),

    createLesson: (orgId: string, request: JsonRecord) =>
      invoke<ScheduledLesson>("create_lesson", { orgId, request }),

    updateLesson: (orgId: string, id: string, request: JsonRecord) =>
      invoke<ScheduledLesson>("update_lesson", { orgId, id, request }),

    deleteLesson: (id: string) => invoke<void>("delete_lesson", { id }),

    // ── Conflict Check ─────────────────────────────────────────────────────
    checkConflicts: (params: {
      orgId: string;
      cycleId: string;
      timeSlotId: string;
      dayOfWeek: number;
      teacherId: string;
      classId: string;
      roomId?: string;
      excludeId?: string;
    }) =>
      invoke<ScheduleConflict[]>("check_schedule_conflicts", {
        orgId: params.orgId,
        cycleId: params.cycleId,
        timeSlotId: params.timeSlotId,
        dayOfWeek: params.dayOfWeek,
        teacherId: params.teacherId,
        classId: params.classId,
        roomId: params.roomId,
        excludeId: params.excludeId,
      }),
  },

  guardians: {
    getByStudent: (studentId: string) =>
      invoke<GuardianLink[]>("get_guardians_by_student", { studentId }),

    getStudentsByGuardian: (guardianId: string) =>
      invoke<GuardianLink[]>("get_students_by_guardian", { guardianId }),

    createLink: (request: JsonRecord) =>
      invoke<GuardianLink>("create_guardian_link", { request }),

    updateLink: (id: string, request: JsonRecord) =>
      invoke<GuardianLink>("update_guardian_link", { id, request }),

    deleteLink: (id: string) => invoke<void>("delete_guardian_link", { id }),
  },

  window: {
    show: () => invoke<void>("show_window"),
    checkForUpdates: () => invoke<void>("check_for_updates"),
  },
} as const;

/** @deprecated Use `commands` — alias for specta migration path */
export const api = commands;
