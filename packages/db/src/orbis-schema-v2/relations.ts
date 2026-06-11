import { relations } from "drizzle-orm";
// Academic imports
import {
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
// Audit imports
import { auditLog, dataTransfers, systemLog } from "./audit";
// Core imports
import {
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
} from "./core";
// Finance imports
import {
  discountTemplates,
  feeTemplates,
  financialEvents,
  financialReports,
  journalEntries,
  ledgerAccounts,
  paymentPlans,
  payments,
  studentDiscounts,
  studentFees,
  treasuries,
  treasuryTransactions,
} from "./finance";

// ============================================================
// CORE RELATIONS
// ============================================================

export const organizationsRelations = relations(organizations, ({ many }) => ({
  users: many(users),
  academicCycles: many(academicCycles),
  orgUnits: many(orgUnits),
  classes: many(classes),
  subjects: many(subjects),
  feeTemplates: many(feeTemplates),
  discountTemplates: many(discountTemplates),
  paymentPlans: many(paymentPlans),
  treasuries: many(treasuries),
  ledgerAccounts: many(ledgerAccounts),
  journalEntries: many(journalEntries),
  financialEvents: many(financialEvents),
  financialReports: many(financialReports),
}));

export const usersRelations = relations(users, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [users.orgId],
    references: [organizations.id],
  }),
  roles: many(userRoles),
  sessions: many(userSessions),
  accounts: many(userAccounts),
  notifications: many(notifications),
  notificationPreferences: many(notificationPreferences),
  documents: many(documents),
  // Academic relations
  taughtClassSubjects: many(classSubjects, { relationName: "primaryTeacher" }),
  assistedClassSubjects: many(classSubjects, {
    relationName: "assistantTeacher",
  }),
  homeroomClasses: many(classes),
  // Finance relations
  auditLogs: many(auditLog),
  systemLogs: many(systemLog),
}));

export const userAccountsRelations = relations(userAccounts, ({ one }) => ({
  user: one(users, {
    fields: [userAccounts.userId],
    references: [users.id],
  }),
}));

export const userSessionsRelations = relations(userSessions, ({ one }) => ({
  user: one(users, {
    fields: [userSessions.userId],
    references: [users.id],
  }),
}));

export const userRolesRelations = relations(userRoles, ({ one }) => ({
  user: one(users, { fields: [userRoles.userId], references: [users.id] }),
}));

export const permissionsRelations = relations(permissions, ({ many }) => ({
  roleAssignments: many(rolePermissions),
  userAssignments: many(userPermissions),
}));

export const rolePermissionsRelations = relations(
  rolePermissions,
  ({ one }) => ({
    permission: one(permissions, {
      fields: [rolePermissions.permissionId],
      references: [permissions.id],
    }),
  }),
);

export const userPermissionsRelations = relations(
  userPermissions,
  ({ one }) => ({
    user: one(users, {
      fields: [userPermissions.userId],
      references: [users.id],
    }),
    permission: one(permissions, {
      fields: [userPermissions.permissionId],
      references: [permissions.id],
    }),
  }),
);

// ============================================================
// ACADEMIC RELATIONS
// ============================================================

export const academicCyclesRelations = relations(
  academicCycles,
  ({ one, many }) => ({
    organization: one(organizations, {
      fields: [academicCycles.orgId],
      references: [organizations.id],
    }),
    parent: one(academicCycles, {
      fields: [academicCycles.parentId],
      references: [academicCycles.id],
    }),
    children: many(academicCycles),
    classes: many(classes),
    enrollments: many(enrollments),
    assessments: many(assessments),
    grades: many(grades),
    attendance: many(attendanceDaily),
    attendanceSummaries: many(attendanceSummary),
    studentFees: many(studentFees),
    timeSlots: many(timeSlots),
    scheduledLessons: many(scheduledLessons),
  }),
);

export const orgUnitsRelations = relations(orgUnits, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [orgUnits.orgId],
    references: [organizations.id],
  }),
  parent: one(orgUnits, {
    fields: [orgUnits.parentId],
    references: [orgUnits.id],
  }),
  children: many(orgUnits),
  classes: many(classes),
  subjects: many(subjects),
  head: one(users, { fields: [orgUnits.headId], references: [users.id] }),
}));

