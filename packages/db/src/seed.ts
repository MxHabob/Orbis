import { eq, sql, getTableName } from "drizzle-orm";
import { randomUUID } from "crypto";
import * as schema from "./schema";
import { db } from "./client";

// ────────────────────────────────────────────────
// Config
// ────────────────────────────────────────────────
const isProduction = process.env.NODE_ENV === "production";

if (isProduction) {
  console.error("❌ Seeding is prohibited in production environment");
  process.exit(1);
}

const now = () => new Date();

// ────────────────────────────────────────────────
// Helper Functions
// ────────────────────────────────────────────────
function randomScore(min: number, max: number): string {
  return (Math.random() * (max - min) + min).toFixed(1);
}

function randomFromArray<T>(arr: T[]): T {
  if (arr.length === 0) {
    throw new Error("randomFromArray called with empty array");
  }
  return arr[Math.floor(Math.random() * arr.length)]!;
}

// ────────────────────────────────────────────────
// Reset Database
// ────────────────────────────────────────────────
async function resetDatabase() {
  console.log("→ Resetting database...");

  const tablesInOrder = [
    schema.attendanceRecords,
    schema.scheduledLessons,
    schema.timeSlots,
    schema.gradeEntries,
    schema.assessmentItems,
    schema.notifications,
    schema.enrollments,
    schema.students,
    schema.classes,
    schema.subjects,
    schema.academicTerms,
    schema.academicYears,
    schema.auditLogs,
    schema.verifications,
    schema.sessions,
    schema.accounts,
    schema.users,
  ];

  for (const table of tablesInOrder) {
    if (!table) continue;
    const tableName = getTableName(table);
    try {
      await db.execute(sql.raw(`TRUNCATE TABLE "${tableName}" RESTART IDENTITY CASCADE;`));
      console.log(`   Truncated: ${tableName}`);
    } catch (err) {
      console.warn(`   Warning: Could not truncate ${tableName}`, err);
    }
  }
  console.log("✓ Reset completed\n");
}

// ────────────────────────────────────────────────
// Seed Academic Years & Terms
// ────────────────────────────────────────────────
async function seedAcademicStructure(): Promise<{ yearId: number; termId: number }> {
  console.log("→ Seeding academic years & terms...");

  const years = [
    { yearName: "2024-2025", isCurrent: false },
    { yearName: "2025-2026", isCurrent: true },
  ];

  await db.insert(schema.academicYears).values(years).onConflictDoNothing();

  const [currentYear] = await db
    .select({ id: schema.academicYears.id })
    .from(schema.academicYears)
    .where(eq(schema.academicYears.isCurrent, true))
    .limit(1);

  if (!currentYear) throw new Error("Current academic year not found");

  const terms = [
    {
      name: "الفصل الدراسي الأول",
      academicYearId: currentYear.id,
      startDate: "2025-09-01",
      endDate: "2026-01-20",
      isCurrent: true,
    },
    {
      name: "الفصل الدراسي الثاني",
      academicYearId: currentYear.id,
      startDate: "2026-02-01",
      endDate: "2026-06-20",
      isCurrent: false,
    },
  ];

  await db.insert(schema.academicTerms).values(terms).onConflictDoNothing();

  const [currentTerm] = await db
    .select({ id: schema.academicTerms.id })
    .from(schema.academicTerms)
    .where(eq(schema.academicTerms.isCurrent, true))
    .limit(1);

  return { yearId: currentYear.id, termId: currentTerm!.id };
}

// ────────────────────────────────────────────────
// Seed Subjects
// ────────────────────────────────────────────────
async function seedSubjects() {
  const subjectsData = [
    { name: "اللغة العربية",       code: "ARA", color: "#FFCDD2" },
    { name: "اللغة الإنجليزية",    code: "ENG", color: "#BBDEFB" },
    { name: "الرياضيات",           code: "MAT", color: "#C8E6C9" },
    { name: "العلوم",              code: "SCI", color: "#FFF9C4" },
    { name: "الدراسات الاجتماعية", code: "SST", color: "#E1BEE7" },
    { name: "التربية الإسلامية",   code: "ISL", color: "#D1C4E9" },
    { name: "القرآن الكريم",       code: "QUR", color: "#C5CAE9" },
    { name: "التربية البدنية",     code: "PE",  color: "#A5D6A7" },
  ];

  await db.insert(schema.subjects).values(subjectsData).onConflictDoNothing();
  return await db.select().from(schema.subjects);
}

