/**
 * @orbis/sdk-desktop — OrbisCommands implementation via Tauri IPC.
 *
 * Every method is a thin wrapper around `invoke("command_name", args)`.
 * NO business logic lives here — it all lives in the Rust ops:: layer.
 */
import { invoke, Channel } from "@tauri-apps/api/core";
import type { OrbisCommands } from "@orbis/sdk-core";
import type {
  Class,
  Enrollment,
  Subject,
  OrgUnit,
  ClassSubject,
  AcademicPeriod,
  StudentGuardian,
  Document,
  Notification,
  Organization,
  Grade,
  Attendance,
  BehaviorRecord,
  Assessment,
  GradeReportCard,
  AttendanceReport,
  GradeAnalytics,
  Payment,
  StudentFee,
  FeeTemplate,
  Treasury,
  DiscountTemplate,
  Account,
  JournalEntry,
  Currency,
  TimeSlot,
  ScheduledLesson,
  Room,
  SolveJob,
  ExamSession,
  ExamSubject,
  ExamSchedule,
  ExamHall,
  SeatingNumber,
  User,
  UserRole,
  UserSession,
} from "@orbis/sdk-core";

// ── Helper ──────────────────────────────────────────────────────
/** Thin wrapper to standardize invoke calls. */
async function cmd<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  return invoke<T>(command, args ?? {});
}

// ── Implementation ──────────────────────────────────────────────

