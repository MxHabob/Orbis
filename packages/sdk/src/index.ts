import * as generated from "./generated";
import { client } from "./generated";

export * from "./generated";

export const sdk = {
  client,
  auth: {
    register: generated.HandlerAuthService.postRegister,
    login: generated.HandlerAuthService.postLogin,
    me: generated.HandlerAuthService.getMe,
    logout: generated.HandlerAuthService.postLogout,
  },
  organization: {
    get: generated.HandlerOrganizationService.getOrganization,
    update: generated.HandlerOrganizationService.putOrganization,
    getStats: generated.HandlerOrganizationService.getOrganizationStats,
  },
  user: {
    list: generated.HandlerUserService.getUsers,
    create: generated.HandlerUserService.postUsers,
    get: generated.HandlerUserService.getUser,
    update: generated.HandlerUserService.putUser,
    delete: generated.HandlerUserService.deleteUser,
    changeRole: generated.HandlerUserService.postUserRole,
    changePassword: generated.HandlerUserService.postUserPassword,
  },
  academicCycle: {
    list: generated.HandlerAcademicCycleService.getCycles,
    create: generated.HandlerAcademicCycleService.postCycles,
    get: generated.HandlerAcademicCycleService.getCycle,
    update: generated.HandlerAcademicCycleService.putCycle,
    activate: generated.HandlerAcademicCycleService.postActivateCycle,
    lock: generated.HandlerAcademicCycleService.postLockCycle,
  },
  orgUnit: {
    list: generated.HandlerOrgUnitService.getUnits,
    create: generated.HandlerOrgUnitService.postUnits,
    update: generated.HandlerOrgUnitService.putUnit,
    delete: generated.HandlerOrgUnitService.deleteUnit,
  },
  class: {
    list: generated.HandlerClassService.getClasses,
    create: generated.HandlerClassService.postClasses,
    update: generated.HandlerClassService.putClass,
    delete: generated.HandlerClassService.deleteClass,
    getStudents: generated.HandlerClassService.getClassStudents,
  },
  enrollment: {
    list: generated.HandlerEnrollmentService.getEnrollments,
    create: generated.HandlerEnrollmentService.postEnrollments,
    get: generated.HandlerEnrollmentService.getEnrollment,
    withdraw: generated.HandlerEnrollmentService.postWithdraw,
    transfer: generated.HandlerEnrollmentService.postTransfer,
    promote: generated.HandlerEnrollmentService.postPromote,
  },
  grade: {
    list: generated.HandlerGradeService.getGrades,
    create: generated.HandlerGradeService.postGrades,
    createBulk: generated.HandlerGradeService.postGradesBulk,
    update: generated.HandlerGradeService.putGrade,
    getReportCard: generated.HandlerGradeService.getReportCard,
    getAnalytics: generated.HandlerGradeService.getGradeAnalytics,
  },
  attendance: {
    list: generated.HandlerAttendanceService.getAttendance,
    create: generated.HandlerAttendanceService.postAttendance,
    createBulk: generated.HandlerAttendanceService.postAttendanceBulk,
    getReport: generated.HandlerAttendanceService.getAttendanceReport,
    getAnalytics: generated.HandlerAttendanceService.getAttendanceAnalytics,
  },
  behavior: {
    list: generated.HandlerBehaviorService.getBehavior,
    update: generated.HandlerBehaviorService.putBehavior,
    delete: generated.HandlerBehaviorService.deleteBehavior,
    submitAi: generated.HandlerBehaviorService.postAiBehavior,
  },
  notification: {
    list: generated.HandlerNotificationService.getNotifications,
    markRead: generated.HandlerNotificationService.postMarkRead,
  },
  dashboard: {
    getStats: generated.HandlerDashboardService.getDashboardStats,
  },
  audit: {
    listLogs: generated.HandlerAuditService.getAuditLogs,
  },
  importExport: {
    importStudents: generated.HandlerImportExportService.postImportStudents,
    importGrades: generated.HandlerImportExportService.postImportGrades,
    importAttendance: generated.HandlerImportExportService.postImportAttendance,
    exportStudents: generated.HandlerImportExportService.getExportStudents,
    exportGrades: generated.HandlerImportExportService.getExportGrades,
    exportAttendance: generated.HandlerImportExportService.getExportAttendance,
    exportReportCards:
      generated.HandlerImportExportService.getExportReportCards,
  },
  finance: {
    getLedgerAccounts: generated.HandlerFinanceService.getLedgerAccounts,
    createLedgerAccount: generated.HandlerFinanceService.postLedgerAccounts,
    getTreasuries: generated.HandlerFinanceService.getTreasuries,
    createTreasury: generated.HandlerFinanceService.postTreasuries,
    getStudentFees: generated.HandlerFinanceService.getStudentFees,
    assignStudentFee: generated.HandlerFinanceService.postStudentFees,
    getPayments: generated.HandlerFinanceService.getPayments,
    recordPayment: generated.HandlerFinanceService.postPayments,
  },
};
