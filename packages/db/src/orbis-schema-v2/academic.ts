import {
  uuid,
  varchar,
  text,
  boolean,
  timestamp,
  date,
  time,
  index,
  unique,
  integer,
  smallint,
  jsonb,
} from "drizzle-orm/pg-core";
import { sql } from "drizzle-orm/sql";
import { academicSchema } from "./types";
import { scoreType, percentType, hijriDateType } from "./types";
import { academicStatusEnum, attendanceEnum } from "./enums";

// ============================================================
// ACADEMIC CYCLES (Years, Terms, Quarters)
// ============================================================

export const academicCycles = academicSchema.table(
  "academic_cycles",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 50 }).notNull(),
    nameAr: varchar("name_ar", { length: 50 }),
    code: varchar("code", { length: 20 }),

    type: varchar("type", { length: 10 }).notNull(),
    parentId: uuid("parent_id"),

    startDate: date("start_date").notNull(),
    endDate: date("end_date").notNull(),

    hijriStartDate: hijriDateType("hijri_start_date"),
    hijriEndDate: hijriDateType("hijri_end_date"),

    examStartDate: date("exam_start_date"),
    examEndDate: date("exam_end_date"),

    weight: percentType("weight"),
    gradingSystem: varchar("grading_system", { length: 10 })
      .default("percentage")
      .notNull(),
    passingGrade: percentType("passing_grade").default(50),
    maxGrade: smallint("max_grade").default(100),

    isCurrent: boolean("is_current").default(false).notNull(),
    isLocked: boolean("is_locked").default(false).notNull(),
    isEnrollmentOpen: boolean("is_enrollment_open").default(false).notNull(),

    registrationFee: integer("registration_fee"),
    lateRegistrationFee: integer("late_registration_fee"),

    settings: jsonb("settings")
      .default("{}")
      .$type<{
        reportCardTemplate?: string;
        certificateTemplate?: string;
        promotionCriteria?: {
          minAttendance?: number;
          minGpa?: number;
          behaviorThreshold?: number;
        };
      }>(),

    sortOrder: smallint("sort_order").default(0),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgCurrentIdx: index("cycle_org_current_idx")
      .on(t.orgId, t.isCurrent)
      .where(sql`${t.type} = 'year' AND ${t.isCurrent} = true`),
    orgTypeIdx: index("cycle_org_type_idx").on(t.orgId, t.type, t.startDate),
    parentIdx: index("cycle_parent_idx").on(t.parentId),
    dateRangeIdx: index("cycle_date_range_idx").on(t.startDate, t.endDate),
    orgCodeIdx: unique("cycle_org_code_idx").on(t.orgId, t.code),
  })
);

// ============================================================
// ORGANIZATIONAL UNITS (Hierarchical)
// ============================================================

export const orgUnits = academicSchema.table(
  "org_units",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),

    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    code: varchar("code", { length: 20 }),

    type: varchar("type", { length: 20 }).notNull(),
    parentId: uuid("parent_id"),
    path: varchar("path", { length: 500 }).notNull(),
    depth: smallint("depth").default(0).notNull(),

    headId: uuid("head_id"),

    level: varchar("level", { length: 20 }),
    schoolType: varchar("school_type", { length: 20 }),

    roomNumber: varchar("room_number", { length: 20 }),
    floor: smallint("floor"),
    building: varchar("building", { length: 50 }),
    capacity: smallint("capacity"),

    address: text("address"),
    phone: varchar("phone", { length: 15 }),
    email: varchar("email", { length: 254 }),

    settings: jsonb("settings")
      .default("{}")
      .$type<{
        maxClassSize?: number;
        levels?: string[];
        logo?: string;
        color?: string;
        facilities?: string[];
      }>(),

    sortOrder: smallint("sort_order").default(0),
    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgTypeIdx: index("unit_org_type_idx").on(
      t.orgId,
      t.type,
      t.isActive
    ),
    pathIdx: index("unit_path_idx").on(t.path),
    headIdx: index("unit_head_idx").on(t.headId),
    parentIdx: index("unit_parent_idx").on(t.parentId),
    orgCodeIdx: unique("unit_org_code_idx").on(t.orgId, t.code),
  })
);

// ============================================================
// CLASSES (Classrooms/Sections)
// ============================================================

