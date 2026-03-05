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
  jsonb,
  time
} from 'drizzle-orm/pg-core';
import { relations, sql } from 'drizzle-orm';

// ==========================================
// 1. Enums (Static Values)
// ==========================================
export const roleEnum = pgEnum('role', ['admin', 'student', 'parent', 'teacher', 'staff']);
export const userSexEnum = pgEnum('user_sex', ['unspecified','male', 'female']);
export const assessmentTypeEnum = pgEnum('assessment_type', ['exam', 'quiz', 'assignment', 'participation', 'oral', 'attendance', 'behavior', 'homework']);
export const classTypeEnum = pgEnum('class_type', ['classroom','laboratory','library','computer_lab','music_room','exam_hall','workshop','multi-purpose_room']);
export const studentStatusEnum = pgEnum('student_status', ['active', 'graduated', 'transferred', 'suspended']);
export const notificationTypeEnum = pgEnum('notification_type', ['grade_published', 'new_assessment', 'attendance_alert', 'announcement', 'message', 'parent_meeting', 'fee_reminder', 'account_action', 'behavior_alert', 'system']);
export const attendanceStatusEnum = pgEnum('attendance_status', ['present','absent','late','excused','sick','authorized_leave','field_trip']);
export const documentTypeEnum = pgEnum('document_type', ['file', 'folder']);
export const promotionStatusEnum = pgEnum('promotion_status', ['promoted', 'retained', 'transferred', 'graduated', 'conditional']);
export const promotionDecisionEnum = pgEnum('promotion_decision', ['auto', 'manual', 'override']);

// =========================================
// 2. Core Tables (Basic Tables)
// ==========================================

// Unified Users Table
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
  locale: varchar('locale', { length: 255 }).default('en'),
  timezone: varchar('timezone', { length: 255 }).default("Asia/Aden"),
  timeFormat: integer('time_format').default(24),
  weekStartsOnMonday: boolean('week_starts_on_monday').default(false),
  metadata: jsonb('metadata'),
  deletedAt: timestamp('deleted_at'),
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

// Academic Years Table
export const academicYears = pgTable('academic_years', {
  id: serial('id').primaryKey(),
  yearName: varchar('year_name', { length: 20 }).unique().notNull(), // e.g. 2025-2026
  startDate: date('start_date'),
  endDate: date('end_date'),
  isCurrent: boolean('is_current').default(false).notNull(),
  deletedAt: timestamp('deleted_at'),
});

// Academic Terms Table (First Term / Second Term)
export const academicTerms = pgTable('academic_terms', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 50 }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id),
  startDate: date('start_date'),
  endDate: date('end_date'),
  isCurrent: boolean('is_current').default(false),
  deletedAt: timestamp('deleted_at'),
});

// Subjects Table
export const subjects = pgTable('subjects', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  code: varchar('code', { length: 50 }).unique(),
  color: text('color').default("#FFF3E0").notNull(),
  deletedAt: timestamp('deleted_at'),
});

// Classes Table (Year-Independent — classes are permanent entities)
export const classes = pgTable('classes', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  type: classTypeEnum('type').default('classroom').notNull(),
  division: varchar('division', { length: 10 }),
  parentClassId: integer('parent_class_id').references((): any => classes.id, { onDelete: 'set null' }),
  color: text('color').default("#e6e6e6").notNull(),
  sortOrder: integer('sort_order').default(0).notNull(),
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  classNameIdx: index('class_name_idx').on(table.name),
  parentIdx: index('classes_parent_idx').on(table.parentClassId),
  sortOrderIdx: index('classes_sort_order_idx').on(table.sortOrder),
}));

// ==========================================
// 3. Optimized Student & Grades (Advanced)
// ==========================================

// Students Table with flexibility optimizations
export const students = pgTable('students', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  studentNumber: varchar('student_number', { length: 20 }).unique().notNull(),
  secretNumber: varchar('secret_number', { length: 20 }),
  currentClassId: integer('current_class_id').references(() => classes.id),
  status: studentStatusEnum('status').default('active'),
  notes: text('notes'),
  stats: jsonb('stats'),
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  studentNumIdx: index('student_num_idx').on(table.studentNumber),
  secretNumIdx: index('secret_num_idx').on(table.secretNumber),
}));

