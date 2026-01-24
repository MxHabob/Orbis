import type { Database } from "@db/client";
import {
  students,
  users,
  classes,
  academicYears,
  parents,
  studentEnrollments,
  gradeEntries,
  attendances,
  payments,
} from "@db/schema";
import { buildSearchQuery } from "@midday/db/utils/search-query";
import { and, asc, desc, eq, inArray, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetStudentByIdParams = {
  userId: string;
};

export const getStudentById = async (
  db: Database,
  params: GetStudentByIdParams,
) => {
  const [result] = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      currentClassId: students.currentClassId,
      currentAcademicYearId: students.currentAcademicYearId,
      enrollmentDate: students.enrollmentDate,
      status: students.status,
      surname: students.surname,
      phone: students.phone,
      address: students.address,
      sex: students.sex,
      birthday: students.birthday,
      nationality: students.nationality,
      religion: students.religion,
      parentId: students.parentId,
      // User info
      name: users.name,
      email: users.email,
      image: users.image,
      role: users.role,
      createdAt: users.createdAt,
      // Current class info
      currentClass: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
        division: classes.division,
      },
      // Current academic year
      currentAcademicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      // Parent info
      parent: {
        userId: parents.userId,
        surname: parents.surname,
        phone: parents.phone,
      },
      // Stats
      enrollmentCount: sql<number>`cast(count(distinct ${studentEnrollments.id}) as int)`,
      gradeCount: sql<number>`cast(count(distinct ${gradeEntries.id}) as int)`,
      attendanceCount: sql<number>`cast(count(distinct ${attendances.id}) as int)`,
      paymentCount: sql<number>`cast(count(distinct ${payments.id}) as int)`,
    })
    .from(students)
    .where(eq(students.userId, params.userId))
    .innerJoin(users, eq(users.id, students.userId))
    .leftJoin(classes, eq(classes.id, students.currentClassId))
    .leftJoin(academicYears, eq(academicYears.id, students.currentAcademicYearId))
    .leftJoin(parents, eq(parents.userId, students.parentId))
    .leftJoin(studentEnrollments, eq(studentEnrollments.studentId, students.userId))
    .leftJoin(gradeEntries, eq(gradeEntries.studentId, students.userId))
    .leftJoin(attendances, eq(attendances.studentId, students.userId))
    .leftJoin(payments, eq(payments.studentId, students.userId))
    .groupBy(
      students.userId,
      users.id,
      classes.id,
      academicYears.id,
      parents.userId,
    );

  return result;
};

type GetStudentByNumberParams = {
  studentNumber: string;
};

export const getStudentByNumber = async (
  db: Database,
  params: GetStudentByNumberParams,
) => {
  const [result] = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      currentClassId: students.currentClassId,
      currentAcademicYearId: students.currentAcademicYearId,
      enrollmentDate: students.enrollmentDate,
      status: students.status,
      surname: students.surname,
      phone: students.phone,
      address: students.address,
      sex: students.sex,
      birthday: students.birthday,
      nationality: students.nationality,
      religion: students.religion,
      parentId: students.parentId,
      name: users.name,
      email: users.email,
      image: users.image,
    })
    .from(students)
    .where(eq(students.studentNumber, params.studentNumber))
    .innerJoin(users, eq(users.id, students.userId))
    .limit(1);

  return result;
};

export type GetStudentsParams = {
  cursor?: string | null;
  pageSize?: number;
  q?: string | null;
  sort?: string[] | null;
  classId?: number | null;
  academicYearId?: number | null;
  status?: string | null;
  sex?: "male" | "female" | null;
};

