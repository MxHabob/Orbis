/**
 * @orbis/sdk-desktop — OrbisCommands implementation via Tauri IPC.
 *
 * Every method is a thin wrapper around `invoke("command_name", args)`.
 * NO business logic lives here — it all lives in the Rust ops:: layer.
 */
import { invoke } from "@tauri-apps/api/core";
import type { OrbisCommands } from "@orbis/sdk-core";
import type {
  Class, Enrollment, Subject, OrgUnit, ClassSubject, AcademicCycle,
  StudentGuardian, Document, Notification, Organization,
  Grade, Attendance, BehaviorRecord, Assessment,
  GradeReportCard, AttendanceReport, GradeAnalytics,
  Payment, StudentFee, FeeTemplate, Treasury, DiscountTemplate,
  LedgerAccount, JournalEntry, Currency,
  TimeSlot, ScheduledLesson, Room, SolveJob,
  ExamSession, ExamSubject, ExamSchedule, ExamHall, SeatingNumber,
  User, UserRole, UserSession,
} from "@orbis/sdk-core";

// ── Helper ──────────────────────────────────────────────────────
/** Thin wrapper to standardize invoke calls. */
async function cmd<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return invoke<T>(command, args ?? {});
}

// ── Implementation ──────────────────────────────────────────────