// Student Enrollments Table
export const enrollments = pgTable('enrollments', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }),
  classId: integer('class_id').references(() => classes.id),
  academicYearId: integer('academic_year_id').references(() => academicYears.id),
  status: studentStatusEnum('status').default('active'),
  enrolledAt: timestamp('enrolled_at').defaultNow().notNull(),
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  studentClassYearIdx: unique('student_class_year_unique').on(table.studentId, table.classId, table.academicYearId),
  classIdx: index('enrollments_class_idx').on(table.classId),
}));


// Assessment Items Table (Attendance, Oral, Quiz, Exam)
export const assessmentItems = pgTable('assessment_items', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  type: assessmentTypeEnum('type').notNull(),
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).notNull(),
  weight: numeric('weight', { precision: 5, scale: 2 }).default('1.0'), // الوزن النسبي
  subjectId: integer('subject_id').references(() => subjects.id),
  classId: integer('class_id').references(() => classes.id),
  month: integer('month').notNull(), // Month number (1-12)
  termId: integer('term_id').references(() => academicTerms.id),
  isPublished: boolean('is_published').default(false).notNull(),
  deletedAt: timestamp('deleted_at'),
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
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  studentAssessmentIdx: unique('student_assessment_unique').on(table.studentId, table.assessmentItemId),
  fastGradeLookupIdx: index('fast_grade_lookup_idx').on(table.assessmentItemId, table.score),
}));

// ==========================================
// 4. Audit Log (Transparency)
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
// Notifications Table
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


export const timeSlots = pgTable('time_slots', {
  id: serial('id').primaryKey(),
  slotNumber: integer('slot_number').notNull(),
  startTime: time('start_time').notNull(),
  endTime:   time('end_time').notNull(),
  durationMinutes: integer('duration_minutes').default(40).notNull(),
  isBreak: boolean('is_break').default(false),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  deletedAt: timestamp('deleted_at'),
}, (t) => ({
  uniqueSlotPerYear: unique('unique_slot_per_year').on(t.academicYearId, t.slotNumber),
  yearSlotIdx: index('time_slots_year_slot_idx').on(t.academicYearId, t.slotNumber),
}));

export const scheduledLessons = pgTable('scheduled_lessons', {
  id: serial('id').primaryKey(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'restrict' }).notNull(),
  teacherId: uuid('teacher_id').references(() => users.id, { onDelete: 'restrict' }).notNull(),
  timeSlotId: integer('time_slot_id').references(() => timeSlots.id, { onDelete: 'restrict' }).notNull(),
  dayOfWeek: integer('day_of_week').notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  academicTermId: integer('academic_term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  roomId: integer('room_id').references(() => classes.id, { onDelete: 'set null' }),
  isActive: boolean('is_active').default(true),
  notes: text('notes'),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (t) => ({
  uniqueLessonConstraint: unique('unique_lesson_per_class_slot_day').on(t.classId,t.timeSlotId,t.dayOfWeek,t.academicYearId),
  teacherDaySlotIdx: index('sched_teacher_day_slot_idx').on(t.teacherId, t.dayOfWeek, t.timeSlotId, t.academicYearId),
  classDaySlotIdx: index('sched_class_day_slot_idx').on(t.classId, t.dayOfWeek, t.timeSlotId, t.academicYearId),
}));

// Certificate Templates Table (Flexible System - 2026)
export const certificateTemplates = pgTable('certificate_templates', {
  id: uuid('id').defaultRandom().primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  isDefault: boolean('is_default').default(false).notNull(),
  
  // Visual & Layout Configuration
  orientation: varchar('orientation', { length: 20 }).default('portrait').notNull(),
  size: varchar('size', { length: 20 }).default('a4').notNull(),
  primaryColor: varchar('primary_color', { length: 20 }).default('#000000'),
  backgroundImageUrl: text('background_image_url'), // For Canva/Custom backgrounds
  
  // Rich Text Content (Tiptap JSON)
  // Rich Text Content (Tiptap JSON / Canvas Elements)
  content: jsonb('content'), // Single field for all certificate elements & configuration
  
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
});

export const attendanceRecords = pgTable('attendance_records', {
    id: serial('id').primaryKey(),
    studentId: uuid('student_id').notNull().references(() => students.userId, { onDelete: 'cascade' }),
    scheduledLessonId: integer('scheduled_lesson_id').notNull().references(() => scheduledLessons.id, { onDelete: 'cascade' }),
    date: date('date').notNull(),
    status: attendanceStatusEnum('status').notNull().default('absent'),
    arrivalTime: time('arrival_time'),
    notes: text('notes'),
    markedById: uuid('marked_by_id').references(() => users.id, { onDelete: 'set null' }),
    isVerified: boolean('is_verified').default(false).notNull(),
    deletedAt: timestamp('deleted_at'),
    createdAt: timestamp('created_at').defaultNow().notNull(),
    updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
    uniqueStudentLesson: unique('unique_attendance_student_lesson').on(table.studentId, table.scheduledLessonId),
    studentDateIdx: index('attendance_student_date_idx').on(table.studentId, table.date),
    lessonDateIdx: index('attendance_lesson_date_idx').on(table.scheduledLessonId, table.date),
    statusIdx: index('attendance_status_idx').on(table.status),
    compositeStudentLessonDate: index('attendance_student_lesson_date_idx').on(table.studentId, table.scheduledLessonId, table.date),
}));

