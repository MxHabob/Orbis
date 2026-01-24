import { 
  pgTable, 
  serial, 
  integer, 
  varchar, 
  text, 
  boolean, 
  timestamp, 
  index,
  primaryKey,
  uuid,
  pgEnum,
  unique,
  numeric,
  date
} from 'drizzle-orm/pg-core';
import { relations } from 'drizzle-orm';

// Enums
export const roleEnum = pgEnum('role', ['admin', 'student', 'parent']);
export const userSexEnum = pgEnum('user_sex', ['male', 'female']);
export const staffRoleEnum = pgEnum('staff_role', ['teacher', 'admin', 'accountant', 'nurse', 'security', 'other']);
export const stageEnum = pgEnum('stage', ['basic', 'secondary']);
export const trackEnum = pgEnum('track', ['scientific', 'literary', 'none']);
export const sectionEnum = pgEnum('section', ['male', 'female', 'mixed']);
export const dayEnum = pgEnum('day', ['monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday']);
export const assessmentTypeEnum = pgEnum('assessment_type', ['exam', 'quiz', 'assignment', 'project', 'participation', 'oral', 'practical', 'ministerial']);
export const examTypeEnum = pgEnum('exam_type', ['monthly', 'semester1', 'semester2', 'final_year', 'ministerial']);
export const studentStatusEnum = pgEnum('student_status', ['active', 'graduated', 'transferred', 'suspended', 'dropped', 'repeating']);
export const paymentStatusEnum = pgEnum('payment_status', ['pending', 'paid', 'overdue']);
export const feeTypeEnum = pgEnum('fee_type', ['tuition', 'registration', 'transportation', 'books', 'other']);
export const departmentEnum = pgEnum('department', ['sciences', 'literary', 'mathematics', 'languages', 'religious', 'social', 'other']);
export const promotionStatusEnum = pgEnum('promotion_status', ['passed', 'failed', 'repeat', 'promoted', 'supplementary']);
export const reportTypeEnum = pgEnum('report_type', ['ministerial_grade9', 'ministerial_grade12']);

// Tables
export const academicYears = pgTable('academic_years', {
  id: serial('id').primaryKey(),
  year: varchar('year', { length: 20 }).unique().notNull(),
  startDate: timestamp('start_date').notNull(),
  endDate: timestamp('end_date'),
  isCurrent: boolean('is_current').default(false).notNull(),
  isActive: boolean('is_active').default(true).notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
}, (table) => ({
  currentIdx: index('academic_years_is_current_idx').on(table.isCurrent),
}));

export const academicTerms = pgTable('academic_terms', {
  id: serial('id').primaryKey(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  name: varchar('name', { length: 100 }).notNull(),
  startDate: timestamp('start_date').notNull(),
  endDate: timestamp('end_date'),
  order: integer('order').notNull(),
}, (table) => ({
  uniqueYearOrder: unique('academic_terms_academic_year_id_order_unique').on(table.academicYearId, table.order),
}));

export const users = pgTable('users', {
  id: uuid('id').defaultRandom().primaryKey(),
  name: varchar('name', { length: 255 }),
  email: varchar('email', { length: 255 }).unique(),
  emailVerified: boolean('email_verified').default(false),
  image: text('image'),
  role: roleEnum('role').default('student').notNull(),
  createdAt: timestamp('created_at').defaultNow().notNull(),
  updatedAt: timestamp('updated_at').defaultNow().notNull(),
});

export const levels = pgTable('levels', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  stage: stageEnum('stage').notNull(),
  order: integer('order').unique().notNull(),
});

export const subjects = pgTable('subjects', {
  id: serial('id').primaryKey(),
  code: varchar('code', { length: 50 }).unique(),
  name: varchar('name', { length: 255 }).notNull(),
  shortName: varchar('short_name', { length: 50 }),
  department: departmentEnum('department'),
  description: text('description'),
});

export const employees = pgTable('employees', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  employeeNumber: varchar('employee_number', { length: 50 }).unique(),
  hireDate: timestamp('hire_date'),
  phone: varchar('phone', { length: 50 }),
  address: text('address'),
  sex: userSexEnum('sex').notNull(),
  birthday: timestamp('birthday'),
  nationality: varchar('nationality', { length: 100 }).default('Yemeni'),
});