export const classes = academicSchema.table(
  "classes",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    unitId: uuid("unit_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    name: varchar("name", { length: 50 }).notNull(),
    nameAr: varchar("name_ar", { length: 50 }),

    level: varchar("level", { length: 20 }).notNull(),
    division: varchar("division", { length: 10 }),

    homeroomTeacherId: uuid("homeroom_teacher_id"),

    capacity: smallint("capacity").default(30),
    minStudents: smallint("min_students").default(5),
    currentStudents: smallint("current_students").default(0),

    roomId: uuid("room_id"),

    color: varchar("color", { length: 7 }).default("#6366f1"),
    coverImage: varchar("cover_image", { length: 500 }),
    icon: varchar("icon", { length: 50 }),

    scheduleTemplate: jsonb("schedule_template").$type<{
      sunday?: Array<{ slot: number; subjectId: string }>;
      monday?: Array<{ slot: number; subjectId: string }>;
      tuesday?: Array<{ slot: number; subjectId: string }>;
      wednesday?: Array<{ slot: number; subjectId: string }>;
      thursday?: Array<{ slot: number; subjectId: string }>;
      friday?: Array<{ slot: number; subjectId: string }>;
      saturday?: Array<{ slot: number; subjectId: string }>;
    }>(),

    isActive: boolean("is_active").default(true).notNull(),
    isDeleted: boolean("is_deleted").default(false).notNull(),

    deletedAt: timestamp("deleted_at"),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unitLevelIdx: index("class_unit_level_idx").on(
      t.unitId,
      t.level,
      t.isActive
    ),
    teacherIdx: index("class_teacher_idx").on(t.homeroomTeacherId),
    cycleIdx: index("class_cycle_idx").on(t.cycleId),
    orgActiveIdx: index("class_org_active_idx").on(
      t.orgId,
      t.isActive,
      t.isDeleted
    ),
    orgCodeIdx: unique("class_org_code_idx").on(t.orgId, t.name, t.cycleId),
  })
);

// ============================================================
// SUBJECTS
// ============================================================

export const subjects = academicSchema.table(
  "subjects",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    unitId: uuid("unit_id"),

    code: varchar("code", { length: 20 }).notNull(),
    name: varchar("name", { length: 100 }).notNull(),
    nameAr: varchar("name_ar", { length: 100 }),
    nameEn: varchar("name_en", { length: 100 }),

    description: text("description"),
    shortDescription: varchar("short_description", { length: 200 }),

    isCore: boolean("is_core").default(true).notNull(),
    isElective: boolean("is_elective").default(false).notNull(),
    creditHours: smallint("credit_hours").default(1),

    requiresLab: boolean("requires_lab").default(false).notNull(),
    requiresPractical: boolean("requires_practical").default(false).notNull(),
    maxStudentsPerClass: smallint("max_students_per_class"),

    defaultGradeGroups: jsonb("default_grade_groups")
      .default("[]")
      .$type<
        Array<{
          name: string;
          weight: number;
          dropLowest?: number;
          aggregation: "average" | "sum" | "best_of" | "weighted_average";
        }>
      >(),

    prerequisites: jsonb("prerequisites").$type<string[]>().default("[]"),
    learningOutcomes: jsonb("learning_outcomes").$type<string[]>().default("[]"),

    color: varchar("color", { length: 7 }).default("#6366f1"),
    icon: varchar("icon", { length: 50 }),
    coverImage: varchar("cover_image", { length: 500 }),

    sortOrder: smallint("sort_order").default(0),
    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    orgCodeIdx: unique("subj_org_code_idx").on(t.orgId, t.code),
    orgActiveIdx: index("subj_org_active_idx").on(t.orgId, t.isActive),
    unitIdx: index("subj_unit_idx").on(t.unitId),
    nameSearchIdx: index("subj_name_search_idx").using(
      "gin",
      sql`to_tsvector('arabic', COALESCE(${t.name}, '') || ' ' || COALESCE(${t.nameAr}, ''))`
    ),
  })
);

// ============================================================
// CLASS SUBJECTS (Assignments with Teachers)
// ============================================================

