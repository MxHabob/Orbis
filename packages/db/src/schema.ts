import { 
  pgTable, 
  serial, 
  integer, 
  varchar, 
  text, 
  boolean, 
  timestamp, 
  index,
  uuid,
  pgEnum,
  unique,
  numeric,
  date,
  jsonb
} from 'drizzle-orm/pg-core';
import { relations, sql } from 'drizzle-orm';

// ==========================================
// 1. Enums (القيم الثابتة)
// ==========================================
export const roleEnum = pgEnum('role', ['admin', 'student', 'parent', 'teacher', 'staff']);
export const userSexEnum = pgEnum('user_sex', ['unspecified','male', 'female']);
export const assessmentTypeEnum = pgEnum('assessment_type', [
  'exam', 'quiz', 'assignment', 'participation', 'oral', 'attendance', 'behavior', 'homework'
]);
export const studentStatusEnum = pgEnum('student_status', ['active', 'graduated', 'transferred', 'suspended']);
export const notificationTypeEnum = pgEnum('notification_type', ['grade_published', 'new_assessment', 'attendance_alert', 'announcement', 'message', 'parent_meeting', 'fee_reminder', 'account_action', 'behavior_alert', 'system']);

// ==========================================
// 2. Core Tables (الجداول الأساسية)
// ==========================================

// جدول المستخدمين الموحد
export const users = pgTable('users', {
  id: uuid('id').defaultRandom().primaryKey(),
  username: varchar('username', { length: 255 }).unique(),
  name: varchar('name', { length: 255 }),
  phone: varchar('phone', { length: 255 }).unique(),
  email: varchar('email', { length: 255 }).unique(),
  emailVerified: boolean('email_verified').default(false).notNull(),
  role: roleEnum('role').default('student').notNull(),
  sex: userSexEnum('sex').default('unspecified').notNull(),
  image: varchar('image', { length: 255 }),
  metadata: jsonb('metadata'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
});


export const accounts = pgTable('accounts', {
  id: uuid('id').defaultRandom().primaryKey(),
  accountId: varchar('account_id', { length: 255 }).notNull(),
  providerId: varchar('provider_id', { length: 255 }).notNull(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  accessToken: text('access_token'),
  refreshToken: text('refresh_token'),
  idToken: text('id_token'),
  scope: text('scope'),
  expiresAt: timestamp('expires_at'),
  password: text('password'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  providerAccountUnique: unique('accounts_provider_account_unique').on(table.providerId, table.accountId),
  providerUserUnique: unique('accounts_provider_user_unique').on(table.providerId, table.userId),
}));

export const sessions = pgTable('sessions', {
  id: uuid('id').defaultRandom().primaryKey(),
  token: varchar('token', { length: 255 }).unique().notNull(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  expiresAt: timestamp('expires_at').notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
  ipAddress: varchar('ip_address', { length: 45 }),
  userAgent: text('user_agent'),
});

export const verifications = pgTable('verifications', {
  id: uuid('id').defaultRandom().primaryKey(),
  identifier: varchar('identifier', { length: 255 }).notNull(),
  value: varchar('value', { length: 255 }).notNull(),
  expiresAt: timestamp('expires_at').notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  identifierValueUnique: unique('verifications_identifier_value_unique').on(table.identifier, table.value),
}));

// جدول السنوات الدراسية
export const academicYears = pgTable('academic_years', {
  id: serial('id').primaryKey(),
  yearName: varchar('year_name', { length: 20 }).unique().notNull(), // مثال: 2025-2026
  isCurrent: boolean('is_current').default(false).notNull(),
});

// جدول الفترات الدراسية (الترم الأول / الترم الثاني)
export const academicTerms = pgTable('academic_terms', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 50 }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id),
  startDate: date('start_date'),
  endDate: date('end_date'),
  isCurrent: boolean('is_current').default(false),
});

// جدول المواد الدراسية
export const subjects = pgTable('subjects', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  code: varchar('code', { length: 50 }).unique(),
  color_code: varchar('color_code', { length: 7 }).default("e6e6e6").notNull(),
});

// جدول الفصول الدراسية
export const classes = pgTable('classes', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  division: varchar('division', { length: 10 }),
  academicYearId: integer('academic_year_id').references(() => academicYears.id),
}, (table) => ({
  classYearIdx: index('class_year_idx').on(table.academicYearId, table.name),
}));

// ==========================================
// 3. Optimized Student & Grades (الطلاب والدرجات المطورة)
// ==========================================

// جدول الطلاب مع تحسينات المرونة
export const students = pgTable('students', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  studentNumber: varchar('student_number', { length: 20 }).unique().notNull(),
  secretNumber: varchar('secret_number', { length: 20 }),
  currentClassId: integer('current_class_id').references(() => classes.id),
  status: studentStatusEnum('status').default('active'),
  stats: jsonb('stats'),
}, (table) => ({
  studentNumIdx: index('student_num_idx').on(table.studentNumber),
  secretNumIdx: index('secret_num_idx').on(table.secretNumber),
}));

// جدول بنود التقييم (مواظبة، شفوي، حضور، اختبار)
export const assessmentItems = pgTable('assessment_items', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  type: assessmentTypeEnum('type').notNull(),
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).notNull(),
  weight: numeric('weight', { precision: 5, scale: 2 }).default('1.0'), // الوزن النسبي
  subjectId: integer('subject_id').references(() => subjects.id),
  classId: integer('class_id').references(() => classes.id),
  month: integer('month').notNull(), // رقم الشهر (1-12)
  termId: integer('term_id').references(() => academicTerms.id),
}, (table) => ({
  itemLookupIdx: index('item_lookup_idx').on(table.classId, table.subjectId, table.month),
}));

