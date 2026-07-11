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
    listClasses: (_orgId, _cycleId?) => http.get("/api/classes"),
    getClass: (classId) => http.get(`/api/classes/${classId}`),
    createClass: (data) => http.post("/api/classes", data),
    updateClass: (classId, data) => http.put(`/api/classes/${classId}`, data),
    deleteClass: (classId) => http.del(`/api/classes/${classId}`),
    assignTeacher: (classId, teacherId) => http.post(`/api/classes/${classId}/teacher`, { teacherId }),

    listSubjects: (_orgId) => http.get("/api/subjects"),
    getSubject: (subjectId) => http.get(`/api/subjects/${subjectId}`),
    createSubject: (data) => http.post("/api/subjects", data),
    updateSubject: (subjectId, data) => http.put(`/api/subjects/${subjectId}`, data),
    deleteSubject: (subjectId) => http.del(`/api/subjects/${subjectId}`),

    listOrgUnits: (_orgId) => http.get("/api/units"),
    createOrgUnit: (data) => http.post("/api/units", data),
    updateOrgUnit: (unitId, data) => http.put(`/api/units/${unitId}`, data),
    deleteOrgUnit: (unitId) => http.del(`/api/units/${unitId}`),

    listEnrollments: (_orgId, _classId?, _cycleId?) => http.get("/api/enrollments"),
    getEnrollment: (enrollmentId) => http.get(`/api/enrollments/${enrollmentId}`),
    createEnrollment: (data) => http.post("/api/enrollments", data),
    withdrawEnrollment: (enrollmentId, reason) => http.post(`/api/enrollments/${enrollmentId}/withdraw`, { reason }),
    transferEnrollment: (enrollmentId, newClassId) => http.post(`/api/enrollments/${enrollmentId}/transfer`, { class_id: newClassId }),
    checkDuplicate: (studentId, cycleId) => http.get(`/api/enrollments/check-duplicate?student_id=${studentId}&cycle_id=${cycleId}`),

    listClassSubjects: (classId, cycleId) => http.get(`/api/class-subjects?class_id=${classId}&cycle_id=${cycleId}`),
    listTeacherSubjects: (teacherId, cycleId) => http.get(`/api/teacher-subjects?teacher_id=${teacherId}&cycle_id=${cycleId}`),
    createClassSubject: (data) => http.post("/api/class-subjects", data),
    updateClassSubject: (id, data) => http.put(`/api/class-subjects/${id}`, data),
    deleteClassSubject: (id) => http.del(`/api/class-subjects/${id}`),

    listCycles: (_orgId) => http.get("/api/cycles"),
    getCycle: (cycleId) => http.get(`/api/cycles/${cycleId}`),
    getCurrentCycle: (_orgId) => http.get("/api/cycles/current"),
    createCycle: (data) => http.post("/api/cycles", data),
    updateCycle: (cycleId, data) => http.put(`/api/cycles/${cycleId}`, data),
    setCurrentCycle: (cycleId) => http.post(`/api/cycles/${cycleId}/activate`),
    lockCycle: (cycleId) => http.post(`/api/cycles/${cycleId}/lock`),
    unlockCycle: (cycleId) => http.post(`/api/cycles/${cycleId}/unlock`),
    toggleGradeEntry: (cycleId, open, _start?, _end?) => http.post(`/api/cycles/${cycleId}/grade-entry/${open ? "open" : "close"}`),

    listGuardiansByStudent: (studentId) => http.get(`/api/guardians/${studentId}`),
    searchGuardians: (query) => http.get(`/api/guardians/search?q=${query}`),

    listDocuments: (ownerId?) => http.get(`/api/documents${ownerId ? `/${ownerId}` : ""}`),

    listNotifications: (_limit?) => http.get("/api/notifications"),
    markNotificationRead: (notificationId) => http.post(`/api/notifications/${notificationId}/read`),
  },

  // ═══════════════════════════════════════════════════════════════
  // ASSESSMENT
  // ═══════════════════════════════════════════════════════════════
  assessment: {
    listGrades: (cycleId, enrollmentId?, classSubjectId?) =>
      http.get(`/api/grades?cycle_id=${cycleId}${enrollmentId ? `&enrollment_id=${enrollmentId}` : ""}${classSubjectId ? `&class_subject_id=${classSubjectId}` : ""}`),
    submitGrade: (_cycleId, data) => http.post("/api/grades", data),
    submitBulkGrades: (_cycleId, grades) => http.post("/api/grades/bulk", grades),
    updateGrade: (gradeId, data) => http.put(`/api/grades/${gradeId}`, data),
    getReportCard: (enrollmentId, cycleId) => http.get(`/api/report-card/${enrollmentId}/${cycleId}`),
    getGradeAnalytics: (cycleId, classSubjectId) => http.get(`/api/grades/analytics?cycle_id=${cycleId}&class_subject_id=${classSubjectId}`),

    listAttendance: (_cycleId, classId?, date?, _enrollmentId?) =>
      http.get(`/api/attendance/${classId ?? "all"}/${date ?? "today"}`),
    recordAttendance: (_cycleId, data) => http.post("/api/attendance", data),
    recordBulkAttendance: (_cycleId, data) => http.post("/api/attendance/bulk", data),
    getAttendanceReport: (_cycleId, enrollmentId) => http.get(`/api/attendance/report/${enrollmentId}`),

    listBehavior: (_cycleId, enrollmentId?) => http.get(`/api/behavior${enrollmentId ? `/${enrollmentId}` : ""}`),
    updateBehavior: (recordId, data) => http.put(`/api/behavior/${recordId}`, data),
    deleteBehavior: (recordId) => http.del(`/api/behavior/${recordId}`),

    listAssessments: (_cycleId, classSubjectId?) =>
      http.get(`/api/assessments${classSubjectId ? `?class_subject_id=${classSubjectId}` : ""}`),
    createAssessment: (data) => http.post("/api/assessments", data),
    updateAssessment: (assessmentId, data) => http.put(`/api/assessments/${assessmentId}`, data),
    deleteAssessment: (assessmentId) => http.del(`/api/assessments/${assessmentId}`),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listLedgerAccounts: () => http.get("/api/ledger-accounts"),
    listTransactionAccounts: () => http.get("/api/ledger-accounts?type=transaction"),
    createLedgerAccount: (data) => http.post("/api/ledger-accounts", data),
    getAccountStatement: (accountId, startDate?, endDate?) =>
      http.get(`/api/ledger-accounts/${accountId}/statement?start=${startDate ?? ""}&end=${endDate ?? ""}`),

    listTreasuries: () => http.get("/api/treasuries"),
    createTreasury: (data) => http.post("/api/treasuries", data),

    listFeeTemplates: () => http.get("/api/fee-templates"),
    listActiveFeeTemplates: () => http.get("/api/fee-templates?active=true"),
    createFeeTemplate: (data) => http.post("/api/fee-templates", data),
    updateFeeTemplate: (templateId, data) => http.put(`/api/fee-templates/${templateId}`, data),
    deleteFeeTemplate: (templateId) => http.del(`/api/fee-templates/${templateId}`),

    listStudentFees: (enrollmentId) => http.get(`/api/student-fees/${enrollmentId}`),
    assignStudentFee: (data) => http.post("/api/student-fees", data),

    listPayments: (_enrollmentId?, _feeId?) => http.get("/api/payments"),
    recordPayment: (data) => http.post("/api/payments", data),

    listDiscountTemplates: () => http.get("/api/discount-templates"),
    createDiscountTemplate: (data) => http.post("/api/discount-templates", data),

    listCurrencies: () => http.get("/api/currencies"),
    createCurrency: (data) => http.post("/api/currencies", data),

    listJournalEntries: () => http.get("/api/journal-entries"),
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (_cycleId) => http.get("/api/time-slots"),
    createTimeSlot: (data) => http.post("/api/time-slots", data),
    updateTimeSlot: (slotId, data) => http.put(`/api/time-slots/${slotId}`, data),
    deleteTimeSlot: (slotId) => http.del(`/api/time-slots/${slotId}`),

    listLessonsByClass: (cycleId, classId) => http.get(`/api/schedule/${classId}/${cycleId}`),
    listLessonsByTeacher: (cycleId, teacherId) => http.get(`/api/schedule/teacher/${teacherId}/${cycleId}`),
    createLesson: (data) => http.post("/api/lessons", data),
    updateLesson: (lessonId, data) => http.put(`/api/lessons/${lessonId}`, data),
    deleteLesson: (lessonId) => http.del(`/api/lessons/${lessonId}`),
    checkConflicts: (cycleId, teacherId, timeSlotId, dayOfWeek, roomId?) =>
      http.post("/api/schedule/conflicts", { cycleId, teacherId, timeSlotId, dayOfWeek, roomId }),

    listRooms: () => http.get("/api/rooms"),
    createRoom: (data) => http.post("/api/rooms", data),
    updateRoom: (roomId, data) => http.put(`/api/rooms/${roomId}`, data),
    deleteRoom: (roomId) => http.del(`/api/rooms/${roomId}`),

    startSolver: (cycleId, mode?) => http.post("/api/schedule/solve", { cycle_id: cycleId, mode }),
    getSolverJob: (jobId) => http.get(`/api/schedule/solve/job/${jobId}`),
    listSolverJobs: (cycleId) => http.get(`/api/schedule/solve/${cycleId}`),
    cancelSolver: (jobId) => http.post(`/api/schedule/solve/${jobId}/cancel`),
    applySolver: (jobId) => http.post(`/api/schedule/solve/${jobId}/apply`),
  },

  // ═══════════════════════════════════════════════════════════════
  // EXAM
  // ═══════════════════════════════════════════════════════════════
  exam: {
    listSessions: (cycleId) => http.get(`/api/exam/sessions/${cycleId}`),
    getSession: (sessionId) => http.get(`/api/exam/sessions/detail/${sessionId}`),
    createSession: (data) => http.post("/api/exam/sessions", data),
    updateSession: (sessionId, data) => http.put(`/api/exam/sessions/${sessionId}`, data),

    listExamSubjects: (sessionId) => http.get(`/api/exam/sessions/${sessionId}/subjects`),
    createExamSubject: (data) => http.post("/api/exam/subjects", data),

    listHalls: () => http.get("/api/exam/halls"),
    createHall: (data) => http.post("/api/exam/halls", data),

    listSchedule: (sessionId) => http.get(`/api/exam/sessions/${sessionId}/schedule`),
    createScheduleEntry: (data) => http.post("/api/exam/schedule", data),

    listSeating: (examSubjectId) => http.get(`/api/exam/seating/${examSubjectId}`),
    generateOrdering: (sessionId) => http.post(`/api/exam/sessions/${sessionId}/seating`),
  },

  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, _ip?) => http.post("/api/auth/login", { identifier, password: passwordHash }),
    logout: (_sessionId) => http.post("/api/auth/logout"),
    changePassword: (userId, newPasswordHash) => http.post(`/api/users/${userId}/password`, { password: newPasswordHash }),
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
  },

  // ═══════════════════════════════════════════════════════════════
  // ORGANIZATION
  // ═══════════════════════════════════════════════════════════════
  organization: {
    getOrganization: (_orgId) => http.get("/api/organization"),
    updateOrganization: (_orgId, data) => http.put("/api/organization", data),
    getSettings: (_orgId) => http.get("/api/organization/settings"),
    updateSettings: (_orgId, settings) => http.put("/api/organization/settings", settings),
    getDashboardStats: (_orgId) => http.get("/api/dashboard"),
  },

  // ═══════════════════════════════════════════════════════════════
  // PLATFORM (Web — subset, no desktop-specific features)
  // ═══════════════════════════════════════════════════════════════
  platform: {
    getCurrentYear: () => http.get("/api/academic-years/current"),
    listYears: (_orgId) => http.get("/api/academic-years"),
    switchYear: (yearId) => http.post(`/api/academic-years/${yearId}/switch`),
    isYearArchived: () => http.get("/api/academic-years/archived-status"),
    getAcademicContext: (_orgId) => http.get("/api/academic-context"),
    getSetupStatus: (_orgId) => http.get("/api/setup-status"),

    listPromotionJobs: (_orgId) => http.get("/api/promotion/jobs"),
    getPromotionJob: (jobId) => http.get(`/api/promotion/jobs/${jobId}`),
    getPromotionDetails: (jobId) => http.get(`/api/promotion/jobs/${jobId}/details`),
    promotionDryRun: (_orgId, config) => http.post("/api/promotion/dry-run", config),
    executePromotion: (jobId) => http.post(`/api/promotion/jobs/${jobId}/execute`),
    rollbackPromotion: (jobId, userId) => http.post(`/api/promotion/jobs/${jobId}/rollback`, { userId }),
    overridePromotionDecision: (request) => http.post("/api/promotion/override", request),
    getPromotionPolicy: (_orgId) => http.get("/api/promotion/policy"),

    listCalendarEvents: (_orgId, cycleId) => http.get(`/api/calendar?cycle_id=${cycleId}`),
    listPendingCalendarEvents: (_orgId) => http.get("/api/calendar/pending"),
    getOrgPolicy: (_orgId) => http.get("/api/organization/policy"),

    listOverrideRequests: (_orgId, _params?) => http.get("/api/overrides"),
    getOverrideRequest: (requestId) => http.get(`/api/overrides/${requestId}`),
    getPendingApprovals: (_orgId, approverRole) => http.get(`/api/overrides/pending?role=${approverRole}`),

    listDocumentTemplates: (_orgId, _params?) => http.get("/api/document-templates"),
    getDocumentTemplate: (_orgId, templateId) => http.get(`/api/document-templates/${templateId}`),

    getAutopilotStatus: (_orgId) => http.get("/api/autopilot/status"),
    getAutopilotTasks: (_orgId) => http.get("/api/autopilot/tasks"),
    getAutopilotDiagnostics: (_orgId) => http.get("/api/autopilot/diagnostics"),
    configureAutopilot: (_orgId, isEnabled, paused, mode) => http.post("/api/autopilot/configure", { isEnabled, paused, mode }),
    runAutopilotTick: (_orgId) => http.post("/api/autopilot/tick"),

    listAgentSessions: (_orgId) => http.get("/api/agent/sessions"),
    getAgentSessionLogs: (_orgId, sessionId) => http.get(`/api/agent/sessions/${sessionId}/logs`),
    approveAgentAction: (_orgId, actionLogId, approvedBy) => http.post(`/api/agent/actions/${actionLogId}/approve`, { approvedBy }),
    rejectAgentAction: (_orgId, actionLogId, rejectedBy, reason) => http.post(`/api/agent/actions/${actionLogId}/reject`, { rejectedBy, reason }),

    login: (identifier, password) => http.post("/api/auth/login", { identifier, password }),
    logout: (_token) => http.post("/api/auth/logout"),

    importStudents: (_orgId, data) => http.post("/api/import/students", data),
    importGrades: (_orgId, data) => http.post("/api/import/grades", data),
    exportStudents: (_orgId) => http.get("/api/export/students"),
    exportGrades: (_orgId) => http.get("/api/export/grades"),

    listAuditLogs: (_orgId) => http.get("/api/audit-logs"),

    transitionEnrollmentStatus: (_orgId, enrollmentId, status) => http.post(`/api/enrollments/${enrollmentId}/status`, { status }),

    createGuardianLink: (data) => http.post("/api/guardians", data),
    updateGuardianLink: (linkId, data) => http.put(`/api/guardians/${linkId}`, data),

    listDocumentsByOwner: (_orgId, ownerId) => http.get(`/api/documents/${ownerId}`),
  },
};
