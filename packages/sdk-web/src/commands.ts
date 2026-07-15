/**
 * @orbis/sdk-web — OrbisCommands implementation via HTTP fetch.
 *
 * Mirror of @orbis/sdk-desktop but calls API Gateway endpoints
 * instead of Tauri IPC. Same interface, different transport.
 */
import type { OrbisCommands } from "@orbis/sdk-core";
import { http } from "./http";

export const commands: OrbisCommands = {
  // ═══════════════════════════════════════════════════════════════
  // ACADEMIC
  // ═══════════════════════════════════════════════════════════════
  academic: {
    listClasses: (_orgId, _periodId?) => http.get("/api/classes"),
    getClass: (classId) => http.get(`/api/classes/${classId}`),
    createClass: (data) => http.post("/api/classes", data),
    updateClass: (classId, data) => http.put(`/api/classes/${classId}`, data),
    deleteClass: (classId) => http.del(`/api/classes/${classId}`),
    assignTeacher: (classId, teacherId) =>
      http.post(`/api/classes/${classId}/teacher`, { teacherId }),

    listSubjects: (_orgId) => http.get("/api/subjects"),
    getSubject: (subjectId) => http.get(`/api/subjects/${subjectId}`),
    createSubject: (data) => http.post("/api/subjects", data),
    updateSubject: (subjectId, data) =>
      http.put(`/api/subjects/${subjectId}`, data),
    deleteSubject: (subjectId) => http.del(`/api/subjects/${subjectId}`),

    listOrgUnits: (_orgId) => http.get("/api/units"),
    createOrgUnit: (data) => http.post("/api/units", data),
    updateOrgUnit: (unitId, data) => http.put(`/api/units/${unitId}`, data),
    deleteOrgUnit: (unitId) => http.del(`/api/units/${unitId}`),

    listEnrollments: (_orgId, _classId?, _periodId?) =>
      http.get("/api/enrollments"),
    getEnrollment: (enrollmentId) =>
      http.get(`/api/enrollments/${enrollmentId}`),
    createEnrollment: (data) => http.post("/api/enrollments", data),
    withdrawEnrollment: (enrollmentId, reason) =>
      http.post(`/api/enrollments/${enrollmentId}/withdraw`, { reason }),
    transferEnrollment: (enrollmentId, newClassId) =>
      http.post(`/api/enrollments/${enrollmentId}/transfer`, {
        class_id: newClassId,
      }),
    checkDuplicate: (studentId, periodId) =>
      http.get(
        `/api/enrollments/check-duplicate?student_id=${studentId}&period_id=${periodId}`,
      ),

    listClassSubjects: (classId, periodId) =>
      http.get(`/api/class-subjects?class_id=${classId}&period_id=${periodId}`),
    listTeacherSubjects: (teacherId, periodId) =>
      http.get(
        `/api/teacher-subjects?teacher_id=${teacherId}&period_id=${periodId}`,
      ),
    createClassSubject: (data) => http.post("/api/class-subjects", data),
    updateClassSubject: (id, data) =>
      http.put(`/api/class-subjects/${id}`, data),
    deleteClassSubject: (id) => http.del(`/api/class-subjects/${id}`),

    listCycles: (_orgId) => http.get("/api/periods"),
    listPeriodsByYear: (_orgId, periodId) =>
      http.get(`/api/periods/year/${periodId}`),
    getCycle: (periodId) => http.get(`/api/periods/${periodId}`),
    createCycle: (data) => http.post("/api/periods", data),
    lockCycle: (_orgId, periodId) => http.post(`/api/periods/${periodId}/lock`),
    unlockPeriod: (_orgId, periodId) =>
      http.post(`/api/periods/${periodId}/unlock`),
    togglePeriodEnrollment: (_orgId, periodId, open) =>
      http.post(
        `/api/periods/${periodId}/enrollment/${open ? "open" : "close"}`,
      ),
    togglePeriodGradeEntry: (_orgId, periodId, open) =>
      http.post(
        `/api/periods/${periodId}/grade-entry/${open ? "open" : "close"}`,
      ),

    listGuardiansByStudent: (studentId) =>
      http.get(`/api/guardians/${studentId}`),
    searchGuardians: (query) => http.get(`/api/guardians/search?q=${query}`),

    listDocuments: (ownerId?) =>
      http.get(`/api/documents${ownerId ? `/${ownerId}` : ""}`),

    listNotifications: (_limit?) => http.get("/api/notifications"),
    markNotificationRead: (notificationId) =>
      http.post(`/api/notifications/${notificationId}/read`),
  },

  // ═══════════════════════════════════════════════════════════════
  // ASSESSMENT
  // ═══════════════════════════════════════════════════════════════
  assessment: {
    listGrades: (periodId, enrollmentId?, classSubjectId?) =>
      http.get(
        `/api/grades?period_id=${periodId}${enrollmentId ? `&enrollment_id=${enrollmentId}` : ""}${classSubjectId ? `&class_subject_id=${classSubjectId}` : ""}`,
      ),
    submitGrade: (_periodId, data) => http.post("/api/grades", data),
    submitBulkGrades: (_periodId, grades) =>
      http.post("/api/grades/bulk", grades),
    updateGrade: (gradeId, data) => http.put(`/api/grades/${gradeId}`, data),
    getReportCard: (enrollmentId, periodId) =>
      http.get(`/api/report-card/${enrollmentId}/${periodId}`),
    getGradeAnalytics: (periodId, classSubjectId) =>
      http.get(
        `/api/grades/analytics?period_id=${periodId}&class_subject_id=${classSubjectId}`,
      ),

    listAttendance: (_periodId, classId?, date?, _enrollmentId?) =>
      http.get(`/api/attendance/${classId ?? "all"}/${date ?? "today"}`),
    recordAttendance: (_periodId, data) => http.post("/api/attendance", data),
    recordBulkAttendance: (_periodId, data) =>
      http.post("/api/attendance/bulk", data),
    getAttendanceReport: (_periodId, enrollmentId) =>
      http.get(`/api/attendance/report/${enrollmentId}`),

    listBehavior: (_periodId, enrollmentId?) =>
      http.get(`/api/behavior${enrollmentId ? `/${enrollmentId}` : ""}`),
    updateBehavior: (recordId, data) =>
      http.put(`/api/behavior/${recordId}`, data),
    deleteBehavior: (recordId) => http.del(`/api/behavior/${recordId}`),

    listAssessments: (_periodId, classSubjectId?) =>
      http.get(
        `/api/assessments${classSubjectId ? `?class_subject_id=${classSubjectId}` : ""}`,
      ),
    createAssessment: (data) => http.post("/api/assessments", data),
    updateAssessment: (assessmentId, data) =>
      http.put(`/api/assessments/${assessmentId}`, data),
    deleteAssessment: (assessmentId) =>
      http.del(`/api/assessments/${assessmentId}`),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listAccounts: () => http.get("/api/ledger-accounts"),
    listTransactionAccounts: () =>
      http.get("/api/ledger-accounts?type=transaction"),
    createAccount: (data) => http.post("/api/ledger-accounts", data),
    getAccountStatement: (accountId, startDate?, endDate?) =>
      http.get(
        `/api/ledger-accounts/${accountId}/statement?start=${startDate ?? ""}&end=${endDate ?? ""}`,
      ),

    listTreasuries: () => http.get("/api/treasuries"),
    createTreasury: (data) => http.post("/api/treasuries", data),

    listFeeTemplates: (_orgId) => http.get("/api/fee-templates"),
    listActiveFeeTemplates: (_orgId) =>
      http.get("/api/fee-templates?active=true"),
    createFeeTemplate: (data) => http.post("/api/fee-templates", data),
    updateFeeTemplate: (templateId, data) =>
      http.put(`/api/fee-templates/${templateId}`, data),
    deleteFeeTemplate: (templateId) =>
      http.del(`/api/fee-templates/${templateId}`),

    listStudentFees: (enrollmentId) =>
      http.get(`/api/student-fees/${enrollmentId}`),
    assignStudentFee: (data) => http.post("/api/student-fees", data),

    listPayments: (_enrollmentId?, _feeId?) => http.get("/api/payments"),
    recordPayment: (data) => http.post("/api/payments", data),

    listDiscountTemplates: (_orgId) => http.get("/api/discount-templates"),
    createDiscountTemplate: (data) =>
      http.post("/api/discount-templates", data),

    listCurrencies: () => http.get("/api/currencies"),
    createCurrency: (data) => http.post("/api/currencies", data),
    updateCurrency: (_orgId, code, data) =>
      http.put(`/api/currencies/${code}`, data),
    deleteCurrency: (_orgId, code) => http.del(`/api/currencies/${code}`),
    seedDefaultFinancials: (_orgId) =>
      http.post("/api/currencies/seed-default"),

    listJournalEntries: () => http.get("/api/journal-entries"),
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (_periodId) => http.get("/api/time-slots"),
    createTimeSlot: (data) => http.post("/api/time-slots", data),
    updateTimeSlot: (slotId, data) =>
      http.put(`/api/time-slots/${slotId}`, data),
    deleteTimeSlot: (slotId) => http.del(`/api/time-slots/${slotId}`),

    listLessonsByClass: (periodId, classId) =>
      http.get(`/api/schedule/${classId}/${periodId}`),
    listLessonsByTeacher: (periodId, teacherId) =>
      http.get(`/api/schedule/teacher/${teacherId}/${periodId}`),
    createLesson: (data) => http.post("/api/lessons", data),
    updateLesson: (lessonId, data) =>
      http.put(`/api/lessons/${lessonId}`, data),
    deleteLesson: (lessonId) => http.del(`/api/lessons/${lessonId}`),
    checkConflicts: (periodId, teacherId, timeSlotId, dayOfWeek, roomId?) =>
      http.post("/api/schedule/conflicts", {
        periodId,
        teacherId,
        timeSlotId,
        dayOfWeek,
        roomId,
      }),

    listRooms: () => http.get("/api/rooms"),
    createRoom: (data) => http.post("/api/rooms", data),
    updateRoom: (roomId, data) => http.put(`/api/rooms/${roomId}`, data),
    deleteRoom: (roomId) => http.del(`/api/rooms/${roomId}`),

    startSolver: (periodId, mode?) =>
      http.post("/api/schedule/solve", { period_id: periodId, mode }),
    getSolverJob: (jobId) => http.get(`/api/schedule/solve/job/${jobId}`),
    listSolverJobs: (periodId) => http.get(`/api/schedule/solve/${periodId}`),
    cancelSolver: (jobId) => http.post(`/api/schedule/solve/${jobId}/cancel`),
    applySolver: (jobId) => http.post(`/api/schedule/solve/${jobId}/apply`),
  },

  // ═══════════════════════════════════════════════════════════════
  // EXAM
  // ═══════════════════════════════════════════════════════════════
  exam: {
    listSessions: (periodId) => http.get(`/api/exam/sessions/${periodId}`),
    getSession: (sessionId) =>
      http.get(`/api/exam/sessions/detail/${sessionId}`),
    createSession: (data) => http.post("/api/exam/sessions", data),
    updateSession: (sessionId, data) =>
      http.put(`/api/exam/sessions/${sessionId}`, data),

    listExamSubjects: (sessionId) =>
      http.get(`/api/exam/sessions/${sessionId}/subjects`),
    createExamSubject: (data) => http.post("/api/exam/subjects", data),

    listHalls: () => http.get("/api/exam/halls"),
    createHall: (data) => http.post("/api/exam/halls", data),

    listSchedule: (sessionId) =>
      http.get(`/api/exam/sessions/${sessionId}/schedule`),
    createScheduleEntry: (data) => http.post("/api/exam/schedule", data),

    listSeating: (examSubjectId) =>
      http.get(`/api/exam/seating/${examSubjectId}`),
    generateOrdering: (sessionId) =>
      http.post(`/api/exam/sessions/${sessionId}/seating`),
    getOrdering: (orgId, periodId) =>
      http.get(`/api/exam/ordering?orgId=${orgId}&periodId=${periodId}`),
    rebuildOrdering: (orgId, periodId, strategy) =>
      http.post(`/api/exam/ordering/rebuild`, { orgId, periodId, strategy }),
    freezeOrdering: (orgId, periodId, userId) =>
      http.post(`/api/exam/ordering/freeze`, { orgId, periodId, userId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, _ip?) =>
      http.post("/api/auth/login", { identifier, password: passwordHash }),
    logout: (_sessionId) => http.post("/api/auth/logout"),
    changePassword: (userId, newPasswordHash) =>
      http.post(`/api/users/${userId}/password`, { password: newPasswordHash }),
    listSessions: (userId) => http.get(`/api/users/${userId}/sessions`),

    listUsers: (_role?, _status?) => http.get("/api/users"),
    getUser: (userId) => http.get(`/api/users/${userId}`),
    searchUsers: (query) => http.get(`/api/users?search=${query}`),
    createUser: (data) => http.post("/api/users", data),
    updateUser: (userId, data) => http.put(`/api/users/${userId}`, data),
    deleteUser: (userId) => http.del(`/api/users/${userId}`),

    listUserRoles: (userId) => http.get(`/api/users/${userId}/roles`),
    assignRole: (data) => http.post("/api/roles/assign", data),
    revokeRole: (roleId) => http.del(`/api/roles/${roleId}`),

    getRolePermissions: () => http.get("/api/permissions/roles"),
    saveRolePermissions: (role, rules) =>
      http.post(`/api/permissions/roles/${role}`, rules),
  },

  // ═══════════════════════════════════════════════════════════════
  // ORGANIZATION
  // ═══════════════════════════════════════════════════════════════
  organization: {
    getOrganization: (_orgId) => http.get("/api/organization"),
    updateOrganization: (_orgId, data) => http.put("/api/organization", data),
    getSettings: (_orgId) => http.get("/api/organization/settings"),
    updateSettings: (_orgId, settings) =>
      http.put("/api/organization/settings", settings),
    getDashboardStats: (_orgId) => http.get("/api/dashboard"),
  },

  // ═══════════════════════════════════════════════════════════════
  // PLATFORM (Web — subset, no desktop-specific features)
  // ═══════════════════════════════════════════════════════════════
  platform: {
    getCurrentYear: (orgId) =>
      http.get(`/api/academic-years/current?orgId=${orgId}`),
    listYears: (_orgId) => http.get("/api/academic-years"),
    switchYear: (periodId) =>
      http.post(`/api/academic-years/${periodId}/switch`),
    isYearArchived: (periodId) =>
      http.get(`/api/academic-years/${periodId}/archived-status`),
    getAcademicContext: (_orgId) => http.get("/api/academic-context"),
    getSetupStatus: (_orgId) => http.get("/api/setup-status"),

    listPromotionJobs: (_orgId) => http.get("/api/promotion/jobs"),
    getPromotionJob: (jobId) => http.get(`/api/promotion/jobs/${jobId}`),
    getPromotionDetails: (jobId) =>
      http.get(`/api/promotion/jobs/${jobId}/details`),
    promotionDryRun: (_orgId, config) =>
      http.post("/api/promotion/dry-run", config),
    executePromotion: (jobId) =>
      http.post(`/api/promotion/jobs/${jobId}/execute`),
    rollbackPromotion: (jobId, userId) =>
      http.post(`/api/promotion/jobs/${jobId}/rollback`, { userId }),
    overridePromotionDecision: (request) =>
      http.post("/api/promotion/override", request),
    getPromotionPolicy: (_orgId) => http.get("/api/promotion/policy"),

    listCalendarEvents: (_orgId, periodId) =>
      http.get(`/api/calendar?period_id=${periodId}`),
    listPendingCalendarEvents: (_orgId) => http.get("/api/calendar/pending"),
    getOrgPolicy: (_orgId) => http.get("/api/organization/policy"),

    listOverrideRequests: (_orgId, _params?) => http.get("/api/overrides"),
    getOverrideRequest: (requestId) => http.get(`/api/overrides/${requestId}`),
    getPendingApprovals: (_orgId, approverRole) =>
      http.get(`/api/overrides/pending?role=${approverRole}`),
    processApproval: (requestId, action) =>
      http.post(`/api/overrides/${requestId}/approve`, action),
    cancelOverride: (requestId, userId) =>
      http.post(`/api/overrides/${requestId}/cancel`, { userId }),

    listDocumentTemplates: (_orgId, _params?) =>
      http.get("/api/document-templates"),
    getDocumentTemplate: (_orgId, templateId) =>
      http.get(`/api/document-templates/${templateId}`),

    // ── AI Autopilot ──────────────────────────────────────────────
    getAutopilotTasks: (orgId) =>
      http.get(`/api/autopilot/tasks?orgId=${orgId}`),
    getAutopilotDiagnostics: (orgId) =>
      http.get(`/api/autopilot/diagnostics?orgId=${orgId}`),
    runAutopilotTick: (orgId) => http.post(`/api/autopilot/tick`, { orgId }),

    listAgentSessions: (_orgId) => http.get("/api/agent/sessions"),
    getAgentSessionLogs: (_orgId, sessionId) =>
      http.get(`/api/agent/sessions/${sessionId}/logs`),
    approveAgentAction: (_orgId, actionLogId, approvedBy) =>
      http.post(`/api/agent/actions/${actionLogId}/approve`, { approvedBy }),
    rejectAgentAction: (_orgId, actionLogId, rejectedBy, reason) =>
      http.post(`/api/agent/actions/${actionLogId}/reject`, {
        rejectedBy,
        reason,
      }),

    login: (identifier, password) =>
      http.post("/api/auth/login", { identifier, password }),
    logout: (_token) => http.post("/api/auth/logout"),

    importStudents: (_orgId, data) => http.post("/api/import/students", data),
    importGrades: (_orgId, data) => http.post("/api/import/grades", data),
    createBulkStudents: (orgId, userId, data) =>
      http.post("/api/import/students", { students: data }),
    createBulkGrades: (orgId, userId, periodId, data) =>
      http.post("/api/import/grades", { grades: data }),
    exportStudents: (_orgId) => http.get("/api/export/students"),
    exportGrades: (_orgId) => http.get("/api/export/grades"),

    listAuditLogs: (_orgId) => http.get("/api/audit-logs"),
    getAllAuditLogs: (orgId) => http.get(`/api/audit-logs?orgId=${orgId}`),

    transitionEnrollmentStatus: (_orgId, enrollmentId, status) =>
      http.post(`/api/enrollments/${enrollmentId}/status`, { status }),

    createGuardianLink: (data) => http.post("/api/guardians", data),
    updateGuardianLink: (linkId, data) =>
      http.put(`/api/guardians/${linkId}`, data),

    listDocumentsByOwner: (_orgId, ownerId) =>
      http.get(`/api/documents/${ownerId}`),
  },
  docforge: {
    previewDocumentTemplate: (templateJson, contextJson) =>
      http.post("/api/docforge/preview", { templateJson, contextJson }),
  },
  document: {
    uploadDocument: async (payload) => {
      const formData = new FormData();
      formData.append("file", payload.file);
      formData.append("entity_type", payload.entity_type);
      formData.append("entity_id", payload.entity_id);
      formData.append("category", payload.category);
      formData.append("visibility", payload.visibility);
      return http.multipartRequest("POST", "/api/documents", formData);
    },
    getDocuments: (ownerId) => http.get(`/api/documents/${ownerId}`),
    deleteDocument: (id) => http.del(`/api/documents/${id}`),
  },
};
