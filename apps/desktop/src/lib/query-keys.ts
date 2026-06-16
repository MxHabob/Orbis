/**
 * Query Keys Factory — Enterprise pattern for type-safe cache management.
 *
 * Each domain gets a namespace with hierarchical keys.
 * Used by useTauriQuery, useTauriMutation, and query-invalidation utilities.
 */
export const queryKeys = {
  // ── Auth ──────────────────────────────────────────────────────────────────
  auth: {
    all: () => ["auth"] as const,
    session: () => ["auth", "session"] as const,
  },

  // ── Users & Organization ─────────────────────────────────────────────────
  user: {
    all: () => ["user"] as const,
    me: () => ["user", "me"] as const,
    list: (orgId: string, params?: Record<string, unknown>) =>
      ["user", "list", orgId, params] as const,
    detail: (userId: string) => ["user", "detail", userId] as const,
  },

  organization: {
    all: () => ["organization"] as const,
    detail: (orgId: string) => ["organization", "detail", orgId] as const,
  },

  // ── Academics ────────────────────────────────────────────────────────────
  cycles: {
    all: (orgId?: string) => ["cycles", orgId] as const,
    current: (orgId: string) => ["cycles", "current", orgId] as const,
    detail: (cycleId: string) => ["cycles", "detail", cycleId] as const,
    terms: (yearId: string) => ["cycles", "terms", yearId] as const,
  },

  // ── Classes ──────────────────────────────────────────────────────────────
  classes: {
    all: (params?: Record<string, unknown>) => ["classes", params] as const,
    detail: (classId: string) => ["classes", "detail", classId] as const,
  },

  // ── Subjects ─────────────────────────────────────────────────────────────
  subjects: {
    all: (orgId?: string) => ["subjects", orgId] as const,
    detail: (subjectId: string) => ["subjects", "detail", subjectId] as const,
  },

  // ── Students / Enrollments ───────────────────────────────────────────────
  students: {
    all: (orgId?: string) => ["students", orgId] as const,
    byClass: (classId: string, params?: Record<string, unknown>) =>
      ["students", "byClass", classId, params] as const,
    detail: (enrollmentId: string) =>
      ["students", "detail", enrollmentId] as const,
  },

  // ── Grades ───────────────────────────────────────────────────────────────
  grades: {
    all: (orgId?: string) => ["grades", orgId] as const,
    grid: (params: Record<string, unknown>) =>
      ["grades", "grid", params] as const,
    analytics: (orgId: string, params?: Record<string, unknown>) =>
      ["grades", "analytics", orgId, params] as const,
    classStatistics: (classId: string, params?: Record<string, unknown>) =>
      ["grades", "classStatistics", classId, params] as const,
    reportCard: (enrollmentId: string) =>
      ["grades", "reportCard", enrollmentId] as const,
  },

  // ── Assessments ──────────────────────────────────────────────────────────
  assessments: {
    all: () => ["assessments"] as const,
    byClass: (classId?: string) =>
      ["assessments", "byClass", classId] as const,
  },

  // ── Attendance ───────────────────────────────────────────────────────────
  attendance: {
    all: (orgId?: string) => ["attendance", orgId] as const,
    report: (enrollmentId: string) =>
      ["attendance", "report", enrollmentId] as const,
    analytics: (orgId: string, params?: Record<string, unknown>) =>
      ["attendance", "analytics", orgId, params] as const,
  },

  // ── Finance ──────────────────────────────────────────────────────────────
  finance: {
    ledgerAccounts: (orgId: string) =>
      ["finance", "ledgerAccounts", orgId] as const,
    treasuries: (orgId: string) => ["finance", "treasuries", orgId] as const,
    studentFees: (orgId: string, enrollmentId?: string) =>
      ["finance", "studentFees", orgId, enrollmentId] as const,
    payments: (orgId: string, enrollmentId?: string) =>
      ["finance", "payments", orgId, enrollmentId] as const,
  },

  // ── Vault / Documents ───────────────────────────────────────────────────
  documents: {
    all: (params?: Record<string, unknown>) =>
      ["documents", params] as const,
    detail: (documentId: string) =>
      ["documents", "detail", documentId] as const,
  },

  // ── Certificates ────────────────────────────────────────────────────────
  certificates: {
    all: () => ["certificates"] as const,
    detail: (certificateId: string) =>
      ["certificates", "detail", certificateId] as const,
  },

  // ── Reports ─────────────────────────────────────────────────────────────
  reports: {
    schoolMetrics: (params?: Record<string, unknown>) =>
      ["reports", "schoolMetrics", params] as const,
  },

  // ── Notifications ───────────────────────────────────────────────────────
  notifications: {
    all: (userId: string, params?: Record<string, unknown>) =>
      ["notifications", userId, params] as const,
  },

  // ── Behavior ────────────────────────────────────────────────────────────
  behavior: {
    all: (orgId: string, params?: Record<string, unknown>) =>
      ["behavior", orgId, params] as const,
  },

  // ── Dashboard / Widgets ─────────────────────────────────────────────────
  dashboard: {
    stats: (orgId: string) => ["dashboard", "stats", orgId] as const,
  },

  widgets: {
    preferences: (userId: string) =>
      ["widgets", "preferences", userId] as const,
  },

  // ── Promotions ──────────────────────────────────────────────────────────
  promotions: {
    preview: (params?: Record<string, unknown>) =>
      ["promotions", "preview", params] as const,
    history: (params?: Record<string, unknown>) =>
      ["promotions", "history", params] as const,
    statistics: (params?: Record<string, unknown>) =>
      ["promotions", "statistics", params] as const,
  },

  // ── Org Units ───────────────────────────────────────────────────────────
  orgUnits: {
    all: (orgId: string, params?: Record<string, unknown>) =>
      ["orgUnits", orgId, params] as const,
  },

  // ── Audit Logs ──────────────────────────────────────────────────────────
  auditLogs: {
    all: (orgId: string) => ["auditLogs", orgId] as const,
  },

  // ── Invoice Templates ───────────────────────────────────────────────────
  invoiceTemplates: {
    all: () => ["invoiceTemplates"] as const,
  },

  // ── Schedule ────────────────────────────────────────────────────────────
  schedule: {
    all: () => ["schedule"] as const,
    timeSlots: (orgId: string, cycleId: string) =>
      ["schedule", "timeSlots", orgId, cycleId] as const,
    classSubjects: (classId?: string, cycleId?: string) =>
      ["schedule", "classSubjects", classId, cycleId] as const,
    classSchedule: (
      orgId: string,
      cycleId: string,
      classId: string,
    ) => ["schedule", "classSchedule", orgId, cycleId, classId] as const,
    teacherSchedule: (
      orgId: string,
      cycleId: string,
      teacherId: string,
    ) =>
      ["schedule", "teacherSchedule", orgId, cycleId, teacherId] as const,
    lesson: (lessonId: string) =>
      ["schedule", "lesson", lessonId] as const,
    conflicts: (params?: Record<string, unknown>) =>
      ["schedule", "conflicts", params] as const,
  },

  // ── Guardians ───────────────────────────────────────────────────────────
  guardians: {
    byStudent: (studentId: string) =>
      ["guardians", "byStudent", studentId] as const,
    byGuardian: (guardianId: string) =>
      ["guardians", "byGuardian", guardianId] as const,
  },
} as const;