export const commands: OrbisCommands = {
  // ═══════════════════════════════════════════════════════════════
  // ACADEMIC
  // ═══════════════════════════════════════════════════════════════
  academic: {
    listClasses: (orgId, periodId?) => cmd("list_classes", { orgId, periodId }),
    getClass: (classId) => cmd("get_class", { classId }),
    createClass: (data) => cmd("create_class", { data }),
    updateClass: (classId, data) => cmd("update_class", { classId, data }),
    deleteClass: (classId) => cmd("delete_class", { classId }),
    assignTeacher: (classId, teacherId) =>
      cmd("assign_teacher", { classId, teacherId }),

    listSubjects: (orgId) => cmd("list_subjects", { orgId }),
    getSubject: (subjectId) => cmd("get_subject", { subjectId }),
    createSubject: (data) => cmd("create_subject", { data }),
    updateSubject: (subjectId, data) =>
      cmd("update_subject", { subjectId, data }),
    deleteSubject: (subjectId) => cmd("delete_subject", { subjectId }),

    listOrgUnits: (orgId) => cmd("list_org_units", { orgId }),
    createOrgUnit: (data) => cmd("create_org_unit", { data }),
    updateOrgUnit: (unitId, data) => cmd("update_org_unit", { unitId, data }),
    deleteOrgUnit: (unitId) => cmd("delete_org_unit", { unitId }),

    listEnrollments: (orgId, classId?, periodId?) =>
      cmd("list_enrollments", { orgId, classId, periodId }),
    getEnrollment: (orgId, enrollmentId) =>
      cmd("get_enrollment", { orgId, enrollmentId }),
    createEnrollment: (data) => cmd("create_enrollment", { data }),
    withdrawEnrollment: (orgId, enrollmentId, reason) =>
      cmd("withdraw_enrollment", { orgId, enrollmentId, reason }),
    transferEnrollment: (orgId, enrollmentId, newClassId) =>
      cmd("transfer_enrollment", { orgId, enrollmentId, newClassId }),
    checkDuplicate: (studentId, periodId) =>
      cmd("check_duplicate_enrollment", { studentId, periodId }),

    listClassSubjects: (orgId, classId, periodId?) =>
      cmd("list_class_subjects", { orgId, classId, periodId }),
    listTeacherSubjects: (orgId, teacherId, periodId?) =>
      cmd("list_teacher_subjects", { orgId, teacherId, periodId }),
    createClassSubject: (data) => cmd("create_class_subject", { data }),
    updateClassSubject: (id, data) => cmd("update_class_subject", { id, data }),
    deleteClassSubject: (id) => cmd("delete_class_subject", { id }),

    listPeriods: (orgId) => cmd("list_periods", { orgId }),
    listPeriodsByYear: (orgId, periodId) =>
      cmd("list_periods_by_year", { orgId, periodId }),
    getPeriod: (periodId) => cmd("get_period", { periodId: periodId }),
    createPeriod: (data) => cmd("create_period", { data }),
    lockPeriod: (orgId, periodId) =>
      cmd("lock_period", { orgId, periodId: periodId }),
    unlockPeriod: (orgId, periodId) =>
      cmd("unlock_period", { orgId, periodId }),
    togglePeriodEnrollment: (orgId, periodId, open) =>
      cmd("toggle_period_enrollment", { orgId, periodId, open }),
    togglePeriodGradeEntry: (orgId, periodId, open) =>
      cmd("toggle_period_grade_entry", { orgId, periodId, open }),

    listGuardiansByStudent: (orgId, studentId) =>
      cmd("list_guardians_by_student", { orgId, studentId }),
    searchGuardians: (orgId, query) =>
      cmd("search_guardians", { orgId, query }),

    listDocuments: (ownerId?) => cmd("list_documents", { ownerId }),
    createDocument: (data) =>
      cmd("create_document", {
        orgId: data.orgId || data.org_id,
        data: {
          owner_id: data.ownerId || data.owner_id,
          name: data.name,
          item_type: data.item_type || data.type || "file",
          key: data.url || data.key || `doc_${Date.now()}`,
        },
      }),
    deleteDocument: (orgId, documentId) =>
      cmd("delete_document", { orgId, documentId }),

    listNotifications: (limit?) => cmd("list_notifications", { limit }),
    markNotificationRead: (notificationId) =>
      cmd("mark_notification_read", { notificationId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // ASSESSMENT
  // ═══════════════════════════════════════════════════════════════
  assessment: {
    listGrades: (periodId, enrollmentId?, classSubjectId?) =>
      cmd("list_grades", { periodId, enrollmentId, classSubjectId }),
    submitGrade: (periodId, data) => cmd("submit_grade", { periodId, data }),
    submitBulkGrades: (periodId, grades) =>
      cmd("submit_bulk_grades", { periodId, grades }),
    updateGrade: (gradeId, data) => cmd("update_grade", { gradeId, data }),
    getReportCard: (enrollmentId, periodId) =>
      cmd("get_report_card", { enrollmentId, periodId }),
    getGradeAnalytics: (orgId, classId, periodId) =>
      cmd("get_grade_analytics", { orgId, classId, periodId }),

    listAttendance: (orgId, periodId, classId?, date?, enrollmentId?) =>
      cmd("list_attendance", { orgId, periodId, classId, date, enrollmentId }),
    recordAttendance: (periodId, data) =>
      cmd("record_attendance", { periodId, data }),
    recordBulkAttendance: (periodId, data) =>
      cmd("record_bulk_attendance", { periodId, data }),
    getAttendanceReport: (periodId, enrollmentId) =>
      cmd("get_attendance_report", { periodId, enrollmentId }),

    listBehavior: (orgId, studentId) =>
      cmd("list_behavior", { orgId, studentId }),
    updateBehavior: (orgId, recordId, data) =>
      cmd("update_behavior", { orgId, recordId, data }),
    deleteBehavior: (orgId, recordId) =>
      cmd("delete_behavior", { orgId, recordId }),
    submitBehavior: (orgId, data) => cmd("submit_behavior", { orgId, data }),

    listAssessments: (periodId, classSubjectId?) =>
      cmd("list_assessments", { periodId, classSubjectId }),
    createAssessment: (data) => cmd("create_assessment", { data }),
    updateAssessment: (assessmentId, data) =>
      cmd("update_assessment", { assessmentId, data }),
    deleteAssessment: (assessmentId) =>
      cmd("delete_assessment", { assessmentId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listAccounts: (orgId) => cmd("list_ledger_accounts", { orgId }),
    listTransactionAccounts: (orgId) =>
      cmd("list_transaction_accounts", { orgId }),
    createAccount: (data) => cmd("create_ledger_account", { data }),
    getAccountStatement: (accountId, startDate?, endDate?) =>
      cmd("get_account_statement", { accountId, startDate, endDate }),

    listTreasuries: (orgId) => cmd("list_treasuries", { orgId }),
    createTreasury: (data) => cmd("create_treasury", { data }),

    listFeeTemplates: (orgId) => cmd("list_fee_templates", { orgId }),
    listActiveFeeTemplates: (orgId) =>
      cmd("list_active_fee_templates", { orgId }),
    createFeeTemplate: (data) => cmd("create_fee_template", { data }),
    updateFeeTemplate: (templateId, data) =>
      cmd("update_fee_template", { templateId, data }),
    deleteFeeTemplate: (templateId) =>
      cmd("delete_fee_template", { templateId }),

    listStudentFees: (enrollmentId) =>
      cmd("list_student_fees", { enrollmentId }),
    listAllStudentFees: (orgId) => cmd("list_all_student_fees", { orgId }),
    assignStudentFee: (data) => cmd("assign_student_fee", { data }),

    listPayments: (orgId) => cmd("list_payments", { orgId }),
    recordPayment: (data) => cmd("record_payment", { data }),

    listDiscountTemplates: (orgId) => cmd("list_discount_templates", { orgId }),
    createDiscountTemplate: (data) => cmd("create_discount_template", { data }),
    listCurrencies: (orgId) => cmd("list_currencies", { orgId }),
    createCurrency: (orgId, data) => cmd("create_currency", { orgId, data }),
    updateCurrency: (orgId, code, data) => cmd("update_currency", { orgId, code, data }),
    deleteCurrency: (orgId, code) => cmd("delete_currency", { orgId, code }),
    seedDefaultFinancials: async (orgId) => {
      return;
    },

    listJournalEntries: (orgId) => cmd("list_journal_entries", { orgId }),

    listFiscalYears: (orgId) => cmd("list_fiscal_years", { orgId }),
    createFiscalYear: (orgId, data) => cmd("create_fiscal_year", { orgId, data }),
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (orgId, periodId) => cmd("list_time_slots", { orgId, periodId }),
    createTimeSlot: (data) => cmd("create_time_slot", { data }),
    updateTimeSlot: (slotId, data) => cmd("update_time_slot", { slotId, data }),
    deleteTimeSlot: (slotId) => cmd("delete_time_slot", { slotId }),

    listLessonsByClass: (orgId, periodId, classId) =>
      cmd("list_lessons_by_class", { orgId, periodId, classId }),
    listLessonsByTeacher: (orgId, periodId, teacherId) =>
      cmd("list_lessons_by_teacher", { orgId, periodId, teacherId }),
    createLesson: (data) => cmd("create_lesson", { data }),
    updateLesson: (lessonId, data) => cmd("update_lesson", { lessonId, data }),
    deleteLesson: (lessonId) => cmd("delete_lesson", { lessonId }),
    checkConflicts: (
      orgId,
      periodId,
      teacherId,
      timeSlotId,
      dayOfWeek,
      roomId?,
    ) =>
      cmd("check_schedule_conflicts", {
        orgId,
        periodId,
        teacherId,
        timeSlotId,
        dayOfWeek,
        roomId,
      }),

    listRooms: (orgId) => cmd("list_rooms", { orgId }),
    createRoom: (data) => cmd("create_room", { data }),
    updateRoom: (roomId, data) => cmd("update_room", { roomId, data }),
    deleteRoom: (roomId) => cmd("delete_room", { roomId }),

    startSolver: (periodId, mode?) => cmd("start_solver", { periodId, mode }),
    getSolverJob: (jobId) => cmd("get_solver_job", { jobId }),
    listSolverJobs: (periodId) => cmd("list_solver_jobs", { periodId }),
    cancelSolver: (jobId) => cmd("cancel_solver", { jobId }),
    applySolver: (jobId) => cmd("apply_solver", { jobId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // EXAM
  // ═══════════════════════════════════════════════════════════════
  exam: {
    listSessions: (periodId) => cmd("list_exam_sessions", { periodId }),
    getSession: (sessionId) => cmd("get_exam_session", { sessionId }),
    createSession: (data) => cmd("create_exam_session", { data }),
    updateSession: (orgId, sessionId, data) =>
      cmd("update_exam_session", { sessionId, data }),

    listExamSubjects: (sessionId) => cmd("list_exam_subjects", { sessionId }),
    getSubjects: (orgId, sessionId) => cmd("list_exam_subjects", { sessionId }),
    createExamSubject: (data) => cmd("create_exam_subject", { data }),

    listHalls: () => cmd("list_exam_halls"),
    getHalls: (orgId, sessionId) => cmd("list_exam_halls", { sessionId }),
    createHall: (orgId, data) => cmd("create_exam_hall", { data }),

    listSchedule: (sessionId) => cmd("list_exam_schedule", { sessionId }),
    getSchedules: (orgId, sessionId) => cmd("list_exam_schedule", { sessionId }),
    createScheduleEntry: (data) => cmd("create_exam_schedule_entry", { data }),
    deleteSchedule: (orgId, scheduleId) => cmd("delete_exam_schedule", { scheduleId }),
    updateScheduleStatus: (orgId, scheduleId, status) => cmd("update_exam_schedule_status", { scheduleId, status }),

    listSeating: (examSubjectId) => cmd("list_seating_numbers", { examSubjectId }),
    getSeatingNumbers: (orgId, sessionId) => cmd("list_seating_numbers", { sessionId }),
    generateSeating: (orgId, sessionId, userId) => cmd("generate_exam_ordering", { sessionId, userId }),
    generateSecrets: (orgId, sessionId, userId) => cmd("generate_exam_secrets", { sessionId, userId }),
    
    getProctors: (orgId, sessionId) => cmd("list_exam_proctors", { sessionId }),
    autoAssignProctors: (orgId, sessionId, scheduleId, data) => cmd("auto_assign_proctors", { scheduleId, data }),
    allocateHalls: (orgId, sessionId, data) => cmd("allocate_exam_halls", { sessionId, data }),
    
    listSolveJobs: (orgId, sessionId) => cmd("list_exam_solver_jobs", { sessionId }),
    startSolve: (orgId, userId, data) => cmd("start_exam_solver", { data }),
    cancelSolve: (orgId, jobId) => cmd("cancel_exam_solver", { jobId }),
    applySolve: (orgId, jobId) => cmd("apply_exam_solver", { jobId }),
    getSolveJob: (orgId, jobId) => cmd("get_exam_solver_job", { jobId }),
    
    generateOrdering: (sessionId) => cmd("generate_exam_ordering", { sessionId }),
    getOrdering: (orgId, periodId) => cmd("get_student_ordering", { orgId, periodId }),
    rebuildOrdering: (orgId, sessionId) => cmd("rebuild_student_ordering", { orgId, sessionId }),
    freezeOrdering: async (_orgId, _periodId, _userId) => {
      // TODO: Implement freeze ordering in Rust backend
    },
  },

  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, ip?) =>
      cmd("login", { identifier, passwordHash, ip }),
    logout: (sessionId) => cmd("logout_session", { sessionId }),
    changePassword: (userId, data) => {
      const hash = typeof data === "string" ? data : data.new_password;
      return cmd("change_password", {
        userId,
        data: { new_password_hash: hash },
      });
    },
    listSessions: (userId) => cmd("list_user_sessions", { userId }),

    listUsers: async (orgId, filter?) => {
      // Pass the filter directly to the Rust backend
      const users = await cmd<User[]>("list_users", { orgId, filter });
      return users;
    },
    getUser: (userId) => cmd("get_user", { userId }),
    searchUsers: (query) => cmd("search_users", { query }),
    createUser: (orgId, createdBy, data) => cmd("create_user", { orgId, data }),
    updateUser: (userId, data) => cmd("update_user", { userId, data }),
    deleteUser: (userId, role?) => cmd("delete_user", { userId }),

    listUserRoles: (userId) => cmd("list_user_roles", { userId }),
    assignRole: (data) => cmd("assign_role", { data }),
    revokeRole: (roleId) => cmd("revoke_role", { roleId }),
    changeRole: (userId, data) => cmd("change_role", { userId, data }),
    saveUserRoles: (userId, roles) => cmd("save_user_roles", { userId, roles }),

    getRolePermissions: (role) => cmd("get_role_permissions", { role }),
    saveRolePermissions: (role, permissions) => cmd("save_role_permissions", { role, permissions }),
  },

  // ═══════════════════════════════════════════════════════════════
  // ORGANIZATION
  // ═══════════════════════════════════════════════════════════════
  organization: {
    getMainOrganization: () => cmd("get_main_organization"),
    getOrganization: (orgId) => cmd("get_organization", { orgId }),
    updateOrganization: (orgId, data) =>
      cmd("update_organization", { orgId, data }),
    getSettings: (orgId) => cmd("get_settings", { orgId }),
    updateSettings: (orgId, settings) =>
      cmd("update_settings", { orgId, settings }),
    getDashboardStats: (orgId) => cmd("get_dashboard_stats", { orgId }),
    registerStudent: (orgId, data) => cmd("register_student", { orgId, data }),
  },

  // ═══════════════════════════════════════════════════════════════
  // PLATFORM (Desktop-specific)
  // ═══════════════════════════════════════════════════════════════
  platform: {
    // Academic Years
    getCurrentYear: (orgId) => cmd("get_current_academic_year", { orgId }),
    listYears: (orgId) => cmd("list_academic_years", { orgId }),
    switchYear: (periodId) => cmd("switch_academic_year", { periodId }),
    isYearArchived: (periodId) => cmd("is_year_archived", { periodId }),
    getAcademicContext: (orgId) => cmd("get_academic_context", { orgId }),
    getSetupStatus: (orgId) => cmd("get_setup_status", { orgId }),
    initializeSchoolSetup: (orgId, payload) =>
      cmd("initialize_school_setup", { orgId, payload }),
    getPeriodTemplates: (orgId) => cmd("get_period_templates", { orgId }),

    // Promotion
    listPromotionJobs: (orgId) => cmd("get_promotion_jobs", { orgId }),
    getPromotionJob: (jobId) => cmd("get_promotion_job", { jobId }),
    getPromotionDetails: (jobId) => cmd("get_promotion_details", { jobId }),
    promotionDryRun: (orgId, config) =>
      cmd("promotion_dry_run", { orgId, config }),
    executePromotion: (jobId) => cmd("execute_promotion", { jobId }),
    rollbackPromotion: (jobId, userId) =>
      cmd("rollback_promotion", { jobId, userId }),
    overridePromotionDecision: (request) =>
      cmd("override_promotion_decision", { request }),
    getPromotionPolicy: (orgId) => cmd("get_promotion_policy", { orgId }),

    // Calendar / Periods
    listCalendarEvents: (orgId, periodId) =>
      cmd("get_calendar_events", { orgId, periodId }),
    listPendingCalendarEvents: (orgId) =>
      cmd("get_pending_calendar_events", { orgId }),
    getOrgPolicy: (orgId) => cmd("get_org_policy", { orgId }),

    // Overrides
    listOverrideRequests: (orgId, params?) =>
      cmd("list_override_requests", { orgId, params }),
    getOverrideRequest: (requestId) =>
      cmd("get_override_request", { requestId }),
    getPendingApprovals: (orgId, approverRole) =>
      cmd("get_pending_approvals", { orgId, approverRole }),
    processApproval: (requestId, action) =>
      cmd("process_approval", { requestId, action }),
    cancelOverride: (requestId, userId) =>
      cmd("cancel_override", { requestId, userId }),

    // Document Templates
    listDocumentTemplates: (orgId, params?) =>
      cmd("get_document_templates", { orgId, params }),
    getDocumentTemplate: (orgId, templateId) =>
      cmd("get_document_template", { orgId, templateId }),
    createDocumentTemplate: (orgId, userId, data) =>
      cmd("create_document_template", { orgId, data }),
    deleteDocumentTemplate: (orgId, templateId) =>
      cmd("delete_document_template", { orgId, templateId }),
    publishDocumentTemplate: (orgId, templateId, userId) =>
      cmd("publish_document_template", { orgId, templateId, userId }),
    archiveDocumentTemplate: (orgId, templateId) =>
      cmd("archive_document_template", { orgId, templateId }),
    restoreDocumentTemplate: (orgId, templateId) =>
      cmd("restore_document_template", { orgId, templateId }),
    updateDocumentTemplate: (orgId, templateId, userId, data) =>
      cmd("update_document_template", { orgId, templateId, data }),

    // AI Autopilot
    getAutopilotTasks: (orgId) => cmd("autopilot_get_tasks", { orgId }),
    getAutopilotDiagnostics: (orgId) =>
      cmd("autopilot_get_diagnostics", { orgId }),
    runAutopilotTick: (orgId) => cmd("autopilot_run_tick", { orgId }),

    // Auth (desktop login flow)
    login: (identifier, password) =>
      cmd("login", { identifier, passwordHash: password }),
    logout: (token) => cmd("logout_session", { sessionId: token }),

    // Import/Export
    importStudents: (orgId, data) => cmd("import_students", { orgId, data }),
    importGrades: (orgId, data) => cmd("import_grades", { orgId, data }),
    createBulkStudents: (orgId, userId, data) =>
      cmd("import_students", { orgId, data: { students: data } }),
    createBulkGrades: (orgId, userId, periodId, data) =>
      cmd("import_grades", { orgId, data: { grades: data } }),
    exportStudents: (orgId) => cmd("export_students", { orgId }),
    exportGrades: (orgId) => cmd("export_grades", { orgId }),

    // Audit
    listAuditLogs: (orgId) => cmd("list_audit_logs", { orgId }),
    getAllAuditLogs: (orgId) => cmd("list_audit_logs", { orgId }),

    // Enrollment (extended)
    transitionEnrollmentStatus: (orgId, enrollmentId, status) =>
      cmd("transition_enrollment_status", { orgId, enrollmentId, status }),

    // Guardians (extended)
    createGuardianLink: (data) => cmd("create_guardian_link", { data }),
    updateGuardianLink: (linkId, data) =>
      cmd("update_guardian_link", { linkId, data }),
    deleteGuardianLink: (linkId) => cmd("delete_guardian_link", { linkId }),

    // Documents
    listDocumentsByOwner: (orgId, ownerId) =>
      cmd("list_documents", { orgId, ownerId }),
  },
  agent: {
    listSessions: (orgId) => cmd("agent_get_sessions", { orgId }),
    createSession: (orgId, type) => cmd("agent_create_session", { orgId, type }),
    query: (orgId: string, sessionId: string, text: string, context?: any, onChunk?: (chunk: string) => void) => {
      if (onChunk) {
        const channel = new Channel<string>();
        channel.onmessage = onChunk;
        return cmd("agent_query", { orgId, sessionId, text, context, onChunk: channel });
      }
      return cmd("agent_query", { orgId, sessionId, text, context });
    },
    openStandaloneWindow: (orgId) => cmd("agent_open_standalone", { orgId }),
  },
  docforge: {
    previewDocumentTemplate: (templateJson, contextJson) =>
      cmd("preview_document_template", { templateJson, contextJson }),
  },
};