export const classSubjects = academicSchema.table(
  "class_subjects",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    classId: uuid("class_id").notNull(),
    subjectId: uuid("subject_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    primaryTeacherId: uuid("primary_teacher_id").notNull(),
    assistantTeacherId: uuid("assistant_teacher_id"),
    substituteTeacherId: uuid("substitute_teacher_id"),

    gradeGroups: jsonb("grade_groups")
      .$type<
        Array<{
          name: string;
          weight: number;
          dropLowest?: number;
          aggregation: "average" | "sum" | "best_of" | "weighted_average";
        }>
      >(),

    weeklyHours: smallint("weekly_hours").default(2),
    theoryHours: smallint("theory_hours").default(1),
    practicalHours: smallint("practical_hours").default(1),

    assessmentConfig: jsonb("assessment_config")
      .default("{}")
      .$type<{
        examWeight?: number;
        quizWeight?: number;
        homeworkWeight?: number;
        participationWeight?: number;
        projectWeight?: number;
      }>(),

    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("cs_unique_idx").on(t.classId, t.subjectId, t.cycleId),
    classIdx: index("cs_class_idx").on(t.classId),
    teacherIdx: index("cs_teacher_idx").on(t.primaryTeacherId),
    cycleIdx: index("cs_cycle_idx").on(t.cycleId),
    subjectIdx: index("cs_subject_idx").on(t.subjectId),
  })
);

// ============================================================
// ENROLLMENTS (Student-Class Registration)
// ============================================================

export const enrollments = academicSchema.table(
  "enrollments",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    studentId: uuid("student_id").notNull(),
    classId: uuid("class_id").notNull(),

    studentNumber: varchar("student_number", { length: 20 }).notNull(),

    previousSchool: varchar("previous_school", { length: 100 }),
    previousStudentNumber: varchar("previous_student_number", { length: 20 }),
    transferDate: date("transfer_date"),
    transferReason: varchar("transfer_reason", { length: 200 }),

    status: academicStatusEnum("status").default("active").notNull(),

    enrollmentDate: date("enrollment_date").notNull(),
    enrollmentType: varchar("enrollment_type", { length: 20 })
      .default("new")
      .notNull(),

    withdrawalDate: date("withdrawal_date"),
    withdrawalReason: varchar("withdrawal_reason", { length: 200 }),

    finalGrade: varchar("final_grade", { length: 5 }),
    finalScore: scoreType("final_score"),
    finalRank: integer("final_rank"),

    isPromoted: boolean("is_promoted"),
    promotionDate: date("promotion_date"),
    promotionDecision: varchar("promotion_decision", { length: 20 }),
    promotionNotes: text("promotion_notes"),

    meta: jsonb("meta")
      .default("{}")
      .$type<{
        transportType?: "bus" | "private" | "walk";
        transportRoute?: string;
        transportFee?: number;
        documents?: Record<string, string>;
        previousGrades?: Array<{ subject: string; grade: string; score: number }>;
      }>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    studentCycleIdx: unique("enroll_student_cycle_idx").on(
      t.studentId,
      t.cycleId
    ),
    classCycleIdx: index("enroll_class_cycle_idx").on(t.classId, t.cycleId),
    orgStatusIdx: index("enroll_org_status_idx")
      .on(t.orgId, t.status)
      .where(sql`${t.status} = 'active'`),
    studentNumberIdx: unique("enroll_student_number_idx").on(
      t.orgId,
      t.studentNumber
    ),
    transferIdx: index("enroll_transfer_idx")
      .on(t.studentId, t.transferDate)
      .where(sql`${t.transferDate} IS NOT NULL`),
  })
);

// ============================================================
// STUDENT GUARDIANS
// ============================================================