export const classesRelations = relations(classes, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [classes.orgId],
    references: [organizations.id],
  }),
  unit: one(orgUnits, { fields: [classes.unitId], references: [orgUnits.id] }),
  cycle: one(academicCycles, {
    fields: [classes.cycleId],
    references: [academicCycles.id],
  }),
  homeroomTeacher: one(users, {
    fields: [classes.homeroomTeacherId],
    references: [users.id],
  }),
  enrollments: many(enrollments),
  classSubjects: many(classSubjects),
  scheduledLessons: many(scheduledLessons),
}));

export const subjectsRelations = relations(subjects, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [subjects.orgId],
    references: [organizations.id],
  }),
  unit: one(orgUnits, { fields: [subjects.unitId], references: [orgUnits.id] }),
  classSubjects: many(classSubjects),
  scheduledLessons: many(scheduledLessons),
}));

export const classSubjectsRelations = relations(
  classSubjects,
  ({ one, many }) => ({
    class: one(classes, {
      fields: [classSubjects.classId],
      references: [classes.id],
    }),
    subject: one(subjects, {
      fields: [classSubjects.subjectId],
      references: [subjects.id],
    }),
    cycle: one(academicCycles, {
      fields: [classSubjects.cycleId],
      references: [academicCycles.id],
    }),
    primaryTeacher: one(users, {
      fields: [classSubjects.primaryTeacherId],
      references: [users.id],
      relationName: "primaryTeacher",
    }),
    assistantTeacher: one(users, {
      fields: [classSubjects.assistantTeacherId],
      references: [users.id],
      relationName: "assistantTeacher",
    }),
    assessments: many(assessments),
  }),
);

export const enrollmentsRelations = relations(enrollments, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [enrollments.orgId],
    references: [organizations.id],
  }),
  cycle: one(academicCycles, {
    fields: [enrollments.cycleId],
    references: [academicCycles.id],
  }),
  student: one(users, {
    fields: [enrollments.studentId],
    references: [users.id],
  }),
  class: one(classes, {
    fields: [enrollments.classId],
    references: [classes.id],
  }),
  grades: many(grades),
  attendance: many(attendanceDaily),
  attendanceSummaries: many(attendanceSummary),
  behaviorRecords: many(behaviorRecords),
  studentFees: many(studentFees),
  payments: many(payments),
  guardians: many(studentGuardians),
  discounts: many(studentDiscounts),
}));

export const studentGuardiansRelations = relations(
  studentGuardians,
  ({ one }) => ({
    student: one(users, {
      fields: [studentGuardians.studentId],
      references: [users.id],
    }),
    guardian: one(users, {
      fields: [studentGuardians.guardianId],
      references: [users.id],
    }),
  }),
);

export const timeSlotsRelations = relations(timeSlots, ({ one, many }) => ({
  cycle: one(academicCycles, {
    fields: [timeSlots.cycleId],
    references: [academicCycles.id],
  }),
  lessons: many(scheduledLessons),
}));

export const scheduledLessonsRelations = relations(
  scheduledLessons,
  ({ one }) => ({
    cycle: one(academicCycles, {
      fields: [scheduledLessons.cycleId],
      references: [academicCycles.id],
    }),
    class: one(classes, {
      fields: [scheduledLessons.classId],
      references: [classes.id],
    }),
    classSubject: one(classSubjects, {
      fields: [scheduledLessons.classSubjectId],
      references: [classSubjects.id],
    }),
    subject: one(subjects, {
      fields: [scheduledLessons.subjectId],
      references: [subjects.id],
    }),
    teacher: one(users, {
      fields: [scheduledLessons.teacherId],
      references: [users.id],
    }),
    substituteTeacher: one(users, {
      fields: [scheduledLessons.substituteTeacherId],
      references: [users.id],
    }),
    timeSlot: one(timeSlots, {
      fields: [scheduledLessons.timeSlotId],
      references: [timeSlots.id],
    }),
  }),
);

export const assessmentsRelations = relations(assessments, ({ one, many }) => ({
  cycle: one(academicCycles, {
    fields: [assessments.cycleId],
    references: [academicCycles.id],
  }),
  classSubject: one(classSubjects, {
    fields: [assessments.classSubjectId],
    references: [classSubjects.id],
  }),
  grades: many(grades),
}));

