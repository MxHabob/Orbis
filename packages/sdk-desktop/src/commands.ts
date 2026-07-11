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
    getEnrollment: (enrollmentId) => cmd("get_enrollment", { enrollmentId }),
    createEnrollment: (data) => cmd("create_enrollment", { data }),
    withdrawEnrollment: (enrollmentId, reason) => cmd("withdraw_enrollment", { enrollmentId, reason }),
    transferEnrollment: (enrollmentId, newClassId) => cmd("transfer_enrollment", { enrollmentId, newClassId }),
    checkDuplicate: (studentId, cycleId) => cmd("check_duplicate_enrollment", { studentId, cycleId }),

    listClassSubjects: (classId, cycleId) => cmd("list_class_subjects", { classId, cycleId }),
    listTeacherSubjects: (teacherId, cycleId) => cmd("list_teacher_subjects", { teacherId, cycleId }),
    createClassSubject: (data) => cmd("create_class_subject", { data }),
    updateClassSubject: (id, data) => cmd("update_class_subject", { id, data }),
    deleteClassSubject: (id) => cmd("delete_class_subject", { id }),

    listCycles: (orgId) => cmd("list_cycles", { orgId }),
    getCycle: (cycleId) => cmd("get_cycle", { cycleId }),
    getCurrentCycle: (orgId) => cmd("get_current_cycle", { orgId }),
    createCycle: (data) => cmd("create_cycle", { data }),
    updateCycle: (cycleId, data) => cmd("update_cycle", { cycleId, data }),
    setCurrentCycle: (cycleId) => cmd("set_current_cycle", { cycleId }),
    lockCycle: (cycleId) => cmd("lock_cycle", { cycleId }),
    unlockCycle: (cycleId) => cmd("unlock_cycle", { cycleId }),
    toggleGradeEntry: (cycleId, open, start?, end?) => cmd("toggle_grade_entry", { cycleId, open, start, end }),

    listGuardiansByStudent: (studentId) => cmd("list_guardians_by_student", { studentId }),
    searchGuardians: (query) => cmd("search_guardians", { query }),

    listDocuments: (ownerId?) => cmd("list_documents", { ownerId }),

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
    getGradeAnalytics: (cycleId, classSubjectId) => cmd("get_grade_analytics", { cycleId, classSubjectId }),

    listAttendance: (cycleId, classId?, date?, enrollmentId?) => cmd("list_attendance", { cycleId, classId, date, enrollmentId }),
    recordAttendance: (cycleId, data) => cmd("record_attendance", { cycleId, data }),
    recordBulkAttendance: (cycleId, data) => cmd("record_bulk_attendance", { cycleId, data }),
    getAttendanceReport: (cycleId, enrollmentId) => cmd("get_attendance_report", { cycleId, enrollmentId }),

    listBehavior: (cycleId, enrollmentId?) => cmd("list_behavior", { cycleId, enrollmentId }),
    updateBehavior: (recordId, data) => cmd("update_behavior", { recordId, data }),
    deleteBehavior: (recordId) => cmd("delete_behavior", { recordId }),

    listAssessments: (cycleId, classSubjectId?) => cmd("list_assessments", { cycleId, classSubjectId }),
    createAssessment: (data) => cmd("create_assessment", { data }),
    updateAssessment: (assessmentId, data) => cmd("update_assessment", { assessmentId, data }),
    deleteAssessment: (assessmentId) => cmd("delete_assessment", { assessmentId }),
  },

  // ═══════════════════════════════════════════════════════════════
  // FINANCE
  // ═══════════════════════════════════════════════════════════════
  finance: {
    listLedgerAccounts: () => cmd("list_ledger_accounts"),
    listTransactionAccounts: () => cmd("list_transaction_accounts"),
    createLedgerAccount: (data) => cmd("create_ledger_account", { data }),
    getAccountStatement: (accountId, startDate?, endDate?) => cmd("get_account_statement", { accountId, startDate, endDate }),

    listTreasuries: () => cmd("list_treasuries"),
    createTreasury: (data) => cmd("create_treasury", { data }),

    listFeeTemplates: () => cmd("list_fee_templates"),
    listActiveFeeTemplates: () => cmd("list_active_fee_templates"),
    createFeeTemplate: (data) => cmd("create_fee_template", { data }),
    updateFeeTemplate: (templateId, data) => cmd("update_fee_template", { templateId, data }),
    deleteFeeTemplate: (templateId) => cmd("delete_fee_template", { templateId }),

    listStudentFees: (enrollmentId) => cmd("list_student_fees", { enrollmentId }),
    assignStudentFee: (data) => cmd("assign_student_fee", { data }),

    listPayments: (enrollmentId?, feeId?) => cmd("list_payments", { enrollmentId, feeId }),
    recordPayment: (data) => cmd("record_payment", { data }),

    listDiscountTemplates: () => cmd("list_discount_templates"),
    createDiscountTemplate: (data) => cmd("create_discount_template", { data }),

    listCurrencies: () => cmd("list_currencies"),
    createCurrency: (data) => cmd("create_currency", { data }),

    listJournalEntries: () => cmd("list_journal_entries"),
  },

  // ═══════════════════════════════════════════════════════════════
  // SCHEDULING
  // ═══════════════════════════════════════════════════════════════
  scheduling: {
    listTimeSlots: (cycleId) => cmd("list_time_slots", { cycleId }),
    createTimeSlot: (data) => cmd("create_time_slot", { data }),
    updateTimeSlot: (slotId, data) => cmd("update_time_slot", { slotId, data }),
    deleteTimeSlot: (slotId) => cmd("delete_time_slot", { slotId }),

    listLessonsByClass: (cycleId, classId) => cmd("list_lessons_by_class", { cycleId, classId }),
    listLessonsByTeacher: (cycleId, teacherId) => cmd("list_lessons_by_teacher", { cycleId, teacherId }),
    createLesson: (data) => cmd("create_lesson", { data }),
    updateLesson: (lessonId, data) => cmd("update_lesson", { lessonId, data }),
    deleteLesson: (lessonId) => cmd("delete_lesson", { lessonId }),
    checkConflicts: (cycleId, teacherId, timeSlotId, dayOfWeek, roomId?) => cmd("check_schedule_conflicts", { cycleId, teacherId, timeSlotId, dayOfWeek, roomId }),

    listRooms: () => cmd("list_rooms"),
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
  },

  // ═══════════════════════════════════════════════════════════════
  // IAM
  // ═══════════════════════════════════════════════════════════════
  iam: {
    login: (identifier, passwordHash, ip?) => cmd("login", { identifier, passwordHash, ip }),
    logout: (sessionId) => cmd("logout_session", { sessionId }),
    changePassword: (userId, newPasswordHash) => cmd("change_password", { userId, newPasswordHash }),
    listSessions: (userId) => cmd("list_user_sessions", { userId }),

    listUsers: (role?, status?) => cmd("list_users", { role, status }),
    getUser: (userId) => cmd("get_user", { userId }),
    searchUsers: (query) => cmd("search_users", { query }),
    createUser: (data) => cmd("create_user", { data }),
    updateUser: (userId, data) => cmd("update_user", { userId, data }),
    deleteUser: (userId) => cmd("delete_user", { userId }),

    listUserRoles: (userId) => cmd("list_user_roles", { userId }),
    assignRole: (data) => cmd("assign_role", { data }),
    revokeRole: (roleId) => cmd("revoke_role", { roleId }),
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
};