export const studentGuardians = academicSchema.table(
  "student_guardians",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    studentId: uuid("student_id").notNull(),
    guardianId: uuid("guardian_id").notNull(),

    relationship: varchar("relationship", { length: 30 })
      .default("parent")
      .notNull(),
    relationshipAr: varchar("relationship_ar", { length: 30 }),

    isPrimary: boolean("is_primary").default(false).notNull(),
    isEmergencyContact: boolean("is_emergency_contact").default(false).notNull(),
    isAuthorizedPickup: boolean("is_authorized_pickup").default(true).notNull(),

    canViewGrades: boolean("can_view_grades").default(true).notNull(),
    canViewAttendance: boolean("can_view_attendance").default(true).notNull(),
    canViewBehavior: boolean("can_view_behavior").default(true).notNull(),
    canViewFinance: boolean("can_view_finance").default(true).notNull(),
    canViewSchedule: boolean("can_view_schedule").default(true).notNull(),
    canReceiveNotifications: boolean("can_receive_notifications")
      .default(true)
      .notNull(),

    isPayer: boolean("is_payer").default(false).notNull(),
    paymentSharePercent: percentType("payment_share_percent").default(100),

    occupation: varchar("occupation", { length: 100 }),
    employer: varchar("employer", { length: 100 }),
    workPhone: varchar("work_phone", { length: 15 }),

    notes: text("notes"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("guardian_unique_idx").on(t.studentId, t.guardianId),
    studentIdx: index("guardian_student_idx").on(t.studentId),
    guardianIdx: index("guardian_guardian_idx").on(t.guardianId),
    primaryIdx: index("guardian_primary_idx")
      .on(t.studentId, t.isPrimary)
      .where(sql`${t.isPrimary} = true`),
    payerIdx: index("guardian_payer_idx")
      .on(t.studentId, t.isPayer)
      .where(sql`${t.isPayer} = true`),
  })
);

// ============================================================
// TIME SLOTS (Periods)
// ============================================================

export const timeSlots = academicSchema.table(
  "time_slots",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    slotNumber: smallint("slot_number").notNull(),
    name: varchar("name", { length: 50 }),
    nameAr: varchar("name_ar", { length: 50 }),

    startTime: time("start_time").notNull(),
    endTime: time("end_time").notNull(),
    durationMinutes: smallint("duration_minutes").notNull(),

    isBreak: boolean("is_break").default(false).notNull(),
    breakType: varchar("break_type", { length: 20 }),

    applicableDays: jsonb("applicable_days")
      .default("[1,2,3,4,5]")
      .$type<number[]>(),

    sortOrder: smallint("sort_order").default(0),
    isActive: boolean("is_active").default(true).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    cycleNumberIdx: unique("slot_cycle_number_idx").on(
      t.cycleId,
      t.slotNumber
    ),
    cycleTimeIdx: index("slot_cycle_time_idx").on(t.cycleId, t.startTime),
    activeIdx: index("slot_active_idx")
      .on(t.cycleId, t.isActive)
      .where(sql`${t.isActive} = true`),
  })
);

// ============================================================
// SCHEDULED LESSONS (Timetable)
// ============================================================

export const scheduledLessons = academicSchema.table(
  "scheduled_lessons",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    classId: uuid("class_id").notNull(),
    classSubjectId: uuid("class_subject_id").notNull(),
    subjectId: uuid("subject_id").notNull(),

    teacherId: uuid("teacher_id").notNull(),
    substituteTeacherId: uuid("substitute_teacher_id"),

    timeSlotId: uuid("time_slot_id").notNull(),
    dayOfWeek: smallint("day_of_week").notNull(),

    roomId: uuid("room_id"),
    roomName: varchar("room_name", { length: 50 }),

    status: varchar("status", { length: 20 })
      .default("scheduled")
      .notNull(),
    isSubstituted: boolean("is_substituted").default(false).notNull(),
    substitutionReason: varchar("substitution_reason", { length: 200 }),

    topic: varchar("topic", { length: 200 }),
    topicAr: varchar("topic_ar", { length: 200 }),
    objectives: text("objectives"),
    materials: jsonb("materials")
      .default("[]")
      .$type<Array<{ name: string; url: string; type: string }>>(),
    homework: text("homework"),
    homeworkDueDate: date("homework_due_date"),

    attendanceTaken: boolean("attendance_taken").default(false).notNull(),
    attendanceTakenAt: timestamp("attendance_taken_at"),
    attendanceTakenBy: uuid("attendance_taken_by"),
    attendanceMethod: varchar("attendance_method", { length: 20 }),

    notes: text("notes"),
    cancellationReason: text("cancellation_reason"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("lesson_unique_idx").on(
      t.classId,
      t.timeSlotId,
      t.dayOfWeek,
      t.cycleId
    ),
    classDayIdx: index("lesson_class_day_idx").on(t.classId, t.dayOfWeek),
    teacherDayIdx: index("lesson_teacher_day_idx").on(
      t.teacherId,
      t.dayOfWeek,
      t.timeSlotId
    ),
    cycleIdx: index("lesson_cycle_idx").on(t.cycleId),
    subjectIdx: index("lesson_subject_idx").on(t.subjectId),
    substituteIdx: index("lesson_substitute_idx")
      .on(t.substituteTeacherId)
      .where(sql`${t.substituteTeacherId} IS NOT NULL`),
    attendanceIdx: index("lesson_attendance_idx")
      .on(t.classId, t.attendanceTaken)
      .where(sql`${t.attendanceTaken} = false`),
  })
);