export const employeeRoles = pgTable('employee_roles', {
  employeeId: uuid('employee_id').references(() => employees.userId, { onDelete: 'cascade' }).notNull(),
  role: staffRoleEnum('role').notNull(),
  startDate: timestamp('start_date'),
  endDate: timestamp('end_date'),
}, (table) => ({
  pk: primaryKey({ columns: [table.employeeId, table.role] }),
}));

export const teacherSubjects = pgTable('teacher_subjects', {
  id: serial('id').primaryKey(),
  employeeId: uuid('employee_id').references(() => employees.userId, { onDelete: 'cascade' }).notNull(),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'cascade' }).notNull(),
  startYear: integer('start_year'),
  endYear: integer('end_year'),
}, (table) => ({
  uniqueEmployeeSubject: unique('teacher_subjects_employee_subject_unique').on(table.employeeId, table.subjectId),
}));

export const curriculumLevels = pgTable('curriculum_levels', {
  id: serial('id').primaryKey(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  levelId: integer('level_id').references(() => levels.id, { onDelete: 'cascade' }).notNull(),
  isActive: boolean('is_active').default(true).notNull(),
}, (table) => ({
  uniqueYearLevel: unique('curriculum_levels_year_level_unique').on(table.academicYearId, table.levelId),
}));

export const classes = pgTable('classes', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  code: varchar('code', { length: 50 }).unique(),
  division: varchar('division', { length: 10 }), // أ، ب، ج، 1، 2...
  capacity: integer('capacity'),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  levelId: integer('level_id').references(() => levels.id, { onDelete: 'cascade' }).notNull(),
  section: sectionEnum('section'),
  track: trackEnum('track'),
  supervisorId: uuid('supervisor_id').references(() => employees.userId, { onDelete: 'set null' }),
  current: boolean('current').default(true).notNull(),
}, (table) => ({
  yearNameIdx: index('classes_academic_year_name_idx').on(table.academicYearId, table.name),
  levelTrackIdx: index('classes_level_track_idx').on(table.levelId, table.track),
  divisionIdx: index('classes_division_idx').on(table.division),
}));

