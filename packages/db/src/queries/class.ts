import type { Database } from "@db/client";
import {
  classes,
  academicYears,
  levels,
  students,
  employees,
  users,
  classSubjects,
  curriculumSubjects,
  subjects,
} from "@db/schema";
import { buildSearchQuery } from "@midday/db/utils/search-query";
import { and, asc, desc, eq, inArray, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetClassByIdParams = {
  id: number;
};

export const getClassById = async (
  db: Database,
  params: GetClassByIdParams,
) => {
  const [result] = await db
    .select({
      id: classes.id,
      name: classes.name,
      code: classes.code,
      division: classes.division,
      capacity: classes.capacity,
      academicYearId: classes.academicYearId,
      levelId: classes.levelId,
      section: classes.section,
      track: classes.track,
      supervisorId: classes.supervisorId,
      current: classes.current,
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      level: {
        id: levels.id,
        name: levels.name,
        stage: levels.stage,
      },
      supervisor: {
        userId: employees.userId,
        name: users.name,
        employeeNumber: employees.employeeNumber,
      },
      studentCount: sql<number>`cast(count(distinct ${students.userId}) as int)`,
      subjectCount: sql<number>`cast(count(distinct ${classSubjects.id}) as int)`,
    })
    .from(classes)
    .where(eq(classes.id, params.id))
    .leftJoin(academicYears, eq(academicYears.id, classes.academicYearId))
    .leftJoin(levels, eq(levels.id, classes.levelId))
    .leftJoin(employees, eq(employees.userId, classes.supervisorId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(students, eq(students.currentClassId, classes.id))
    .leftJoin(classSubjects, eq(classSubjects.classId, classes.id))
    .groupBy(
      classes.id,
      academicYears.id,
      levels.id,
      employees.userId,
      users.id,
    );

  return result;
};

export type GetClassesParams = {
  cursor?: string | null;
  pageSize?: number;
  q?: string | null;
  sort?: string[] | null;
  academicYearId?: number | null;
  levelId?: number | null;
  section?: string | null;
  track?: string | null;
  currentOnly?: boolean;
};

export const getClasses = async (
  db: Database,
  params: GetClassesParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 25,
    q,
    academicYearId,
    levelId,
    section,
    track,
    currentOnly = false,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (academicYearId) {
    whereConditions.push(eq(classes.academicYearId, academicYearId));
  }

  if (levelId) {
    whereConditions.push(eq(classes.levelId, levelId));
  }

  if (section) {
    whereConditions.push(eq(classes.section, section as any));
  }

  if (track) {
    whereConditions.push(eq(classes.track, track as any));
  }

  if (currentOnly) {
    whereConditions.push(eq(classes.current, true));
  }

  // Apply search query filter
  if (q) {
    whereConditions.push(
      sql`(${classes.name} ILIKE '%' || ${q} || '%' OR ${classes.code} ILIKE '%' || ${q} || '%')`,
    );
  }

  const query = db
    .select({
      id: classes.id,
      name: classes.name,
      code: classes.code,
      division: classes.division,
      capacity: classes.capacity,
      academicYearId: classes.academicYearId,
      levelId: classes.levelId,
      section: classes.section,
      track: classes.track,
      supervisorId: classes.supervisorId,
      current: classes.current,
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      level: {
        id: levels.id,
        name: levels.name,
        stage: levels.stage,
      },
      supervisor: {
        userId: employees.userId,
        name: users.name,
      },
      studentCount: sql<number>`cast(count(distinct ${students.userId}) as int)`,
      subjectCount: sql<number>`cast(count(distinct ${classSubjects.id}) as int)`,
    })
    .from(classes)
    .leftJoin(academicYears, eq(academicYears.id, classes.academicYearId))
    .leftJoin(levels, eq(levels.id, classes.levelId))
    .leftJoin(employees, eq(employees.userId, classes.supervisorId))
    .leftJoin(users, eq(users.id, employees.userId))
    .leftJoin(students, eq(students.currentClassId, classes.id))
    .leftJoin(classSubjects, eq(classSubjects.classId, classes.id))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(
      classes.id,
      academicYears.id,
      levels.id,
      employees.userId,
      users.id,
    );

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(classes.name))
        : query.orderBy(desc(classes.name));
    } else if (column === "code") {
      isAscending
        ? query.orderBy(asc(classes.code))
        : query.orderBy(desc(classes.code));
    } else if (column === "division") {
      isAscending
        ? query.orderBy(asc(classes.division))
        : query.orderBy(desc(classes.division));
    } else if (column === "student_count") {
      isAscending
        ? query.orderBy(asc(sql`count(distinct ${students.userId})`))
        : query.orderBy(desc(sql`count(distinct ${students.userId})`));
    }
  } else {
    // Default sort by name ascending
    query.orderBy(asc(classes.name));
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

export type CreateClassParams = {
  name: string;
  code?: string | null;
  division?: string | null;
  capacity?: number | null;
  academicYearId: number;
  levelId: number;
  section?: string | null;
  track?: string | null;
  supervisorId?: string | null;
  current?: boolean;
};

export const createClass = async (
  db: Database,
  params: CreateClassParams,
) => {
  const { current = true, ...rest } = params;

  const [result] = await db
    .insert(classes)
    .values({
      ...rest,
      current,
    })
    .returning();

  return result;
};

export type UpdateClassParams = {
  id: number;
  name?: string;
  code?: string | null;
  division?: string | null;
  capacity?: number | null;
  academicYearId?: number;
  levelId?: number;
  section?: string | null;
  track?: string | null;
  supervisorId?: string | null;
  current?: boolean;
};

export const updateClass = async (
  db: Database,
  params: UpdateClassParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(classes)
    .set(rest)
    .where(eq(classes.id, id))
    .returning();

  return result;
};

export type DeleteClassParams = {
  id: number;
};

export const deleteClass = async (
  db: Database,
  params: DeleteClassParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(classes)
    .where(eq(classes.id, id))
    .returning();

  return result;
};

// Class Subjects

export type GetClassSubjectsParams = {
  classId: number;
};

export const getClassSubjects = async (
  db: Database,
  params: GetClassSubjectsParams,
) => {
  const { classId } = params;

  const data = await db
    .select({
      id: classSubjects.id,
      classId: classSubjects.classId,
      curriculumSubjectId: classSubjects.curriculumSubjectId,
      subject: {
        id: subjects.id,
        code: subjects.code,
        name: subjects.name,
        shortName: subjects.shortName,
        department: subjects.department,
      },
      curriculumSubject: {
        id: curriculumSubjects.id,
        isMandatory: curriculumSubjects.isMandatory,
        weeklyPeriods: curriculumSubjects.weeklyPeriods,
        maxScore: curriculumSubjects.maxScore,
      },
    })
    .from(classSubjects)
    .where(eq(classSubjects.classId, classId))
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, classSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .orderBy(asc(curriculumSubjects.orderInReport));

  return data;
};

export type AddClassSubjectParams = {
  classId: number;
  curriculumSubjectId: number;
};

export const addClassSubject = async (
  db: Database,
  params: AddClassSubjectParams,
) => {
  const [result] = await db
    .insert(classSubjects)
    .values(params)
    .returning();

  return result;
};

export type RemoveClassSubjectParams = {
  id: number;
};

export const removeClassSubject = async (
  db: Database,
  params: RemoveClassSubjectParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(classSubjects)
    .where(eq(classSubjects.id, id))
    .returning();

  return result;
};

// Get classes by academic year and level
export type GetClassesByYearAndLevelParams = {
  academicYearId: number;
  levelId?: number | null;
};

export const getClassesByYearAndLevel = async (
  db: Database,
  params: GetClassesByYearAndLevelParams,
) => {
  const { academicYearId, levelId } = params;

  const whereConditions: SQL[] = [eq(classes.academicYearId, academicYearId)];

  if (levelId) {
    whereConditions.push(eq(classes.levelId, levelId));
  }

  const data = await db
    .select({
      id: classes.id,
      name: classes.name,
      code: classes.code,
      division: classes.division,
      capacity: classes.capacity,
      section: classes.section,
      track: classes.track,
      level: {
        id: levels.id,
        name: levels.name,
      },
      studentCount: sql<number>`cast(count(distinct ${students.userId}) as int)`,
    })
    .from(classes)
    .leftJoin(levels, eq(levels.id, classes.levelId))
    .leftJoin(students, eq(students.currentClassId, classes.id))
    .where(and(...whereConditions))
    .groupBy(classes.id, levels.id)
    .orderBy(asc(levels.order), asc(classes.name));

  return data;
};