export const gradesRelations = relations(grades, ({ one, many }) => ({
  cycle: one(academicCycles, {
    fields: [grades.cycleId],
    references: [academicCycles.id],
  }),
  enrollment: one(enrollments, {
    fields: [grades.enrollmentId],
    references: [enrollments.id],
  }),
  assessment: one(assessments, {
    fields: [grades.assessmentId],
    references: [assessments.id],
  }),
  history: many(gradeHistory),
}));

export const gradeHistoryRelations = relations(gradeHistory, ({ one }) => ({
  grade: one(grades, {
    fields: [gradeHistory.gradeId],
    references: [grades.id],
  }),
}));

export const attendanceDailyRelations = relations(
  attendanceDaily,
  ({ one }) => ({
    cycle: one(academicCycles, {
      fields: [attendanceDaily.cycleId],
      references: [academicCycles.id],
    }),
    enrollment: one(enrollments, {
      fields: [attendanceDaily.enrollmentId],
      references: [enrollments.id],
    }),
  }),
);

export const attendanceSummaryRelations = relations(
  attendanceSummary,
  ({ one }) => ({
    enrollment: one(enrollments, {
      fields: [attendanceSummary.enrollmentId],
      references: [enrollments.id],
    }),
    cycle: one(academicCycles, {
      fields: [attendanceSummary.cycleId],
      references: [academicCycles.id],
    }),
  }),
);

export const behaviorRecordsRelations = relations(
  behaviorRecords,
  ({ one }) => ({
    cycle: one(academicCycles, {
      fields: [behaviorRecords.cycleId],
      references: [academicCycles.id],
    }),
    enrollment: one(enrollments, {
      fields: [behaviorRecords.enrollmentId],
      references: [enrollments.id],
    }),
  }),
);

// ============================================================
// FINANCE RELATIONS
// ============================================================

export const ledgerAccountsRelations = relations(
  ledgerAccounts,
  ({ one, many }) => ({
    organization: one(organizations, {
      fields: [ledgerAccounts.orgId],
      references: [organizations.id],
    }),
    parent: one(ledgerAccounts, {
      fields: [ledgerAccounts.parentId],
      references: [ledgerAccounts.id],
    }),
    children: many(ledgerAccounts),
    debitEntries: many(journalEntries, { relationName: "debitAccount" }),
    creditEntries: many(journalEntries, { relationName: "creditAccount" }),
  }),
);

export const journalEntriesRelations = relations(journalEntries, ({ one }) => ({
  organization: one(organizations, {
    fields: [journalEntries.orgId],
    references: [organizations.id],
  }),
  debitAccount: one(ledgerAccounts, {
    fields: [journalEntries.debitAccountId],
    references: [ledgerAccounts.id],
    relationName: "debitAccount",
  }),
  creditAccount: one(ledgerAccounts, {
    fields: [journalEntries.creditAccountId],
    references: [ledgerAccounts.id],
    relationName: "creditAccount",
  }),
}));

export const feeTemplatesRelations = relations(
  feeTemplates,
  ({ one, many }) => ({
    organization: one(organizations, {
      fields: [feeTemplates.orgId],
      references: [organizations.id],
    }),
    revenueAccount: one(ledgerAccounts, {
      fields: [feeTemplates.revenueAccountId],
      references: [ledgerAccounts.id],
    }),
    receivableAccount: one(ledgerAccounts, {
      fields: [feeTemplates.receivableAccountId],
      references: [ledgerAccounts.id],
    }),
    studentFees: many(studentFees),
  }),
);

export const discountTemplatesRelations = relations(
  discountTemplates,
  ({ one, many }) => ({
    organization: one(organizations, {
      fields: [discountTemplates.orgId],
      references: [organizations.id],
    }),
    applications: many(studentDiscounts),
  }),
);

export const paymentPlansRelations = relations(
  paymentPlans,
  ({ one, many }) => ({
    organization: one(organizations, {
      fields: [paymentPlans.orgId],
      references: [organizations.id],
    }),
    studentFees: many(studentFees),
  }),
);

