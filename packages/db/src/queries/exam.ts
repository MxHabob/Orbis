import type { Database } from "@db/client";
import {
  exams,
  examCommittees,
  examSeatings,
  academicYears,
  academicTerms,
  students,
  users,
  classes,
} from "@db/schema";
import { and, asc, desc, eq, inArray, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetExamByIdParams = {
  id: number;
};

export const getExamById = async (
  db: Database,
  params: GetExamByIdParams,
) => {
  const [result] = await db
    .select({
      id: exams.id,
      title: exams.title,
      startTime: exams.startTime,
      endTime: exams.endTime,
      type: exams.type,
      termId: exams.termId,
      academicYearId: exams.academicYearId,
      term: {
        id: academicTerms.id,
        name: academicTerms.name,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      committeeCount: sql<number>`cast(count(distinct ${examCommittees.id}) as int)`,
      studentCount: sql<number>`cast(count(distinct ${examSeatings.id}) as int)`,
    })
    .from(exams)
    .where(eq(exams.id, params.id))
    .leftJoin(academicTerms, eq(academicTerms.id, exams.termId))
    .leftJoin(academicYears, eq(academicYears.id, exams.academicYearId))
    .leftJoin(examCommittees, eq(examCommittees.examId, exams.id))
    .leftJoin(examSeatings, eq(examSeatings.committeeId, examCommittees.id))
    .groupBy(exams.id, academicTerms.id, academicYears.id);

  return result;
};

export type GetExamsParams = {
  cursor?: string | null;
  pageSize?: number;
  sort?: string[] | null;
  academicYearId?: number | null;
  termId?: number | null;
  type?: string | null;
};

export const getExams = async (
  db: Database,
  params: GetExamsParams,
) => {
  const { sort, cursor, pageSize = 25, academicYearId, termId, type } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (academicYearId) {
    whereConditions.push(eq(exams.academicYearId, academicYearId));
  }

  if (termId) {
    whereConditions.push(eq(exams.termId, termId));
  }

  if (type) {
    whereConditions.push(eq(exams.type, type as any));
  }

  const query = db
    .select({
      id: exams.id,
      title: exams.title,
      startTime: exams.startTime,
      endTime: exams.endTime,
      type: exams.type,
      termId: exams.termId,
      academicYearId: exams.academicYearId,
      term: {
        id: academicTerms.id,
        name: academicTerms.name,
      },
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      committeeCount: sql<number>`cast(count(distinct ${examCommittees.id}) as int)`,
      studentCount: sql<number>`cast(count(distinct ${examSeatings.id}) as int)`,
    })
    .from(exams)
    .leftJoin(academicTerms, eq(academicTerms.id, exams.termId))
    .leftJoin(academicYears, eq(academicYears.id, exams.academicYearId))
    .leftJoin(examCommittees, eq(examCommittees.examId, exams.id))
    .leftJoin(examSeatings, eq(examSeatings.committeeId, examCommittees.id))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(exams.id, academicTerms.id, academicYears.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "title") {
      isAscending
        ? query.orderBy(asc(exams.title))
        : query.orderBy(desc(exams.title));
    } else if (column === "start_time") {
      isAscending
        ? query.orderBy(asc(exams.startTime))
        : query.orderBy(desc(exams.startTime));
    } else if (column === "type") {
      isAscending
        ? query.orderBy(asc(exams.type))
        : query.orderBy(desc(exams.type));
    }
  } else {
    // Default sort by start_time descending
    query.orderBy(desc(exams.startTime));
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

export type CreateExamParams = {
  title: string;
  startTime: Date;
  endTime: Date;
  type: string;
  termId?: number | null;
  academicYearId?: number | null;
};

export const createExam = async (
  db: Database,
  params: CreateExamParams,
) => {
  const [result] = await db
    .insert(exams)
    .values(params)
    .returning();

  return result;
};

export type UpdateExamParams = {
  id: number;
  title?: string;
  startTime?: Date;
  endTime?: Date;
  type?: string;
  termId?: number | null;
  academicYearId?: number | null;
};

export const updateExam = async (
  db: Database,
  params: UpdateExamParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(exams)
    .set(rest)
    .where(eq(exams.id, id))
    .returning();

  return result;
};

export type DeleteExamParams = {
  id: number;
};

export const deleteExam = async (
  db: Database,
  params: DeleteExamParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(exams)
    .where(eq(exams.id, id))
    .returning();

  return result;
};

// Exam Committees

export type GetExamCommitteesParams = {
  examId: number;
};

export const getExamCommittees = async (
  db: Database,
  params: GetExamCommitteesParams,
) => {
  const { examId } = params;

  const data = await db
    .select({
      id: examCommittees.id,
      name: examCommittees.name,
      examId: examCommittees.examId,
      capacity: examCommittees.capacity,
      location: examCommittees.location,
      classId: examCommittees.classId,
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      studentCount: sql<number>`cast(count(${examSeatings.id}) as int)`,
    })
    .from(examCommittees)
    .where(eq(examCommittees.examId, examId))
    .leftJoin(classes, eq(classes.id, examCommittees.classId))
    .leftJoin(examSeatings, eq(examSeatings.committeeId, examCommittees.id))
    .groupBy(examCommittees.id, classes.id)
    .orderBy(asc(examCommittees.name));

  return data;
};

export type CreateExamCommitteeParams = {
  name: string;
  examId: number;
  capacity?: number | null;
  location?: string | null;
  classId?: number | null;
};

export const createExamCommittee = async (
  db: Database,
  params: CreateExamCommitteeParams,
) => {
  const [result] = await db
    .insert(examCommittees)
    .values(params)
    .returning();

  return result;
};

export type UpdateExamCommitteeParams = {
  id: number;
  name?: string;
  capacity?: number | null;
  location?: string | null;
  classId?: number | null;
};

export const updateExamCommittee = async (
  db: Database,
  params: UpdateExamCommitteeParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(examCommittees)
    .set(rest)
    .where(eq(examCommittees.id, id))
    .returning();

  return result;
};

export type DeleteExamCommitteeParams = {
  id: number;
};

export const deleteExamCommittee = async (
  db: Database,
  params: DeleteExamCommitteeParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(examCommittees)
    .where(eq(examCommittees.id, id))
    .returning();

  return result;
};

// Exam Seatings

export type GetExamSeatingsParams = {
  committeeId: number;
};

export const getExamSeatings = async (
  db: Database,
  params: GetExamSeatingsParams,
) => {
  const { committeeId } = params;

  const data = await db
    .select({
      id: examSeatings.id,
      studentId: examSeatings.studentId,
      committeeId: examSeatings.committeeId,
      seatingNumber: examSeatings.seatingNumber,
      assignedAt: examSeatings.assignedAt,
      student: {
        userId: students.userId,
        studentNumber: students.studentNumber,
        surname: students.surname,
        name: users.name,
      },
    })
    .from(examSeatings)
    .where(eq(examSeatings.committeeId, committeeId))
    .leftJoin(students, eq(students.userId, examSeatings.studentId))
    .leftJoin(users, eq(users.id, students.userId))
    .orderBy(asc(examSeatings.seatingNumber));

  return data;
};

export type CreateExamSeatingParams = {
  studentId: string;
  committeeId: number;
  seatingNumber: string;
};

export const createExamSeating = async (
  db: Database,
  params: CreateExamSeatingParams,
) => {
  const [result] = await db
    .insert(examSeatings)
    .values(params)
    .returning();

  return result;
};

export type BulkCreateExamSeatingsParams = {
  seatings: Array<{
    studentId: string;
    committeeId: number;
    seatingNumber: string;
  }>;
};

export const bulkCreateExamSeatings = async (
  db: Database,
  params: BulkCreateExamSeatingsParams,
) => {
  const { seatings } = params;

  const results = await db
    .insert(examSeatings)
    .values(seatings)
    .returning();

  return results;
};

export type DeleteExamSeatingParams = {
  id: number;
};

export const deleteExamSeating = async (
  db: Database,
  params: DeleteExamSeatingParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(examSeatings)
    .where(eq(examSeatings.id, id))
    .returning();

  return result;
};

// Get student exam seating
export type GetStudentExamSeatingParams = {
  studentId: string;
  examId: number;
};

export const getStudentExamSeating = async (
  db: Database,
  params: GetStudentExamSeatingParams,
) => {
  const { studentId, examId } = params;

  const [result] = await db
    .select({
      id: examSeatings.id,
      seatingNumber: examSeatings.seatingNumber,
      assignedAt: examSeatings.assignedAt,
      committee: {
        id: examCommittees.id,
        name: examCommittees.name,
        location: examCommittees.location,
      },
      exam: {
        id: exams.id,
        title: exams.title,
        startTime: exams.startTime,
        endTime: exams.endTime,
      },
    })
    .from(examSeatings)
    .where(eq(examSeatings.studentId, studentId))
    .leftJoin(examCommittees, eq(examCommittees.id, examSeatings.committeeId))
    .leftJoin(exams, eq(exams.id, examCommittees.examId))
    .where(and(
      eq(examSeatings.studentId, studentId),
      eq(exams.id, examId),
    ))
    .limit(1);

  return result;
};