export const getStudents = async (
  db: Database,
  params: GetStudentsParams,
) => {
  const {
    sort,
    cursor,
    pageSize = 25,
    q,
    classId,
    academicYearId,
    status,
    sex,
  } = params;

  const whereConditions: SQL[] = [];

  // Apply filters
  if (classId) {
    whereConditions.push(eq(students.currentClassId, classId));
  }

  if (academicYearId) {
    whereConditions.push(eq(students.currentAcademicYearId, academicYearId));
  }

  if (status) {
    whereConditions.push(eq(students.status, status as any));
  }

  if (sex) {
    whereConditions.push(eq(students.sex, sex));
  }

  // Apply search query filter
  if (q) {
    if (!Number.isNaN(Number.parseInt(q))) {
      // Search by student number
      whereConditions.push(
        sql`${students.studentNumber} ILIKE '%' || ${q} || '%'`,
      );
    } else {
      // Search by name or surname
      whereConditions.push(
        sql`(${users.name} ILIKE '%' || ${q} || '%' OR ${students.surname} ILIKE '%' || ${q} || '%')`,
      );
    }
  }

  const query = db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      currentClassId: students.currentClassId,
      currentAcademicYearId: students.currentAcademicYearId,
      enrollmentDate: students.enrollmentDate,
      status: students.status,
      surname: students.surname,
      phone: students.phone,
      address: students.address,
      sex: students.sex,
      birthday: students.birthday,
      nationality: students.nationality,
      religion: students.religion,
      parentId: students.parentId,
      name: users.name,
      email: users.email,
      image: users.image,
      createdAt: users.createdAt,
      currentClass: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      currentAcademicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      gradeCount: sql<number>`cast(count(distinct ${gradeEntries.id}) as int)`,
      attendanceCount: sql<number>`cast(count(distinct ${attendances.id}) as int)`,
    })
    .from(students)
    .innerJoin(users, eq(users.id, students.userId))
    .leftJoin(classes, eq(classes.id, students.currentClassId))
    .leftJoin(academicYears, eq(academicYears.id, students.currentAcademicYearId))
    .leftJoin(gradeEntries, eq(gradeEntries.studentId, students.userId))
    .leftJoin(attendances, eq(attendances.studentId, students.userId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(
      students.userId,
      users.id,
      classes.id,
      academicYears.id,
    );

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(users.name))
        : query.orderBy(desc(users.name));
    } else if (column === "student_number") {
      isAscending
        ? query.orderBy(asc(students.studentNumber))
        : query.orderBy(desc(students.studentNumber));
    } else if (column === "created_at") {
      isAscending
        ? query.orderBy(asc(users.createdAt))
        : query.orderBy(desc(users.createdAt));
    } else if (column === "enrollment_date") {
      isAscending
        ? query.orderBy(asc(students.enrollmentDate))
        : query.orderBy(desc(students.enrollmentDate));
    }
  } else {
    // Default sort by created_at descending
    query.orderBy(desc(users.createdAt));
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

export type CreateStudentParams = {
  userId?: string;
  studentNumber: string;
  currentClassId?: number | null;
  currentAcademicYearId?: number | null;
  enrollmentDate?: Date | null;
  status?: string;
  surname: string;
  phone?: string | null;
  address?: string | null;
  sex: "male" | "female";
  birthday: Date;
  nationality?: string;
  religion?: string;
  parentId?: string | null;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const createStudent = async (
  db: Database,
  params: CreateStudentParams,
) => {
  const {
    userId,
    name,
    email,
    image,
    studentNumber,
    currentClassId,
    currentAcademicYearId,
    enrollmentDate,
    status = "active",
    surname,
    phone,
    address,
    sex,
    birthday,
    nationality = "Yemeni",
    religion = "Muslim",
    parentId,
  } = params;

  // Create user first
  const [user] = await db
    .insert(users)
    .values({
      id: userId,
      name,
      email,
      image,
      role: "student",
    })
    .returning();

  // Create student
  const [student] = await db
    .insert(students)
    .values({
      userId: user.id,
      studentNumber,
      currentClassId,
      currentAcademicYearId,
      enrollmentDate,
      status: status as any,
      surname,
      phone,
      address,
      sex,
      birthday,
      nationality,
      religion,
      parentId,
    })
    .returning();

  return { ...student, name: user.name, email: user.email, image: user.image };
};

export type UpdateStudentParams = {
  userId: string;
  studentNumber?: string;
  currentClassId?: number | null;
  currentAcademicYearId?: number | null;
  enrollmentDate?: Date | null;
  status?: string;
  surname?: string;
  phone?: string | null;
  address?: string | null;
  sex?: "male" | "female";
  birthday?: Date;
  nationality?: string;
  religion?: string;
  parentId?: string | null;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const updateStudent = async (
  db: Database,
  params: UpdateStudentParams,
) => {
  const { userId, name, email, image, ...studentData } = params;

  // Update user if user fields provided
  if (name !== undefined || email !== undefined || image !== undefined) {
    await db
      .update(users)
      .set({
        name,
        email,
        image,
        updatedAt: new Date(),
      })
      .where(eq(users.id, userId));
  }

  // Update student
  const [result] = await db
    .update(students)
    .set(studentData)
    .where(eq(students.userId, userId))
    .returning();

  return result;
};

export type DeleteStudentParams = {
  userId: string;
};

export const deleteStudent = async (
  db: Database,
  params: DeleteStudentParams,
) => {
  const { userId } = params;

  // Delete user (cascade will delete student)
  const [result] = await db
    .delete(users)
    .where(eq(users.id, userId))
    .returning();

  return result;
};

// Student Enrollments

export type GetStudentEnrollmentsParams = {
  studentId: string;
};

export const getStudentEnrollments = async (
  db: Database,
  params: GetStudentEnrollmentsParams,
) => {
  const { studentId } = params;

  const data = await db
    .select({
      id: studentEnrollments.id,
      studentId: studentEnrollments.studentId,
      academicYearId: studentEnrollments.academicYearId,
      classId: studentEnrollments.classId,
      status: studentEnrollments.status,
      finalAverage: studentEnrollments.finalAverage,
      ministerialTotalScore: studentEnrollments.ministerialTotalScore,
      ministerialAverage: studentEnrollments.ministerialAverage,
      promotionDecision: studentEnrollments.promotionDecision,
      academicYear: {
        id: academicYears.id,
        year: academicYears.year,
      },
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
    })
    .from(studentEnrollments)
    .where(eq(studentEnrollments.studentId, studentId))
    .leftJoin(academicYears, eq(academicYears.id, studentEnrollments.academicYearId))
    .leftJoin(classes, eq(classes.id, studentEnrollments.classId))
    .orderBy(desc(academicYears.startDate));

  return data;
};

export type CreateStudentEnrollmentParams = {
  studentId: string;
  academicYearId: number;
  classId: number;
  status?: string | null;
};

export const createStudentEnrollment = async (
  db: Database,
  params: CreateStudentEnrollmentParams,
) => {
  const [result] = await db
    .insert(studentEnrollments)
    .values(params)
    .returning();

  return result;
};

export type UpdateStudentEnrollmentParams = {
  id: number;
  status?: string | null;
  finalAverage?: number | null;
  ministerialTotalScore?: number | null;
  ministerialAverage?: number | null;
  promotionDecision?: string | null;
};

export const updateStudentEnrollment = async (
  db: Database,
  params: UpdateStudentEnrollmentParams,
) => {
  const { id, ...rest } = params;

  const [result] = await db
    .update(studentEnrollments)
    .set(rest)
    .where(eq(studentEnrollments.id, id))
    .returning();

  return result;
};

// Get students by class
export type GetStudentsByClassParams = {
  classId: number;
  academicYearId?: number | null;
};

export const getStudentsByClass = async (
  db: Database,
  params: GetStudentsByClassParams,
) => {
  const { classId, academicYearId } = params;

  const whereConditions: SQL[] = [eq(students.currentClassId, classId)];

  if (academicYearId) {
    whereConditions.push(eq(students.currentAcademicYearId, academicYearId));
  }

  const data = await db
    .select({
      userId: students.userId,
      studentNumber: students.studentNumber,
      surname: students.surname,
      sex: students.sex,
      status: students.status,
      name: users.name,
      email: users.email,
      image: users.image,
    })
    .from(students)
    .innerJoin(users, eq(users.id, students.userId))
    .where(and(...whereConditions))
    .orderBy(asc(students.studentNumber));

  return data;
};
