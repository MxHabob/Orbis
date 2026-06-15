/**
 * Typed Tauri Commands API — Zero-network IPC layer.
 *
 * Every function maps to a #[tauri::command] in src-tauri/src/commands/.
 * The Rust backend accesses the embedded SQLite database directly via
 * core_domain::Service, giving sub-millisecond response times.
 */
import { invoke } from "@tauri-apps/api/core";

// ── API Namespace ──────────────────────────────────────────────────────────

export const api = {
  // ── Auth ────────────────────────────────────────────────────────────────
  auth: {
    login: (email: string, password: string) =>
      invoke<{ user: unknown; access_token: string; refresh_token: string }>(
        "login",
        { request: { email, password } },
      ),

    register: (data: {
      email: string;
      password: string;
      full_name: string;
      org_slug: string;
    }) => invoke<unknown>("register", { request: data }),

    validateToken: (token: string) =>
      invoke<unknown>("validate_token", { token }),

    logout: (token: string) => invoke<void>("logout", { token }),
  },

  // ── Users & Organization ───────────────────────────────────────────────
  users: {
    getOrganization: (orgId: string) =>
      invoke<unknown>("get_organization", { orgId }),

    updateOrganization: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_organization", { orgId, request }),

    getUsers: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_users", { orgId, params: params ?? {} }),

    getUser: (userId: string) => invoke<unknown>("get_user", { userId }),

    createUser: (
      orgId: string,
      createdBy: string,
      request: Record<string, unknown>,
    ) => invoke<unknown>("create_user", { orgId, createdBy, request }),

    updateUser: (userId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_user", { userId, request }),

    deleteUser: (userId: string, deletedBy: string) =>
      invoke<void>("delete_user", { userId, deletedBy }),

    changeUserRole: (userId: string, request: { role: string }) =>
      invoke<unknown>("change_user_role", { userId, request }),

    changeUserPassword: (userId: string, request: { new_password: string }) =>
      invoke<unknown>("change_user_password", { userId, request }),
  },

  // ── Academics (Cycles / Years / Terms) ─────────────────────────────────
  academics: {
    getCycles: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_cycles", { orgId, params: params ?? {} }),

    getCycle: (cycleId: string) => invoke<unknown>("get_cycle", { cycleId }),

    createCycle: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_cycle", { orgId, request }),

    updateCycle: (cycleId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_cycle", { cycleId, request }),

    activateCycle: (orgId: string, cycleId: string) =>
      invoke<unknown>("activate_cycle", { orgId, cycleId }),

    lockCycle: (cycleId: string) => invoke<unknown>("lock_cycle", { cycleId }),

    // Convenience: get current year
    getCurrentYear: (orgId: string) =>
      invoke<unknown[]>("get_cycles", {
        orgId,
        params: { is_current: 1 },
      }).then((cycles) => cycles[0] ?? null),

    // Convenience: get all years
    getYears: (orgId: string) =>
      invoke<unknown[]>("get_cycles", {
        orgId,
        params: { cycle_type: "year" },
      }),

    // Convenience: get all terms for a year
    getTerms: (orgId: string, yearId: string) =>
      invoke<unknown[]>("get_cycles", {
        orgId,
        params: { cycle_type: "term" },
      }).then((cycles) => cycles.filter((c: any) => c.parent_id === yearId)),
  },

  // ── Classes ────────────────────────────────────────────────────────────
  classes: {
    getAll: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_classes", { orgId, params: params ?? {} }),

    create: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_class", { orgId, request }),

    update: (classId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_class", { classId, request }),

    delete: (classId: string) => invoke<void>("delete_class", { classId }),
  },

  // ── Enrollments (Students) ─────────────────────────────────────────────
  enrollments: {
    getAll: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_enrollments", { orgId, params: params ?? {} }),

    getById: (enrollmentId: string) =>
      invoke<unknown>("get_enrollment", { enrollmentId }),

    create: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_enrollment", { orgId, request }),

    withdraw: (enrollmentId: string, request: { reason: string }) =>
      invoke<unknown>("withdraw_enrollment", { enrollmentId, request }),

    transfer: (enrollmentId: string, request: { new_class_id: string }) =>
      invoke<unknown>("transfer_enrollment", { enrollmentId, request }),

    promote: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("promote_students", { orgId, request }),
  },

  // ── Grades ─────────────────────────────────────────────────────────────
  grades: {
    getAll: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_grades", { orgId, params: params ?? {} }),

    submit: (
      orgId: string,
      gradedBy: string,
      request: Record<string, unknown>,
    ) => invoke<unknown>("submit_grade", { orgId, gradedBy, request }),

    submitBulk: (
      orgId: string,
      gradedBy: string,
      request: Record<string, unknown>,
    ) => invoke<unknown[]>("submit_grades_bulk", { orgId, gradedBy, request }),

    update: (gradeId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_grade", { gradeId, request }),

    getReportCard: (enrollmentId: string) =>
      invoke<unknown>("get_report_card", { enrollmentId }),

    getAnalytics: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown>("get_grade_analytics", { orgId, params: params ?? {} }),
  },

  // ── Attendance ─────────────────────────────────────────────────────────
  attendance: {
    getAll: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_attendance", { orgId, params: params ?? {} }),

    record: (
      orgId: string,
      recordedBy: string,
      request: Record<string, unknown>,
    ) => invoke<unknown>("record_attendance", { orgId, recordedBy, request }),

    recordBulk: (
      orgId: string,
      recordedBy: string,
      request: Record<string, unknown>,
    ) =>
      invoke<unknown[]>("record_bulk_attendance", {
        orgId,
        recordedBy,
        request,
      }),

    getReport: (enrollmentId: string) =>
      invoke<unknown>("get_attendance_report", { enrollmentId }),

    getAnalytics: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown>("get_attendance_analytics", {
        orgId,
        params: params ?? {},
      }),
  },

  // ── Finance ────────────────────────────────────────────────────────────
  finance: {
    getLedgerAccounts: (orgId: string) =>
      invoke<unknown[]>("get_ledger_accounts", { orgId }),

    createLedgerAccount: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_ledger_account", { orgId, request }),

    getTreasuries: (orgId: string) =>
      invoke<unknown[]>("get_treasuries", { orgId }),

    createTreasury: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_treasury", { orgId, request }),

    getStudentFees: (orgId: string, enrollmentId?: string) =>
      invoke<unknown[]>("get_student_fees", { orgId, enrollmentId }),

    assignStudentFee: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("assign_student_fee", { orgId, request }),

    getPayments: (orgId: string, enrollmentId?: string) =>
      invoke<unknown[]>("get_payments", { orgId, enrollmentId }),

    recordPayment: (
      orgId: string,
      createdBy: string,
      request: Record<string, unknown>,
    ) => invoke<unknown>("record_payment", { orgId, createdBy, request }),
  },

  // ── Behavior ───────────────────────────────────────────────────────────
  behavior: {
    getRecords: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_behavior_records", {
        orgId,
        params: params ?? {},
      }),

    submitAI: (request: Record<string, unknown>) =>
      invoke<unknown>("submit_ai_behavior", { request }),

    updateRecord: (
      recordId: string,
      reviewedBy: string,
      request: Record<string, unknown>,
    ) =>
      invoke<unknown>("update_behavior_record", {
        recordId,
        reviewedBy,
        request,
      }),

    deleteRecord: (recordId: string) =>
      invoke<void>("delete_behavior_record", { recordId }),
  },

  // ── Notifications ──────────────────────────────────────────────────────
  notifications: {
    getAll: (userId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_notifications", {
        userId,
        params: params ?? {},
      }),

    markRead: (notificationId: string) =>
      invoke<void>("mark_notification_read", { notificationId }),
  },

  // ── Org Units ──────────────────────────────────────────────────────────
  orgUnits: {
    getAll: (orgId: string, params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_org_units", { orgId, params: params ?? {} }),

    create: (orgId: string, request: Record<string, unknown>) =>
      invoke<unknown>("create_org_unit", { orgId, request }),

    update: (unitId: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_org_unit", { unitId, request }),

    delete: (unitId: string) => invoke<void>("delete_org_unit", { unitId }),
  },

  // ── Dashboard ──────────────────────────────────────────────────────────
  dashboard: {
    getStats: (orgId: string) =>
      invoke<unknown>("get_dashboard_stats", { orgId }),
  },

  // ── Audit Logs ─────────────────────────────────────────────────────────
  auditLogs: {
    getAll: (orgId: string) => invoke<unknown[]>("get_audit_logs", { orgId }),
  },

  // ── Subjects ─────────────────────────────────────────────────────────────
  subjects: {
    getAll: (orgId: string) => invoke<unknown[]>("get_subjects", { orgId }),
    getById: (id: string) => invoke<unknown>("get_subject", { id }),
    create: (orgId: string, request: Record<string, any>) =>
      invoke<unknown>("create_subject", { orgId, request }),
    update: (id: string, request: Record<string, any>) =>
      invoke<unknown>("update_subject", { id, request }),
    delete: (id: string) => invoke<void>("delete_subject", { id }),
  },

  // ── Assessments ──────────────────────────────────────────────────────────
  assessments: {
    getByClass: (params: Record<string, any>) =>
      invoke<unknown[]>("get_assessments", {
        classId: params.classId,
        subjectId: params.subjectId,
        termId: params.termId,
        month: params.month,
      }),
    create: (request: Record<string, unknown>) =>
      invoke<unknown>("create_assessment", { request }),
    update: (id: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_assessment", { id, request }),
    delete: (id: string) => invoke<void>("delete_assessment", { id }),
  },

  // ── Vault / Documents ───────────────────────────────────────────────────
  documents: {
    getAll: (params?: Record<string, unknown>) =>
      invoke<unknown[]>("get_documents", { params: params ?? {} }),
    getById: (id: string) => invoke<unknown>("get_document", { id }),
    create: (request: Record<string, unknown>) =>
      invoke<unknown>("create_document", { request }),
    delete: (id: string) => invoke<void>("delete_document", { id }),
  },

  // ── Certificates ────────────────────────────────────────────────────────
  certificates: {
    getAll: () => invoke<unknown[]>("get_certificates"),
    getById: (id: string) => invoke<unknown>("get_certificate", { id }),
    create: (request: Record<string, unknown>) =>
      invoke<unknown>("create_certificate", { request }),
    update: (id: string, request: Record<string, unknown>) =>
      invoke<unknown>("update_certificate", { id, request }),
    delete: (id: string) => invoke<void>("delete_certificate", { id }),
  },

  // ── Widgets ─────────────────────────────────────────────────────────────
  widgets: {
    getWidgetPreferences: (userId: string) =>
      invoke<unknown>("get_widget_preferences", { userId }),
    updateWidgetPreferences: (userId: string, preferences: unknown) =>
      invoke<unknown>("update_widget_preferences", { userId, preferences }),
  },

  // ── Window / App ───────────────────────────────────────────────────────
  window: {
    show: () => invoke<void>("show_window"),
    checkForUpdates: () => invoke<void>("check_for_updates"),
    getSavedServerUrl: () => invoke<string | null>("get_saved_server_url"),
    saveServerUrl: (url: string) =>
      invoke<void>("save_server_url", { url }),
  },
} as const;
