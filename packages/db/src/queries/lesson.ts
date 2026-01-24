import type { Database } from "@db/client";
import {
  lessons,
  teacherClassSubjects,
  classes,
  employees,
  users,
  curriculumSubjects,
  subjects,
  academicYears,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetLessonByIdParams = {
  id: number;
};

export const getLessonById = async (
  db: Database,
  params: GetLessonByIdParams,
) => {
  const [result] = await db
    .select({
      id: lessons.id,
      name: lessons.name,
      day: lessons.day,
      startTime: lessons.startTime,
      endTime: lessons.endTime,
      teacherClassSubjectId: lessons.teacherClassSubjectId,
      academicYearId: lessons.academicYearId,
      teacherId: lessons.teacherId,
      classId: lessons.classId,
      teacher: {
        userId: employees.userId,
        name: users.name,
        employeeNumber: employees.employeeNumber,
      },
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
    })
    .from(lessons)
    .where(eq(lessons.id, params.id))
    .leftJoin(employees, eq(employees.userId, lessons.teacherId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(classes, eq(classes.id, lessons.classId))
    .leftJoin(academicYears, eq(academicYears.id, lessons.academicYearId))
    .leftJoin(
      teacherClassSubjects,
      eq(teacherClassSubjects.id, lessons.teacherClassSubjectId),
    )
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .limit(1);

  return result;
};

export type GetLessonsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  classId?: number | null;
  teacherId?: string | null;
  academicYearId?: number | null;
  day?: string | null;
};

export const getLessons = async (
  db: Database,
  params: GetLessonsParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 50,
    classId,
    teacherId,
    academicYearId,
    day,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (classId) {
    whereConditions.push(eq(lessons.classId, classId));
  }

  if (teacherId) {
    whereConditions.push(eq(lessons.teacherId, teacherId));
  }

  if (academicYearId) {
    whereConditions.push(eq(lessons.academicYearId, academicYearId));
  }

  if (day) {
    whereConditions.push(eq(lessons.day, day as any));
  }

  const query = db
    .select({
      id: lessons.id,
      name: lessons.name,
      day: lessons.day,
      startTime: lessons.startTime,
      endTime: lessons.endTime,
      teacherClassSubjectId: lessons.teacherClassSubjectId,
      academicYearId: lessons.academicYearId,
      teacherId: lessons.teacherId,
      classId: lessons.classId,
      teacher: {
        userId: employees.userId,
        name: users.name,
      },
      class: {
        id: classes.id,
        name: classes.name,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
    })
    .from(lessons)
    .leftJoin(employees, eq(employees.userId, lessons.teacherId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(classes, eq(classes.id, lessons.classId))
    .leftJoin(
      teacherClassSubjects,
      eq(teacherClassSubjects.id, lessons.teacherClassSubjectId),
    )
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "day") {
      isAscending
        ? query.orderBy(asc(lessons.day))
        : query.orderBy(desc(lessons.day));
    } else if (column === "start_time") {
      isAscending
        ? query.orderBy(asc(lessons.startTime))
        : query.orderBy(desc(lessons.startTime));
    }
  } else {
    // Default sort by day and start_time
    query.orderBy(asc(lessons.day), asc(lessons.startTime));
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

export type CreateLessonParams = {
  name?: string | null;
  day: string;
  startTime: Date;
  endTime: Date;
  teacherClassSubjectId: number;
  academicYearId?: number | null;
  teacherId?: string | null;
  classId?: number | null;
};

export const createLesson = async (
  db: Database,
  params: CreateLessonParams,
) => {
  const [result] = await db
    .insert(lessons)
    .values(params)
    .returning();

  return result;
};

export type UpdateLessonParams = {
  id: number;
  name?: string | null;
  day?: string;
  startTime?: Date;
  endTime?: Date;
  teacherClassSubjectId?: number;
};

export const updateLesson = async (
  db: Database,
  params: UpdateLessonParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(lessons)
    .set(rest)
    .where(eq(lessons.id, id))
    .returning();

  return result;
};

export type DeleteLessonParams = {
  id: number;
};

export const deleteLesson = async (
  db: Database,
  params: DeleteLessonParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(lessons)
    .where(eq(lessons.id, id))
    .returning();

  return result;
};

// Get class schedule
export type GetClassScheduleParams = {
  classId: number;
  academicYearId?: number | null;
};

export const getClassSchedule = async (
  db: Database,
  params: GetClassScheduleParams,
) => {
  const { classId, academicYearId } = params;

  const whereConditions: SQL[] = [eq(lessons.classId, classId)];

  if (academicYearId) {
    whereConditions.push(eq(lessons.academicYearId, academicYearId));
  }

  const data = await db
    .select({
      id: lessons.id,
      name: lessons.name,
      day: lessons.day,
      startTime: lessons.startTime,
      endTime: lessons.endTime,
      teacher: {
        userId: employees.userId,
        name: users.name,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
        shortName: subjects.shortName,
      },
    })
    .from(lessons)
    .where(and(...whereConditions))
    .leftJoin(employees, eq(employees.userId, lessons.teacherId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(
      teacherClassSubjects,
      eq(teacherClassSubjects.id, lessons.teacherClassSubjectId),
    )
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .orderBy(asc(lessons.day), asc(lessons.startTime));

  return data;
};

// Get teacher schedule
export type GetTeacherScheduleParams = {
  teacherId: string;
  academicYearId?: number | null;
};

export const getTeacherSchedule = async (
  db: Database,
  params: GetTeacherScheduleParams,
) => {
  const { teacherId, academicYearId } = params;

  const whereConditions: SQL[] = [eq(lessons.teacherId, teacherId)];

  if (academicYearId) {
    whereConditions.push(eq(lessons.academicYearId, academicYearId));
  }

  const data = await db
    .select({
      id: lessons.id,
      name: lessons.name,
      day: lessons.day,
      startTime: lessons.startTime,
      endTime: lessons.endTime,
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
        shortName: subjects.shortName,
      },
    })
    .from(lessons)
    .where(and(...whereConditions))
    .leftJoin(classes, eq(classes.id, lessons.classId))
    .leftJoin(
      teacherClassSubjects,
      eq(teacherClassSubjects.id, lessons.teacherClassSubjectId),
    )
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .orderBy(asc(lessons.day), asc(lessons.startTime));

  return data;
};

// Bulk create lessons
export type BulkCreateLessonsParams = {
  lessons: Array<{
    name?: string | null;
    day: string;
    startTime: Date;
    endTime: Date;
    teacherClassSubjectId: number;
    academicYearId?: number | null;
    teacherId?: string | null;
    classId?: number | null;
  }>;
};

export const bulkCreateLessons = async (
  db: Database,
  params: BulkCreateLessonsParams,
) => {
  const { lessons: lessonList } = params;

  const results = await db
    .insert(lessons)
    .values(lessonList)
    .returning();

  return results;
};

// Get schedule conflicts
export type GetScheduleConflictsParams = {
  teacherId?: string | null;
  classId?: number | null;
  day: string;
  startTime: Date;
  endTime: Date;
  excludeLessonId?: number | null;
};

export const getScheduleConflicts = async (
  db: Database,
  params: GetScheduleConflictsParams,
) => {
  const { teacherId, classId, day, startTime, endTime, excludeLessonId } = params;

  const whereConditions: SQL[] = [
    eq(lessons.day, day as any),
    sql`(
      (${lessons.startTime} >= ${startTime} AND ${lessons.startTime} < ${endTime}) OR
      (${lessons.endTime} > ${startTime} AND ${lessons.endTime} <= ${endTime}) OR
      (${lessons.startTime} <= ${startTime} AND ${lessons.endTime} >= ${endTime})
    )`,
  ];

  if (teacherId) {
    whereConditions.push(eq(lessons.teacherId, teacherId));
  }

  if (classId) {
    whereConditions.push(eq(lessons.classId, classId));
  }

  if (excludeLessonId) {
    whereConditions.push(sql`${lessons.id} != ${excludeLessonId}`);
  }

  const data = await db
    .select({
      id: lessons.id,
      name: lessons.name,
      day: lessons.day,
      startTime: lessons.startTime,
      endTime: lessons.endTime,
      teacher: {
        userId: employees.userId,
        name: users.name,
      },
      class: {
        id: classes.id,
        name: classes.name,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
      },
    })
    .from(lessons)
    .where(and(...whereConditions))
    .leftJoin(employees, eq(employees.userId, lessons.teacherId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(classes, eq(classes.id, lessons.classId))
    .leftJoin(
      teacherClassSubjects,
      eq(teacherClassSubjects.id, lessons.teacherClassSubjectId),
    )
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId));

  return data;
};