// ============================================================
// ASSESSMENTS (Exams, Quizzes, Homework)
// ============================================================

export const assessments = academicSchema.table(
  "assessments",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    classSubjectId: uuid("class_subject_id").notNull(),

    title: varchar("title", { length: 100 }).notNull(),
    titleAr: varchar("title_ar", { length: 100 }),
    description: text("description"),

    type: varchar("type", { length: 20 }).notNull(),

    maxScore: scoreType("max_score").notNull(),
    passingScore: scoreType("passing_score"),
    weight: percentType("weight").default(100).notNull(),

    gradeGroupName: varchar("grade_group_name", { length: 50 }).notNull(),

    assessmentDate: date("assessment_date"),
    dueDate: date("due_date"),
    durationMinutes: smallint("duration_minutes"),

    isPublished: boolean("is_published").default(false).notNull(),
    publishedAt: timestamp("published_at"),
    publishedBy: uuid("published_by"),

    isLocked: boolean("is_locked").default(false).notNull(),
    lockedAt: timestamp("locked_at"),
    lockedBy: uuid("locked_by"),

    gradingMethod: varchar("grading_method", { length: 20 }).default("manual"),
    rubricId: uuid("rubric_id"),

    instructions: text("instructions"),
    instructionsAr: text("instructions_ar"),
    attachments: jsonb("attachments")
      .default("[]")
      .$type<Array<{ name: string; url: string; type: string; size: number }>>(),

    createdBy: uuid("created_by").notNull(),
    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    csCycleIdx: index("assess_cs_cycle_idx").on(t.classSubjectId, t.cycleId),
    publishedIdx: index("assess_published_idx")
      .on(t.cycleId, t.isPublished, t.isLocked)
      .where(sql`${t.isPublished} = true`),
    dueIdx: index("assess_due_idx").on(t.cycleId, t.dueDate),
    typeIdx: index("assess_type_idx").on(t.cycleId, t.type),
    dateIdx: index("assess_date_idx").on(t.cycleId, t.assessmentDate),
    lockedIdx: index("assess_locked_idx")
      .on(t.cycleId, t.isLocked)
      .where(sql`${t.isLocked} = false`),
  })
);

// ============================================================
// GRADES (Performance Critical)
// ============================================================

export const grades = academicSchema.table(
  "grades",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    enrollmentId: uuid("enrollment_id").notNull(),
    assessmentId: uuid("assessment_id").notNull(),

    score: scoreType("score").notNull(),

    gradeLetter: varchar("grade_letter", { length: 5 }),
    gradePoints: smallint("grade_points"),

    isAbsent: boolean("is_absent").default(false).notNull(),
    isExcused: boolean("is_excused").default(false).notNull(),
    isIncomplete: boolean("is_incomplete").default(false).notNull(),
    isLate: boolean("is_late").default(false).notNull(),

    comment: varchar("comment", { length: 500 }),
    teacherFeedback: varchar("teacher_feedback", { length: 500 }),
    studentReflection: varchar("student_reflection", { length: 500 }),

    version: smallint("version").default(1).notNull(),
    gradedBy: uuid("graded_by").notNull(),
    gradedAt: timestamp("graded_at").defaultNow().notNull(),
    modifiedBy: uuid("modified_by"),
    modifiedAt: timestamp("modified_at"),
    modifyReason: varchar("modify_reason", { length: 200 }),

    batchId: varchar("batch_id", { length: 50 }),
    importSource: varchar("import_source", { length: 20 }),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("grade_enroll_assess_idx").on(t.enrollmentId, t.assessmentId),
    enrollCycleIdx: index("grade_enroll_cycle_idx").on(t.enrollmentId, t.cycleId),
    assessIdx: index("grade_assess_idx").on(t.assessmentId),
    assessEnrollIdx: index("grade_assess_enroll_idx").on(
      t.assessmentId,
      t.enrollmentId,
      t.score
    ),
    cycleScoreIdx: index("grade_cycle_score_idx")
      .on(t.cycleId, t.score)
      .where(sql`${t.isAbsent} = false`),
    batchIdx: index("grade_batch_idx")
      .on(t.batchId)
      .where(sql`${t.batchId} IS NOT NULL`),
    absentIdx: index("grade_absent_idx")
      .on(t.cycleId, t.isAbsent)
      .where(sql`${t.isAbsent} = true`),
  })
);

