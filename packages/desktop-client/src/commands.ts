/**
 * Typed Tauri IPC commands — maps 1:1 to #[tauri::command] in src-tauri.
 * Auto-generated bindings (specta) will replace manual types incrementally.
 */
import { invoke } from "./core";
import type {
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
  UserSession,
  BatchJobStatus,
  AcademicYear,
  AcademicYearInfo,
  AcademicContextResponse,
  CalendarEvent,
  CalendarProcessingReport,
  EventProcessingResult,
  OrgAcademicPolicy,
  PromotionPolicy,
  AcademicOverrideRequest,
  ApprovalAction,
  CreateAcademicYearRequest,
  PromotionJob,
  PromotionDetail,
  PromotionConfig,
  PromotionPreview,
  OverrideRequest,
} from "./types";

export type {
  CommandError,
  User,
  LoginResponse,
  JsonRecord,
  UserSession,
} from "./types";

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

    getUserSessions: (userId: string) =>
      invoke<UserSession[]>("get_user_sessions", { userId }),

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

    getUserRoles: (userId: string) =>
      invoke<string[]>("get_user_roles", { userId }),

    saveUserRoles: (userId: string, roles: string[]) =>
      invoke<void>("save_user_roles", { userId, roles }),
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

    unlockCycle: (cycleId: string) =>
      invoke<JsonRecord>("unlock_cycle", { cycleId }),

    openGradeEntry: (cycleId: string, start?: string, end?: string) =>
      invoke<JsonRecord>("open_grade_entry", {
        cycleId,
        start: start ?? null,
        end: end ?? null,
      }),

    closeGradeEntry: (cycleId: string) =>
      invoke<JsonRecord>("close_grade_entry", { cycleId }),

    getCurrentYear: (orgId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { is_current: 1 },
      }).then((cycles: JsonRecord[]) => cycles[0] ?? null),

    getYears: (orgId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { cycle_type: "year" },
      }),

    getTerms: (orgId: string, yearId: string) =>
      invoke<JsonRecord[]>("get_cycles", {
        orgId,
        params: { cycle_type: "term" },
      }).then((cycles: JsonRecord[]) =>
        cycles.filter((c) => c.parent_id === yearId),
      ),

    initializeSchoolSetup: (orgId: string, config: any) =>
      invoke<void>("initialize_school_setup", { orgId, config }),

    getSetupStatus: (orgId: string) =>
      invoke<boolean>("get_setup_status", { orgId }),
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

    transitionStatus: (
      orgId: string,
      enrollmentId: string,
      targetStatus: string,
      withdrawalReason?: string,
    ) =>
      invoke<Enrollment>("transition_enrollment_status", {
        orgId,
        enrollmentId,
        targetStatus,
        withdrawalReason: withdrawalReason ?? null,
      }),

    checkDuplicate: (
      orgId: string,
      params: { email?: string; phone?: string; nationalId?: string },
    ) =>
      invoke<User[]>("check_student_duplicate", {
        orgId,
        email: params.email ?? null,
        phone: params.phone ?? null,
        nationalId: params.nationalId ?? null,
      }),
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

    bulkSubjectGradeEntry: (
      orgId: string,
      gradedBy: string,
      request: JsonRecord,
    ) =>
      invoke<JsonRecord>("bulk_subject_grade_entry", {
        orgId,
        gradedBy,
        request,
      }),

    getGradeMatrix: (
      orgId: string,
      cycleId: string,
      classId: string,
      subjectId: string,
    ) =>
      invoke<JsonRecord>("get_grade_matrix", {
        orgId,
        cycleId,
        classId,
        subjectId,
      }),

    compileSemesterGrades: (
      orgId: string,
      semesterNum: number,
      gradedBy: string,
    ) =>
      invoke<void>("compile_semester_grades", { orgId, semesterNum, gradedBy }),

    upsertGradeCell: (orgId: string, gradedBy: string, request: JsonRecord) =>
      invoke<JsonRecord>("upsert_grade_cell", { orgId, gradedBy, request }),
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

    getActiveFeeTemplates: (orgId: string) =>
      invoke<JsonRecord[]>("get_active_fee_templates", { orgId }),

    getFeeTemplates: (orgId: string) =>
      invoke<JsonRecord[]>("get_fee_templates", { orgId }),

    createFeeTemplate: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_fee_template", { orgId, request }),

    updateFeeTemplate: (orgId: string, id: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_fee_template", { orgId, id, request }),

    deleteFeeTemplate: (orgId: string, id: string) =>
      invoke<number>("delete_fee_template", { orgId, id }),

    assignStudentFee: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("assign_student_fee", { orgId, request }),

    getPayments: (orgId: string, enrollmentId?: string) =>
      invoke<JsonRecord[]>("get_payments", { orgId, enrollmentId }),

    recordPayment: (orgId: string, createdBy: string, request: JsonRecord) =>
      invoke<JsonRecord>("record_payment", { orgId, createdBy, request }),

    getJournalEntries: (orgId: string) =>
      invoke<JsonRecord[]>("get_journal_entries", { orgId }),

    createJournalEntry: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_journal_entry", { orgId, request }),

    getActiveDiscountTemplates: (orgId: string) =>
      invoke<JsonRecord[]>("get_active_discount_templates", { orgId }),

    getDiscountTemplates: (orgId: string) =>
      invoke<JsonRecord[]>("get_discount_templates", { orgId }),

    createDiscountTemplate: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_discount_template", { orgId, request }),

    updateDiscountTemplate: (orgId: string, id: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_discount_template", { orgId, id, request }),

    deleteDiscountTemplate: (orgId: string, id: string) =>
      invoke<number>("delete_discount_template", { orgId, id }),

    getCurrencies: (orgId: string) =>
      invoke<JsonRecord[]>("get_currencies", { orgId }),

    getCurrency: (orgId: string, code: string) =>
      invoke<JsonRecord | null>("get_currency", { orgId, code }),

    createCurrency: (c: JsonRecord) =>
      invoke<JsonRecord>("create_currency", { c }),

    updateCurrency: (orgId: string, code: string, updates: JsonRecord) =>
      invoke<JsonRecord>("update_currency", { orgId, code, updates }),

    deleteCurrency: (orgId: string, code: string) =>
      invoke<JsonRecord>("delete_currency", { orgId, code }),

    getAccountStatement: (
      orgId: string,
      accountId: string,
      startDate: string,
      endDate: string,
    ) =>
      invoke<JsonRecord>("get_account_statement", {
        orgId,
        accountId,
        startDate,
        endDate,
      }),

    seedDefaultFinancials: (orgId: string) =>
      invoke<void>("seed_default_financials", { orgId }),
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

    // ── Solver & AI Scheduler ────────────────────────────────────────────────
    startSolve: (
      orgId: string,
      userId: string,
      req: { cycle_id: string; mode: string },
    ) => invoke<JsonRecord>("start_schedule_solve", { orgId, userId, req }),

    getSolveJob: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("get_schedule_solve_job", { orgId, jobId }),

    listSolveJobs: (orgId: string, cycleId?: string) =>
      invoke<JsonRecord[]>("list_schedule_solve_jobs", { orgId, cycleId }),

    cancelSolve: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("cancel_schedule_solve", { orgId, jobId }),

    applySolve: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("apply_schedule_solve", { orgId, jobId }),

    // ── Rooms ──────────────────────────────────────────────────────────────
    getRooms: (orgId: string) => invoke<JsonRecord[]>("get_rooms", { orgId }),

    createRoom: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_room", { orgId, request }),

    updateRoom: (orgId: string, id: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_room", { orgId, id, request }),

    deleteRoom: (orgId: string, id: string) =>
      invoke<void>("delete_room", { orgId, id }),
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

    search: (orgId: string, query: string) =>
      invoke<User[]>("search_guardians", { orgId, query }),
  },

  exams: {
    // Sessions
    getSessions: (orgId: string, cycleId?: string) =>
      invoke<JsonRecord[]>("get_exam_sessions", {
        orgId,
        cycleId: cycleId ?? null,
      }),

    getSession: (orgId: string, sessionId: string) =>
      invoke<JsonRecord>("get_exam_session", { orgId, sessionId }),

    createSession: (orgId: string, userId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_exam_session", { orgId, userId, request }),

    updateSession: (orgId: string, sessionId: string, request: JsonRecord) =>
      invoke<JsonRecord>("update_exam_session", { orgId, sessionId, request }),

    // Subjects
    getSubjects: (orgId: string, sessionId: string) =>
      invoke<JsonRecord[]>("get_exam_subjects", { orgId, sessionId }),

    createSubject: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_exam_subject", { orgId, request }),

    // Student Ordering
    getOrdering: (orgId: string, cycleId: string, classId?: string) =>
      invoke<JsonRecord[]>("get_student_ordering", {
        orgId,
        cycleId,
        classId: classId ?? null,
      }),

    rebuildOrdering: (orgId: string, cycleId: string, strategy: string) =>
      invoke<JsonRecord>("rebuild_student_ordering", {
        orgId,
        cycleId,
        strategy,
      }),

    freezeOrdering: (orgId: string, cycleId: string, userId: string) =>
      invoke<JsonRecord>("freeze_student_ordering", { orgId, cycleId, userId }),

    // Seating & Secret Numbers
    generateSeating: (orgId: string, sessionId: string, userId: string) =>
      invoke<JsonRecord>("generate_seating_numbers", {
        orgId,
        sessionId,
        userId,
      }),

    generateSecrets: (orgId: string, sessionId: string, userId: string) =>
      invoke<JsonRecord>("generate_secret_numbers", {
        orgId,
        sessionId,
        userId,
      }),

    // Schedules
    getSchedules: (orgId: string, sessionId: string) =>
      invoke<JsonRecord[]>("get_exam_schedules", { orgId, sessionId }),

    startSolve: (orgId: string, userId: string, request: JsonRecord) =>
      invoke<JsonRecord>("start_exam_solve", { orgId, userId, request }),

    getSolveJob: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("get_exam_solve_job", { orgId, jobId }),

    listSolveJobs: (orgId: string, examSessionId?: string) =>
      invoke<JsonRecord[]>("list_exam_solve_jobs", {
        orgId,
        examSessionId: examSessionId ?? null,
      }),

    cancelSolve: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("cancel_exam_solve", { orgId, jobId }),

    applySolve: (orgId: string, jobId: string) =>
      invoke<JsonRecord>("apply_exam_solve", { orgId, jobId }),

    deleteSchedule: (orgId: string, scheduleId: string) =>
      invoke<void>("delete_exam_schedule", { orgId, scheduleId }),

    updateScheduleStatus: (orgId: string, scheduleId: string, status: string) =>
      invoke<JsonRecord>("update_exam_schedule_status", {
        orgId,
        scheduleId,
        status,
      }),

    // Logistics
    allocateHalls: (orgId: string, sessionId: string, config: JsonRecord) =>
      invoke<JsonRecord>("allocate_students_to_halls", {
        orgId,
        sessionId,
        config,
      }),

    autoAssignProctors: (
      orgId: string,
      sessionId: string,
      scheduleId: string,
      config: JsonRecord,
    ) =>
      invoke<JsonRecord>("auto_assign_proctors", {
        orgId,
        sessionId,
        scheduleId,
        config,
      }),

    getHalls: (orgId: string, sessionId: string) =>
      invoke<JsonRecord[]>("get_exam_halls", { orgId, sessionId }),

    createHall: (orgId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_exam_hall", { orgId, request }),

    getProctors: (orgId: string, sessionId: string, scheduleId?: string) =>
      invoke<JsonRecord[]>("get_exam_proctors", {
        orgId,
        sessionId,
        scheduleId: scheduleId ?? null,
      }),

    getHallStudents: (orgId: string, sessionId: string, hallId: string) =>
      invoke<JsonRecord[]>("get_hall_students", { orgId, sessionId, hallId }),

    getSeatingNumbers: (orgId: string, sessionId: string, classId?: string) =>
      invoke<JsonRecord[]>("get_seating_numbers", {
        orgId,
        sessionId,
        classId: classId ?? null,
      }),
  },

  documentTemplates: {
    getTemplates: (
      orgId: string,
      params?: {
        status?: string;
        document_type?: string;
        entity_type?: string;
        category_id?: string;
        search?: string;
      },
    ) =>
      invoke<JsonRecord[]>("get_document_templates", {
        orgId,
        status: params?.status,
        documentType: params?.document_type,
        entityType: params?.entity_type,
        categoryId: params?.category_id,
        search: params?.search,
      }),

    getTemplate: (orgId: string, id: string) =>
      invoke<JsonRecord>("get_document_template", { orgId, id }),

    createTemplate: (orgId: string, userId: string, request: JsonRecord) =>
      invoke<JsonRecord>("create_document_template", {
        orgId,
        userId,
        request,
      }),

    updateTemplate: (
      orgId: string,
      id: string,
      userId: string,
      request: JsonRecord,
    ) =>
      invoke<JsonRecord>("update_document_template", {
        orgId,
        id,
        userId,
        request,
      }),

    deleteTemplate: (orgId: string, id: string) =>
      invoke<void>("delete_document_template", { orgId, id }),

    publishTemplate: (orgId: string, id: string, userId: string) =>
      invoke<JsonRecord>("publish_document_template", { orgId, id, userId }),

    archiveTemplate: (orgId: string, id: string) =>
      invoke<JsonRecord>("archive_document_template", { orgId, id }),

    restoreTemplate: (orgId: string, id: string) =>
      invoke<JsonRecord>("restore_document_template", { orgId, id }),

    getVersions: (id: string) =>
      invoke<JsonRecord[]>("get_document_template_versions", { id }),

    revertToVersion: (
      orgId: string,
      id: string,
      version: number,
      userId: string,
    ) =>
      invoke<JsonRecord>("revert_document_template_to_version", {
        orgId,
        id,
        version,
        userId,
      }),
  },

  permissions: {
    getPermissions: () => invoke<any[]>("get_permissions"),
    getRolePermissions: () => invoke<any[]>("get_role_permissions"),
    saveRolePermissions: (role: string, rules: any[]) =>
      invoke<void>("save_role_permissions", { role, rules }),
    getUserPermissions: (userId: string) =>
      invoke<Record<string, string>>("get_user_permissions", { userId }),
    saveUserPermissions: (
      userId: string,
      permissions: Record<string, string>,
    ) => invoke<void>("save_user_permissions", { userId, permissions }),
    getAvailableRoles: () => invoke<string[]>("get_available_roles"),
  },

  docforge: {
    previewDocumentTemplate: (templateJson: string, contextJson: string) =>
      invoke<number[]>("preview_document_template", {
        templateJson,
        contextJson,
      }),
    renderDocument: (orgId: string, templateId: string, entityId: string) =>
      invoke<number[]>("render_document", { orgId, templateId, entityId }),
    getDatasourceSchemas: () =>
      invoke<Record<string, any>>("get_datasource_schemas"),
    evaluateTemplateExpressions: (
      expressionTemplate: string,
      contextJson: string,
    ) =>
      invoke<string>("evaluate_template_expressions", {
        expressionTemplate,
        contextJson,
      }),
    startBatchPrint: (
      orgId: string,
      templateId: string,
      entityIds: string[],
      outputDir: string,
    ) =>
      invoke<BatchJobStatus>("start_batch_print", {
        orgId,
        templateId,
        entityIds,
        outputDir,
      }),
    pauseBatchPrint: (jobId: string) =>
      invoke<boolean>("pause_batch_print", { jobId }),
    resumeBatchPrint: (jobId: string) =>
      invoke<boolean>("resume_batch_print", { jobId }),
    cancelBatchPrint: (jobId: string) =>
      invoke<boolean>("cancel_batch_print", { jobId }),
    getBatchPrintStatus: (jobId: string) =>
      invoke<BatchJobStatus>("get_batch_print_status", { jobId }),

    // ── Multi-Up Printing ──────────────────────────────────────
    computeMultiUpLayout: (configJson: string) =>
      invoke<any>("compute_multi_up_layout", { configJson }),
    previewMultiUp: (
      orgId: string,
      templateId: string,
      entityIds: string[],
      configJson: string,
    ) =>
      invoke<number[]>("preview_multi_up", {
        orgId,
        templateId,
        entityIds,
        configJson,
      }),
    startMultiUpBatchPrint: (
      orgId: string,
      templateId: string,
      entityIds: string[],
      configJson: string,
      outputDir: string,
    ) =>
      invoke<BatchJobStatus>("start_multi_up_batch_print", {
        orgId,
        templateId,
        entityIds,
        configJson,
        outputDir,
      }),
  },

  bulkImport: {
    createStudents: (orgId: string, createdBy: string, rows: JsonRecord[]) =>
      invoke<JsonRecord>("bulk_create_students", { orgId, createdBy, rows }),

    createTeachers: (orgId: string, createdBy: string, rows: JsonRecord[]) =>
      invoke<JsonRecord>("bulk_create_teachers", { orgId, createdBy, rows }),

    createGrades: (orgId: string, gradedBy: string, cycleId: string | null, rows: JsonRecord[]) =>
      invoke<JsonRecord>("bulk_create_grades", { orgId, gradedBy, cycleId, rows }),
  },

  window: {
    show: () => invoke<void>("show_window"),
    checkForUpdates: () => invoke<void>("check_for_updates"),
    openAgent: () => invoke<void>("open_agent_window"),
  },

  // ── Academic Context ──────────────────────────────────────────────────
  academicContext: {
    getContext: (orgId: string) =>
      invoke<AcademicContextResponse>("get_academic_context", { orgId }),

    resolveActiveCycle: (orgId: string) =>
      invoke<JsonRecord | null>("resolve_active_cycle", { orgId }),

    isWritable: () =>
      invoke<boolean>("is_year_writable", {}),
  },

  // ── Academic Calendar ──────────────────────────────────────────────────
  calendar: {
    getEvents: (orgId: string, cycleId: string) =>
      invoke<CalendarEvent[]>("get_calendar_events", { orgId, cycleId }),

    getPending: (orgId: string) =>
      invoke<CalendarEvent[]>("get_pending_calendar_events", { orgId }),

    create: (orgId: string, request: JsonRecord) =>
      invoke<CalendarEvent>("create_calendar_event", { orgId, request }),

    update: (eventId: string, request: JsonRecord) =>
      invoke<CalendarEvent>("update_calendar_event", { eventId, request }),

    delete: (eventId: string) =>
      invoke<void>("delete_calendar_event", { eventId }),

    execute: (eventId: string, userId: string) =>
      invoke<EventProcessingResult>("execute_calendar_event", { eventId, userId }),

    generateDefaults: (orgId: string, cycleId: string) =>
      invoke<CalendarEvent[]>("generate_default_events", { orgId, cycleId }),

    processPending: (orgId: string, userId: string) =>
      invoke<CalendarProcessingReport>("process_pending_events", { orgId, userId }),

    getPolicy: (orgId: string) =>
      invoke<OrgAcademicPolicy>("get_org_academic_policy", { orgId }),

    updatePolicy: (orgId: string, request: JsonRecord) =>
      invoke<OrgAcademicPolicy>("update_org_academic_policy", { orgId, request }),
  },

  // ── Academic Year Management ──────────────────────────────────────────
  academicYears: {
    list: (orgId: string) =>
      invoke<AcademicYearInfo[]>("list_academic_years", { orgId }),

    getCurrent: () =>
      invoke<AcademicYear | null>("get_current_academic_year", {}),

    create: (orgId: string, request: CreateAcademicYearRequest) =>
      invoke<AcademicYear>("create_academic_year", { orgId, request }),

    switch: (yearId: string) =>
      invoke<void>("switch_academic_year", { yearId }),

    archive: (yearId: string, userId: string) =>
      invoke<AcademicYear>("archive_academic_year", { yearId, userId }),

    isArchived: () =>
      invoke<boolean>("is_current_year_archived", {}),

    setActive: (orgId: string, yearId: string) =>
      invoke<void>("set_active_academic_year", { orgId, yearId }),
  },

  // ── Promotion ────────────────────────────────────────────────────────
  promotion: {
    dryRun: (orgId: string, config: PromotionConfig) =>
      invoke<PromotionJob>("promotion_dry_run", { orgId, config }),

    getJobs: (orgId: string) =>
      invoke<PromotionJob[]>("get_promotion_jobs", { orgId }),

    getJob: (jobId: string) =>
      invoke<PromotionJob | null>("get_promotion_job", { jobId }),

    getDetails: (jobId: string) =>
      invoke<PromotionDetail[]>("get_promotion_details", { jobId }),

    override: (request: OverrideRequest) =>
      invoke<PromotionDetail>("override_promotion_decision", { request }),

    execute: (jobId: string) =>
      invoke<PromotionJob>("execute_promotion", { jobId }),

    rollback: (jobId: string, userId: string) =>
      invoke<void>("rollback_promotion", { jobId, userId }),

    getPolicy: (orgId: string) =>
      invoke<PromotionPolicy>("get_promotion_policy", { orgId }),

    updatePolicy: (orgId: string, request: JsonRecord) =>
      invoke<PromotionPolicy>("update_promotion_policy", { orgId, request }),
  },

  // ── Override & Approval ─────────────────────────────────────────────
  overrides: {
    create: (orgId: string, request: JsonRecord) =>
      invoke<AcademicOverrideRequest>("create_override_request", { orgId, request }),

    processApproval: (requestId: string, action: ApprovalAction) =>
      invoke<AcademicOverrideRequest>("process_approval", { requestId, action }),

    cancel: (requestId: string, userId: string) =>
      invoke<AcademicOverrideRequest>("cancel_override_request", { requestId, userId }),

    list: (orgId: string, params?: JsonRecord) =>
      invoke<AcademicOverrideRequest[]>("list_override_requests", { orgId, params: params ?? {} }),

    get: (requestId: string) =>
      invoke<AcademicOverrideRequest>("get_override_request", { requestId }),

    getPending: (orgId: string, approverRole: string) =>
      invoke<AcademicOverrideRequest[]>("get_pending_approvals", { orgId, approverRole }),
  },
} as const;

/** @deprecated Use `commands` — alias for specta migration path */
export const api = commands;
