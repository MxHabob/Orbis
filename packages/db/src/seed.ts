// packages/db/src/seed.ts
import { drizzle } from "drizzle-orm/node-postgres";
import { Pool } from "pg";
import { eq, sql, getTableName } from "drizzle-orm";
import { randomUUID } from "crypto";

import * as schema from "./schema";

// ────────────────────────────────────────────────
// Configuration & DB Setup
// ────────────────────────────────────────────────

const isProduction = process.env.NODE_ENV === "production";

if (isProduction) {
  console.error("❌ Seeding is prohibited in production environment");
  process.exit(1);
}

const pool = new Pool({
  connectionString: process.env.DATABASE_PRIMARY_URL,
  max: 5,
  idleTimeoutMillis: 10000,
  connectionTimeoutMillis: 5000,
});

const db = drizzle(pool, {
  schema,
  casing: "snake_case",
  logger: true,
});

// ────────────────────────────────────────────────
// Utility Helpers
// ────────────────────────────────────────────────

const now = () => new Date();

async function resetDatabase() {
  console.log("→ Resetting database tables...");

  // Explicit list of tables only (no relations, no enums)
  const tables = [
    schema.gradeEntries,
    schema.assessmentItems,
    schema.students,
    schema.classes,
    schema.subjects,
    schema.academicTerms,
    schema.academicYears,
    schema.notifications,
    schema.auditLogs,
    schema.verifications,
    schema.sessions,
    schema.accounts,
    schema.users, // users last → many foreign keys reference it
  ];

  for (const table of tables) {
    if (!table) continue;
    const tableName = getTableName(table);

    try {
      // Correct PostgreSQL syntax: TRUNCATE table_name RESTART IDENTITY CASCADE;
      await db.execute(
        sql.raw(`TRUNCATE TABLE "${tableName}" RESTART IDENTITY CASCADE;`)
      );
      console.log(`  Truncated: ${tableName}`);
    } catch (err) {
      console.warn(`  Warning: Failed to truncate ${tableName}:`, err);
    }
  }

  console.log("✓ Database reset completed");
}

// ────────────────────────────────────────────────
// Seed Functions
// ────────────────────────────────────────────────

async function seedAcademicYears(): Promise<number> {
  console.log("→ Seeding academic years...");

  const years = [
    { yearName: "2024-2025", isCurrent: false },
    { yearName: "2025-2026", isCurrent: true },
  ];

  await db.insert(schema.academicYears).values(years).onConflictDoNothing();

  const [current] = await db
    .select({ id: schema.academicYears.id })
    .from(schema.academicYears)
    .where(eq(schema.academicYears.isCurrent, true))
    .limit(1);

  if (!current) throw new Error("No current academic year found after seeding");

  return current.id;
}

async function seedAcademicTerms(yearId: number): Promise<number> {
  console.log("→ Seeding academic terms...");

  const terms = [
    {
      name: "الفصل الدراسي الأول",
      academicYearId: yearId,
      startDate: "2025-09-01",
      endDate: "2026-01-15",
      isCurrent: true,
    },
    {
      name: "الفصل الدراسي الثاني",
      academicYearId: yearId,
      startDate: "2026-02-01",
      endDate: "2026-06-20",
      isCurrent: false,
    },
  ];

  await db.insert(schema.academicTerms).values(terms).onConflictDoNothing();

  const [current] = await db
    .select({ id: schema.academicTerms.id })
    .from(schema.academicTerms)
    .where(eq(schema.academicTerms.isCurrent, true))
    .limit(1);

  if (!current) throw new Error("No current term found after seeding");

  return current.id;
}

async function seedSubjects() {
  console.log("→ Seeding subjects...");

  const subjects = [
    { name: "اللغة العربية", code: "ARA", color_code: "#FFCDD2" },
    { name: "اللغة الإنجليزية", code: "ENG", color_code: "#BBDEFB" },
    { name: "الرياضيات", code: "MAT", color_code: "#C8E6C9" },
    { name: "العلوم", code: "SCI", color_code: "#FFF9C4" },
    { name: "الدراسات الاجتماعية", code: "SST", color_code: "#E1BEE7" },
    { name: "التربية الإسلامية", code: "ISL", color_code: "#D1C4E9" },
  ];

  await db.insert(schema.subjects).values(subjects).onConflictDoNothing();
  console.log(`  ✓ ${subjects.length} subjects seeded`);
}