export const gradeHistory = academicSchema.table(
  "grade_history",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    gradeId: uuid("grade_id").notNull(),

    score: scoreType("score"),
    gradeLetter: varchar("grade_letter", { length: 5 }),
    isAbsent: boolean("is_absent"),
    isExcused: boolean("is_excused"),
    comment: varchar("comment", { length: 500 }),

    changedBy: uuid("changed_by"),
    changedAt: timestamp("changed_at").defaultNow().notNull(),
    changeReason: varchar("change_reason", { length: 200 }),
    changeType: varchar("change_type", { length: 20 }).notNull(),

    snapshot: jsonb("snapshot").default("{}").$type<Record<string, unknown>>(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
  },
  (t) => ({
    gradeIdx: index("gh_grade_idx").on(t.gradeId, t.changedAt),
    changedByIdx: index("gh_changed_by_idx").on(t.changedBy),
  })
);

// ============================================================
// ATTENDANCE (Daily Aggregated)
// ============================================================

export const attendanceDaily = academicSchema.table(
  "attendance_daily",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    enrollmentId: uuid("enrollment_id").notNull(),
    date: date("date").notNull(),

    status: attendanceEnum("status").notNull(),

    lessons: jsonb("lessons")
      .default("[]")
      .$type<
        Array<{
          lessonId: string;
          status: "present" | "absent" | "late" | "excused";
          minutesLate?: number;
          markedBy?: string;
          markedAt?: string;
        }>
      >(),

    totalLessons: smallint("total_lessons").default(0),
    presentLessons: smallint("present_lessons").default(0),
    absentLessons: smallint("absent_lessons").default(0),
    lateLessons: smallint("late_lessons").default(0),
    excusedLessons: smallint("excused_lessons").default(0),
    lateMinutes: smallint("late_minutes").default(0),

    excuseType: varchar("excuse_type", { length: 20 }),
    excuseDocument: varchar("excuse_document", { length: 500 }),
    excuseNotes: varchar("excuse_notes", { length: 200 }),

    isVerified: boolean("is_verified").default(false).notNull(),
    verifiedBy: uuid("verified_by"),
    verifiedAt: timestamp("verified_at"),

    markedBy: uuid("marked_by").notNull(),
    markedAt: timestamp("marked_at").defaultNow().notNull(),
    markMethod: varchar("mark_method", { length: 20 }).default("manual"),

    deviceId: varchar("device_id", { length: 100 }),
    location: jsonb("location").$type<{
      lat?: number;
      lng?: number;
      accuracy?: number;
    }>(),

    notes: text("notes"),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("attend_enroll_date_idx").on(t.enrollmentId, t.date),
    cycleDateIdx: index("attend_cycle_date_idx").on(t.cycleId, t.date),
    enrollCycleIdx: index("attend_enroll_cycle_idx").on(t.enrollmentId, t.cycleId),
    statusIdx: index("attend_status_idx")
      .on(t.cycleId, t.status)
      .where(sql`${t.status} != 'present'`),
    absentAlertIdx: index("attend_absent_alert_idx")
      .on(t.cycleId, t.date, t.status)
      .where(sql`${t.status} = 'absent' AND ${t.isVerified} = false`),
    verifyIdx: index("attend_verify_idx")
      .on(t.cycleId, t.isVerified)
      .where(sql`${t.isVerified} = false AND ${t.excuseType} IS NOT NULL`),
  })
);

