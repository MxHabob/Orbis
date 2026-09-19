/**
 * @module api/query-keys
 *
 * Centralized, type-safe query key factory for TanStack Query across the Orbis application.
 * Follows TanStack Query best practices for cache keys:
 * [scope, entity, action, params]
 */

export const queryKeys = {
  // ── Auth & IAM ──────────────────────────────────────────────
  auth: {
    all: ["auth"] as const,
    session: () => [...queryKeys.auth.all, "session"] as const,
    user: (id: string) => [...queryKeys.auth.all, "user", id] as const,
    users: () => [...queryKeys.auth.all, "users"] as const,
  },

  // ── Organization ────────────────────────────────────────────
  org: {
    all: ["org"] as const,
    current: () => [...queryKeys.org.all, "current"] as const,
    settings: () => [...queryKeys.org.all, "settings"] as const,
    dashboard: () => [...queryKeys.org.all, "dashboard"] as const,
    units: () => [...queryKeys.org.all, "units"] as const,
    unit: (id: string) => [...queryKeys.org.all, "unit", id] as const,
  },

  // ── Calendar ────────────────────────────────────────────────
  calendar: {
    all: ["calendar"] as const,
    context: () => [...queryKeys.calendar.all, "context"] as const,
    years: () => [...queryKeys.calendar.all, "years"] as const,
    year: (id: string) => [...queryKeys.calendar.all, "year", id] as const,
    periods: () => [...queryKeys.calendar.all, "periods"] as const,
    period: (id: string) => [...queryKeys.calendar.all, "period", id] as const,
    periodTemplates: () =>
      [...queryKeys.calendar.all, "periodTemplates"] as const,
  },

  // ── Academic ────────────────────────────────────────────────
  academic: {
    all: ["academic"] as const,
    subjects: () => [...queryKeys.academic.all, "subjects"] as const,
    subject: (id: string) =>
      [...queryKeys.academic.all, "subject", id] as const,
    classes: () => [...queryKeys.academic.all, "classes"] as const,
    class: (id: string) => [...queryKeys.academic.all, "class", id] as const,
    classSubjects: (classId: string, periodId: string) =>
      [
        ...queryKeys.academic.all,
        "classSubjects",
        { classId, periodId },
      ] as const,
    prerequisites: (studentId: string, subjectId: string) =>
      [
        ...queryKeys.academic.all,
        "prerequisites",
        { studentId, subjectId },
      ] as const,
  },

  // ── Student ─────────────────────────────────────────────────
  student: {
    all: ["student"] as const,
    list: (filter?: Record<string, unknown>) =>
      [...queryKeys.student.all, "list", filter ?? {}] as const,
    detail: (id: string) =>
      [...queryKeys.student.all, "detail", id] as const,
    enrollments: (filter?: Record<string, unknown>) =>
      [...queryKeys.student.all, "enrollments", filter ?? {}] as const,
    enrollment: (id: string) =>
      [...queryKeys.student.all, "enrollment", id] as const,
    guardians: (studentId: string) =>
      [...queryKeys.student.all, "guardians", studentId] as const,
  },

  // ── Scheduling ──────────────────────────────────────────────
  scheduling: {
    all: ["scheduling"] as const,
    rooms: () => [...queryKeys.scheduling.all, "rooms"] as const,
    room: (id: string) => [...queryKeys.scheduling.all, "room", id] as const,
    timeSlots: (periodId: string) =>
      [...queryKeys.scheduling.all, "timeSlots", periodId] as const,
    lessons: (filter?: { classSubjectId?: string; periodId?: string }) =>
      [...queryKeys.scheduling.all, "lessons", filter ?? {}] as const,
  },

  // ── Assessment ──────────────────────────────────────────────
  assessment: {
    all: ["assessment"] as const,
    schemes: () => [...queryKeys.assessment.all, "schemes"] as const,
    scheme: (id: string) =>
      [...queryKeys.assessment.all, "scheme", id] as const,
    grades: (filter?: { studentId?: string; classSubjectId?: string }) =>
      [...queryKeys.assessment.all, "grades", filter ?? {}] as const,
    attendance: (classId: string, date: string) =>
      [...queryKeys.assessment.all, "attendance", { classId, date }] as const,
    gpa: (studentId: string, periodId: string) =>
      [...queryKeys.assessment.all, "gpa", { studentId, periodId }] as const,
  },

  // ── Student Cohort Progression ──────────────────────────────
  progression: {
    all: ["progression"] as const,
    jobs: () => [...queryKeys.progression.all, "jobs"] as const,
    job: (id: string) => [...queryKeys.progression.all, "job", id] as const,
  },

  // ── Workflow & Overrides ────────────────────────────────────
  workflow: {
    all: ["workflow"] as const,
    overrides: (status?: string) =>
      [...queryKeys.workflow.all, "overrides", status ?? "all"] as const,
    override: (id: string) =>
      [...queryKeys.workflow.all, "override", id] as const,
  },

  // ── Exam ────────────────────────────────────────────────────
  exam: {
    all: ["exam"] as const,
    sessions: (periodId: string) =>
      [...queryKeys.exam.all, "sessions", periodId] as const,
    session: (id: string) => [...queryKeys.exam.all, "session", id] as const,
    schedules: (sessionId: string) =>
      [...queryKeys.exam.all, "schedules", sessionId] as const,
  },

  // ── Finance ─────────────────────────────────────────────────
  finance: {
    all: ["finance"] as const,
    accounts: () => [...queryKeys.finance.all, "accounts"] as const,
    feeTemplates: () => [...queryKeys.finance.all, "feeTemplates"] as const,
    studentFees: (filter?: {
      enrollmentId?: string;
      studentId?: string;
      status?: string;
    }) => [...queryKeys.finance.all, "studentFees", filter ?? {}] as const,
    payments: (filter?: {
      feeId?: string;
      studentId?: string;
      fromDate?: string;
      toDate?: string;
    }) => [...queryKeys.finance.all, "payments", filter ?? {}] as const,
    currencies: () => [...queryKeys.finance.all, "currencies"] as const,
    journalEntries: () =>
      [...queryKeys.finance.all, "journalEntries"] as const,
    journalEntry: (id: string) =>
      [...queryKeys.finance.all, "journalEntry", id] as const,
    discountTemplates: () =>
      [...queryKeys.finance.all, "discountTemplates"] as const,
  },

  // ── Document ────────────────────────────────────────────────
  document: {
    all: ["document"] as const,
    list: () => [...queryKeys.document.all, "list"] as const,
    detail: (id: string) => [...queryKeys.document.all, "detail", id] as const,
  },

  // ── Integration ─────────────────────────────────────────────
  integration: {
    all: ["integration"] as const,
    list: () => [...queryKeys.integration.all, "list"] as const,
  },

  // ── Reports & Templates ─────────────────────────────────────
  report: {
    all: ["report"] as const,
    definitions: (filter?: { domain?: string; category?: string }) =>
      [...queryKeys.report.all, "definitions", filter ?? {}] as const,
    definition: (code: string) =>
      [...queryKeys.report.all, "definition", code] as const,
    templates: (filter?: {
      documentType?: string;
      entityType?: string;
      status?: string;
    }) => [...queryKeys.report.all, "templates", filter ?? {}] as const,
    template: (id: string) =>
      [...queryKeys.report.all, "template", id] as const,
    templateVersions: (id: string) =>
      [...queryKeys.report.all, "templateVersions", id] as const,
    templateBindings: (definitionId: string) =>
      [...queryKeys.report.all, "templateBindings", definitionId] as const,
    jobStatus: (jobId: string) =>
      [...queryKeys.report.all, "jobStatus", jobId] as const,
  },

  // ── Autopilot System ────────────────────────────────────────
  autopilot: {
    all: ["autopilot"] as const,
    tasks: (filter?: {
      status?: string;
      task_type?: string;
      limit?: number;
      offset?: number;
    }) => [...queryKeys.autopilot.all, "tasks", filter ?? {}] as const,
    task: (id: string) => [...queryKeys.autopilot.all, "task", id] as const,
    diagnostics: () => [...queryKeys.autopilot.all, "diagnostics"] as const,
  },
};