export const curriculumSubjects = pgTable('curriculum_subjects', {
  id: serial('id').primaryKey(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  levelId: integer('level_id').references(() => levels.id, { onDelete: 'set null' }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
  subjectId: integer('subject_id').references(() => subjects.id, { onDelete: 'cascade' }).notNull(),
  isMandatory: boolean('is_mandatory').default(true).notNull(),
  weeklyPeriods: numeric('weekly_periods', { precision: 4, scale: 2 }),
  orderInReport: integer('order_in_report'),
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).default('100'),
}, (table) => ({
  uniqueYearLevelSubject: unique('curriculum_subjects_year_level_subject_unique').on(table.academicYearId, table.levelId, table.subjectId),
  yearIdx: index('curriculum_subjects_year_idx').on(table.academicYearId),
}));

export const classSubjects = pgTable('class_subjects', {
  id: serial('id').primaryKey(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  curriculumSubjectId: integer('curriculum_subject_id').references(() => curriculumSubjects.id, { onDelete: 'cascade' }).notNull(),
}, (table) => ({
  uniqueClassCurriculum: unique('class_subjects_class_curriculum_unique').on(table.classId, table.curriculumSubjectId),
}));

export const teacherClassSubjects = pgTable('teacher_class_subjects', {
  id: serial('id').primaryKey(),
  employeeId: uuid('employee_id').references(() => employees.userId, { onDelete: 'cascade' }).notNull(),
  curriculumSubjectId: integer('curriculum_subject_id').references(() => curriculumSubjects.id, { onDelete: 'cascade' }).notNull(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  classSubjectId: integer('class_subject_id').references(() => classSubjects.id, { onDelete: 'set null' }),
  teacherSubjectId: integer('teacher_subject_id').references(() => teacherSubjects.id, { onDelete: 'set null' }),
  startDate: timestamp('start_date'),
  endDate: timestamp('end_date'),
}, (table) => ({
  uniqueAssignment: unique('teacher_class_subjects_unique_assignment').on(table.employeeId, table.curriculumSubjectId, table.classId),
}));

export const assessmentCategories = pgTable('assessment_categories', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  weight: numeric('weight', { precision: 5, scale: 2 }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  curriculumSubjectId: integer('curriculum_subject_id').references(() => curriculumSubjects.id, { onDelete: 'cascade' }),
  order: integer('order'),
}, (table) => ({
  uniqueNameSubject: unique('assessment_categories_name_subject_unique').on(table.name, table.curriculumSubjectId),
}));

export const assessmentItems = pgTable('assessment_items', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  type: assessmentTypeEnum('type').notNull(),
  maxScore: numeric('max_score', { precision: 6, scale: 2 }).default('100').notNull(),
  weight: numeric('weight', { precision: 5, scale: 2 }),
  dueDate: timestamp('due_date'),
  assessmentCategoryId: integer('assessment_category_id').references(() => assessmentCategories.id, { onDelete: 'set null' }),
  curriculumSubjectId: integer('curriculum_subject_id').references(() => curriculumSubjects.id, { onDelete: 'set null' }),
  termId: integer('term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  isMinisterial: boolean('is_ministerial').default(false).notNull(),
});

export const students = pgTable('students', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  studentNumber: varchar('student_number', { length: 50 }).unique().notNull(),
  currentClassId: integer('current_class_id').references(() => classes.id, { onDelete: 'set null' }),
  currentAcademicYearId: integer('current_academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  enrollmentDate: timestamp('enrollment_date'),
  status: studentStatusEnum('status').default('active').notNull(),
  surname: varchar('surname', { length: 255 }).notNull(),
  phone: varchar('phone', { length: 50 }).unique(),
  address: text('address'),
  sex: userSexEnum('sex').notNull(),
  birthday: timestamp('birthday').notNull(),
  nationality: varchar('nationality', { length: 100 }).default('Yemeni'),
  religion: varchar('religion', { length: 50 }).default('Muslim'),
  parentId: uuid('parent_id').references(() => parents.userId, { onDelete: 'set null' }),
}, (table) => ({
  currentClassIdx: index('students_current_class_idx').on(table.currentClassId),
  currentYearIdx: index('students_current_year_idx').on(table.currentAcademicYearId),
  studentNumberIdx: index('students_student_number_idx').on(table.studentNumber),
}));

export const parents = pgTable('parents', {
  userId: uuid('user_id').primaryKey().references(() => users.id, { onDelete: 'cascade' }),
  surname: varchar('surname', { length: 255 }).notNull(),
  phone: varchar('phone', { length: 50 }).unique().notNull(),
  address: text('address'),
});

export const studentEnrollments = pgTable('student_enrollments', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'cascade' }).notNull(),
  status: varchar('status', { length: 100 }),
  finalAverage: numeric('final_average', { precision: 5, scale: 2 }),
  ministerialTotalScore: numeric('ministerial_total_score', { precision: 7, scale: 2 }),
  ministerialAverage: numeric('ministerial_average', { precision: 5, scale: 2 }),
  promotionDecision: promotionStatusEnum('promotion_decision'),
}, (table) => ({
  uniqueStudentYear: unique('student_enrollments_student_year_unique').on(table.studentId, table.academicYearId),
}));

export const lessons = pgTable('lessons', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 255 }),
  day: dayEnum('day').notNull(),
  startTime: timestamp('start_time').notNull(),
  endTime: timestamp('end_time').notNull(),
  teacherClassSubjectId: integer('teacher_class_subject_id').references(() => teacherClassSubjects.id, { onDelete: 'cascade' }).notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
  teacherId: uuid('teacher_id').references(() => employees.userId, { onDelete: 'set null' }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
});