async function seedClasses(yearId: number) {
  console.log("→ Seeding classes...");

  const classesData = [
    { name: "الصف الأول الابتدائي", division: "أ", academicYearId: yearId },
    { name: "الصف الأول الابتدائي", division: "ب", academicYearId: yearId },
    { name: "الصف الثاني الابتدائي", division: "أ", academicYearId: yearId },
    { name: "الصف الثالث الاعدادي", division: "أ", academicYearId: yearId },
  ];

  await db.insert(schema.classes).values(classesData).onConflictDoNothing();

  const inserted = await db.select().from(schema.classes);
  console.log(`  ✓ ${inserted.length} classes created`);

  return inserted;
}

async function seedUsersAndStudents() {
  console.log("→ Seeding users and students...");

  const admin = {
    id: randomUUID(),
    username: "admin",
    name: "مدير النظام",
    email: "admin@school.com",
    phone: "+966501234567",
    role: "admin" as const,
    sex: "unspecified" as const,
    emailVerified: true,
    createdAt: now(),
    updatedAt: now(),
  };

  const teacher = {
    id: randomUUID(),
    username: "teacher1",
    name: "أ. محمد أحمد",
    email: "teacher1@school.com",
    phone: "+966551234567",
    role: "teacher" as const,
    sex: "male" as const,
    emailVerified: true,
    createdAt: now(),
    updatedAt: now(),
  };

  const studentsData = [
    {
      user: {
        id: randomUUID(),
        username: "stu1001",
        name: "سارة عبدالله",
        email: "sara1001@school.com",
        phone: "+966501001001",
        role: "student" as const,
        sex: "female" as const,
        createdAt: now(),
        updatedAt: now(),
      },
      studentNumber: "STU-1001",
    },
    {
      user: {
        id: randomUUID(),
        username: "stu1002",
        name: "عمر خالد",
        email: "omar1002@school.com",
        phone: "+966501002002",
        role: "student" as const,
        sex: "male" as const,
        createdAt: now(),
        updatedAt: now(),
      },
      studentNumber: "STU-1002",
    },
  ];

  // Insert users
  await db
    .insert(schema.users)
    .values([admin, teacher, ...studentsData.map((s) => s.user)])
    .onConflictDoNothing();

  // Insert students
  await db
    .insert(schema.students)
    .values(
      studentsData.map((s) => ({
        userId: s.user.id,
        studentNumber: s.studentNumber,
        status: "active" as const,
      })),
    )
    .onConflictDoNothing();

  console.log("  ✓ Admin, 1 teacher, and 2 students created");

  return { adminId: admin.id, teacherId: teacher.id, studentsData };
}

async function assignStudentsToClasses(classes: any[], studentsData: any[]) {
  if (classes.length < 2 || studentsData.length < 2) return;

  await db
    .update(schema.students)
    .set({ currentClassId: classes[0].id })
    .where(eq(schema.students.studentNumber, studentsData[0].studentNumber));

  await db
    .update(schema.students)
    .set({ currentClassId: classes[1].id })
    .where(eq(schema.students.studentNumber, studentsData[1].studentNumber));

  console.log("  ✓ Students assigned to classes");
}

