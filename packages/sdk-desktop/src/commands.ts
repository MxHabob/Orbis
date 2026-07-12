/**
 * @orbis/sdk-desktop — OrbisCommands implementation via Tauri IPC.
 *
 * Every method is a thin wrapper around `invoke("command_name", args)`.
 * NO business logic lives here — it all lives in the Rust ops:: layer.
 */
import { invoke } from "@tauri-apps/api/core";
import type { OrbisCommands } from "@orbis/sdk-core";
import type {
  Class, Enrollment, Subject, OrgUnit, ClassSubject, AcademicPeriod,
  StudentGuardian, Document, Notification, Organization,
  Grade, Attendance, BehaviorRecord, Assessment,
  GradeReportCard, AttendanceReport, GradeAnalytics,
  Payment, StudentFee, FeeTemplate, Treasury, DiscountTemplate,
  Account, JournalEntry, Currency,
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
    listClasses: (orgId, yearId?) => cmd("list_classes", { orgId, yearId }),
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

    listEnrollments: (orgId, classId?, yearId?) => cmd("list_enrollments", { orgId, classId, yearId }),
    getEnrollment: (orgId, enrollmentId) => cmd("get_enrollment", { orgId, enrollmentId }),
    createEnrollment: (data) => cmd("create_enrollment", { data }),
    withdrawEnrollment: (orgId, enrollmentId, reason) => cmd("withdraw_enrollment", { orgId, enrollmentId, reason }),
    transferEnrollment: (orgId, enrollmentId, newClassId) => cmd("transfer_enrollment", { orgId, enrollmentId, newClassId }),
    checkDuplicate: (studentId, yearId) => cmd("check_duplicate_enrollment", { studentId, yearId }),

    listClassSubjects: (orgId, classId, periodId?) => cmd("list_class_subjects", { orgId, classId, periodId }),
    listTeacherSubjects: (orgId, teacherId, periodId?) => cmd("list_teacher_subjects", { orgId, teacherId, periodId }),
    createClassSubject: (data) => cmd("create_class_subject", { data }),
    updateClassSubject: (id, data) => cmd("update_class_subject", { id, data }),
    deleteClassSubject: (id) => cmd("delete_class_subject", { id }),

    listPeriods: (orgId) => cmd("list_periods", { orgId }),
    listPeriodsByYear: (orgId, yearId) => cmd("list_periods_by_year", { orgId, yearId }),
    getPeriod: (periodId) => cmd("get_period", { periodId: periodId }),
    createPeriod: (data) => cmd("create_period", { data }),
    lockPeriod: (orgId, periodId) => cmd("lock_period", { orgId, periodId: periodId }),
    unlockPeriod: (orgId, periodId) => cmd("unlock_period", { orgId, periodId }),
    togglePeriodEnrollment: (orgId, periodId, open) => cmd("toggle_period_enrollment", { orgId, periodId, open }),
    togglePeriodGradeEntry: (orgId, periodId, open) => cmd("toggle_period_grade_entry", { orgId, periodId, open }),

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
    listGrades: (periodId, enrollmentId?, classSubjectId?) => cmd("list_grades", { periodId, enrollmentId, classSubjectId }),
    submitGrade: (periodId, data) => cmd("submit_grade", { periodId, data }),
    submitBulkGrades: (periodId, grades) => cmd("submit_bulk_grades", { periodId, grades }),
    updateGrade: (gradeId, data) => cmd("update_grade", { gradeId, data }),
    getReportCard: (enrollmentId, periodId) => cmd("get_report_card", { enrollmentId, periodId }),
    getGradeAnalytics: (orgId, classId, periodId) => cmd("get_grade_analytics", { orgId, classId, periodId }),

    listAttendance: (orgId, periodId, classId?, date?, enrollmentId?) => cmd("list_attendance", { orgId, periodId, classId, date, enrollmentId }),
    recordAttendance: (periodId, data) => cmd("record_attendance", { periodId, data }),
    recordBulkAttendance: (periodId, data) => cmd("record_bulk_attendance", { periodId, data }),
    getAttendanceReport: (periodId, enrollmentId) => cmd("get_attendance_report", { periodId, enrollmentId }),

    listBehavior: (orgId, studentId) => cmd("list_behavior", { orgId, studentId }),
    updateBehavior: (orgId, recordId, data) => cmd("update_behavior", { orgId, recordId, data }),
    deleteBehavior: (orgId, recordId) => cmd("delete_behavior", { orgId, recordId }),
    submitBehavior: (orgId, data) => cmd("submit_behavior", { orgId, data }),

    listAssessments: (periodId, classSubjectId?) => cmd("list_assessments", { periodId, classSubjectId }),
    createAssessment: (data) => cmd("create_assessment", { data }),
    updateAssessment: (assessmentId, data) => cmd("update_assessment", { assessmentId, data }),
    deleteAssessment: (assessmentId) => cmd("delete_assessment", { assessmentId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listAccounts: (orgId) => cmd("list_ledger_accounts", { orgId }),
    listTransactionAccounts: (orgId) => cmd("list_transaction_accounts", { orgId }),
    createAccount: (data) => cmd("create_ledger_account", { data }),
    getAccountStatement: (accountId, startDate?, endDate?) => cmd("get_account_statement", { accountId, startDate, endDate }),

    listTreasuries: (orgId) => cmd("list_treasuries", { orgId }),
    createTreasury: (data) => cmd("create_treasury", { data }),

    listFeeTemplates: (orgId) => cmd("list_fee_templates", { orgId }),
    listActiveFeeTemplates: (orgId) => cmd("list_active_fee_templates", { orgId }),
    createFeeTemplate: (data) => cmd("create_fee_template", { data }),
    updateFeeTemplate: (templateId, data) => cmd("update_fee_template", { templateId, data }),
    deleteFeeTemplate: (templateId) => cmd("delete_fee_template", { templateId }),

    listStudentFees: (enrollmentId) => cmd("list_student_fees", { enrollmentId }),
    assignStudentFee: (data) => cmd("assign_student_fee", { data }),

    listPayments: (enrollmentId?, feeId?) => cmd("list_payments", { enrollmentId, feeId }),
    recordPayment: (data) => cmd("record_payment", { data }),

    listDiscountTemplates: (orgId) => cmd("list_discount_templates", { orgId }),
    createDiscountTemplate: (data) => cmd("create_discount_template", { data }),
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

    listJournalEntries: (orgId) => cmd("list_journal_entries", { orgId }),

    listFiscalYears: async (orgId) => {
      const stored = typeof window !== "undefined" ? window.localStorage.getItem("orbis_fiscal_years") : null;
      if (stored) {
        try { return JSON.parse(stored); } catch {}
      }
      return [];
    },
    createFiscalYear: async (orgId) => {
      if (typeof window === "undefined") return {} as any;
      const years = await commands.finance.listFiscalYears(orgId);
      const yearStr = new Date().getFullYear().toString();
      const newYear = {
        id: "fy_" + Date.now(),
        org_id: orgId,
        name: `السنة المالية ${yearStr}`,
        start_date: `${yearStr}-01-01`,
        end_date: `${yearStr}-12-31`,
        status: "open",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      };
      years.push(newYear as any);
      window.localStorage.setItem("orbis_fiscal_years", JSON.stringify(years));
      return newYear as any;
    },
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (orgId, yearId) => cmd("list_time_slots", { orgId, yearId }),
    createTimeSlot: (data) => cmd("create_time_slot", { data }),
    updateTimeSlot: (slotId, data) => cmd("update_time_slot", { slotId, data }),
    deleteTimeSlot: (slotId) => cmd("delete_time_slot", { slotId }),

    listLessonsByClass: (orgId, yearId, classId) => cmd("list_lessons_by_class", { orgId, yearId, classId }),
    listLessonsByTeacher: (orgId, yearId, teacherId) => cmd("list_lessons_by_teacher", { orgId, yearId, teacherId }),
    createLesson: (data) => cmd("create_lesson", { data }),
    updateLesson: (lessonId, data) => cmd("update_lesson", { lessonId, data }),
    deleteLesson: (lessonId) => cmd("delete_lesson", { lessonId }),
    checkConflicts: (orgId, yearId, teacherId, timeSlotId, dayOfWeek, roomId?) => cmd("check_schedule_conflicts", { orgId, yearId, teacherId, timeSlotId, dayOfWeek, roomId }),

    listRooms: (orgId) => cmd("list_rooms", { orgId }),
    createRoom: (data) => cmd("create_room", { data }),
    updateRoom: (roomId, data) => cmd("update_room", { roomId, data }),
    deleteRoom: (roomId) => cmd("delete_room", { roomId }),

    startSolver: (yearId, mode?) => cmd("start_solver", { yearId, mode }),
    getSolverJob: (jobId) => cmd("get_solver_job", { jobId }),
    listSolverJobs: (yearId) => cmd("list_solver_jobs", { yearId }),
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
    updateSession: (sessionId, data) => cmd("update_exam_session", { sessionId, data }),

    listExamSubjects: (sessionId) => cmd("list_exam_subjects", { sessionId }),
    createExamSubject: (data) => cmd("create_exam_subject", { data }),

    listHalls: () => cmd("list_exam_halls"),
    createHall: (data) => cmd("create_exam_hall", { data }),

    listSchedule: (sessionId) => cmd("list_exam_schedule", { sessionId }),
    createScheduleEntry: (data) => cmd("create_exam_schedule_entry", { data }),

    listSeating: (examSubjectId) => cmd("list_seating_numbers", { examSubjectId }),
    generateOrdering: (sessionId) => cmd("generate_exam_ordering", { sessionId }),
    getOrdering: async (orgId, periodId) => {
      const stored = typeof window !== "undefined" ? window.localStorage.getItem(`orbis_exam_ordering_${periodId}`) : null;
      if (stored) {
        try { return JSON.parse(stored); } catch {}
      }
      return [];
    },
    rebuildOrdering: async (orgId, periodId, strategy) => {
      if (typeof window === "undefined") return [];
      const enrollments = await commands.academic.listEnrollments(orgId);
      const classes = await commands.academic.listClasses(orgId, periodId);
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

      window.localStorage.setItem(`orbis_exam_ordering_${periodId}`, JSON.stringify(ordered));
      return ordered;
    },
    freezeOrdering: async (orgId, periodId, userId) => {
      if (typeof window !== "undefined") {
        window.localStorage.setItem(`orbis_exam_ordering_frozen_${periodId}`, "true");
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
    getCurrentYear: (orgId) => cmd("get_current_academic_year", { orgId }),
    listYears: (orgId) => cmd("list_academic_years", { orgId }),
    switchYear: (yearId) => cmd("switch_academic_year", { yearId }),
    isYearArchived: (yearId) => cmd("is_year_archived", { yearId }),
    getAcademicContext: (orgId) => cmd("get_academic_context", { orgId }),
    getSetupStatus: (orgId) => cmd("get_setup_status", { orgId }),
    initializeSchoolSetup: (orgId, payload) => cmd("initialize_school_setup", { orgId, payload }),

    // Promotion
    listPromotionJobs: (orgId) => cmd("get_promotion_jobs", { orgId }),
    getPromotionJob: (jobId) => cmd("get_promotion_job", { jobId }),
    getPromotionDetails: (jobId) => cmd("get_promotion_details", { jobId }),
    promotionDryRun: (orgId, config) => cmd("promotion_dry_run", { orgId, config }),
    executePromotion: (jobId) => cmd("execute_promotion", { jobId }),
    rollbackPromotion: (jobId, userId) => cmd("rollback_promotion", { jobId, userId }),
    overridePromotionDecision: (request) => cmd("override_promotion_decision", { request }),
    getPromotionPolicy: (orgId) => cmd("get_promotion_policy", { orgId }),

    // Calendar / Periods
    listCalendarEvents: (orgId, yearId) => cmd("get_calendar_events", { orgId, yearId }),
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
    createBulkStudents: (orgId, userId, data) => cmd("import_students", { orgId, data: { students: data } }),
    createBulkGrades: (orgId, userId, periodId, data) => cmd("import_grades", { orgId, data: { grades: data } }),
    exportStudents: (orgId) => cmd("export_students", { orgId }),
    exportGrades: (orgId) => cmd("export_grades", { orgId }),

    // Audit
    listAuditLogs: (orgId) => cmd("list_audit_logs", { orgId }),
    getAllAuditLogs: (orgId) => cmd("list_audit_logs", { orgId }),

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
  }
};