// ==========================================
// Parent-Student Relationship (Many-to-Many)
// ==========================================
export const parentStudents = pgTable('parent_students', {
  id: serial('id').primaryKey(),
  parentId: uuid('parent_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  relationship: varchar('relationship', { length: 50 }).default('parent').notNull(), // parent, mother, father, guardian
  isPrimary: boolean('is_primary').default(false).notNull(), // Primary contact
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  parentStudentUnique: unique('parent_student_unique').on(table.parentId, table.studentId),
  parentIdx: index('ps_parent_idx').on(table.parentId),
  studentIdx: index('ps_student_idx').on(table.studentId),
}));


// ==========================================
// Vault / Documents Tables
// ==========================================

export const documents = pgTable('documents', {
  id: uuid('id').defaultRandom().primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  title: varchar('title', { length: 255 }),
  size: integer('size'),
  type: documentTypeEnum('type').default('file').notNull(),
  mimetype: varchar('mimetype', { length: 100 }),
  extension: varchar('extension', { length: 20 }),
  key: text('key'), // S3 Key
  pathTokens: jsonb('path_tokens').$type<string[]>(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  parentFolderId: uuid('parent_folder_id').references((): any => documents.id, { onDelete: 'cascade' }),
  metadata: jsonb('metadata').$type<Record<string, any>>(),
  processingStatus: varchar('processing_status', { length: 20 }).default('completed'),
  isPinned: boolean('is_pinned').default(false).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
  deletedAt: timestamp('deleted_at'),
}, (table) => ({
  userDocumentIdx: index('user_document_idx').on(table.userId, table.createdAt.desc()),
}));


export const shortLinks = pgTable('short_links', {
  id: uuid('id').defaultRandom().primaryKey(),
  code: varchar('code', { length: 255 }).unique().notNull(),
  url: text('url').notNull(),
  userId: uuid('user_id').references(() => users.id, { onDelete: 'cascade' }).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  expiresAt: timestamp('expires_at'),
}, (table) => ({
  codeIdx: index('short_links_code_idx').on(table.code),
}));

// ==========================================
// Student Promotions Table (Promotion System)
// ==========================================
export const studentPromotions = pgTable('student_promotions', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  fromClassId: integer('from_class_id').references(() => classes.id).notNull(),
  toClassId: integer('to_class_id').references(() => classes.id),
  fromAcademicYearId: integer('from_academic_year_id').references(() => academicYears.id).notNull(),
  toAcademicYearId: integer('to_academic_year_id').references(() => academicYears.id).notNull(),
  finalScore: numeric('final_score', { precision: 6, scale: 2 }),
  status: promotionStatusEnum('status').notNull(),
  decision: promotionDecisionEnum('decision').default('auto').notNull(),
  decisionById: uuid('decision_by_id').references(() => users.id, { onDelete: 'set null' }),
  notes: text('notes'),
  promotedAt: timestamp('promoted_at').defaultNow().notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
}, (table) => ({
  studentYearIdx: index('promo_student_year_idx').on(table.studentId, table.fromAcademicYearId),
  fromClassIdx: index('promo_from_class_idx').on(table.fromClassId, table.fromAcademicYearId),
  statusIdx: index('promo_status_idx').on(table.status),
  uniqueStudentPromotion: unique('unique_student_promotion_per_year').on(table.studentId, table.fromAcademicYearId),
}));


// ==========================================
// 5. Relations (Programmatic relations for easy querying)
// ==========================================