export const studentFeesRelations = relations(studentFees, ({ one, many }) => ({
  cycle: one(academicCycles, {
    fields: [studentFees.cycleId],
    references: [academicCycles.id],
  }),
  enrollment: one(enrollments, {
    fields: [studentFees.enrollmentId],
    references: [enrollments.id],
  }),
  feeTemplate: one(feeTemplates, {
    fields: [studentFees.feeTemplateId],
    references: [feeTemplates.id],
  }),
  paymentPlan: one(paymentPlans, {
    fields: [studentFees.paymentPlanId],
    references: [paymentPlans.id],
  }),
  payments: many(payments),
  discounts: many(studentDiscounts),
  parentFee: one(studentFees, {
    fields: [studentFees.parentFeeId],
    references: [studentFees.id],
  }),
  childFees: many(studentFees),
}));

export const studentDiscountsRelations = relations(
  studentDiscounts,
  ({ one }) => ({
    discountTemplate: one(discountTemplates, {
      fields: [studentDiscounts.discountTemplateId],
      references: [discountTemplates.id],
    }),
    enrollment: one(enrollments, {
      fields: [studentDiscounts.enrollmentId],
      references: [enrollments.id],
    }),
    fee: one(studentFees, {
      fields: [studentDiscounts.appliedToFeeId],
      references: [studentFees.id],
    }),
  }),
);

export const paymentsRelations = relations(payments, ({ one }) => ({
  fee: one(studentFees, {
    fields: [payments.feeId],
    references: [studentFees.id],
  }),
  enrollment: one(enrollments, {
    fields: [payments.enrollmentId],
    references: [enrollments.id],
  }),
  treasury: one(treasuries, {
    fields: [payments.treasuryId],
    references: [treasuries.id],
  }),
  journalEntry: one(journalEntries, {
    fields: [payments.journalEntryId],
    references: [journalEntries.id],
  }),
}));

export const treasuriesRelations = relations(treasuries, ({ one, many }) => ({
  organization: one(organizations, {
    fields: [treasuries.orgId],
    references: [organizations.id],
  }),
  ledgerAccount: one(ledgerAccounts, {
    fields: [treasuries.ledgerAccountId],
    references: [ledgerAccounts.id],
  }),
  transactions: many(treasuryTransactions),
}));

export const treasuryTransactionsRelations = relations(
  treasuryTransactions,
  ({ one }) => ({
    treasury: one(treasuries, {
      fields: [treasuryTransactions.treasuryId],
      references: [treasuries.id],
    }),
    journalEntry: one(journalEntries, {
      fields: [treasuryTransactions.journalEntryId],
      references: [journalEntries.id],
    }),
  }),
);

export const financialEventsRelations = relations(
  financialEvents,
  ({ one }) => ({
    organization: one(organizations, {
      fields: [financialEvents.orgId],
      references: [organizations.id],
    }),
    journalEntry: one(journalEntries, {
      fields: [financialEvents.journalEntryId],
      references: [journalEntries.id],
    }),
  }),
);

export const financialReportsRelations = relations(
  financialReports,
  ({ one }) => ({
    organization: one(organizations, {
      fields: [financialReports.orgId],
      references: [organizations.id],
    }),
  }),
);

// ============================================================
// NOTIFICATIONS & DOCUMENTS RELATIONS
// ============================================================

export const notificationsRelations = relations(notifications, ({ one }) => ({
  sender: one(users, {
    fields: [notifications.senderId],
    references: [users.id],
  }),
  recipient: one(users, {
    fields: [notifications.recipientId],
    references: [users.id],
  }),
}));

export const notificationPreferencesRelations = relations(
  notificationPreferences,
  ({ one }) => ({
    user: one(users, {
      fields: [notificationPreferences.userId],
      references: [users.id],
    }),
  }),
);

export const documentsRelations = relations(documents, ({ one, many }) => ({
  owner: one(users, { fields: [documents.ownerId], references: [users.id] }),
  uploader: one(users, {
    fields: [documents.uploadedBy],
    references: [users.id],
  }),
  parent: one(documents, {
    fields: [documents.parentId],
    references: [documents.id],
  }),
  children: many(documents),
}));

// ============================================================
// AUDIT RELATIONS
// ============================================================

export const auditLogRelations = relations(auditLog, ({ one }) => ({
  user: one(users, { fields: [auditLog.userId], references: [users.id] }),
}));

export const systemLogRelations = relations(systemLog, ({ one }) => ({
  user: one(users, { fields: [systemLog.userId], references: [users.id] }),
}));

export const dataTransfersRelations = relations(dataTransfers, ({ one }) => ({
  organization: one(organizations, {
    fields: [dataTransfers.orgId],
    references: [organizations.id],
  }),
}));
