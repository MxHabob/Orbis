import type { Database } from "@db/client";
import {
  subjects,
  curriculumSubjects,
  academicYears,
  levels,
  classes,
  teacherSubjects,
  employees,
  users,
} from "@db/schema";
import { and, asc, desc, eq, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetSubjectByIdParams = {
  id: number;
};

export const getSubjectById = async (
  db: Database,
  params: GetSubjectByIdParams,
) => {
  const [result] = await db
    .select({
      id: subjects.id,
      code: subjects.code,
      name: subjects.name,
      shortName: subjects.shortName,
      department: subjects.department,
      description: subjects.description,
      teacherCount: sql<number>`cast(count(distinct ${teacherSubjects.employeeId}) as int)`,
      curriculumCount: sql<number>`cast(count(distinct ${curriculumSubjects.id}) as int)`,
    })
    .from(subjects)
    .where(eq(subjects.id, params.id))
    .leftJoin(teacherSubjects, eq(teacherSubjects.subjectId, subjects.id))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.subjectId, subjects.id))
    .groupBy(subjects.id);

  return result;
};

export type GetSubjectsParams = {
  cursor?: string | null;
  pageSize?: number;
  q?: string | null;
  sort?: string[] | null;
  department?: string | null;
};

export const getSubjects = async (
  db: Database,
  params: GetSubjectsParams,
) => {
  const { sort, cursor, pageSize = 50, q, department } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (department) {
    whereConditions.push(eq(subjects.department, department as any));
  }

  // Apply search query filter
  if (q) {
    whereConditions.push(
      sql`(${subjects.name} ILIKE '%' || ${q} || '%' OR ${subjects.code} ILIKE '%' || ${q} || '%')`,
    );
  }

  const query = db
    .select({
      id: subjects.id,
      code: subjects.code,
      name: subjects.name,
      shortName: subjects.shortName,
      department: subjects.department,
      description: subjects.description,
      teacherCount: sql<number>`cast(count(distinct ${teacherSubjects.employeeId}) as int)`,
    })
    .from(subjects)
    .leftJoin(teacherSubjects, eq(teacherSubjects.subjectId, subjects.id))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(subjects.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(subjects.name))
        : query.orderBy(desc(subjects.name));
    } else if (column === "code") {
      isAscending
        ? query.orderBy(asc(subjects.code))
        : query.orderBy(desc(subjects.code));
    } else if (column === "department") {
      isAscending
        ? query.orderBy(asc(subjects.department))
        : query.orderBy(desc(subjects.department));
    }
  } else {
    // Default sort by name ascending
    query.orderBy(asc(subjects.name));
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

export type CreateSubjectParams = {
  code?: string | null;
  name: string;
  shortName?: string | null;
  department?: string | null;
  description?: string | null;
};

export const createSubject = async (
  db: Database,
  params: CreateSubjectParams,
) => {
  const [result] = await db
    .insert(subjects)
    .values(params)
    .returning();

  return result;
};

export type UpdateSubjectParams = {
  id: number;
  code?: string | null;
  name?: string;
  shortName?: string | null;
  department?: string | null;
  description?: string | null;
};

export const updateSubject = async (
  db: Database,
  params: UpdateSubjectParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(subjects)
    .set(rest)
    .where(eq(subjects.id, id))
    .returning();

  return result;
};

export type DeleteSubjectParams = {
  id: number;
};

export const deleteSubject = async (
  db: Database,
  params: DeleteSubjectParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(subjects)
    .where(eq(subjects.id, id))
    .returning();

  return result;
};

// Curriculum Subjects

export type GetCurriculumSubjectsParams = {
  academicYearId: number;
  levelId?: number | null;
  classId?: number | null;
};

export const getCurriculumSubjects = async (
  db: Database,
  params: GetCurriculumSubjectsParams,
) => {
  const { academicYearId, levelId, classId } = params;

  const whereConditions: SQL[] = [
    eq(curriculumSubjects.academicYearId, academicYearId),
  ];

  if (levelId) {
    whereConditions.push(eq(curriculumSubjects.levelId, levelId));
  }

  if (classId) {
    whereConditions.push(eq(curriculumSubjects.classId, classId));
  }

  const data = await db
    .select({
      id: curriculumSubjects.id,
      academicYearId: curriculumSubjects.academicYearId,
      levelId: curriculumSubjects.levelId,
      classId: curriculumSubjects.classId,
      subjectId: curriculumSubjects.subjectId,
      isMandatory: curriculumSubjects.isMandatory,
      weeklyPeriods: curriculumSubjects.weeklyPeriods,
      orderInReport: curriculumSubjects.orderInReport,
      maxScore: curriculumSubjects.maxScore,
      subject: {
        id: subjects.id,
        code: subjects.code,
        name: subjects.name,
        shortName: subjects.shortName,
        department: subjects.department,
      },
      level: {
        id: levels.id,
        name: levels.name,
      },
      class: {
        id: classes.id,
        name: classes.name,
      },
    })
    .from(curriculumSubjects)
    .where(and(...whereConditions))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(levels, eq(levels.id, curriculumSubjects.levelId))
    .leftJoin(classes, eq(classes.id, curriculumSubjects.classId))
    .orderBy(asc(curriculumSubjects.orderInReport));

  return data;
};

export type CreateCurriculumSubjectParams = {
  academicYearId: number;
  levelId?: number | null;
  classId?: number | null;
  subjectId: number;
  isMandatory?: boolean;
  weeklyPeriods?: number | null;
  orderInReport?: number | null;
  maxScore?: number | null;
};

export const createCurriculumSubject = async (
  db: Database,
  params: CreateCurriculumSubjectParams,
) => {
  const { isMandatory = true, maxScore = 100, ...rest } = params;

  const [result] = await db
    .insert(curriculumSubjects)
    .values({
      ...rest,
      isMandatory,
      maxScore,
    })
    .returning();

  return result;
};

export type UpdateCurriculumSubjectParams = {
  id: number;
  isMandatory?: boolean;
  weeklyPeriods?: number | null;
  orderInReport?: number | null;
  maxScore?: number | null;
};

export const updateCurriculumSubject = async (
  db: Database,
  params: UpdateCurriculumSubjectParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(curriculumSubjects)
    .set(rest)
    .where(eq(curriculumSubjects.id, id))
    .returning();

  return result;
};

export type DeleteCurriculumSubjectParams = {
  id: number;
};

export const deleteCurriculumSubject = async (
  db: Database,
  params: DeleteCurriculumSubjectParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(curriculumSubjects)
    .where(eq(curriculumSubjects.id, id))
    .returning();

  return result;
};

// Get subjects by teacher
export type GetSubjectsByTeacherParams = {
  teacherId: string;
};

export const getSubjectsByTeacher = async (
  db: Database,
  params: GetSubjectsByTeacherParams,
) => {
  const { teacherId } = params;

  const data = await db
    .select({
      id: subjects.id,
      code: subjects.code,
      name: subjects.name,
      shortName: subjects.shortName,
      department: subjects.department,
      teacherSubjectId: teacherSubjects.id,
      startYear: teacherSubjects.startYear,
      endYear: teacherSubjects.endYear,
    })
    .from(teacherSubjects)
    .where(eq(teacherSubjects.employeeId, teacherId))
    .innerJoin(subjects, eq(subjects.id, teacherSubjects.subjectId))
    .orderBy(asc(subjects.name));

  return data;
};

// Get teachers by subject
export type GetTeachersBySubjectParams = {
  subjectId: number;
};

export const getTeachersBySubject = async (
  db: Database,
  params: GetTeachersBySubjectParams,
) => {
  const { subjectId } = params;

  const data = await db
    .select({
      userId: employees.userId,
      employeeNumber: employees.employeeNumber,
      name: users.name,
      email: users.email,
      phone: employees.phone,
      teacherSubjectId: teacherSubjects.id,
      startYear: teacherSubjects.startYear,
      endYear: teacherSubjects.endYear,
    })
    .from(teacherSubjects)
    .where(eq(teacherSubjects.subjectId, subjectId))
    .innerJoin(employees, eq(employees.userId, teacherSubjects.employeeId))
    .innerJoin(users, eq(users.id, employees.userId))
    .orderBy(asc(users.name));

  return data;
};