async function seedAssessmentsAndGrades(
  termId: number,
  classes: any[],
  subjects: any[],
  teacherId: string,
  studentIdsByClass: Map<number, string[]>,
) {
  console.log("→ Seeding assessment items and grades...");

  const assessmentItemsData: (typeof schema.assessmentItems.$inferInsert)[] = [];
  const months = [9, 10];

  for (const cls of classes) {
    for (const subj of subjects.slice(0, 4)) {
      for (const month of months) {
        assessmentItemsData.push({
          title: month === 9 ? "Monthly Quiz – September" : "Monthly Exam – October",
          type: (month === 9 ? "quiz" : "exam") as any,
          maxScore: month === 9 ? "20" : "30",
          weight: "2.0",
          subjectId: subj.id,
          classId: cls.id,
          month,
          termId,
        });
      }
    }
  }

  await db.insert(schema.assessmentItems).values(assessmentItemsData).onConflictDoNothing();

  const items = await db
    .select()
    .from(schema.assessmentItems)
    .where(eq(schema.assessmentItems.termId, termId));

  console.log(`  ✓ ${items.length} assessment items created`);

  // Grades
  const grades: (typeof schema.gradeEntries.$inferInsert)[] = [];

  for (const item of items) {
    if (item.classId === null) continue;
    const classStudentIds = studentIdsByClass.get(item.classId) || [];

    for (const studentId of classStudentIds.slice(0, 2)) {
      const score = Number((Math.random() * 10 + 10).toFixed(1));

      grades.push({
        studentId,
        assessmentItemId: item.id,
        score: score.toString(),
        isAbsent: false,
        comments: score > 15 ? "Excellent" : score > 10 ? "Good" : "Needs improvement",
        gradedById: teacherId,
        updatedAt: now(),
      });
    }

    // Random absence
    if (Math.random() > 0.7 && classStudentIds.length > 0) {
      grades.push({
        studentId: classStudentIds[0],
        assessmentItemId: item.id,
        score: null,
        isAbsent: true,
        comments: "Absent without excuse",
        gradedById: teacherId,
        updatedAt: now(),
      });
    }
  }

  if (grades.length > 0) {
    await db.insert(schema.gradeEntries).values(grades).onConflictDoNothing();
    console.log(`  ✓ ${grades.length} grade / absence records inserted`);
  }
}

async function seedNotifications(
  adminId: string,
  teacherId: string,
  studentIds: string[],
  classes: any[],
  firstAssessmentId?: number,
) {
  console.log("→ Seeding sample notifications...");

  const notifications = [
    {
      senderId: teacherId,
      recipientId: studentIds[0]!,
      type: "grade_published" as const,
      title: "Grades Published - Monthly Quiz",
      message: "You can now view your September grades in Mathematics.",
      isRead: false,
      isHighPriority: true,
      relatedEntityType: "assessment_item",
      relatedEntityId: firstAssessmentId?.toString(),
      createdAt: now(),
    },
    {
      senderId: adminId,
      recipientId: studentIds[0]!, // Schema requires recipientId to be not null
      classId: classes[0]?.id,
      type: "announcement" as const,
      title: "Parent-Teacher Meeting",
      message: "A parent meeting will be held next Sunday at 5:00 PM.",
      isRead: false,
      isHighPriority: true,
      createdAt: now(),
    },
    {
      senderId: teacherId,
      recipientId: studentIds[1]!,
      type: "attendance_alert" as const,
      title: "Absence Alert",
      message: "You were marked absent in the Arabic Language quiz without excuse.",
      isRead: false,
      isHighPriority: true,
      createdAt: now(),
    },
  ];

  await db.insert(schema.notifications).values(notifications).onConflictDoNothing();
  console.log("  ✓ Sample notifications created");
}

// ────────────────────────────────────────────────
// Main Execution
// ────────────────────────────────────────────────

async function main() {
  console.log("🌱 Starting database seeding...");

  try {
    await resetDatabase();

    const yearId = await seedAcademicYears();
    const termId = await seedAcademicTerms(yearId);
    await seedSubjects();

    const classes = await seedClasses(yearId);
    const { adminId, teacherId, studentsData } = await seedUsersAndStudents();

    await assignStudentsToClasses(classes, studentsData);

    // Prepare student IDs grouped by class
    const studentIdsByClass = new Map<number, string[]>();

    for (const cls of classes) {
      const studentsInClass = await db
        .select({ userId: schema.students.userId })
        .from(schema.students)
        .where(eq(schema.students.currentClassId, cls.id));

      studentIdsByClass.set(
        cls.id,
        studentsInClass.map((s) => s.userId),
      );
    }

    const subjects = await db.select().from(schema.subjects);

    await seedAssessmentsAndGrades(termId, classes, subjects, teacherId, studentIdsByClass);

    const firstItem = await db
      .select({ id: schema.assessmentItems.id })
      .from(schema.assessmentItems)
      .limit(1);

    await seedNotifications(
      adminId,
      teacherId,
      studentsData.map((s) => s.user.id),
      classes,
      firstItem[0]?.id,
    );

    console.log("\n✨ Seeding completed successfully!");
  } catch (error) {
    console.error("❌ Seeding failed:", error);
    process.exit(1);
  } finally {
    await pool.end();
    console.log("→ Database connection pool closed");
  }
}

main().catch((err) => {
  console.error("Unexpected error in main:", err);
  process.exit(1);
});