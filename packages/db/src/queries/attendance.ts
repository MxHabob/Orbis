import type { Database } from "@db/client";
import {
  attendances,
  students,
  users,
  lessons,
  academicYears,
  academicTerms,
  classes,
  teacherClassSubjects,
  curriculumSubjects,
  subjects,
} from "@db/schema";
import { and, asc, desc, eq, gte, lte, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetAttendanceByIdParams = {
  id: number;
};

export const getAttendanceById = async (
  db: Database,
  params: GetAttendanceByIdParams,
) => {
  const [result] = await db
    .select({
      id: attendances.id,
      date: attendances.date,
      present: attendances.present,
      studentId: attendances.studentId,
      lessonId: attendances.lessonId,
      termId: attendances.termId,
      academicYearId: attendances.academicYearId,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
      lesson: {
        id: lessons.id,
        name: lessons.name,
        day: lessons.day,
        startTime: lessons.startTime,
        endTime: lessons.endTime,
      },
    })
    .from(attendances)
    .where(eq(attendances.id, params.id))
    .leftJoin(students, eq(students.userId, attendances.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(lessons, eq(lessons.id, attendances.lessonId))
    .limit(1);

  return result;
};

export type GetAttendancesParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  studentId?: string | null;
  classId?: number | null;
  lessonId?: number | null;
  academicYearId?: number | null;
  termId?: number | null;
  dateFrom?: Date | null;
  dateTo?: Date | null;
  present?: boolean | null;
};

export const getAttendances = async (
  db: Database,
  params: GetAttendancesParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 50,
    studentId,
    classId,
    lessonId,
    academicYearId,
    termId,
    dateFrom,
    dateTo,
    present,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (studentId) {
    whereConditions.push(eq(attendances.studentId, studentId));
  }

  if (classId) {
    whereConditions.push(eq(students.currentClassId, classId));
  }

  if (lessonId) {
    whereConditions.push(eq(attendances.lessonId, lessonId));
  }

  if (academicYearId) {
    whereConditions.push(eq(attendances.academicYearId, academicYearId));
  }

  if (termId) {
    whereConditions.push(eq(attendances.termId, termId));
  }

  if (dateFrom) {
    whereConditions.push(gte(attendances.date, dateFrom));
  }

  if (dateTo) {
    whereConditions.push(lte(attendances.date, dateTo));
  }

  if (present !== null && present !== undefined) {
    whereConditions.push(eq(attendances.present, present));
  }

  const query = db
    .select({
      id: attendances.id,
      date: attendances.date,
      present: attendances.present,
      studentId: attendances.studentId,
      lessonId: attendances.lessonId,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
      lesson: {
        id: lessons.id,
        name: lessons.name,
        day: lessons.day,
      },
    })
    .from(attendances)
    .leftJoin(students, eq(students.userId, attendances.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(lessons, eq(lessons.id, attendances.lessonId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "date") {
      isAscending
        ? query.orderBy(asc(attendances.date))
        : query.orderBy(desc(attendances.date));
    } else if (column === "student_number") {
      isAscending
        ? query.orderBy(asc(students.studentNumber))
        : query.orderBy(desc(students.studentNumber));
    }
  } else {
    // Default sort by date descending
    query.orderBy(desc(attendances.date));
  }

  // Apply pagination
  const offset = cursor ? Number.parseInt(cursor, 10) : 0;
  query.limit(pageSize).offset(offset);

  const data = await query;

  const nextCursor =
    data && data.length === pageSize
      ? (offset + pageSize).toString()
      : undefined;

  return {
    meta: {
      cursor: nextCursor ?? null,
      hasPreviousPage: offset > 0,
      hasNextPage: data && data.length === pageSize,
    },
    data,
  };
};

export type CreateAttendanceParams = {
  date: Date;
  present: boolean;
  studentId: string;
  lessonId?: number | null;
  termId?: number | null;
  academicYearId?: number | null;
};

export const createAttendance = async (
  db: Database,
  params: CreateAttendanceParams,
) => {
  const [result] = await db
    .insert(attendances)
    .values(params)
    .returning();

  return result;
};

export type UpdateAttendanceParams = {
  id: number;
  date?: Date;
  present?: boolean;
  lessonId?: number | null;
  termId?: number | null;
};

export const updateAttendance = async (
  db: Database,
  params: UpdateAttendanceParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(attendances)
    .set(rest)
    .where(eq(attendances.id, id))
    .returning();

  return result;
};

export type DeleteAttendanceParams = {
  id: number;
};

export const deleteAttendance = async (
  db: Database,
  params: DeleteAttendanceParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(attendances)
    .where(eq(attendances.id, id))
    .returning();

  return result;
};

// Bulk attendance recording
export type BulkCreateAttendancesParams = {
  attendances: Array<{
    date: Date;
    present: boolean;
    studentId: string;
    lessonId?: number | null;
    termId?: number | null;
    academicYearId?: number | null;
  }>;
};

export const bulkCreateAttendances = async (
  db: Database,
  params: BulkCreateAttendancesParams,
) => {
  const { attendances: attendanceList } = params;

  const results = await db
    .insert(attendances)
    .values(attendanceList)
    .returning();

  return results;
};

// Get student attendance summary
export type GetStudentAttendanceSummaryParams = {
  studentId: string;
  academicYearId: number;
  termId?: number | null;
  dateFrom?: Date | null;
  dateTo?: Date | null;
};

export const getStudentAttendanceSummary = async (
  db: Database,
  params: GetStudentAttendanceSummaryParams,
) => {
  const { studentId, academicYearId, termId, dateFrom, dateTo } = params;

  const whereConditions: SQL[] = [
    eq(attendances.studentId, studentId),
    eq(attendances.academicYearId, academicYearId),
  ];

  if (termId) {
    whereConditions.push(eq(attendances.termId, termId));
  }

  if (dateFrom) {
    whereConditions.push(gte(attendances.date, dateFrom));
  }

  if (dateTo) {
    whereConditions.push(lte(attendances.date, dateTo));
  }

  const [result] = await db
    .select({
      totalDays: sql<number>`cast(count(*) as int)`,
      presentDays: sql<number>`cast(sum(case when ${attendances.present} then 1 else 0 end) as int)`,
      absentDays: sql<number>`cast(sum(case when not ${attendances.present} then 1 else 0 end) as int)`,
      attendanceRate: sql<number>`
        case 
          when count(*) > 0 
          then (cast(sum(case when ${attendances.present} then 1 else 0 end) as float) / count(*)) * 100
          else 0
        end
      `,
    })
    .from(attendances)
    .where(and(...whereConditions));

  return result;
};

// Get class attendance for a specific date
export type GetClassAttendanceForDateParams = {
  classId: number;
  date: Date;
  lessonId?: number | null;
};

export const getClassAttendanceForDate = async (
  db: Database,
  params: GetClassAttendanceForDateParams,
) => {
  const { classId, date, lessonId } = params;

  const whereConditions: SQL[] = [
    eq(students.currentClassId, classId),
    sql`date(${attendances.date}) = date(${date})`,
  ];

  if (lessonId) {
    whereConditions.push(eq(attendances.lessonId, lessonId));
  }

  const data = await db
    .select({
      attendanceId: attendances.id,
      studentId: students.userId,
      studentNumber: students.studentNumber,
      studentName: users.name,
      surname: students.surname,
      present: attendances.present,
      date: attendances.date,
    })
    .from(students)
    .where(eq(students.currentClassId, classId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(
      attendances,
      and(
        eq(attendances.studentId, students.userId),
        sql`date(${attendances.date}) = date(${date})`,
        lessonId ? eq(attendances.lessonId, lessonId) : sql`true`,
      ),
    )
    .orderBy(asc(students.studentNumber));

  return data;
};

// Get attendance statistics by class
export type GetClassAttendanceStatsParams = {
  classId: number;
  academicYearId: number;
  termId?: number | null;
  dateFrom?: Date | null;
  dateTo?: Date | null;
};

export const getClassAttendanceStats = async (
  db: Database,
  params: GetClassAttendanceStatsParams,
) => {
  const { classId, academicYearId, termId, dateFrom, dateTo } = params;

  const whereConditions: SQL[] = [
    eq(students.currentClassId, classId),
    eq(attendances.academicYearId, academicYearId),
  ];

  if (termId) {
    whereConditions.push(eq(attendances.termId, termId));
  }

  if (dateFrom) {
    whereConditions.push(gte(attendances.date, dateFrom));
  }

  if (dateTo) {
    whereConditions.push(lte(attendances.date, dateTo));
  }

  const data = await db
    .select({
      studentId: students.userId,
      studentNumber: students.studentNumber,
      studentName: users.name,
      surname: students.surname,
      totalDays: sql<number>`cast(count(${attendances.id}) as int)`,
      presentDays: sql<number>`cast(sum(case when ${attendances.present} then 1 else 0 end) as int)`,
      absentDays: sql<number>`cast(sum(case when not ${attendances.present} then 1 else 0 end) as int)`,
      attendanceRate: sql<number>`
        case 
          when count(${attendances.id}) > 0 
          then (cast(sum(case when ${attendances.present} then 1 else 0 end) as float) / count(${attendances.id})) * 100
          else 0
        end
      `,
    })
    .from(students)
    .where(eq(students.currentClassId, classId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(
      attendances,
      and(
        eq(attendances.studentId, students.userId),
        ...whereConditions.slice(1), // Skip the classId condition
      ),
    )
    .groupBy(students.userId, students.studentNumber, users.name, students.surname)
    .orderBy(asc(students.studentNumber));

  return data;
};
