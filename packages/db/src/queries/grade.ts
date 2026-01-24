import type { Database } from "@db/client";
import {
  gradeEntries,
  students,
  users,
  assessmentItems,
  assessmentCategories,
  curriculumSubjects,
  subjects,
  academicYears,
  academicTerms,
  employees,
  classes,
} from "@db/schema";
import { and, asc, desc, eq, inArray, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetGradeByIdParams = {
  id: number;
};

export const getGradeById = async (
  db: Database,
  params: GetGradeByIdParams,
) => {
  const [result] = await db
    .select({
      id: gradeEntries.id,
      studentId: gradeEntries.studentId,
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
      isAbsent: gradeEntries.isAbsent,
      comments: gradeEntries.comments,
      gradedAt: gradeEntries.gradedAt,
      gradedById: gradeEntries.gradedById,
      academicYearId: gradeEntries.academicYearId,
      termId: gradeEntries.termId,
      isFinal: gradeEntries.isFinal,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
      assessmentItem: {
        id: assessmentItems.id,
        title: assessmentItems.title,
        type: assessmentItems.type,
        maxScore: assessmentItems.maxScore,
        weight: assessmentItems.weight,
        isMinisterial: assessmentItems.isMinisterial,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
      gradedBy: {
        userId: employees.userId,
        name: sql<string>`${users.name}`,
      },
    })
    .from(gradeEntries)
    .where(eq(gradeEntries.id, params.id))
    .leftJoin(students, eq(students.userId, gradeEntries.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .leftJoin(employees, eq(employees.userId, gradeEntries.gradedById))
    .limit(1);

  return result;
};

export type GetGradesParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  studentId?: string | null;
  assessmentItemId?: number | null;
  academicYearId?: number | null;
  termId?: number | null;
  classId?: number | null;
  subjectId?: number | null;
  isFinal?: boolean | null;
  isMinisterial?: boolean | null;
};

export const getGrades = async (
  db: Database,
  params: GetGradesParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 50,
    studentId,
    assessmentItemId,
    academicYearId,
    termId,
    classId,
    subjectId,
    isFinal,
    isMinisterial,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (studentId) {
    whereConditions.push(eq(gradeEntries.studentId, studentId));
  }

  if (assessmentItemId) {
    whereConditions.push(eq(gradeEntries.assessmentItemId, assessmentItemId));
  }

  if (academicYearId) {
    whereConditions.push(eq(gradeEntries.academicYearId, academicYearId));
  }

  if (termId) {
    whereConditions.push(eq(gradeEntries.termId, termId));
  }

  if (classId) {
    whereConditions.push(eq(students.currentClassId, classId));
  }

  if (subjectId) {
    whereConditions.push(eq(curriculumSubjects.subjectId, subjectId));
  }

  if (isFinal !== null && isFinal !== undefined) {
    whereConditions.push(eq(gradeEntries.isFinal, isFinal));
  }

  if (isMinisterial !== null && isMinisterial !== undefined) {
    whereConditions.push(eq(assessmentItems.isMinisterial, isMinisterial));
  }

  const query = db
    .select({
      id: gradeEntries.id,
      studentId: gradeEntries.studentId,
      assessmentItemId: gradeEntries.assessmentItemId,
      score: gradeEntries.score,
      isAbsent: gradeEntries.isAbsent,
      comments: gradeEntries.comments,
      gradedAt: gradeEntries.gradedAt,
      isFinal: gradeEntries.isFinal,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
      assessmentItem: {
        id: assessmentItems.id,
        title: assessmentItems.title,
        type: assessmentItems.type,
        maxScore: assessmentItems.maxScore,
      },
      subject: {
        id: subjects.id,
        name: subjects.name,
        code: subjects.code,
      },
    })
    .from(gradeEntries)
    .leftJoin(students, eq(students.userId, gradeEntries.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "score") {
      isAscending
        ? query.orderBy(asc(gradeEntries.score))
        : query.orderBy(desc(gradeEntries.score));
    } else if (column === "graded_at") {
      isAscending
        ? query.orderBy(asc(gradeEntries.gradedAt))
        : query.orderBy(desc(gradeEntries.gradedAt));
    } else if (column === "student_number") {
      isAscending
        ? query.orderBy(asc(students.studentNumber))
        : query.orderBy(desc(students.studentNumber));
    }
  } else {
    // Default sort by graded_at descending
    query.orderBy(desc(gradeEntries.gradedAt));
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

export type CreateGradeEntryParams = {
  studentId: string;
  assessmentItemId: number;
  score?: number | null;
  isAbsent?: boolean;
  comments?: string | null;
  gradedById?: string | null;
  academicYearId: number;
  termId: number;
  isFinal?: boolean;
};

export const createGradeEntry = async (
  db: Database,
  params: CreateGradeEntryParams,
) => {
  const { isAbsent = false, isFinal = false, ...rest } = params;

  const [result] = await db
    .insert(gradeEntries)
    .values({
      ...rest,
      isAbsent,
      isFinal,
    })
    .returning();

  return result;
};

export type UpdateGradeEntryParams = {
  id: number;
  score?: number | null;
  isAbsent?: boolean;
  comments?: string | null;
  gradedById?: string | null;
  isFinal?: boolean;
};

export const updateGradeEntry = async (
  db: Database,
  params: UpdateGradeEntryParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(gradeEntries)
    .set(rest)
    .where(eq(gradeEntries.id, id))
    .returning();

  return result;
};

export type DeleteGradeEntryParams = {
  id: number;
};

export const deleteGradeEntry = async (
  db: Database,
  params: DeleteGradeEntryParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(gradeEntries)
    .where(eq(gradeEntries.id, id))
    .returning();

  return result;
};

// Bulk grade entry for assessment
export type BulkCreateGradeEntriesParams = {
  grades: Array<{
    studentId: string;
    assessmentItemId: number;
    score?: number | null;
    isAbsent?: boolean;
    comments?: string | null;
  }>;
  gradedById: string;
  academicYearId: number;
  termId: number;
};

export const bulkCreateGradeEntries = async (
  db: Database,
  params: BulkCreateGradeEntriesParams,
) => {
  const { grades, gradedById, academicYearId, termId } = params;

  const values = grades.map((grade) => ({
    ...grade,
    gradedById,
    academicYearId,
    termId,
    isAbsent: grade.isAbsent ?? false,
    isFinal: false,
  }));

  const results = await db.insert(gradeEntries).values(values).returning();

  return results;
};

// Get student grades summary
export type GetStudentGradesSummaryParams = {
  studentId: string;
  academicYearId: number;
  termId?: number | null;
};

export const getStudentGradesSummary = async (
  db: Database,
  params: GetStudentGradesSummaryParams,
) => {
  const { studentId, academicYearId, termId } = params;

  const whereConditions: SQL[] = [
    eq(gradeEntries.studentId, studentId),
    eq(gradeEntries.academicYearId, academicYearId),
  ];

  if (termId) {
    whereConditions.push(eq(gradeEntries.termId, termId));
  }

  const data = await db
    .select({
      subjectId: subjects.id,
      subjectName: subjects.name,
      subjectCode: subjects.code,
      totalScore: sql<number>`coalesce(sum(${gradeEntries.score}), 0)`,
      maxScore: sql<number>`coalesce(sum(${assessmentItems.maxScore}), 0)`,
      average: sql<number>`
        case 
          when sum(${assessmentItems.maxScore}) > 0 
          then (coalesce(sum(${gradeEntries.score}), 0) / sum(${assessmentItems.maxScore})) * 100
          else 0
        end
      `,
      gradeCount: sql<number>`cast(count(${gradeEntries.id}) as int)`,
      absentCount: sql<number>`cast(sum(case when ${gradeEntries.isAbsent} then 1 else 0 end) as int)`,
    })
    .from(gradeEntries)
    .where(and(...whereConditions))
    .leftJoin(assessmentItems, eq(assessmentItems.id, gradeEntries.assessmentItemId))
    .leftJoin(curriculumSubjects, eq(curriculumSubjects.id, assessmentItems.curriculumSubjectId))
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .groupBy(subjects.id, subjects.name, subjects.code)
    .orderBy(asc(subjects.name));

  return data;
};

// Get class grades for assessment
export type GetClassGradesForAssessmentParams = {
  assessmentItemId: number;
  classId: number;
};

export const getClassGradesForAssessment = async (
  db: Database,
  params: GetClassGradesForAssessmentParams,
) => {
  const { assessmentItemId, classId } = params;

  const data = await db
    .select({
      gradeId: gradeEntries.id,
      studentId: students.userId,
      studentNumber: students.studentNumber,
      studentName: users.name,
      surname: students.surname,
      score: gradeEntries.score,
      isAbsent: gradeEntries.isAbsent,
      comments: gradeEntries.comments,
      gradedAt: gradeEntries.gradedAt,
    })
    .from(students)
    .where(eq(students.currentClassId, classId))
    .leftJoin(users, eq(users.id, students.userId))
    .leftJoin(
      gradeEntries,
      and(
        eq(gradeEntries.studentId, students.userId),
        eq(gradeEntries.assessmentItemId, assessmentItemId),
      ),
    )
    .orderBy(asc(students.studentNumber));

  return data;
};
