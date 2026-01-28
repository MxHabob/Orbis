import { eq, sql, getTableName } from "drizzle-orm";
import { randomUUID } from "crypto";
import * as schema from "./schema";
import { notificationTypeEnum } from "./schema";
import { db } from "./client";

// ────────────────────────────────────────────────
// Config & DB Setup
// ────────────────────────────────────────────────
const isProduction = process.env.NODE_ENV === "production";

if (isProduction) {
  console.error("❌ Seeding is prohibited in production environment");
  process.exit(1);
}


const now = () => new Date();

async function resetDatabase() {
  console.log("→ Resetting database tables...");

  const tables = [
    schema.gradeEntries,
    schema.assessmentItems,
    schema.students,
    schema.enrollments,
    schema.classes,
    schema.subjects,
    schema.academicTerms,
    schema.academicYears,
    schema.notifications,
    schema.auditLogs,
    schema.verifications,
    schema.sessions,
    schema.accounts,
    schema.users, // users last (most foreign keys point to it)
  ];

  for (const table of tables) {
    if (!table) continue;
    const tableName = getTableName(table);
    try {
      await db.execute(
        sql.raw(`TRUNCATE TABLE "${tableName}" RESTART IDENTITY CASCADE;`)
      );
      console.log(`   Truncated: ${tableName}`);
    } catch (err) {
      console.warn(`   Warning: Failed to truncate ${tableName}:`, err);
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
    { yearName: "2025", isCurrent: false },
    { yearName: "2026", isCurrent: true },
  ];

  await db.insert(schema.academicYears).values(years).onConflictDoNothing();

  const [current] = await db
    .select({ id: schema.academicYears.id })
    .from(schema.academicYears)
    .where(eq(schema.academicYears.isCurrent, true))
    .limit(1);

  if (!current) throw new Error("No current academic year found");
  return current.id;
}

async function seedAcademicTerms(yearId: number): Promise<number> {
  console.log("→ Seeding academic terms...");
  const terms = [
    {
      name: "First Semester",
      academicYearId: yearId,
      startDate: "2025-09-01",
      endDate: "2026-01-15",
      isCurrent: true,
    },
    {
      name: "Second Semester",
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

  if (!current) throw new Error("No current term found");
  return current.id;
}

async function seedSubjects() {
  console.log("→ Seeding subjects...");
  const subjects = [
    { name: "Arabic Language",      code: "ARA", color: "#ffcdd2" },
    { name: "English Language",     code: "ENG", color: "#bbdefb" },
    { name: "Mathematics",          code: "MAT", color: "#c8e6c9" },
    { name: "Science",              code: "SCI", color: "#fff9c4" },
    { name: "Social Studies",       code: "SST", color: "#e1bee7" },
    { name: "Islamic Education",    code: "ISL", color: "#d1c4e9" },
    { name: "Holy Quran",           code: "HQ", color: "#d1c4e9" },
    { name: "Physical Education",   code: "PE", color: "#c8e6c9" },
  ];

  await db.insert(schema.subjects).values(subjects).onConflictDoNothing();
  console.log(`   ✓ ${subjects.length} subjects seeded`);

  return await db.select().from(schema.subjects);
}

async function seedClasses(yearId: number) {
  console.log("→ Seeding classes with hierarchy (parent grade → divisions A/B)...");

  const gradeNames = [
    "First grade",
    "Second grade",
    "Third grade",
    "Fourth grade",
    "Fifth grade",
    "Sixth grade",
    "Seventh grade",
    "Eighth grade",
    "Ninth grade",
    "Tenth grade",
    "Eleventh grade",
    "Twelfth grade",
  ];

  const divisions = ["A", "B"];
  const colors = [
    "#e3f2fd", "#e8f5e9", "#f3e5f5", "#ffecb3",
    "#e1f5fe", "#f1f8e9", "#ede7f6", "#fff3e0",
    "#e0f7fa", "#dcedc8", "#d1c4e9", "#ffecb3",
  ];

  const classesData: typeof schema.classes.$inferInsert[] = [];
  let colorIndex = 0;

  for (const gradeName of gradeNames) {
    const color = colors[colorIndex % colors.length];
    colorIndex++;

    // Parent (grade level)
    classesData.push({
      name: gradeName,
      type: 'classroom' as const,
      division: null,
      parentClassId: null,
      academicYearId: yearId,
      color,
    });

    // Children (divisions A & B)
    for (const div of divisions) {
      classesData.push({
        name: `${gradeName} (${div})`,
        type: 'classroom' as const,
        division: div,
        parentClassId: null,           // will be updated later
        academicYearId: yearId,
        color,                    // same color as parent
      });
    }
  }

  // Step 2: Insert all classes at once
  await db.insert(schema.classes)
    .values(classesData)
    .onConflictDoNothing();

  // Step 3: Fetch inserted classes in insertion order
  const insertedClasses = await db
    .select()
    .from(schema.classes)
    .where(eq(schema.classes.academicYearId, yearId))
    .orderBy(schema.classes.id);

  console.log(`   Inserted ${insertedClasses.length} classes`);

  // Step 4: Link children to their parents (safe iteration)
  const updatePromises: Promise<any>[] = [];

  for (let i = 0; i < insertedClasses.length; i += 3) {
    const parent = insertedClasses[i];
    if (!parent) {
      console.warn(`   Skipping invalid parent at index ${i}`);
      continue;
    }

    const childA = insertedClasses[i + 1];
    const childB = insertedClasses[i + 2];

    if (childA) {
      updatePromises.push(
        db.update(schema.classes)
          .set({ parentClassId: parent.id })
          .where(eq(schema.classes.id, childA.id))
      );
    }

    if (childB) {
      updatePromises.push(
        db.update(schema.classes)
          .set({ parentClassId: parent.id })
          .where(eq(schema.classes.id, childB.id))
      );
    }
  }

  if (updatePromises.length > 0) {
    await Promise.all(updatePromises);
    console.log(`   ✓ Linked ${updatePromises.length} division classes to parents`);
  } else {
    console.log("   No linking needed (empty result?)");
  }

  // Final fetch for return (with hierarchy info)
  const finalClasses = await db
    .select()
    .from(schema.classes)
    .where(eq(schema.classes.academicYearId, yearId))
    .orderBy(schema.classes.id);

  const parentCount = finalClasses.filter(c => c.parentClassId === null).length;
  const childCount = finalClasses.length - parentCount;

  console.log(
    `   ✓ Created ${parentCount} parent classes + ${childCount} divisions ` +
    `(total: ${finalClasses.length})`
  );

  return finalClasses;
}

async function seedUsersAndStudents() {
  console.log("→ Seeding users and students...");

  const admin = {
    id: randomUUID(),
    username: "admin",
    name: "System Administrator",
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
    name: "Mr. Ahmed Mohammed",
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
        name: "Sarah Abdullah",
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
        name: "Omar Khaled",
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

  // Insert student profiles
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

  console.log("   ✓ Admin, 1 teacher, and 2 students created");

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

  console.log("   ✓ Students assigned to classes");
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
    for (const subj of subjects.slice(0, 4)) { // first 4 subjects
      for (const month of months) {
        assessmentItemsData.push({
          title: month === 9 ? "Monthly Quiz – September" : "Midterm Exam – October",
          type: month === 9 ? "quiz" : "exam",
          maxScore: month === 9 ? "20" : "30",
          weight: "2.0",
          subjectId: subj.id,
          classId: cls.id,
          month,
          termId,
          isPublished: true,
        });
      }
    }
  }

  await db.insert(schema.assessmentItems).values(assessmentItemsData).onConflictDoNothing();

  const items = await db
    .select()
    .from(schema.assessmentItems)
    .where(eq(schema.assessmentItems.termId, termId));

  console.log(`   ✓ ${items.length} assessment items created`);

  // ── Grades ─────────────────────────────────────
  const grades: (typeof schema.gradeEntries.$inferInsert)[] = [];

  for (const item of items) {
    if (!item.classId) continue;
    const classStudentIds = studentIdsByClass.get(item.classId) || [];

    for (const studentId of classStudentIds.slice(0, 2)) {
      const score = Number((Math.random() * 10 + 10).toFixed(1)); // 10–20 range
      grades.push({
        studentId,
        assessmentItemId: item.id,
        score: score.toString(),
        isAbsent: false,
        comments: score >= 16 ? "Excellent" : score >= 12 ? "Good" : "Needs improvement",
        gradedById: teacherId,
        updatedAt: now(),
      });
    }

    // Random absence example
    if (Math.random() > 0.7 && classStudentIds.length > 0) {
      grades.push({
        studentId: classStudentIds[0],
        assessmentItemId: item.id,
        score: null,
        isAbsent: true,
        comments: "Absent without valid excuse",
        gradedById: teacherId,
        updatedAt: now(),
      });
    }
  }

  if (grades.length > 0) {
    await db.insert(schema.gradeEntries).values(grades).onConflictDoNothing();
    console.log(`   ✓ ${grades.length} grade/absence records inserted`);
  }

  return items;
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
      type: notificationTypeEnum.enumValues[0],
      title: "Grades Published - Monthly Quiz",
      message: "Your September quiz grades in Mathematics are now available.",
      isRead: false,
      isHighPriority: true,
      relatedEntityType: "assessment_item",
      relatedEntityId: firstAssessmentId?.toString(),
      createdAt: now(),
    },
    {
      senderId: adminId,
      recipientId: studentIds[0]!,
      classId: classes[0]?.id,
      type: notificationTypeEnum.enumValues[1],
      title: "Parent-Teacher Meeting",
      message: "A parent-teacher meeting is scheduled for next Sunday at 5:00 PM.",
      isRead: false,
      isHighPriority: true,
      createdAt: now(),
    },
    {
      senderId: teacherId,
      recipientId: studentIds[1]!,
      type: notificationTypeEnum.enumValues[2],
      title: "Absence Notification",
      message: "You were marked absent during the Arabic Language quiz.",
      isRead: false,
      isHighPriority: true,
      createdAt: now(),
    },
  ];

  await db.insert(schema.notifications).values(notifications).onConflictDoNothing();
  console.log("   ✓ Sample notifications created");
}

// ────────────────────────────────────────────────
// Main
// ────────────────────────────────────────────────
async function main() {
  console.log("🌱 Starting database seeding...");

  try {
    await resetDatabase();

    const yearId     = await seedAcademicYears();
    const termId     = await seedAcademicTerms(yearId);
    const subjects   = await seedSubjects();
    const classes    = await seedClasses(yearId);

    const { adminId, teacherId, studentsData } =
      await seedUsersAndStudents();

    await assignStudentsToClasses(classes, studentsData);

    // Group students by class for easier grade seeding
    const studentIdsByClass = new Map<number, string[]>();
    for (const cls of classes) {
      const res = await db
        .select({ userId: schema.students.userId })
        .from(schema.students)
        .where(eq(schema.students.currentClassId, cls.id));

      studentIdsByClass.set(cls.id, res.map(r => r.userId));
    }

    const items = await seedAssessmentsAndGrades(
      termId,
      classes,
      subjects,
      teacherId,
      studentIdsByClass,
    );

    await seedNotifications(
      adminId,
      teacherId,
      studentsData.map(s => s.user.id),
      classes,
      items[0]?.id,
    );

    console.log("\n✨ Seeding completed successfully! ✓");
  } catch (error) {
    console.error("❌ Seeding failed:", error);
    process.exit(1);
  }
}

main().catch((err) => {
  console.error("Unexpected error:", err);
  process.exit(1);
});