export const attendances = pgTable('attendances', {
  id: serial('id').primaryKey(),
  date: timestamp('date').notNull(),
  present: boolean('present').notNull(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  lessonId: integer('lesson_id').references(() => lessons.id, { onDelete: 'set null' }),
  termId: integer('term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
});

export const gradeEntries = pgTable('grade_entries', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  assessmentItemId: integer('assessment_item_id').references(() => assessmentItems.id, { onDelete: 'cascade' }).notNull(),
  score: numeric('score', { precision: 6, scale: 2 }),
  isAbsent: boolean('is_absent').default(false).notNull(),
  comments: text('comments'),
  gradedAt: timestamp('graded_at').defaultNow().notNull(),
  gradedById: uuid('graded_by_id').references(() => employees.userId, { onDelete: 'set null' }),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'cascade' }).notNull(),
  termId: integer('term_id').references(() => academicTerms.id, { onDelete: 'cascade' }).notNull(),
  isFinal: boolean('is_final').default(false).notNull(),
}, (table) => ({
  uniqueStudentAssessment: unique('grade_entries_student_assessment_unique').on(table.studentId, table.assessmentItemId),
  studentYearIdx: index('grade_entries_student_year_idx').on(table.studentId, table.academicYearId),
  assessmentItemIdx: index('grade_entries_assessment_item_idx').on(table.assessmentItemId),
  termIdx: index('grade_entries_term_idx').on(table.termId),
}));

export const exams = pgTable('exams', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  startTime: timestamp('start_time').notNull(),
  endTime: timestamp('end_time').notNull(),
  type: examTypeEnum('type').notNull(),
  termId: integer('term_id').references(() => academicTerms.id, { onDelete: 'set null' }),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
});

export const examCommittees = pgTable('exam_committees', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  examId: integer('exam_id').references(() => exams.id, { onDelete: 'cascade' }).notNull(),
  capacity: integer('capacity'),
  location: varchar('location', { length: 255 }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
});

export const examSeatings = pgTable('exam_seatings', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  committeeId: integer('committee_id').references(() => examCommittees.id, { onDelete: 'cascade' }).notNull(),
  seatingNumber: varchar('seating_number', { length: 50 }).unique().notNull(),
  assignedAt: timestamp('assigned_at').defaultNow().notNull(),
});

export const payments = pgTable('payments', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  feeType: feeTypeEnum('fee_type').notNull(),
  amount: numeric('amount', { precision: 10, scale: 2 }).notNull(),
  dueDate: timestamp('due_date').notNull(),
  paidDate: timestamp('paid_date'),
  status: paymentStatusEnum('status').notNull(),
  description: text('description'),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
}, (table) => ({
  studentYearIdx: index('payments_student_year_idx').on(table.studentId, table.academicYearId),
}));

export const certificates = pgTable('certificates', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  issueDate: timestamp('issue_date').notNull(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'set null' }),
  details: text('details'),
});

export const healthRecords = pgTable('health_records', {
  id: serial('id').primaryKey(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
  record: text('record').notNull(),
  date: timestamp('date').defaultNow().notNull(),
});

export const extracurriculars = pgTable('extracurriculars', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 255 }).notNull(),
  description: text('description'),
  startDate: timestamp('start_date').notNull(),
  endDate: timestamp('end_date'),
});

export const extracurricularStudents = pgTable('extracurricular_students', {
  extracurricularId: integer('extracurricular_id').references(() => extracurriculars.id, { onDelete: 'cascade' }).notNull(),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'cascade' }).notNull(),
}, (table) => ({
  pk: primaryKey({ columns: [table.extracurricularId, table.studentId] }),
}));

export const ministerialReports = pgTable('ministerial_reports', {
  id: serial('id').primaryKey(),
  title: varchar('title', { length: 255 }).notNull(),
  reportDate: timestamp('report_date').notNull(),
  content: text('content').notNull(),
  generatedBy: varchar('generated_by', { length: 255 }).notNull(),
  levelId: integer('level_id').references(() => levels.id, { onDelete: 'set null' }),
  classId: integer('class_id').references(() => classes.id, { onDelete: 'set null' }),
  studentId: uuid('student_id').references(() => students.userId, { onDelete: 'set null' }),
  totalScore: numeric('total_score', { precision: 7, scale: 2 }),
  average: numeric('average', { precision: 5, scale: 2 }),
  passStatus: varchar('pass_status', { length: 100 }),
  reportType: reportTypeEnum('report_type'),
});