// ────────────────────────────────────────────────
// Seed Classes (Grades + Divisions)
// ────────────────────────────────────────────────
async function seedClasses(yearId: number) {
  const grades = [
    "الصف الأول", "الصف الثاني", "الصف الثالث", "الصف الرابع",
    "الصف الخامس", "الصف السادس", "الصف السابع", "الصف الثامن",
    "الصف التاسع", "الصف العاشر", "الصف الحادي عشر", "الصف الثاني عشر"
  ];

  const divisions = ["أ", "ب", "ج"];
  const classesData: any[] = [];

  for (const grade of grades) {
    // Parent grade
    classesData.push({
      name: grade,
      type: "classroom",
      division: null,
      parentClassId: null,
      academicYearId: yearId,
      color: "#E3F2FD",
    });

    // Divisions
    for (const div of divisions) {
      classesData.push({
        name: `${grade} (${div})`,
        type: "classroom",
        division: div,
        parentClassId: null, // سيتم تحديثه لاحقًا
        academicYearId: yearId,
        color: "#E3F2FD",
      });
    }
  }

  await db.insert(schema.classes).values(classesData).onConflictDoNothing();

  // ربط الأقسام بالصفوف الأم
  const allClasses = await db
    .select()
    .from(schema.classes)
    .where(eq(schema.classes.academicYearId, yearId))
    .orderBy(schema.classes.id);

  const updates = [];
  for (let i = 0; i < allClasses.length; i += divisions.length + 1) {
    const parent = allClasses[i];
    if (!parent) continue;

    for (let j = 1; j <= divisions.length; j++) {
      const child = allClasses[i + j];
      if (child) {
        updates.push(
          db.update(schema.classes)
            .set({ parentClassId: parent.id })
            .where(eq(schema.classes.id, child.id))
        );
      }
    }
  }

  if (updates.length > 0) await Promise.all(updates);

  return await db
    .select()
    .from(schema.classes)
    .where(eq(schema.classes.academicYearId, yearId));
}

// ────────────────────────────────────────────────
// Seed Users (Admins, Teachers, Students)
// ────────────────────────────────────────────────
async function seedUsers() {
  const usersData = [
    // Admin
    {
      id: randomUUID(),
      username: "admin",
      name: "مدير النظام",
      email: "admin@madrasa.com",
      phone: "+966500000001",
      role: "admin" as const,
      sex: "unspecified" as const,
      emailVerified: true,
    },
    // Teachers (4 teachers)
    ...[1, 2, 3, 4].map((i) => ({
      id: randomUUID(),
      username: `teacher${i}`,
      name: `أ. ${["محمد", "أحمد", "خالد", "عبدالله"][i - 1]} السعدي`,
      email: `teacher${i}@madrasa.com`,
      phone: `+96650111111${i}`,
      role: "teacher" as const,
      sex: "male" as const,
      emailVerified: true,
    })),
    // Students (12 students - 3 per class example)
    ...[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].map((i) => ({
      id: randomUUID(),
      username: `stu${1000 + i}`,
      name: ["سارة", "عمر", "نورة", "يوسف", "لين", "زيد", "ريم", "خالد", "مريم", "عبدالرحمن", "فاطمة", "حسن"][i - 1] + " " + ["عبدالله", "محمد", "أحمد", "خالد"][(i - 1) % 4],
      email: `stu${1000 + i}@madrasa.com`,
      phone: `+96650100${1000 + i}`,
      role: "student" as const,
      sex: (i % 2 === 1 ? "female" : "male") as "female" | "male",
      emailVerified: true,
    })),
  ];

  await db.insert(schema.users).values(usersData).onConflictDoNothing();

  // Insert student profiles
  const studentsProfiles = usersData
    .filter(u => u.role === "student")
    .map(u => ({
      userId: u.id,
      studentNumber: `ST${10000 + parseInt(u.username!.replace("stu", ""))}`,
      status: "active" as const,
    }));

  await db.insert(schema.students).values(studentsProfiles).onConflictDoNothing();

  const teachers = await db
    .select({ id: schema.users.id })
    .from(schema.users)
    .where(eq(schema.users.role, "teacher"));

  return {
    adminId: usersData[0]!.id,
    teacherIds: teachers.map(t => t.id),
    studentIds: studentsProfiles.map(s => s.userId),
  };
}

