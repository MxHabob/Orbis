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
    users: (orgId?: string) => [...queryKeys.auth.all, "users", orgId ?? "current"] as const,
  },

  // ── Organization ────────────────────────────────────────────
  org: {
    all: ["org"] as const,
    current: () => [...queryKeys.org.all, "current"] as const,
    settings: (orgId?: string) => [...queryKeys.org.all, "settings", orgId ?? "current"] as const,
    dashboard: (orgId?: string) => [...queryKeys.org.all, "dashboard", orgId ?? "current"] as const,
    units: (orgId?: string) => [...queryKeys.org.all, "units", orgId ?? "current"] as const,
    unit: (id: string) => [...queryKeys.org.all, "unit", id] as const,
  },

  // ── Calendar ────────────────────────────────────────────────
  calendar: {
    all: ["calendar"] as const,
    context: (orgId?: string) => [...queryKeys.calendar.all, "context", orgId ?? "current"] as const,
    years: (orgId?: string) => [...queryKeys.calendar.all, "years", orgId ?? "current"] as const,
    year: (id: string) => [...queryKeys.calendar.all, "year", id] as const,
    periods: (orgId?: string) => [...queryKeys.calendar.all, "periods", orgId ?? "current"] as const,
    period: (id: string) => [...queryKeys.calendar.all, "period", id] as const,
    periodTemplates: () =>
      [...queryKeys.calendar.all, "periodTemplates"] as const,
  },

  // ── Academic ────────────────────────────────────────────────
  academic: {
    all: ["academic"] as const,
    subjects: (orgId?: string) => [...queryKeys.academic.all, "subjects", orgId ?? "current"] as const,
    subject: (id: string) =>
      [...queryKeys.academic.all, "subject", id] as const,
    classes: (orgId?: string) => [...queryKeys.academic.all, "classes", orgId ?? "current"] as const,
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
    list: (orgIdOrFilter?: string | Record<string, unknown>, filter?: Record<string, unknown>) => {
      if (typeof orgIdOrFilter === "string") {
        return [...queryKeys.student.all, "list", orgIdOrFilter, filter ?? {}] as const;
      }
      return [...queryKeys.student.all, "list", "current", orgIdOrFilter ?? {}] as const;
    },
    detail: (id: string) =>
      [...queryKeys.student.all, "detail", id] as const,
    enrollments: (orgIdOrFilter?: string | Record<string, unknown>, filter?: Record<string, unknown>) => {
      if (typeof orgIdOrFilter === "string") {
        return [...queryKeys.student.all, "enrollments", orgIdOrFilter, filter ?? {}] as const;
      }
      return [...queryKeys.student.all, "enrollments", "current", orgIdOrFilter ?? {}] as const;
    },
    enrollment: (id: string) =>
      [...queryKeys.student.all, "enrollment", id] as const,
    guardians: (studentId: string) =>
      [...queryKeys.student.all, "guardians", studentId] as const,
    attendance: (studentId: string) =>
      [...queryKeys.student.all, "attendance", studentId] as const,
    behavior: (studentId: string) =>
      [...queryKeys.student.all, "behavior", studentId] as const,
    documents: (studentId: string) =>
      [...queryKeys.student.all, "documents", studentId] as const,
    documentRequirements: (studentId: string) =>
      [...queryKeys.student.all, "documentRequirements", studentId] as const,
    fees: (studentId: string) =>
      [...queryKeys.student.all, "fees", studentId] as const,
    payments: (studentId: string) =>
      [...queryKeys.student.all, "payments", studentId] as const,
  },

  // ── Employee Affairs ────────────────────────────────────────
  employee: {
    all: ["employee"] as const,
    list: (orgIdOrFilter?: string | Record<string, unknown>, filter?: Record<string, unknown>) => {
      if (typeof orgIdOrFilter === "string") {
        return [...queryKeys.employee.all, "list", orgIdOrFilter, filter ?? {}] as const;
      }
      return [...queryKeys.employee.all, "list", "current", orgIdOrFilter ?? {}] as const;
    },
    detail: (id: string) =>
      [...queryKeys.employee.all, "detail", id] as const,
    stats: (orgId?: string) =>
      [...queryKeys.employee.all, "stats", orgId ?? "current"] as const,
  },

  // ── User Groups ─────────────────────────────────────────────
  group: {
    all: ["group"] as const,
    list: (orgId?: string) =>
      [...queryKeys.group.all, "list", orgId ?? "current"] as const,
    detail: (id: string) =>
      [...queryKeys.group.all, "detail", id] as const,
    members: (groupId: string) =>
      [...queryKeys.group.all, "members", groupId] as const,
  },

  // ── Scheduling ──────────────────────────────────────────────
  scheduling: {
    all: ["scheduling"] as const,
    rooms: (orgId?: string) => [...queryKeys.scheduling.all, "rooms", orgId ?? "current"] as const,
    room: (id: string) => [...queryKeys.scheduling.all, "room", id] as const,
    timeSlots: (periodId: string) =>
      [...queryKeys.scheduling.all, "timeSlots", periodId] as const,
    lessons: (filter?: { classSubjectId?: string; periodId?: string }) =>
      [...queryKeys.scheduling.all, "lessons", filter ?? {}] as const,
  },

  // ── Assessment ──────────────────────────────────────────────
  assessment: {
    all: ["assessment"] as const,
    schemes: (orgId?: string) => [...queryKeys.assessment.all, "schemes", orgId ?? "current"] as const,
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
    jobs: (orgId?: string) => [...queryKeys.progression.all, "jobs", orgId ?? "current"] as const,
    job: (id: string) => [...queryKeys.progression.all, "job", id] as const,
  },

  // ── Workflow & Overrides ────────────────────────────────────
  workflow: {
    all: ["workflow"] as const,
    overrides: (orgIdOrStatus?: string, status?: string) => {
      if (status !== undefined) {
        return [...queryKeys.workflow.all, "overrides", orgIdOrStatus ?? "current", status] as const;
      }
      return [...queryKeys.workflow.all, "overrides", "current", orgIdOrStatus ?? "all"] as const;
    },
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
    seating: (sessionId: string) =>
      [...queryKeys.exam.all, "seating", sessionId] as const,
    secrets: (scheduleId: string) =>
      [...queryKeys.exam.all, "secrets", scheduleId] as const,
    proctors: (scheduleId: string) =>
      [...queryKeys.exam.all, "proctors", scheduleId] as const,
  },

  // ── Finance ─────────────────────────────────────────────────
  finance: {
    all: ["finance"] as const,
    accounts: (orgId?: string) => [...queryKeys.finance.all, "accounts", orgId ?? "current"] as const,
    feeTemplates: (orgId?: string) => [...queryKeys.finance.all, "feeTemplates", orgId ?? "current"] as const,
    studentFees: (filterOrOrg?: string | unknown, filter?: unknown) => {
      if (typeof filterOrOrg === "string") {
        return [...queryKeys.finance.all, "studentFees", filterOrOrg, (filter as Record<string, unknown>) ?? {}] as const;
      }
      return [...queryKeys.finance.all, "studentFees", "current", (filterOrOrg as Record<string, unknown>) ?? {}] as const;
    },
    payments: (filterOrOrg?: string | unknown, filter?: unknown) => {
      if (typeof filterOrOrg === "string") {
        return [...queryKeys.finance.all, "payments", filterOrOrg, (filter as Record<string, unknown>) ?? {}] as const;
      }
      return [...queryKeys.finance.all, "payments", "current", (filterOrOrg as Record<string, unknown>) ?? {}] as const;
    },
    currencies: () => [...queryKeys.finance.all, "currencies"] as const,
    journalEntries: (orgId?: string) =>
      [...queryKeys.finance.all, "journalEntries", orgId ?? "current"] as const,
    journalEntry: (id: string) =>
      [...queryKeys.finance.all, "journalEntry", id] as const,
    discountTemplates: (orgId?: string) =>
      [...queryKeys.finance.all, "discountTemplates", orgId ?? "current"] as const,
  },

  // ── Document ────────────────────────────────────────────────
  document: {
    all: ["document"] as const,
    list: (orgIdOrFilter?: string | { entity_type?: string; entity_id?: string }, filter?: { entity_type?: string; entity_id?: string }) => {
      if (typeof orgIdOrFilter === "string") {
        return [...queryKeys.document.all, "list", orgIdOrFilter, filter ?? {}] as const;
      }
      return [...queryKeys.document.all, "list", "current", orgIdOrFilter ?? {}] as const;
    },
    byEntity: (entityType: string, entityId: string) => [...queryKeys.document.all, "entity", entityType, entityId] as const,
    detail: (id: string) => [...queryKeys.document.all, "detail", id] as const,
    requirements: (orgId?: string) => [...queryKeys.document.all, "requirements", orgId ?? "current"] as const,
    requirement: (id: string) => [...queryKeys.document.all, "requirement", id] as const,
    studentFulfillment: (studentId: string, params?: Record<string, unknown>) => [...queryKeys.document.all, "studentFulfillment", studentId, params ?? {}] as const,
  },

  // ── Folder & Knowledge Base ─────────────────────────────────
  folder: {
    all: ["folder"] as const,
    list: (parentId?: string) => ["folder", "list", parentId ?? "root"] as const,
    detail: (id: string) => ["folder", "detail", id] as const,
    contents: (id: string) => ["folder", "contents", id] as const,
    student: (studentId: string) => ["folder", "student", studentId] as const,
    curriculum: () => ["folder", "curriculum"] as const,
  },

  // ── Integration ─────────────────────────────────────────────
  integration: {
    all: ["integration"] as const,
    list: (orgId?: string) => [...queryKeys.integration.all, "list", orgId ?? "current"] as const,
  },

  // ── Reports & Templates ─────────────────────────────────────
  report: {
    all: ["report"] as const,
    definitions: (filterOrOrg?: string | { domain?: string; category?: string }, filter?: { domain?: string; category?: string }) => {
      if (typeof filterOrOrg === "string") {
        return [...queryKeys.report.all, "definitions", filterOrOrg, filter ?? {}] as const;
      }
      return [...queryKeys.report.all, "definitions", "current", filterOrOrg ?? {}] as const;
    },
    definition: (code: string) =>
      [...queryKeys.report.all, "definition", code] as const,
    templates: (filterOrOrg?: string | {
      documentType?: string;
      entityType?: string;
      status?: string;
    }, filter?: {
      documentType?: string;
      entityType?: string;
      status?: string;
    }) => {
      if (typeof filterOrOrg === "string") {
        return [...queryKeys.report.all, "templates", filterOrOrg, filter ?? {}] as const;
      }
      return [...queryKeys.report.all, "templates", "current", filterOrOrg ?? {}] as const;
    },
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
    diagnostics: (orgId?: string) => [...queryKeys.autopilot.all, "diagnostics", orgId ?? "current"] as const,
  },
};