export const academicYearsRelations = relations(academicYears, ({ many }) => ({
  terms: many(academicTerms, { relationName: "yearTerms" }),
  enrollments: many(enrollments, { relationName: "enrollmentYear" }),
  promotionsFrom: many(studentPromotions, { relationName: "promotionsFromYear" }),
  promotionsTo: many(studentPromotions, { relationName: "promotionsToYear" }),
  lessons: many(scheduledLessons, { relationName: "yearLessons" }),
  timeSlots: many(timeSlots, { relationName: "yearTimeSlots" }),
}));

export const academicTermsRelations = relations(academicTerms, ({ one, many }) => ({
  year: one(academicYears, { fields: [academicTerms.academicYearId], references: [academicYears.id], relationName: "yearTerms" }),
  assessments: many(assessmentItems, { relationName: "termAssessments" }),
  notifications: many(notifications, { relationName: "termNotifications" }),
  lessons: many(scheduledLessons, { relationName: "termLessons" }),
}));

export const subjectsRelations = relations(subjects, ({ many }) => ({
  assessments: many(assessmentItems, { relationName: "subjectAssessments" }),
  lessons: many(scheduledLessons, { relationName: "subjectLessons" }),
}));

export const timeSlotsRelations = relations(timeSlots, ({ one, many }) => ({
  year: one(academicYears, { fields: [timeSlots.academicYearId], references: [academicYears.id], relationName: "yearTimeSlots" }),
  lessons: many(scheduledLessons, { relationName: "slotLessons" }),
}));

export const usersRelations = relations(users, ({ many, one }) => ({
  studentProfile: one(students, { fields: [users.id], references: [students.userId], relationName: "studentProfile" }),
  accounts: many(accounts, { relationName: "userAccounts" }),
  sessions: many(sessions, { relationName: "userSessions" }),
  auditLogs: many(auditLogs, { relationName: "userAuditLogs" }),
  gradedEntries: many(gradeEntries, { relationName: "gradedByUser" }),
  sentNotifications: many(notifications, { relationName: "sentNotifications" }),
  receivedNotifications: many(notifications, { relationName: "receivedNotifications" }),
  parentChildren: many(parentStudents, { relationName: "parentToStudents" }),
  docs: many(documents),
  shortLinks: many(shortLinks),
  lessons: many(scheduledLessons, { relationName: "teacherLessons" }),
  promotionDecisions: many(studentPromotions, { relationName: "promotionDecisions" }),
  attendanceRecords: many(attendanceRecords, { relationName: "markedAttendance" }),
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
  enrollments: many(enrollments, { relationName: "studentEnrollments" }),
  class: one(classes, { fields: [students.currentClassId], references: [classes.id], relationName: "studentClass" }),
  attendanceRecords: many(attendanceRecords, { relationName: 'studentAttendance' }),
  // Student → Parents (parents linked to this student)
  parentLinks: many(parentStudents, { relationName: "studentToParents" }),
  promotions: many(studentPromotions, { relationName: "studentPromotions" }),
}));

export const enrollmentsRelations = relations(enrollments, ({ one }) => ({
  student: one(students, { fields: [enrollments.studentId], references: [students.userId], relationName: "studentEnrollments" }),
  class: one(classes, { fields: [enrollments.classId], references: [classes.id], relationName: "enrollmentClass" }),
  academicYear: one(academicYears, { fields: [enrollments.academicYearId], references: [academicYears.id], relationName: "enrollmentYear" }),
}));

export const gradeEntriesRelations = relations(gradeEntries, ({ one }) => ({
  student: one(students, { fields: [gradeEntries.studentId], references: [students.userId], relationName: "studentGrades" }),
  assessmentItem: one(assessmentItems, { fields: [gradeEntries.assessmentItemId], references: [assessmentItems.id], relationName: "assessmentGrades" }),
  grader: one(users, { fields: [gradeEntries.gradedById], references: [users.id], relationName: "gradedByUser" }),
}));

export const assessmentItemsRelations = relations(assessmentItems, ({ one, many }) => ({
  subject: one(subjects, { fields: [assessmentItems.subjectId], references: [subjects.id], relationName: "subjectAssessments" }),
  class: one(classes, { fields: [assessmentItems.classId], references: [classes.id], relationName: "classAssessments" }),
  grades: many(gradeEntries, { relationName: "assessmentGrades" }),
  term: one(academicTerms, { fields: [assessmentItems.termId], references: [academicTerms.id], relationName: "termAssessments" }),
}));

export const auditLogsRelations = relations(auditLogs, ({ one }) => ({
  user: one(users, { fields: [auditLogs.userId], references: [users.id], relationName: "userAuditLogs" }),
}));

