// -----------------------------------------------------------
// Types & Enums
// -----------------------------------------------------------
export {
  coreSchema,
  academicSchema,
  financeSchema,
  auditSchema,
  moneyType,
  scoreType,
  percentType,
  hijriDateType,
} from "./types";

export {
  userStatusEnum,
  roleEnum,
  academicStatusEnum,
  attendanceEnum,
  paymentStatusEnum,
  notificationChannelEnum,
} from "./enums";

// -----------------------------------------------------------
// Core Schema Tables
// -----------------------------------------------------------
export {
  organizations,
  users,
  userAccounts,
  userSessions,
  verifications,
  userRoles,
  permissions,
  rolePermissions,
  userPermissions,
  notifications,
  notificationPreferences,
  documents,
} from "./core";

// -----------------------------------------------------------
// Academic Schema Tables
// -----------------------------------------------------------
export {
  academicCycles,
  orgUnits,
  classes,
  subjects,
  classSubjects,
  enrollments,
  studentGuardians,
  timeSlots,
  scheduledLessons,
  assessments,
  grades,
  gradeHistory,
  attendanceDaily,
  attendanceSummary,
  behaviorRecords,
} from "./academic";

// -----------------------------------------------------------
// Finance Schema Tables
// -----------------------------------------------------------
export {
  // Core Engine
  ledgerAccounts,
  journalEntries,
  
  // Rule Templates
  feeTemplates,
  discountTemplates,
  paymentPlans,
  
  // Instances
  studentFees,
  studentDiscounts,
  
  // Payments & Treasury
  payments,
  treasuries,
  treasuryTransactions,
  
  // Events & Reports
  financialEvents,
  financialReports,
} from "./finance";

// -----------------------------------------------------------
// Audit Schema Tables
// -----------------------------------------------------------
export {
  auditLog,
  systemLog,
  dataTransfers,
} from "./audit";

// -----------------------------------------------------------
// Relations
// -----------------------------------------------------------
export {
  // Core
  organizationsRelations,
  usersRelations,
  userAccountsRelations,
  userSessionsRelations,
  userRolesRelations,
  permissionsRelations,
  rolePermissionsRelations,
  userPermissionsRelations,
  
  // Academic
  academicCyclesRelations,
  orgUnitsRelations,
  classesRelations,
  subjectsRelations,
  classSubjectsRelations,
  enrollmentsRelations,
  studentGuardiansRelations,
  timeSlotsRelations,
  scheduledLessonsRelations,
  assessmentsRelations,
  gradesRelations,
  gradeHistoryRelations,
  attendanceDailyRelations,
  attendanceSummaryRelations,
  behaviorRecordsRelations,
  
  // Finance
  ledgerAccountsRelations,
  journalEntriesRelations,
  feeTemplatesRelations,
  discountTemplatesRelations,
  paymentPlansRelations,
  studentFeesRelations,
  studentDiscountsRelations,
  paymentsRelations,
  treasuriesRelations,
  treasuryTransactionsRelations,
  financialEventsRelations,
  financialReportsRelations,
  
  // Notifications & Documents
  notificationsRelations,
  notificationPreferencesRelations,
  documentsRelations,
  
  // Audit
  auditLogRelations,
  systemLogRelations,
  dataTransfersRelations,
} from "./relations";

// -----------------------------------------------------------
// Schema Object for Drizzle Client
// -----------------------------------------------------------
import * as core from "./core";
import * as academic from "./academic";
import * as finance from "./finance";
import * as audit from "./audit";

export const schema = {
  // Core
  organizations: core.organizations,
  users: core.users,
  userAccounts: core.userAccounts,
  userSessions: core.userSessions,
  userRoles: core.userRoles,
  permissions: core.permissions,
  rolePermissions: core.rolePermissions,
  userPermissions: core.userPermissions,
  verifications: core.verifications,
  notifications: core.notifications,
  notificationPreferences: core.notificationPreferences,
  documents: core.documents,

  // Academic
  academicCycles: academic.academicCycles,
  orgUnits: academic.orgUnits,
  classes: academic.classes,
  subjects: academic.subjects,
  classSubjects: academic.classSubjects,
  enrollments: academic.enrollments,
  studentGuardians: academic.studentGuardians,
  timeSlots: academic.timeSlots,
  scheduledLessons: academic.scheduledLessons,
  assessments: academic.assessments,
  grades: academic.grades,
  gradeHistory: academic.gradeHistory,
  attendanceDaily: academic.attendanceDaily,
  attendanceSummary: academic.attendanceSummary,
  behaviorRecords: academic.behaviorRecords,

  // Finance
  ledgerAccounts: finance.ledgerAccounts,
  journalEntries: finance.journalEntries,
  feeTemplates: finance.feeTemplates,
  discountTemplates: finance.discountTemplates,
  paymentPlans: finance.paymentPlans,
  studentFees: finance.studentFees,
  studentDiscounts: finance.studentDiscounts,
  payments: finance.payments,
  treasuries: finance.treasuries,
  treasuryTransactions: finance.treasuryTransactions,
  financialEvents: finance.financialEvents,
  financialReports: finance.financialReports,

  // Audit
  auditLog: audit.auditLog,
  systemLog: audit.systemLog,
  dataTransfers: audit.dataTransfers,
} as const;

export default schema;