export const gradeEntries = pgTable('grade_entries', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }),
  assessmentItemId: integer('assessment_item_id').references(() => assessmentItems.id),
  score: numeric('score', { precision: 6, scale: 2 }),
  isAbsent: boolean('is_absent').default(false),
  comments: text('comments'),
  metadata: jsonb('metadata'),
  gradedById: uuid('graded_by_id').references(() => users.id),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  studentAssessmentIdx: unique('student_assessment_unique').on(table.studentId, table.assessmentItemId),
  fastGradeLookupIdx: index('fast_grade_lookup_idx').on(table.assessmentItemId, table.score),
}));

// ==========================================
// 4. Audit Log (سجل التدقيق لضمان الشفافية)
// ==========================================
export const auditLogs = pgTable('audit_logs', {
  id: serial('id').primaryKey(),
  action: varchar('action', { length: 50 }).notNull(),
  tableName: varchar('table_name', { length: 50 }).notNull(),
  recordId: varchar('record_id', { length: 100 }).notNull(),
  oldData: jsonb('old_data'),
  newData: jsonb('new_data'),
  userId: uuid('user_id').references(() => users.id),
  timestamp: timestamp('timestamp').defaultNow().notNull(),
});


// ==========================================
// جدول الإشعارات (Notifications)
// ==========================================

export const notifications = pgTable('notifications', {
  id: serial('id').primaryKey(),
  senderId: uuid('sender_id').references(() => users.id, { onDelete: 'set null' }),
  recipientId: uuid('recipient_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  type: notificationTypeEnum('type').notNull(),
  title: varchar('title', { length: 255 }).notNull(),
  message: text('message').notNull(),
  relatedEntityType: varchar('related_entity_type', { length: 50 }),
  relatedEntityId: varchar('related_entity_id', { length: 100 }),
  isRead: boolean('is_read').default(false).notNull(),
  isArchived: boolean('is_archived').default(false).notNull(),
  isHighPriority: boolean('is_high_priority').default(false).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  deliveredAt: timestamp('delivered_at'),
  metadata: jsonb('metadata'),
}, (table) => ({
  recipientCreatedIdx: index('notif_recipient_created_idx').on(table.recipientId, table.createdAt.desc()),
  recipientUnreadIdx: index('notif_recipient_unread_idx').on(table.recipientId).where(sql`${table.isRead} = false`),
  typeRecipientIdx:    index('notif_type_recipient_idx').on(table.type, table.recipientId),
}));
// ==========================================
// 5. Relations (العلاقات البرمجية لسهولة الاستعلام)
// ==========================================

export const usersRelations = relations(users, ({ many, one }) => ({
  studentProfile: one(students, { fields: [users.id], references: [students.userId], relationName: "studentProfile" }),
  accounts: many(accounts, { relationName: "userAccounts" }),
  sessions: many(sessions, { relationName: "userSessions" }),
  auditLogs: many(auditLogs, { relationName: "userAuditLogs" }),
  gradedEntries: many(gradeEntries, { relationName: "gradedByUser" }),
  sentNotifications: many(notifications, { relationName: "sentNotifications" }),
  receivedNotifications: many(notifications, { relationName: "receivedNotifications" }),
}));

export const accountsRelations = relations(accounts, ({ one }) => ({
  user: one(users, { fields: [accounts.userId], references: [users.id], relationName: "userAccounts" }),
}));

export const sessionsRelations = relations(sessions, ({ one }) => ({
  user: one(users, { fields: [sessions.userId], references: [users.id], relationName: "userSessions" }),
}));

export const studentsRelations = relations(students, ({ one, many }) => ({
  user: one(users, { fields: [students.userId], references: [users.id], relationName: "studentProfile" }),
  grades: many(gradeEntries, { relationName: "studentGrades" }),
  class: one(classes, { fields: [students.currentClassId], references: [classes.id], relationName: "studentClass" }),
}));

export const gradeEntriesRelations = relations(gradeEntries, ({ one }) => ({
  student: one(students, { fields: [gradeEntries.studentId], references: [students.userId], relationName: "studentGrades" }),
  assessmentItem: one(assessmentItems, { fields: [gradeEntries.assessmentItemId], references: [assessmentItems.id], relationName: "assessmentGrades" }),
  grader: one(users, { fields: [gradeEntries.gradedById], references: [users.id], relationName: "gradedByUser" }),
}));

export const assessmentItemsRelations = relations(assessmentItems, ({ one, many }) => ({
  subject: one(subjects, { fields: [assessmentItems.subjectId], references: [subjects.id], relationName: "assessmentItems" }),
  class: one(classes, { fields: [assessmentItems.classId], references: [classes.id], relationName: "assessmentItems" }),
  grades: many(gradeEntries, { relationName: "assessmentGrades" }),
}));

export const auditLogsRelations = relations(auditLogs, ({ one }) => ({
  user: one(users, { fields: [auditLogs.userId], references: [users.id], relationName: "userAuditLogs" }),
}));

export const notificationsRelations = relations(notifications, ({ one }) => ({
  recipient: one(users, { fields: [notifications.recipientId], references: [users.id], relationName: "receivedNotifications" }),
  sender: one(users, { fields: [notifications.senderId], references: [users.id], relationName: "sentNotifications" }),
  class: one(classes, { fields: [notifications.classId], references: [classes.id], relationName: "notifications" }),
  term: one(academicTerms, { fields: [notifications.academicTermId], references: [academicTerms.id], relationName: "notifications" }),
}));