export const commands: OrbisCommands = {
  // ═══════════════════════════════════════════════════════════════
  // ACADEMIC
  // ═══════════════════════════════════════════════════════════════
  academic: {
    listClasses: (orgId, cycleId?) => cmd("list_classes", { orgId, cycleId }),
    getClass: (classId) => cmd("get_class", { classId }),
    createClass: (data) => cmd("create_class", { data }),
    updateClass: (classId, data) => cmd("update_class", { classId, data }),
    deleteClass: (classId) => cmd("delete_class", { classId }),
    assignTeacher: (classId, teacherId) => cmd("assign_teacher", { classId, teacherId }),

    listSubjects: (orgId) => cmd("list_subjects", { orgId }),
    getSubject: (subjectId) => cmd("get_subject", { subjectId }),
    createSubject: (data) => cmd("create_subject", { data }),
    updateSubject: (subjectId, data) => cmd("update_subject", { subjectId, data }),
    deleteSubject: (subjectId) => cmd("delete_subject", { subjectId }),

    listOrgUnits: (orgId) => cmd("list_org_units", { orgId }),
    createOrgUnit: (data) => cmd("create_org_unit", { data }),
    updateOrgUnit: (unitId, data) => cmd("update_org_unit", { unitId, data }),
    deleteOrgUnit: (unitId) => cmd("delete_org_unit", { unitId }),

    listEnrollments: (orgId, classId?, cycleId?) => cmd("list_enrollments", { orgId, classId, cycleId }),
    getEnrollment: (orgId, enrollmentId) => cmd("get_enrollment", { orgId, enrollmentId }),
    createEnrollment: (data) => cmd("create_enrollment", { data }),
    withdrawEnrollment: (orgId, enrollmentId, reason) => cmd("withdraw_enrollment", { orgId, enrollmentId, reason }),
    transferEnrollment: (orgId, enrollmentId, newClassId) => cmd("transfer_enrollment", { orgId, enrollmentId, newClassId }),
    checkDuplicate: (studentId, cycleId) => cmd("check_duplicate_enrollment", { studentId, cycleId }),

    listClassSubjects: (orgId, classId, cycleId?) => cmd("list_class_subjects", { orgId, classId, cycleId }),
    listTeacherSubjects: (orgId, teacherId, cycleId?) => cmd("list_teacher_subjects", { orgId, teacherId, cycleId }),
    createClassSubject: (data) => cmd("create_class_subject", { data }),
    updateClassSubject: (id, data) => cmd("update_class_subject", { id, data }),
    deleteClassSubject: (id) => cmd("delete_class_subject", { id }),

    listCycles: (orgId) => cmd("list_cycles", { orgId }),
    getCycle: (cycleId) => cmd("get_cycle", { cycleId }),
    getCurrentCycle: (orgId) => cmd("get_current_cycle", { orgId }),
    createCycle: (data) => cmd("create_cycle", { data }),
    updateCycle: (cycleId, data) => cmd("update_cycle", { cycleId, data }),
    setCurrentCycle: (cycleId) => cmd("set_current_cycle", { cycleId }),
    activateCycle: (orgId, cycleId) => cmd("set_current_cycle", { cycleId }),
    lockCycle: (cycleId) => cmd("lock_cycle", { cycleId }),
    unlockCycle: (cycleId) => cmd("unlock_cycle", { cycleId }),
    toggleGradeEntry: (cycleId, open, start?, end?) => cmd("toggle_grade_entry", { cycleId, open, start, end }),

    listGuardiansByStudent: (orgId, studentId) => cmd("list_guardians_by_student", { orgId, studentId }),
    searchGuardians: (orgId, query) => cmd("search_guardians", { orgId, query }),

    listDocuments: (ownerId?) => cmd("list_documents", { ownerId }),
    createDocument: (data) => cmd("create_document", { orgId: data.orgId || data.org_id, data: { owner_id: data.ownerId || data.owner_id, name: data.name, item_type: data.item_type || data.type || "file", key: data.url || data.key || `doc_${Date.now()}` } }),
    deleteDocument: (orgId, documentId) => cmd("delete_document", { orgId, documentId }),

    listNotifications: (limit?) => cmd("list_notifications", { limit }),
    markNotificationRead: (notificationId) => cmd("mark_notification_read", { notificationId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // ASSESSMENT
  // ═══════════════════════════════════════════════════════════════
  assessment: {
    listGrades: (cycleId, enrollmentId?, classSubjectId?) => cmd("list_grades", { cycleId, enrollmentId, classSubjectId }),
    submitGrade: (cycleId, data) => cmd("submit_grade", { cycleId, data }),
    submitBulkGrades: (cycleId, grades) => cmd("submit_bulk_grades", { cycleId, grades }),
    updateGrade: (gradeId, data) => cmd("update_grade", { gradeId, data }),
    getReportCard: (enrollmentId, cycleId) => cmd("get_report_card", { enrollmentId, cycleId }),
    getGradeAnalytics: (orgId, classId, cycleId) => cmd("get_grade_analytics", { orgId, classId, cycleId }),

    listAttendance: (orgId, cycleId, classId?, date?, enrollmentId?) => cmd("list_attendance", { orgId, cycleId, classId, date, enrollmentId }),
    recordAttendance: (cycleId, data) => cmd("record_attendance", { cycleId, data }),
    recordBulkAttendance: (cycleId, data) => cmd("record_bulk_attendance", { cycleId, data }),
    getAttendanceReport: (cycleId, enrollmentId) => cmd("get_attendance_report", { cycleId, enrollmentId }),

    listBehavior: (orgId, studentId) => cmd("list_behavior", { orgId, studentId }),
    updateBehavior: (orgId, recordId, data) => cmd("update_behavior", { orgId, recordId, data }),
    deleteBehavior: (orgId, recordId) => cmd("delete_behavior", { orgId, recordId }),
    submitBehavior: (orgId, data) => cmd("submit_behavior", { orgId, data }),

    listAssessments: (cycleId, classSubjectId?) => cmd("list_assessments", { cycleId, classSubjectId }),
    createAssessment: (data) => cmd("create_assessment", { data }),
    updateAssessment: (assessmentId, data) => cmd("update_assessment", { assessmentId, data }),
    deleteAssessment: (assessmentId) => cmd("delete_assessment", { assessmentId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listLedgerAccounts: (orgId) => cmd("list_ledger_accounts", { orgId }),
    listTransactionAccounts: () => cmd("list_transaction_accounts"),
    createLedgerAccount: (data) => cmd("create_ledger_account", { data }),
    getAccountStatement: (accountId, startDate?, endDate?) => cmd("get_account_statement", { accountId, startDate, endDate }),

    listTreasuries: () => cmd("list_treasuries"),
    createTreasury: (data) => cmd("create_treasury", { data }),

    listFeeTemplates: (orgId) => cmd("list_fee_templates", { orgId }),
    getFeeTemplates: (orgId) => cmd("list_fee_templates", { orgId }),
    listActiveFeeTemplates: (orgId) => cmd("list_active_fee_templates", { orgId }),
    createFeeTemplate: (data) => cmd("create_fee_template", { data }),
    updateFeeTemplate: (templateId, data) => cmd("update_fee_template", { templateId, data }),
    deleteFeeTemplate: (templateId) => cmd("delete_fee_template", { templateId }),

    listStudentFees: (enrollmentId) => cmd("list_student_fees", { enrollmentId }),
    assignStudentFee: (data) => cmd("assign_student_fee", { data }),

    listPayments: (enrollmentId?, feeId?) => cmd("list_payments", { enrollmentId, feeId }),
    getPayments: (orgId, studentId?) => cmd("list_payments", { orgId, enrollmentId: studentId }),
    recordPayment: (data) => cmd("record_payment", { data }),

    listDiscountTemplates: (orgId) => cmd("list_discount_templates", { orgId }),
    getDiscountTemplates: (orgId) => cmd("list_discount_templates", { orgId }),
    createDiscountTemplate: (data) => cmd("create_discount_template", { data }),

    getCurrencies: (orgId) => commands.finance.listCurrencies(),
    listCurrencies: async () => {
      const stored = typeof window !== "undefined" ? window.localStorage.getItem("orbis_currencies") : null;
      if (stored) {
        try { return JSON.parse(stored); } catch {}
      }
      const defaults = [
        { code: "SAR", name: "Saudi Riyal", symbol: "ر.س", symbol_position: "after", decimal_places: 2, exchange_rate: 1.0, is_base: true },
        { code: "USD", name: "US Dollar", symbol: "$", symbol_position: "before", decimal_places: 2, exchange_rate: 3.75, is_base: false },
        { code: "EUR", name: "Euro", symbol: "€", symbol_position: "before", decimal_places: 2, exchange_rate: 4.0, is_base: false }
      ];
      if (typeof window !== "undefined") {
        window.localStorage.setItem("orbis_currencies", JSON.stringify(defaults));
      }
      return defaults;
    },
    createCurrency: async (data: any) => {
      if (typeof window === "undefined") return data;
      const currencies = await commands.finance.listCurrencies();
      const newCurr = {
        code: data.code,
        name: data.name,
        symbol: data.symbol,
        symbol_position: data.symbol_position || "after",
        decimal_places: data.decimal_places || 2,
        exchange_rate: Number(data.exchange_rate) || 1.0,
        is_base: !!data.is_base
      };
      if (newCurr.is_base) {
        currencies.forEach(c => c.is_base = false);
      }
      currencies.push(newCurr as any);
      window.localStorage.setItem("orbis_currencies", JSON.stringify(currencies));
      return newCurr as any;
    },
    updateCurrency: async (orgId, code, data) => {
      if (typeof window === "undefined") return {} as any;
      const currencies = await commands.finance.listCurrencies();
      const target = currencies.find(c => c.code === code);
      if (target) {
        if (data.exchange_rate !== undefined) target.exchange_rate = Number(data.exchange_rate);
        window.localStorage.setItem("orbis_currencies", JSON.stringify(currencies));
        return target;
      }
      throw new Error("Currency not found");
    },
    deleteCurrency: async (orgId, code) => {
      if (typeof window === "undefined") return;
      let currencies = await commands.finance.listCurrencies();
      currencies = currencies.filter(c => c.code !== code);
      window.localStorage.setItem("orbis_currencies", JSON.stringify(currencies));
    },
    seedDefaultFinancials: async (orgId) => {
      return;
    },

    listJournalEntries: () => cmd("list_journal_entries"),
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (orgId, cycleId) => cmd("list_time_slots", { orgId, cycleId }),
    createTimeSlot: (data) => cmd("create_time_slot", { data }),
    updateTimeSlot: (slotId, data) => cmd("update_time_slot", { slotId, data }),
    deleteTimeSlot: (slotId) => cmd("delete_time_slot", { slotId }),

    listLessonsByClass: (orgId, cycleId, classId) => cmd("list_lessons_by_class", { orgId, cycleId, classId }),
    listLessonsByTeacher: (orgId, cycleId, teacherId) => cmd("list_lessons_by_teacher", { orgId, cycleId, teacherId }),
    createLesson: (data) => cmd("create_lesson", { data }),
    updateLesson: (lessonId, data) => cmd("update_lesson", { lessonId, data }),
    deleteLesson: (lessonId) => cmd("delete_lesson", { lessonId }),
    checkConflicts: (orgId, cycleId, teacherId, timeSlotId, dayOfWeek, roomId?) => cmd("check_schedule_conflicts", { orgId, cycleId, teacherId, timeSlotId, dayOfWeek, roomId }),

    listRooms: (orgId) => cmd("list_rooms", { orgId }),
    createRoom: (data) => cmd("create_room", { data }),
    updateRoom: (roomId, data) => cmd("update_room", { roomId, data }),
    deleteRoom: (roomId) => cmd("delete_room", { roomId }),

    startSolver: (cycleId, mode?) => cmd("start_solver", { cycleId, mode }),
    getSolverJob: (jobId) => cmd("get_solver_job", { jobId }),
    listSolverJobs: (cycleId) => cmd("list_solver_jobs", { cycleId }),
    cancelSolver: (jobId) => cmd("cancel_solver", { jobId }),
    applySolver: (jobId) => cmd("apply_solver", { jobId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // EXAM
  // ═══════════════════════════════════════════════════════════════
  exam: {
    listSessions: (cycleId) => cmd("list_exam_sessions", { cycleId }),
    getSession: (sessionId) => cmd("get_exam_session", { sessionId }),
    createSession: (data) => cmd("create_exam_session", { data }),
    updateSession: (sessionId, data) => cmd("update_exam_session", { sessionId, data }),

    listExamSubjects: (sessionId) => cmd("list_exam_subjects", { sessionId }),
    createExamSubject: (data) => cmd("create_exam_subject", { data }),

    listHalls: () => cmd("list_exam_halls"),
    createHall: (data) => cmd("create_exam_hall", { data }),

    listSchedule: (sessionId) => cmd("list_exam_schedule", { sessionId }),
    createScheduleEntry: (data) => cmd("create_exam_schedule_entry", { data }),

    listSeating: (examSubjectId) => cmd("list_seating_numbers", { examSubjectId }),
    generateOrdering: (sessionId) => cmd("generate_exam_ordering", { sessionId }),
    getOrdering: async (orgId, cycleId) => {
      const stored = typeof window !== "undefined" ? window.localStorage.getItem(`orbis_exam_ordering_${cycleId}`) : null;
      if (stored) {
        try { return JSON.parse(stored); } catch {}
      }
      return [];
    },
    rebuildOrdering: async (orgId, cycleId, strategy) => {
      if (typeof window === "undefined") return [];
      const enrollments = await commands.academic.listEnrollments(orgId);
      const classes = await commands.academic.listClasses(orgId, cycleId);
      const classMap = new Map(classes.map(c => [c.id, c.name_ar || c.name]));
      
      const students = enrollments.map(e => ({
        id: e.id,
        student_id: e.student_id,
        student_name_ar: e.full_name || e.name_ar || "طالب",
        student_name_en: e.name_en || "Student",
        academic_number: e.student_number || e.id,
        class_name: classMap.get(e.class_id) || "الصف",
        status: e.status || "active"
      }));

      if (strategy === "alphabetical_ar") {
        students.sort((a, b) => a.student_name_ar.localeCompare(b.student_name_ar, "ar"));
      } else if (strategy === "alphabetical_en") {
        students.sort((a, b) => a.student_name_en.localeCompare(b.student_name_en, "en"));
      }
      
      const ordered = students.map((s, idx) => ({
        ...s,
        ordinal: idx + 1
      }));

      window.localStorage.setItem(`orbis_exam_ordering_${cycleId}`, JSON.stringify(ordered));
      return ordered;
    },
    freezeOrdering: async (orgId, cycleId, userId) => {
      if (typeof window !== "undefined") {
        window.localStorage.setItem(`orbis_exam_ordering_frozen_${cycleId}`, "true");
      }
    },
  },

  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, ip?) => cmd("login", { identifier, passwordHash, ip }),
    logout: (sessionId) => cmd("logout_session", { sessionId }),
    changePassword: (userId, data) => {
      const hash = typeof data === "string" ? data : data.new_password;
      return cmd("change_password", { userId, data: { new_password_hash: hash } });
    },
    listSessions: (userId) => cmd("list_user_sessions", { userId }),

    listUsers: async (orgId, filter?) => {
      const users = await cmd<User[]>("list_users", { orgId });
      if (filter) {
        return users.filter(u => {
          if (filter.role && u.role !== filter.role && u.primary_role !== filter.role) return false;
          if (filter.status && u.status !== filter.status) return false;
          return true;
        });
      }
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
  },

  // ═══════════════════════════════════════════════════════════════
  // ORGANIZATION
  // ═══════════════════════════════════════════════════════════════
  organization: {
    getOrganization: (orgId) => cmd("get_organization", { orgId }),
    updateOrganization: (orgId, data) => cmd("update_organization", { orgId, data }),
    getSettings: (orgId) => cmd("get_settings", { orgId }),
    updateSettings: (orgId, settings) => cmd("update_settings", { orgId, settings }),
    getDashboardStats: (orgId) => cmd("get_dashboard_stats", { orgId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // PLATFORM (Desktop-specific)
  // ═══════════════════════════════════════════════════════════════
  platform: {
    // Academic Years
    getCurrentYear: () => cmd("get_current_academic_year"),
    listYears: (orgId) => cmd("list_academic_years", { orgId }),
    switchYear: (yearId) => cmd("switch_academic_year", { yearId }),
    isYearArchived: () => cmd("is_year_archived"),
    getAcademicContext: (orgId) => cmd("get_academic_context", { orgId }),
    getSetupStatus: (orgId) => cmd("get_setup_status", { orgId }),

    // Promotion
    listPromotionJobs: (orgId) => cmd("get_promotion_jobs", { orgId }),
    getPromotionJob: (jobId) => cmd("get_promotion_job", { jobId }),
    getPromotionDetails: (jobId) => cmd("get_promotion_details", { jobId }),
    promotionDryRun: (orgId, config) => cmd("promotion_dry_run", { orgId, config }),
    executePromotion: (jobId) => cmd("execute_promotion", { jobId }),
    rollbackPromotion: (jobId, userId) => cmd("rollback_promotion", { jobId, userId }),
    overridePromotionDecision: (request) => cmd("override_promotion_decision", { request }),
    getPromotionPolicy: (orgId) => cmd("get_promotion_policy", { orgId }),

    // Calendar
    listCalendarEvents: (orgId, cycleId) => cmd("get_calendar_events", { orgId, cycleId }),
    listPendingCalendarEvents: (orgId) => cmd("get_pending_calendar_events", { orgId }),
    getOrgPolicy: (orgId) => cmd("get_org_policy", { orgId }),

    // Overrides
    listOverrideRequests: (orgId, params?) => cmd("list_override_requests", { orgId, params }),
    getOverrideRequest: (requestId) => cmd("get_override_request", { requestId }),
    getPendingApprovals: (orgId, approverRole) => cmd("get_pending_approvals", { orgId, approverRole }),
    processApproval: (requestId, action) => cmd("process_approval", { requestId, action }),
    cancelOverride: (requestId, userId) => cmd("cancel_override", { requestId, userId }),

    // Document Templates
    listDocumentTemplates: (orgId, params?) => cmd("get_document_templates", { orgId, params }),
    getDocumentTemplate: (orgId, templateId) => cmd("get_document_template", { orgId, templateId }),
    createDocumentTemplate: (orgId, userId, data) => cmd("create_document_template", { orgId, data }),
    deleteDocumentTemplate: (orgId, templateId) => cmd("delete_document_template", { orgId, templateId }),
    publishDocumentTemplate: (orgId, templateId, userId) => cmd("publish_document_template", { orgId, templateId, userId }),
    archiveDocumentTemplate: (orgId, templateId) => cmd("archive_document_template", { orgId, templateId }),
    restoreDocumentTemplate: (orgId, templateId) => cmd("restore_document_template", { orgId, templateId }),
    updateDocumentTemplate: (orgId, templateId, userId, data) => cmd("update_document_template", { orgId, templateId, data }),

    // AI Autopilot
    getAutopilotStatus: (orgId) => cmd("autopilot_get_status", { orgId }),
    getAutopilotTasks: (orgId) => cmd("autopilot_get_tasks", { orgId }),
    getAutopilotDiagnostics: (orgId) => cmd("autopilot_get_diagnostics", { orgId }),
    configureAutopilot: (orgId, isEnabled, paused, mode) => cmd("autopilot_configure", { orgId, isEnabled, paused, mode }),
    runAutopilotTick: (orgId) => cmd("autopilot_run_tick", { orgId }),

    // AI Agent
    listAgentSessions: (orgId) => cmd("agent_get_sessions", { orgId }),
    getAgentSessionLogs: (orgId, sessionId) => cmd("agent_get_session_logs", { orgId, sessionId }),
    approveAgentAction: (orgId, actionLogId, approvedBy) => cmd("agent_approve_action", { orgId, actionLogId, approvedBy }),
    rejectAgentAction: (orgId, actionLogId, rejectedBy, reason) => cmd("agent_reject_action", { orgId, actionLogId, rejectedBy, reason }),

    // Auth (desktop login flow)
    login: (identifier, password) => cmd("login", { identifier, passwordHash: password }),
    logout: (token) => cmd("logout_session", { sessionId: token }),

    // Import/Export
    importStudents: (orgId, data) => cmd("import_students", { orgId, data }),
    importGrades: (orgId, data) => cmd("import_grades", { orgId, data }),
    exportStudents: (orgId) => cmd("export_students", { orgId }),
    exportGrades: (orgId) => cmd("export_grades", { orgId }),

    // Audit
    listAuditLogs: (orgId) => cmd("list_audit_logs", { orgId }),

    // Enrollment (extended)
    transitionEnrollmentStatus: (orgId, enrollmentId, status) => cmd("transition_enrollment_status", { orgId, enrollmentId, status }),

    // Guardians (extended)
    createGuardianLink: (data) => cmd("create_guardian_link", { data }),
    updateGuardianLink: (linkId, data) => cmd("update_guardian_link", { linkId, data }),
    deleteGuardianLink: (linkId) => cmd("delete_guardian_link", { linkId }),

    // Documents
    listDocumentsByOwner: (orgId, ownerId) => cmd("list_documents", { orgId, ownerId }),
  },
  docforge: {
    previewDocumentTemplate: (templateJson, contextJson) => cmd("preview_document_template", { templateJson, contextJson }),
  },
  permissions: {
    getRolePermissions: async () => {
      const stored = typeof window !== "undefined" ? window.localStorage.getItem("orbis_role_permissions") : null;
      if (stored) {
        try { return JSON.parse(stored); } catch {}
      }
      return [
        { role: "super_admin", document_type: "*", level: 3, only_if_creator: false, actions: ["*"] },
        { role: "admin", document_type: "*", level: 2, only_if_creator: false, actions: ["read", "write"] },
        { role: "teacher", document_type: "student", level: 1, only_if_creator: false, actions: ["read"] },
        { role: "teacher", document_type: "grades", level: 1, only_if_creator: false, actions: ["read", "write"] },
        { role: "accountant", document_type: "finance", level: 2, only_if_creator: false, actions: ["*"] }
      ];
    },
    saveRolePermissions: async (role, rules) => {
      if (typeof window === "undefined") return;
      const stored = window.localStorage.getItem("orbis_role_permissions");
      let allRules: any[] = [];
      if (stored) {
        try { allRules = JSON.parse(stored); } catch {}
      }
      allRules = allRules.filter(r => r.role !== role).concat(rules);
      window.localStorage.setItem("orbis_role_permissions", JSON.stringify(allRules));
    }
  },
  bulkImport: {
    createStudents: (orgId, userId, data) => commands.platform.importStudents(orgId, { students: data }),
    createGrades: (orgId, userId, cycleId, data) => commands.platform.importGrades(orgId, { grades: data }),
  },
  auditLogs: {
    getAll: (orgId) => commands.platform.listAuditLogs(orgId),
  }
};
