import { db } from "@db/client";
import {
  academicYears,
  attendanceRecords,
  classes,
  enrollments,
  gradeEntries,
  students,
  users,
} from "@db/schema";
import { and, avg, count, desc, eq, gte, lte, sql } from "drizzle-orm";

export interface MetricFilters {
  from?: string;
  to?: string;
}

export async function getStudentCount(filters?: MetricFilters) {
  const conditions = [eq(students.status, "active")];

  // Note: Student count is usually a snapshot, but we can filter by enrollment date if needed
  // if (filters?.from) conditions.push(gte(enrollments.enrolledAt, new Date(filters.from)));
  // if (filters?.to) conditions.push(lte(enrollments.enrolledAt, new Date(filters.to)));

  const [result] = await db
    .select({ count: count() })
    .from(students)
    .where(and(...conditions));

  return result?.count || 0;
}

export async function getTeacherCount(filters?: MetricFilters) {
  const [result] = await db
    .select({ count: count() })
    .from(users)
    .where(eq(users.role, "teacher"));
  return result?.count || 0;
}

export async function getAverageAttendanceRate(filters?: MetricFilters) {
  const conditions: any[] = [];

  if (filters?.from) conditions.push(gte(attendanceRecords.date, filters.from));
  if (filters?.to) conditions.push(lte(attendanceRecords.date, filters.to));

  const [stats] = await db
    .select({
      total: count(),
      present: sql<number>`sum(case when ${attendanceRecords.status} = 'present' then 1 else 0 end)`,
    })
    .from(attendanceRecords)
    .where(conditions.length > 0 ? and(...conditions) : undefined);

  if (!stats || !stats.total) return 0;
  return Math.round((Number(stats.present) / Number(stats.total)) * 100);
}

export async function getGenderDistribution(filters?: MetricFilters) {
  return db
    .select({
      sex: users.sex,
      count: count(),
    })
    .from(students)
    .innerJoin(users, eq(students.userId, users.id))
    .groupBy(users.sex);
}

export async function getClassDistribution(filters?: MetricFilters) {
  // Get current year
  const [currentYear] = await db
    .select({ id: academicYears.id })
    .from(academicYears)
    .where(eq(academicYears.isCurrent, true))
    .limit(1);

  return db
    .select({
      className: classes.name,
      studentCount: count(enrollments.id),
    })
    .from(classes)
    .leftJoin(
      enrollments,
      and(
        eq(classes.id, enrollments.classId),
        eq(enrollments.status, "active"),
        currentYear
          ? eq(enrollments.academicYearId, currentYear.id)
          : undefined,
      ),
    )
    .where(sql`${classes.deletedAt} IS NULL`)
    .groupBy(classes.name)
    .orderBy(desc(count(enrollments.id)))
    .limit(10);
}

export async function getRecentEnrolledStudents(filters?: MetricFilters) {
  const conditions: any[] = [];
  if (filters?.from)
    conditions.push(gte(enrollments.enrolledAt, new Date(filters.from)));
  if (filters?.to)
    conditions.push(lte(enrollments.enrolledAt, new Date(filters.to)));

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
    .where(conditions.length > 0 ? and(...conditions) : undefined)
    .orderBy(desc(enrollments.enrolledAt))
    .limit(5);
}

export async function getOverallPerformance(filters?: MetricFilters) {
  const conditions: any[] = [sql`${gradeEntries.score} is not null`];

  if (filters?.from)
    conditions.push(gte(gradeEntries.updatedAt, new Date(filters.from)));
  if (filters?.to)
    conditions.push(lte(gradeEntries.updatedAt, new Date(filters.to)));

  const [result] = await db
    .select({
      avgScore: avg(sql<number>`cast(${gradeEntries.score} as float)`),
    })
    .from(gradeEntries)
    .where(and(...conditions));

  return Math.round(Number(result?.avgScore || 0));
}