export const holidays = pgTable('holidays', {
  id: serial('id').primaryKey(),
  name: varchar('name', { length: 100 }).notNull(),
  date: date('date').notNull(),
  academicYearId: integer('academic_year_id').references(() => academicYears.id, { onDelete: 'set null' }),
}, (table) => ({
  dateIdx: index('holidays_date_idx').on(table.date),
}));

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

// ==================== Relations ====================

export const academicYearsRelations = relations(academicYears, ({ many }) => ({
  terms: many(academicTerms),
  classes: many(classes),
  curriculumSubjects: many(curriculumSubjects),
  lessons: many(lessons),
  exams: many(exams),
  attendances: many(attendances),
  payments: many(payments),
  curriculumLevels: many(curriculumLevels),
  gradeEntries: many(gradeEntries),
  assessmentCategories: many(assessmentCategories),
  enrollments: many(studentEnrollments),
  currentStudents: many(students, { relationName: 'currentAcademicYear' }),
  holidays: many(holidays),
}));

export const academicTermsRelations = relations(academicTerms, ({ one, many }) => ({
  academicYear: one(academicYears, {
    fields: [academicTerms.academicYearId],
    references: [academicYears.id],
  }),
  assessmentItems: many(assessmentItems),
  exams: many(exams),
  attendances: many(attendances),
  gradeEntries: many(gradeEntries),
}));

export const usersRelations = relations(users, ({ one, many }) => ({
  employee: one(employees, { fields: [users.id], references: [employees.userId] }),
  student: one(students, { fields: [users.id], references: [students.userId] }),
  parent: one(parents, { fields: [users.id], references: [parents.userId] }),
  accounts: many(accounts),
  sessions: many(sessions),
}));

export const employeesRelations = relations(employees, ({ one, many }) => ({
  user: one(users, { fields: [employees.userId], references: [users.id] }),
  roles: many(employeeRoles),
  teacherSubjects: many(teacherSubjects),
  supervisedClasses: many(classes, { relationName: 'supervisor' }),
  lessons: many(lessons),
  gradeEntries: many(gradeEntries),
  teacherClassSubjects: many(teacherClassSubjects),
}));

export const levelsRelations = relations(levels, ({ many }) => ({
  curriculumLevels: many(curriculumLevels),
  classes: many(classes),
  curriculumSubjects: many(curriculumSubjects),
  ministerialReports: many(ministerialReports),
}));

export const subjectsRelations = relations(subjects, ({ many }) => ({
  teacherSubjects: many(teacherSubjects),
  curriculumSubjects: many(curriculumSubjects),
}));

export const classesRelations = relations(classes, ({ one, many }) => ({
  academicYear: one(academicYears, { fields: [classes.academicYearId], references: [academicYears.id] }),
  level: one(levels, { fields: [classes.levelId], references: [levels.id] }),
  supervisor: one(employees, { fields: [classes.supervisorId], references: [employees.userId], relationName: 'supervisor' }),
  students: many(students, { relationName: 'currentClass' }),
  lessons: many(lessons),
  classSubjects: many(classSubjects),
  examCommittees: many(examCommittees),
  enrollments: many(studentEnrollments),
  teacherClassSubjects: many(teacherClassSubjects),
  ministerialReports: many(ministerialReports),
}));

export const curriculumSubjectsRelations = relations(curriculumSubjects, ({ one, many }) => ({
  academicYear: one(academicYears, { fields: [curriculumSubjects.academicYearId], references: [academicYears.id] }),
  level: one(levels, { fields: [curriculumSubjects.levelId], references: [levels.id] }),
  class: one(classes, { fields: [curriculumSubjects.classId], references: [classes.id] }),
  subject: one(subjects, { fields: [curriculumSubjects.subjectId], references: [subjects.id] }),
  teacherAssignments: many(teacherClassSubjects),
  assessmentCategories: many(assessmentCategories),
  assessmentItems: many(assessmentItems),
  classSubjects: many(classSubjects),
}));

