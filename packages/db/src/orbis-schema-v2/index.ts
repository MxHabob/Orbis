// -----------------------------------------------------------
// Types & Enums
// -----------------------------------------------------------

// -----------------------------------------------------------
// Academic Schema Tables
// -----------------------------------------------------------
export {
  academicCycles,
  assessments,
  attendanceDaily,
  attendanceSummary,
  behaviorRecords,
  classes,
  classSubjects,
  enrollments,
  gradeHistory,
  grades,
  orgUnits,
  scheduledLessons,
  studentGuardians,
  subjects,
  timeSlots,
} from "./academic";
// -----------------------------------------------------------
// Audit Schema Tables
// -----------------------------------------------------------
export {
  auditLog,
  dataTransfers,
  systemLog,
} from "./audit";

// -----------------------------------------------------------
// Core Schema Tables
// -----------------------------------------------------------
export {
  documents,
  notificationPreferences,
  notifications,
  organizations,
  permissions,
  rolePermissions,
  userAccounts,
  userPermissions,
  userRoles,
  userSessions,
  users,
  verifications,
} from "./core";
export {
  academicStatusEnum,
  attendanceEnum,
  notificationChannelEnum,
  paymentStatusEnum,
  roleEnum,
  userStatusEnum,
} from "./enums";

// -----------------------------------------------------------
// Finance Schema Tables
// -----------------------------------------------------------
export {
  discountTemplates,
  // Rule Templates
  feeTemplates,
  // Events & Reports
  financialEvents,
  financialReports,
  journalEntries,
  // Core Engine
  ledgerAccounts,
  paymentPlans,
  // Payments & Treasury
  payments,
  studentDiscounts,
  // Instances
  studentFees,
  treasuries,
  treasuryTransactions,
} from "./finance";
// -----------------------------------------------------------
// Relations
// -----------------------------------------------------------
export {
  // Academic
  academicCyclesRelations,
  assessmentsRelations,
  attendanceDailyRelations,
  attendanceSummaryRelations,
  // Audit
  auditLogRelations,
  behaviorRecordsRelations,
  classesRelations,
  classSubjectsRelations,
  dataTransfersRelations,
  discountTemplatesRelations,
  documentsRelations,
  enrollmentsRelations,
  feeTemplatesRelations,
  financialEventsRelations,
  financialReportsRelations,
  gradeHistoryRelations,
  gradesRelations,
  journalEntriesRelations,
  // Finance
  ledgerAccountsRelations,
  notificationPreferencesRelations,
  // Notifications & Documents
  notificationsRelations,
  // Core
  organizationsRelations,
  orgUnitsRelations,
  paymentPlansRelations,
  paymentsRelations,
  permissionsRelations,
  rolePermissionsRelations,
  scheduledLessonsRelations,
  studentDiscountsRelations,
  studentFeesRelations,
  studentGuardiansRelations,
  subjectsRelations,
  systemLogRelations,
  timeSlotsRelations,
  treasuriesRelations,
  treasuryTransactionsRelations,
  userAccountsRelations,
  userPermissionsRelations,
  userRolesRelations,
  userSessionsRelations,
  usersRelations,
} from "./relations";
export {
  academicSchema,
  auditSchema,
  coreSchema,
  financeSchema,
  hijriDateType,
  moneyType,
  percentType,
  scoreType,
} from "./types";

import * as academic from "./academic";
import * as audit from "./audit";
// -----------------------------------------------------------
// Schema Object for Drizzle Client
// -----------------------------------------------------------
import * as core from "./core";
import * as finance from "./finance";

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
