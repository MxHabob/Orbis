
import { db } from "@db/client";
import { 
  students, 
  users, 
  classes, 
  attendanceRecords, 
  gradeEntries, 
  academicYears,
  enrollments
} from "@db/schema";
import { eq, sql, and, count, avg, desc } from "drizzle-orm";

export async function getStudentCount() {
  const [result] = await db
    .select({ count: count() })
    .from(students)
    .where(eq(students.status, "active"));
  return result?.count || 0;
}

export async function getTeacherCount() {
  const [result] = await db
    .select({ count: count() })
    .from(users)
    .where(eq(users.role, "teacher"));
  return result?.count || 0;
}

export async function getAverageAttendanceRate() {
  // Get current academic year
  const [currentYear] = await db
    .select()
    .from(academicYears)
    .where(eq(academicYears.isCurrent, true))
    .limit(1);

  if (!currentYear) return 0;

  // Simple calculation: (Present count / Total records) * 100
  // Note: This is an approximation. Ideally we'd filter by current year relations.
  // Assuming attendanceRecords are growing and we want recent trends, 
  // or we can filter by date if we want specificity.
  
  const [stats] = await db
    .select({
      total: count(),
      present: sql<number>`sum(case when ${attendanceRecords.status} = 'present' then 1 else 0 end)`
    })
    .from(attendanceRecords)
    // Add date filter if column exists and is relevant to current year, 
    // but for now let's take overall or recent.
    // .where(and(gte(attendanceRecords.date, currentYear.startDate), lte(attendanceRecords.date, currentYear.endDate)))
    ;

  if (!stats || !stats.total) return 0;
  return (Number(stats.present) / Number(stats.total)) * 100;
}

export async function getGenderDistribution() {
  return db
    .select({
      sex: users.sex,
      count: count(),
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .groupBy(users.sex);
}

export async function getClassDistribution() {
  // Returns number of students per class
  return db
    .select({
      className: classes.name,
      studentCount: count(students.userId)
    })
    .from(classes)
    .leftJoin(students, eq(classes.id, students.currentClassId))
    .where(eq(students.status, 'active'))
    .groupBy(classes.name)
    .orderBy(desc(count(students.userId)))
    .limit(10);
}

export async function getRecentEnrolledStudents() {
    return db
        .select({
            id: users.id,
            name: users.name,
            image: users.image,
            email: users.email,
            enrolledAt: enrollments.enrolledAt,
        })
        .from(enrollments)
        .innerJoin(students, eq(enrollments.studentId, students.userId))
        .innerJoin(users, eq(students.userId, users.id))
        .orderBy(desc(enrollments.enrolledAt))
        .limit(5);
}

export async function getOverallPerformance() {
  // Rough estimate of overall average grade
  const [result] = await db
    .select({
      avgScore: avg(sql<number>`cast(${gradeEntries.score} as float)`)
    })
    .from(gradeEntries)
    .where(sql`${gradeEntries.score} is not null and ${gradeEntries.score} != ''`);
    
  return Number(result?.avgScore || 0);
}