export const notificationsRelations = relations(notifications, ({ one }) => ({
  recipient: one(users, { fields: [notifications.recipientId], references: [users.id], relationName: "receivedNotifications" }),
  sender: one(users, { fields: [notifications.senderId], references: [users.id], relationName: "sentNotifications" }),
  class: one(classes, { fields: [notifications.classId], references: [classes.id], relationName: "classNotifications" }),
  term: one(academicTerms, { fields: [notifications.academicTermId], references: [academicTerms.id], relationName: "termNotifications" }),
}));

export const classesRelations = relations(classes, ({ one, many }) => ({
  parent: one(classes, { fields: [classes.parentClassId], references: [classes.id], relationName: "classChildren" }),
  children: many(classes, { relationName: "classChildren" }),
  enrollments: many(enrollments, { relationName: "enrollmentClass" }),
  students: many(students, { relationName: "studentClass" }),
  assessments: many(assessmentItems, { relationName: "classAssessments" }),
  lessons: many(scheduledLessons, { relationName: "classLessons" }),
  notifications: many(notifications, { relationName: "classNotifications" }),
  promotionsFrom: many(studentPromotions, { relationName: "promotionsFrom" }),
  promotionsTo: many(studentPromotions, { relationName: "promotionsTo" }),
}));

export const scheduledLessonsRelations = relations(scheduledLessons, ({ one, many }) => ({
  class: one(classes, { fields: [scheduledLessons.classId], references: [classes.id], relationName: "classLessons" }),
  subject: one(subjects, { fields: [scheduledLessons.subjectId], references: [subjects.id], relationName: "subjectLessons" }),
  teacher: one(users, { fields: [scheduledLessons.teacherId], references: [users.id], relationName: "teacherLessons" }),
  timeSlot: one(timeSlots, { fields: [scheduledLessons.timeSlotId], references: [timeSlots.id], relationName: "slotLessons" }),
  academicYear: one(academicYears, { fields: [scheduledLessons.academicYearId], references: [academicYears.id], relationName: "yearLessons" }),
  term: one(academicTerms, { fields: [scheduledLessons.academicTermId], references: [academicTerms.id], relationName: "termLessons" }),
  attendanceRecords: many(attendanceRecords, { relationName: 'lessonAttendance' }),
}));

export const attendanceRecordsRelations = relations(attendanceRecords, ({ one }) => ({
  student: one(students, { fields: [attendanceRecords.studentId], references: [students.userId], relationName: 'studentAttendance' }),
  scheduledLesson: one(scheduledLessons, { fields: [attendanceRecords.scheduledLessonId], references: [scheduledLessons.id], relationName: 'lessonAttendance' }),
  markedBy: one(users, { fields: [attendanceRecords.markedById], references: [users.id], relationName: 'markedAttendance' }),
}));

export const parentStudentsRelations = relations(parentStudents, ({ one }) => ({
  parent: one(users, { fields: [parentStudents.parentId], references: [users.id], relationName: "parentToStudents" }),
  student: one(students, { fields: [parentStudents.studentId], references: [students.userId], relationName: "studentToParents" }),
}));

export const documentsRelations = relations(documents, ({ one, many }) => ({
  user: one(users, { fields: [documents.userId], references: [users.id] }),
  parentFolder: one(documents, { fields: [documents.parentFolderId], references: [documents.id], relationName: "folderChildren" }),
  children: many(documents, { relationName: "folderChildren" }),
}));

export const shortLinksRelations = relations(shortLinks, ({ one }) => ({
  user: one(users, { fields: [shortLinks.userId], references: [users.id] }),
}));

export const studentPromotionsRelations = relations(studentPromotions, ({ one }) => ({
  student: one(students, { fields: [studentPromotions.studentId], references: [students.userId], relationName: "studentPromotions" }),
  fromClass: one(classes, { fields: [studentPromotions.fromClassId], references: [classes.id], relationName: "promotionsFrom" }),
  toClass: one(classes, { fields: [studentPromotions.toClassId], references: [classes.id], relationName: "promotionsTo" }),
  fromYear: one(academicYears, { fields: [studentPromotions.fromAcademicYearId], references: [academicYears.id], relationName: "promotionsFromYear" }),
  toYear: one(academicYears, { fields: [studentPromotions.toAcademicYearId], references: [academicYears.id], relationName: "promotionsToYear" }),
  decisionBy: one(users, { fields: [studentPromotions.decisionById], references: [users.id], relationName: "promotionDecisions" }),
}));