export const teacherClassSubjectsRelations = relations(teacherClassSubjects, ({ one, many }) => ({
  employee: one(employees, {
    fields: [teacherClassSubjects.employeeId],
    references: [employees.userId],
  }),
  curriculumSubject: one(curriculumSubjects, {
    fields: [teacherClassSubjects.curriculumSubjectId],
    references: [curriculumSubjects.id],
  }),
  class: one(classes, {
    fields: [teacherClassSubjects.classId],
    references: [classes.id],
  }),
  classSubject: one(classSubjects, {
    fields: [teacherClassSubjects.classSubjectId],
    references: [classSubjects.id],
  }),
  teacherSubject: one(teacherSubjects, {
    fields: [teacherClassSubjects.teacherSubjectId],
    references: [teacherSubjects.id],
  }),
  lessons: many(lessons),
}));

export const assessmentCategoriesRelations = relations(assessmentCategories, ({ one, many }) => ({
  curriculumSubject: one(curriculumSubjects, {
    fields: [assessmentCategories.curriculumSubjectId],
    references: [curriculumSubjects.id],
  }),
  academicYear: one(academicYears, {
    fields: [assessmentCategories.academicYearId],
    references: [academicYears.id],
  }),
  items: many(assessmentItems),
}));

export const assessmentItemsRelations = relations(assessmentItems, ({ one, many }) => ({
  category: one(assessmentCategories, {
    fields: [assessmentItems.assessmentCategoryId],
    references: [assessmentCategories.id],
  }),
  curriculumSubject: one(curriculumSubjects, {
    fields: [assessmentItems.curriculumSubjectId],
    references: [curriculumSubjects.id],
  }),
  term: one(academicTerms, {
    fields: [assessmentItems.termId],
    references: [academicTerms.id],
  }),
  grades: many(gradeEntries),
}));

export const studentsRelations = relations(students, ({ one, many }) => ({
  user: one(users, { fields: [students.userId], references: [users.id] }),
  currentClass: one(classes, { fields: [students.currentClassId], references: [classes.id], relationName: 'currentClass' }),
  currentAcademicYear: one(academicYears, { fields: [students.currentAcademicYearId], references: [academicYears.id], relationName: 'currentAcademicYear' }),
  parent: one(parents, { fields: [students.parentId], references: [parents.userId] }),
  enrollments: many(studentEnrollments),
  grades: many(gradeEntries),
  attendances: many(attendances),
  payments: many(payments),
  certificates: many(certificates),
  examSeatings: many(examSeatings),
  extracurriculars: many(extracurricularStudents),
  healthRecords: many(healthRecords),
  ministerialReports: many(ministerialReports),
}));

export const parentsRelations = relations(parents, ({ one, many }) => ({
  user: one(users, { fields: [parents.userId], references: [users.id] }),
  students: many(students),
}));

export const lessonsRelations = relations(lessons, ({ one, many }) => ({
  teacherClassSubject: one(teacherClassSubjects, { fields: [lessons.teacherClassSubjectId], references: [teacherClassSubjects.id] }),
  academicYear: one(academicYears, { fields: [lessons.academicYearId], references: [academicYears.id] }),
  teacher: one(employees, { fields: [lessons.teacherId], references: [employees.userId] }),
  class: one(classes, { fields: [lessons.classId], references: [classes.id] }),
  attendances: many(attendances),
}));

export const examsRelations = relations(exams, ({ one, many }) => ({
  term: one(academicTerms, { fields: [exams.termId], references: [academicTerms.id] }),
  academicYear: one(academicYears, { fields: [exams.academicYearId], references: [academicYears.id] }),
  committees: many(examCommittees),
  assessmentItems: many(assessmentItems),
}));

export const extracurricularsRelations = relations(extracurriculars, ({ many }) => ({
  students: many(extracurricularStudents),
}));

export const extracurricularStudentsRelations = relations(extracurricularStudents, ({ one }) => ({
  extracurricular: one(extracurriculars, { fields: [extracurricularStudents.extracurricularId], references: [extracurriculars.id] }),
  student: one(students, { fields: [extracurricularStudents.studentId], references: [students.userId] }),
}));

export const holidaysRelations = relations(holidays, ({ one }) => ({
  academicYear: one(academicYears, {
    fields: [holidays.academicYearId],
    references: [academicYears.id],
  }),
}));