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
    listClasses: (periodId?) => cmd("list_classes", { periodId }),
    getClass: (classId) => cmd("get_class", { classId }),
    createClass: (data) => cmd("create_class", { data }),
    updateClass: (classId, data) => cmd("update_class", { classId, data }),
    deleteClass: (classId) => cmd("delete_class", { classId }),
    assignTeacher: (classId, teacherId) =>
      cmd("assign_teacher", { classId, teacherId }),
    listSubjects: (orgId) => cmd("list_subjects", {}),
    getSubject: (subjectId) => cmd("get_subject", { subjectId }),
    createSubject: (data) => cmd("create_subject", { data }),
    updateSubject: (subjectId, data) =>
      cmd("update_subject", { subjectId, data }),
    deleteSubject: (subjectId) => cmd("delete_subject", { subjectId }),
    listEnrollments: (classId?, periodId?) =>
      cmd("list_enrollments", { classId, periodId }),
    listEnrollmentsPaginated: (cursor?: string | null, limit?: number | null) =>
      cmd("list_enrollments_paginated", { cursor, limit }),
    getEnrollment: (enrollmentId) =>
      cmd("get_enrollment", { enrollmentId }),
    createEnrollment: (data) => cmd("create_enrollment", { data }),
    withdrawEnrollment: (enrollmentId, reason) =>
      cmd("withdraw_enrollment", { enrollmentId, reason }),
    transferEnrollment: (enrollmentId, newClassId) =>
      cmd("transfer_enrollment", { enrollmentId, newClassId }),
    checkDuplicate: (studentId, periodId) =>
      cmd("check_duplicate_enrollment", { studentId, periodId }),
    // ── Period Gate ──
    checkEnrollmentEligibility: (periodId, classId) =>
      cmd("check_enrollment_eligibility", { periodId, classId }),
    confirmEnrollmentDraft: (enrollmentId) =>
      cmd("confirm_enrollment_draft", { enrollmentId }),
    checkPeriodGate: (gateType, periodId) =>
      cmd("check_period_gate", { gateType, periodId }),
    listClassSubjects: (classId, periodId?) =>
      cmd("list_class_subjects", { classId, periodId }),
    listTeacherSubjects: (teacherId, periodId?) =>
      cmd("list_teacher_subjects", { teacherId, periodId }),
    createClassSubject: (data) => cmd("create_class_subject", { data }),
    updateClassSubject: (id, data) => cmd("update_class_subject", { id, data }),
    deleteClassSubject: (id) => cmd("delete_class_subject", { id }),
    listPeriods: (orgId) => cmd("list_periods", {}),
    listPeriodsByYear: (periodId) =>
      cmd("list_periods_by_year", { periodId }),
    getPeriod: (periodId) => cmd("get_period", { periodId: periodId }),
    createPeriod: (data) => cmd("create_period", { data }),
    updatePeriod: (periodId, data) => cmd("update_period", { periodId, data }),
    deletePeriod: (periodId) => cmd("delete_period", { periodId }),
    lockPeriod: (periodId) =>
      cmd("lock_period", { periodId: periodId }),
    unlockPeriod: (periodId) =>
      cmd("unlock_period", { periodId }),
    togglePeriodEnrollment: (periodId, open) =>
      cmd("toggle_period_enrollment", { periodId, open }),
    togglePeriodGradeEntry: (periodId, open) =>
      cmd("toggle_period_grade_entry", { periodId, open }),
    listGuardiansByStudent: (studentId) =>
      cmd("list_guardians_by_student", { studentId }),
    searchGuardians: (query) =>
      cmd("search_guardians", { query }),
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
    deleteDocument: (documentId) =>
      cmd("delete_document", { documentId }),
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
    getGradeAnalytics: (classId, periodId) =>
      cmd("get_grade_analytics", { classId, periodId }),
    listAttendance: (periodId, classId?, date?, enrollmentId?) =>
      cmd("list_attendance", { periodId, classId, date, enrollmentId }),
    recordAttendance: (periodId, data) =>
      cmd("record_attendance", { periodId, data }),
    recordBulkAttendance: (periodId, data) =>
      cmd("record_bulk_attendance", { periodId, data }),
    getAttendanceReport: (periodId, enrollmentId) =>
      cmd("get_attendance_report", { periodId, enrollmentId }),
    listBehavior: (studentId) =>
      cmd("list_behavior", { studentId }),
    updateBehavior: (recordId, data) =>
      cmd("update_behavior", { recordId, data }),
    deleteBehavior: (recordId) =>
      cmd("delete_behavior", { recordId }),
    submitBehavior: (data) => cmd("submit_behavior", { data }),
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
    listAccounts: (orgId) => cmd("list_ledger_accounts", {}),
    listTransactionAccounts: (orgId) =>
      cmd("list_transaction_accounts", {}),
    createAccount: (data) => cmd("create_ledger_account", { data }),
    getAccountStatement: (accountId, startDate?, endDate?) =>
      cmd("get_account_statement", { accountId, startDate, endDate }),
    listTreasuries: (orgId) => cmd("list_treasuries", {}),
    createTreasury: (data) => cmd("create_treasury", { data }),
    listFeeTemplates: (orgId) => cmd("list_fee_templates", {}),
    listActiveFeeTemplates: (orgId) =>
      cmd("list_active_fee_templates", {}),
    createFeeTemplate: (data) => cmd("create_fee_template", { data }),
    updateFeeTemplate: (templateId, data) =>
      cmd("update_fee_template", { templateId, data }),
    deleteFeeTemplate: (templateId) =>
      cmd("delete_fee_template", { templateId }),
    listStudentFees: (enrollmentId) =>
      cmd("list_student_fees", { enrollmentId }),
    listAllStudentFees: (orgId) => cmd("list_all_student_fees", {}),
    assignStudentFee: (data) => cmd("assign_student_fee", { data }),
    listPayments: (orgId) => cmd("list_payments", {}),
    recordPayment: (data) => cmd("record_payment", { data }),
    listDiscountTemplates: (orgId) => cmd("list_discount_templates", {}),
    createDiscountTemplate: (data) => cmd("create_discount_template", { data }),
    listCurrencies: (orgId) => cmd("list_currencies", {}),
    createCurrency: (data) => cmd("create_currency", { data }),
    updateCurrency: (code, data) => cmd("update_currency", { code, data }),
    deleteCurrency: (code) => cmd("delete_currency", { code }),
    seedDefaultFinancials: async (orgId) => {
      return;
    },
    listJournalEntries: (orgId) => cmd("list_journal_entries", {}),
    listFiscalYears: (orgId) => cmd("list_fiscal_years", {}),
  },
  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (periodId) => cmd("list_time_slots", { periodId }),
    createTimeSlot: (data) => cmd("create_time_slot", { data }),
    updateTimeSlot: (slotId, data) => cmd("update_time_slot", { slotId, data }),
    deleteTimeSlot: (slotId) => cmd("delete_time_slot", { slotId }),
    listLessonsByClass: (periodId, classId) =>
      cmd("list_lessons_by_class", { periodId, classId }),
    listLessonsByTeacher: (periodId, teacherId) =>
      cmd("list_lessons_by_teacher", { periodId, teacherId }),
    createLesson: (data) => cmd("create_lesson", { data }),
    updateLesson: (lessonId, data) => cmd("update_lesson", { lessonId, data }),
    deleteLesson: (lessonId) => cmd("delete_lesson", { lessonId }),
    checkConflicts: (
      periodId,
      teacherId,
      timeSlotId,
      dayOfWeek,
      roomId?,
    ) =>
      cmd("check_schedule_conflicts", {
        periodId,
        teacherId,
        timeSlotId,
        dayOfWeek,
        roomId,
      }),
    listRooms: (orgId) => cmd("list_rooms", {}),
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
    updateSession: (sessionId, data) =>
      cmd("update_exam_session", { sessionId, data }),
    listExamSubjects: (sessionId) => cmd("list_exam_subjects", { sessionId }),
    getSubjects: (sessionId) => cmd("list_exam_subjects", { sessionId }),
    createExamSubject: (data) => cmd("create_exam_subject", { data }),
    listHalls: () => cmd("list_exam_halls"),
    getHalls: (sessionId) => cmd("list_exam_halls", { sessionId }),
    createHall: (data) => cmd("create_exam_hall", { data }),
    listSchedule: (sessionId) => cmd("list_exam_schedule", { sessionId }),
    getSchedules: (sessionId) => cmd("list_exam_schedule", { sessionId }),
    createScheduleEntry: (data) => cmd("create_exam_schedule_entry", { data }),
    deleteSchedule: (scheduleId) => cmd("delete_exam_schedule", { scheduleId }),
    updateScheduleStatus: (scheduleId, status) => cmd("update_exam_schedule_status", { scheduleId, status }),
    listSeating: (examSubjectId) => cmd("list_seating_numbers", { examSubjectId }),
    getSeatingNumbers: (sessionId) => cmd("list_seating_numbers", { sessionId }),
    generateSeating: (sessionId, userId) => cmd("generate_exam_ordering", { sessionId, userId }),
    generateSecrets: (sessionId, userId) => cmd("generate_exam_secrets", { sessionId, userId }),
    getProctors: (sessionId) => cmd("list_exam_proctors", { sessionId }),
    autoAssignProctors: (sessionId, scheduleId, data) => cmd("auto_assign_proctors", { scheduleId, data }),
    allocateHalls: (sessionId, data) => cmd("allocate_exam_halls", { sessionId, data }),
    listSolveJobs: (sessionId) => cmd("list_exam_solver_jobs", { sessionId }),
    startSolve: (userId, data) => cmd("start_exam_solver", { data }),
    cancelSolve: (jobId) => cmd("cancel_exam_solver", { jobId }),
    applySolve: (jobId) => cmd("apply_exam_solver", { jobId }),
    getSolveJob: (jobId) => cmd("get_exam_solver_job", { jobId }),
    generateOrdering: (sessionId) => cmd("generate_exam_ordering", { sessionId }),
    getOrdering: (periodId) => cmd("get_student_ordering", { periodId }),
    rebuildOrdering: (sessionId) => cmd("rebuild_student_ordering", { sessionId }),
    freezeOrdering: async (__periodId, _userId) => {
      // TODO: Implement freeze ordering in Rust backend
    },
  },
  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, ip?) =>
      cmd("login", { identifier, passwordHash, ip }),
    restoreSession: (token) => cmd("restore_session", { token }),
    logout: (sessionId) => cmd("logout_session", { sessionId }),
    changePassword: (userId, data) => {
      const hash = typeof data === "string" ? data : data.new_password;
      return cmd("change_password", {
        userId,
        data: { new_password_hash: hash },
      });
    },
    listSessions: (userId) => cmd("list_user_sessions", { userId }),
    listUsers: async (filter?) => {
      // Pass the filter directly to the Rust backend
      const users = await cmd<User[]>("list_users", { filter });
      return users;
    },
    listUsersPaginated: async (cursor?, limit?, filter?) => {
      return cmd("list_users_paginated", { cursor, limit, filter });
    },
    getUser: (userId) => cmd("get_user", { userId }),
    searchUsers: (query) => cmd("search_users", { query }),
    createUser: (createdBy, data) => cmd("create_user", { data }),
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
    getOrganization: (orgId) => cmd("get_organization", {}),
    updateOrganization: (data) =>
      cmd("update_organization", { data }),
    getSettings: (orgId) => cmd("get_settings", {}),
    updateSettings: (settings) =>
      cmd("update_settings", { settings }),
    getDashboardStats: (orgId) => cmd("get_dashboard_stats", {}),
    registerStudent: (data) => cmd("register_student", { data }),
    listOrgUnits: (orgId) => cmd("list_org_units", {}),
    createOrgUnit: (data) => cmd("create_org_unit", { data }),
    updateOrgUnit: (unitId, data) => cmd("update_org_unit", { unitId, data }),
    deleteOrgUnit: (unitId) => cmd("delete_org_unit", { unitId }),
  },
  // ═══════════════════════════════════════════════════════════════
  // PLATFORM (Desktop-specific)
  // ═══════════════════════════════════════════════════════════════
  platform: {
    // Academic Years
    getCurrentYear: (orgId) => cmd("get_current_academic_year", {}),
    listYears: (orgId) => cmd("list_academic_years", {}),
    getSetupStatus: (orgId) => cmd("get_setup_status", {}),
    initializeSchoolSetup: (payload) =>
      cmd("initialize_school_setup", { payload }),
    getPeriodTemplates: (orgId) => cmd("get_period_templates", {}),
    // Promotion
    listPromotionJobs: (orgId) => cmd("get_promotion_jobs", {}),
    getPromotionJob: (jobId) => cmd("get_promotion_job", { jobId }),
    getPromotionDetails: (jobId) => cmd("get_promotion_details", { jobId }),
    promotionDryRun: (config) =>
      cmd("promotion_dry_run", { config }),
    executePromotion: (jobId) => cmd("execute_promotion", { jobId }),
    rollbackPromotion: (jobId, userId) =>
      cmd("rollback_promotion", { jobId, userId }),
    overridePromotionDecision: (request) =>
      cmd("override_promotion_decision", { request }),
    getPromotionPolicy: (orgId) => cmd("get_promotion_policy", {}),
    // Calendar / Periods
    listCalendarEvents: (periodId) =>
      cmd("get_calendar_events", { periodId }),
    listPendingCalendarEvents: (orgId) =>
      cmd("get_pending_calendar_events", {}),
    createCalendarEvent: (data) => cmd("create_calendar_event", { ...data }),
    executeCalendarEvent: (eventId) => cmd("execute_calendar_event", { eventId }),
    deleteCalendarEvent: (eventId) => cmd("delete_calendar_event", { eventId }),
    generateDefaultCalendar: (periodId) => cmd("generate_default_calendar", { periodId }),
    getOrgPolicy: (orgId) => cmd("get_org_policy", {}),
    updateOrgPolicy: (policy) => cmd("update_org_policy", { policy }),
    listAgentSessions: (orgId) => cmd("list_agent_sessions", {}),
    getAgentSessionLogs: (sessionId) => cmd("get_agent_session_logs", { sessionId }),
    approveAgentAction: (actionLogId, approvedBy) => cmd("approve_agent_action", { actionLogId, approvedBy }),
    rejectAgentAction: (actionLogId, rejectedBy, reason) => cmd("reject_agent_action", { actionLogId, rejectedBy, reason }),
    // Overrides
    listOverrideRequests: (params?) =>
      cmd("list_override_requests", { params }),
    getOverrideRequest: (requestId) =>
      cmd("get_override_request", { requestId }),
    getPendingApprovals: (approverRole) =>
      cmd("get_pending_approvals", { approverRole }),
    processApproval: (requestId, action) =>
      cmd("process_approval", { requestId, action }),
    cancelOverride: (requestId, userId) =>
      cmd("cancel_override", { requestId, userId }),
    // Document Templates
    getTemplateSchemas: () =>
      cmd("get_template_schemas", {}),
    listDocumentTemplates: (orgId, params?) =>
      cmd("list_document_templates", { orgId, params }),
    getDocumentTemplate: (templateId) =>
      cmd("get_document_template", { templateId }),
    createDocumentTemplate: (userId, data) =>
      cmd("create_document_template", { data }),
    deleteDocumentTemplate: (templateId) =>
      cmd("delete_document_template", { templateId }),
    publishDocumentTemplate: (templateId, userId) =>
      cmd("publish_document_template", { templateId, userId }),
    archiveDocumentTemplate: (templateId) =>
      cmd("archive_document_template", { templateId }),
    restoreDocumentTemplate: (templateId) =>
      cmd("restore_document_template", { templateId }),
    updateDocumentTemplate: (templateId, userId, data) =>
      cmd("update_document_template", { templateId, data }),
    // AI Autopilot
    getAutopilotTasks: (orgId) => cmd("autopilot_get_tasks", {}),
    getAutopilotDiagnostics: (orgId) =>
      cmd("autopilot_get_diagnostics", {}),
    runAutopilotTick: (orgId) => cmd("autopilot_run_tick", {}),
    // Auth (desktop login flow)
    login: (identifier, password) =>
      cmd("login", { identifier, passwordHash: password }),
    logout: (token) => cmd("logout_session", { sessionId: token }),
    // Import/Export
    importStudents: (data) => cmd("import_students", { data }),
    importGrades: (data) => cmd("import_grades", { data }),
    createBulkStudents: (userId, data) =>
      cmd("import_students", { data: { students: data } }),
    createBulkGrades: (userId, periodId, data) =>
      cmd("import_grades", { data: { grades: data } }),
    exportStudents: (orgId) => cmd("export_students", {}),
    exportGrades: (orgId) => cmd("export_grades", {}),
    // Audit
    listAuditLogs: (orgId) => cmd("list_audit_logs", {}),
    getAllAuditLogs: (orgId) => cmd("list_audit_logs", {}),
    // Enrollment (extended)
    transitionEnrollmentStatus: (enrollmentId, status) =>
      cmd("transition_enrollment_status", { enrollmentId, status }),
    // Guardians (extended)
    createGuardianLink: (data) => cmd("create_guardian_link", { data }),
    updateGuardianLink: (linkId, data) =>
      cmd("update_guardian_link", { linkId, data }),
    deleteGuardianLink: (linkId) => cmd("delete_guardian_link", { linkId }),
    // Documents
    listDocumentsByOwner: (ownerId) =>
      cmd("list_documents", { ownerId }),
  },
  agent: {
    listSessions: (orgId) => cmd("agent_get_sessions", {}),
    createSession: (type) => cmd("agent_create_session", { type }),
    query: (sessionId: string, text: string, context?: any, onChunk?: (chunk: string) => void) => {
      if (onChunk) {
        const channel = new Channel<string>();
        channel.onmessage = onChunk;
        return cmd("agent_query", { sessionId, text, context, onChunk: channel });
      }
      return cmd("agent_query", { sessionId, text, context });
    },
    openStandaloneWindow: (orgId) => cmd("agent_open_standalone", {}),
  },
  docforge: {
    previewDocumentTemplate: (templateJson, contextJson) =>
      cmd("preview_document_template", { templateJson, contextJson }),
  },
  document: {
    uploadDocument: async (payload) => {
      // In a desktop environment, we might need to read the file to bytes 
      // or pass the file path to Rust. Assuming we pass an array buffer or path.
      // But standard web File API is used here. For large files, Tauri supports HTTP uploads or raw bytes.
      const buffer = await payload.file.arrayBuffer();
      const bytes = Array.from(new Uint8Array(buffer));
      return cmd("upload_document", { 
        fileName: payload.file.name,
        mimeType: payload.file.type,
        bytes,
        entityType: payload.entity_type,
        entityId: payload.entity_id,
        category: payload.category,
        visibility: payload.visibility
      });
    },
    getDocuments: (params) => cmd("get_documents", params),
    deleteDocument: (id) => cmd("delete_document", { id }),
  },
};