export const attendanceSummary = academicSchema.table(
  "attendance_summary",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    enrollmentId: uuid("enrollment_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    periodType: varchar("period_type", { length: 20 }).notNull(),
    periodStart: date("period_start").notNull(),
    periodEnd: date("period_end").notNull(),

    totalDays: smallint("total_days").default(0),
    presentDays: smallint("present_days").default(0),
    absentDays: smallint("absent_days").default(0),
    lateDays: smallint("late_days").default(0),
    excusedDays: smallint("excused_days").default(0),
    sickDays: smallint("sick_days").default(0),

    attendanceRate: percentType("attendance_rate"),

    hasAlert: boolean("has_alert").default(false).notNull(),
    alertType: varchar("alert_type", { length: 30 }),
    alertSentAt: timestamp("alert_sent_at"),

    lastCalculatedAt: timestamp("last_calculated_at").defaultNow(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    unique: unique("attend_sum_unique_idx").on(
      t.enrollmentId,
      t.periodType,
      t.periodStart
    ),
    enrollIdx: index("attend_sum_enroll_idx").on(t.enrollmentId),
    cycleAlertIdx: index("attend_sum_alert_idx")
      .on(t.cycleId, t.hasAlert)
      .where(sql`${t.hasAlert} = true`),
  })
);

// ============================================================
// BEHAVIOR RECORDS
// ============================================================

export const behaviorRecords = academicSchema.table(
  "behavior_records",
  {
    id: uuid("id").defaultRandom().primaryKey(),
    orgId: uuid("org_id").notNull(),
    cycleId: uuid("cycle_id").notNull(),

    enrollmentId: uuid("enrollment_id").notNull(),

    type: varchar("type", { length: 20 }).notNull(),
    severity: varchar("severity", { length: 10 })
      .default("minor")
      .notNull(),
    category: varchar("category", { length: 50 }),

    title: varchar("title", { length: 100 }).notNull(),
    titleAr: varchar("title_ar", { length: 100 }),
    description: text("description").notNull(),

    incidentDate: date("incident_date").notNull(),
    incidentTime: time("incident_time"),
    location: varchar("location", { length: 100 }),

    points: smallint("points").default(0),

    actionTaken: varchar("action_taken", { length: 100 }),
    actionDetails: text("action_details"),
    actionDate: date("action_date"),
    actionCompleted: boolean("action_completed").default(false),
    actionCompletedAt: timestamp("action_completed_at"),

    reportedBy: uuid("reported_by").notNull(),
    reviewedBy: uuid("reviewed_by"),
    reviewedAt: timestamp("reviewed_at"),
    witnesses: jsonb("witnesses").$type<string[]>().default("[]"),

    parentNotified: boolean("parent_notified").default(false).notNull(),
    parentNotifiedAt: timestamp("parent_notified_at"),
    parentResponse: text("parent_response"),
    parentAcknowledged: boolean("parent_acknowledged").default(false),

    evidenceUrls: jsonb("evidence_urls").$type<string[]>().default("[]"),

    status: varchar("status", { length: 20 }).default("active").notNull(),
    resolutionNotes: text("resolution_notes"),
    resolvedAt: timestamp("resolved_at"),
    resolvedBy: uuid("resolved_by"),

    isPrivate: boolean("is_private").default(false).notNull(),

    createdAt: timestamp("created_at").defaultNow().notNull(),
    updatedAt: timestamp("updated_at").defaultNow().notNull(),
  },
  (t) => ({
    enrollIdx: index("behavior_enroll_idx").on(t.enrollmentId),
    cycleTypeIdx: index("behavior_cycle_type_idx").on(t.cycleId, t.type),
    dateIdx: index("behavior_date_idx").on(t.incidentDate),
    severityIdx: index("behavior_severity_idx")
      .on(t.cycleId, t.severity)
      .where(sql`${t.severity} IN ('major', 'critical')`),
    statusIdx: index("behavior_status_idx")
      .on(t.cycleId, t.status)
      .where(sql`${t.status} = 'active'`),
    reporterIdx: index("behavior_reporter_idx").on(t.reportedBy),
    parentNotifyIdx: index("behavior_parent_notify_idx")
      .on(t.enrollmentId, t.parentNotified)
      .where(sql`${t.parentNotified} = false AND ${t.severity} IN ('major', 'critical')`),
  })
);