// ────────────────────────────────────────────────
// Time Slots (مواعيد الحصص)
// ────────────────────────────────────────────────
async function seedTimeSlots(yearId: number) {
  const slots = [
    { slotNumber: 1, startTime: "07:30:00", endTime: "08:15:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 2, startTime: "08:20:00", endTime: "09:05:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 3, startTime: "09:10:00", endTime: "09:55:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 4, startTime: "10:15:00", endTime: "11:00:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 5, startTime: "11:05:00", endTime: "11:50:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 6, startTime: "12:20:00", endTime: "13:05:00", durationMinutes: 45, isBreak: false },
    { slotNumber: 7, startTime: "13:10:00", endTime: "13:55:00", durationMinutes: 45, isBreak: false },
  ];

  const data = slots.map(s => ({ ...s, academicYearId: yearId }));

  await db.insert(schema.timeSlots).values(data).onConflictDoNothing();
  return await db.select().from(schema.timeSlots).where(eq(schema.timeSlots.academicYearId, yearId));
}

// ────────────────────────────────────────────────
// Scheduled Lessons (جدول الحصص)
// ────────────────────────────────────────────────
async function seedScheduledLessons(
  yearId: number,
  termId: number,
  classIds: number[],
  subjectIds: number[],
  teacherIds: string[],
  timeSlotIds: number[]
) {
  const days = [0, 1, 2, 3, 4]; // Sunday → Thursday (0=Sunday in many systems)

  const lessons: any[] = [];

  for (const clsId of classIds.slice(0, 6)) { // limit to first 6 classes for example
    for (const subjId of subjectIds) {
      for (const day of days) {
        const slot = randomFromArray(timeSlotIds);
        const teacher = randomFromArray(teacherIds);

        lessons.push({
          classId: clsId,
          subjectId: subjId,
          teacherId: teacher,
          timeSlotId: slot,
          dayOfWeek: day,
          academicYearId: yearId,
          academicTermId: termId,
          isActive: true,
        });
      }
    }
  }

  await db.insert(schema.scheduledLessons).values(lessons).onConflictDoNothing();
  console.log(`   → Created ${lessons.length} scheduled lessons`);
}

// ────────────────────────────────────────────────
// Attendance Records (سجلات الحضور)
// ────────────────────────────────────────────────
async function seedSampleAttendance(
  studentIds: string[],
  lessonIds: number[],
  teacherIds: string[]
) {
  const statuses = ["present", "present", "present", "late", "absent", "excused"];

  const records: any[] = [];

  // Sample for ~30% of combinations
  for (const studentId of studentIds.slice(0, 8)) {
    for (const lessonId of lessonIds.slice(0, 5)) {
      if (Math.random() > 0.7) continue;

      const status = randomFromArray(statuses);
      records.push({
        studentId,
        scheduledLessonId: lessonId,
        date: "2025-10-15",
        status,
        arrivalTime: status === "late" ? "08:25:00" : null,
        markedById: randomFromArray(teacherIds),
        isVerified: status !== "absent",
      });
    }
  }

  if (records.length > 0) {
    await db.insert(schema.attendanceRecords).values(records).onConflictDoNothing();
    console.log(`   → Inserted ${records.length} attendance records`);
  }
}

// ────────────────────────────────────────────────
// Main Seeding Flow
// ────────────────────────────────────────────────
async function main() {
  try {
    await resetDatabase();

    const { yearId, termId } = await seedAcademicStructure();
    const subjects    = await seedSubjects();
    const classes     = await seedClasses(yearId);
    const { adminId, teacherIds, studentIds } = await seedUsers();

    // Assign students to classes (simple round-robin example)
    const classIds = classes.map(c => c.id);
    for (let i = 0; i < studentIds.length; i++) {
      const clsId = classIds[i % classIds.length];
      await db.update(schema.students)
        .set({ currentClassId: clsId })
        .where(eq(schema.students.userId, studentIds[i]!));
    }

    const timeSlots = await seedTimeSlots(yearId);
    const timeSlotIds = timeSlots.map(s => s.id);

    await seedScheduledLessons(
      yearId,
      termId,
      classIds,
      subjects.map(s => s.id),
      teacherIds,
      timeSlotIds
    );

    const lessons = await db.select({ id: schema.scheduledLessons.id })
      .from(schema.scheduledLessons)
      .where(eq(schema.scheduledLessons.academicYearId, yearId));

    await seedSampleAttendance(studentIds, lessons.map(l => l.id), teacherIds);

    console.log("\n" + "=".repeat(60));
    console.log("           Seeding completed successfully ✓");
    console.log("=".repeat(60) + "\n");

  } catch (err) {
    console.error("❌ Seeding failed:", err);
    process.exit(1);
  }
}

main();