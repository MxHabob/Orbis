import type { Database } from "@db/client";
import {
  employees,
  users,
  employeeRoles,
  teacherSubjects,
  subjects,
  teacherClassSubjects,
  classes,
  curriculumSubjects,
  lessons,
} from "@db/schema";
import { and, asc, desc, eq, inArray, sql } from "drizzle-orm";
import type { SQL } from "drizzle-orm/sql/sql";

type GetTeacherByIdParams = {
  userId: string;
};

export const getTeacherById = async (
  db: Database,
  params: GetTeacherByIdParams,
) => {
  const [result] = await db
    .select({
      userId: employees.userId,
      employeeNumber: employees.employeeNumber,
      hireDate: employees.hireDate,
      phone: employees.phone,
      address: employees.address,
      sex: employees.sex,
      birthday: employees.birthday,
      nationality: employees.nationality,
      name: users.name,
      email: users.email,
      image: users.image,
      role: users.role,
      createdAt: users.createdAt,
      subjectCount: sql<number>`cast(count(distinct ${teacherSubjects.id}) as int)`,
      classCount: sql<number>`cast(count(distinct ${teacherClassSubjects.classId}) as int)`,
      lessonCount: sql<number>`cast(count(distinct ${lessons.id}) as int)`,
    })
    .from(employees)
    .where(eq(employees.userId, params.userId))
    .innerJoin(users, eq(users.id, employees.userId))
    .leftJoin(teacherSubjects, eq(teacherSubjects.employeeId, employees.userId))
    .leftJoin(teacherClassSubjects, eq(teacherClassSubjects.employeeId, employees.userId))
    .leftJoin(lessons, eq(lessons.teacherId, employees.userId))
    .groupBy(employees.userId, users.id);

  return result;
};

export type GetTeachersParams = {
  cursor?: string | null;
  pageSize?: number;
  q?: string | null;
  sort?: string[] | null;
  subjectId?: number | null;
  sex?: "male" | "female" | null;
};

export const getTeachers = async (
  db: Database,
  params: GetTeachersParams,
) => {
  const { sort, cursor, pageSize = 25, q, subjectId, sex } = params;

  const whereConditions: SQL[] = [];

  // Filter only teachers
  whereConditions.push(
    sql`exists (
      select 1 from ${employeeRoles} 
      where ${employeeRoles.employeeId} = ${employees.userId} 
      and ${employeeRoles.role} = 'teacher'
    )`,
  );

  // Apply filters
  if (subjectId) {
    whereConditions.push(
      sql`exists (
        select 1 from ${teacherSubjects} 
        where ${teacherSubjects.employeeId} = ${employees.userId} 
        and ${teacherSubjects.subjectId} = ${subjectId}
      )`,
    );
  }

  if (sex) {
    whereConditions.push(eq(employees.sex, sex));
  }

  // Apply search query filter
  if (q) {
    whereConditions.push(
      sql`(${users.name} ILIKE '%' || ${q} || '%' OR ${employees.employeeNumber} ILIKE '%' || ${q} || '%')`,
    );
  }

  const query = db
    .select({
      userId: employees.userId,
      employeeNumber: employees.employeeNumber,
      hireDate: employees.hireDate,
      phone: employees.phone,
      sex: employees.sex,
      name: users.name,
      email: users.email,
      image: users.image,
      createdAt: users.createdAt,
      subjectCount: sql<number>`cast(count(distinct ${teacherSubjects.id}) as int)`,
      classCount: sql<number>`cast(count(distinct ${teacherClassSubjects.classId}) as int)`,
    })
    .from(employees)
    .innerJoin(users, eq(users.id, employees.userId))
    .leftJoin(teacherSubjects, eq(teacherSubjects.employeeId, employees.userId))
    .leftJoin(teacherClassSubjects, eq(teacherClassSubjects.employeeId, employees.userId))
    .where(whereConditions.length > 0 ? and(...whereConditions) : undefined)
    .groupBy(employees.userId, users.id);

  // Apply sorting
  if (sort && sort.length === 2) {
    const [column, direction] = sort;
    const isAscending = direction === "asc";

    if (column === "name") {
      isAscending
        ? query.orderBy(asc(users.name))
        : query.orderBy(desc(users.name));
    } else if (column === "employee_number") {
      isAscending
        ? query.orderBy(asc(employees.employeeNumber))
        : query.orderBy(desc(employees.employeeNumber));
    } else if (column === "hire_date") {
      isAscending
        ? query.orderBy(asc(employees.hireDate))
        : query.orderBy(desc(employees.hireDate));
    } else if (column === "created_at") {
      isAscending
        ? query.orderBy(asc(users.createdAt))
        : query.orderBy(desc(users.createdAt));
    }
  } else {
    // Default sort by name ascending
    query.orderBy(asc(users.name));
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

export type CreateTeacherParams = {
  userId?: string;
  employeeNumber?: string | null;
  hireDate?: Date | null;
  phone?: string | null;
  address?: string | null;
  sex: "male" | "female";
  birthday?: Date | null;
  nationality?: string;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const createTeacher = async (
  db: Database,
  params: CreateTeacherParams,
) => {
  const {
    userId,
    name,
    email,
    image,
    employeeNumber,
    hireDate,
    phone,
    address,
    sex,
    birthday,
    nationality = "Yemeni",
  } = params;

  // Create user first
  const [user] = await db
    .insert(users)
    .values({
      id: userId,
      name,
      email,
      image,
      role: "admin", // Teachers are admins in the system
    })
    .returning();

  // Create employee
  const [employee] = await db
    .insert(employees)
    .values({
      userId: user.id,
      employeeNumber,
      hireDate,
      phone,
      address,
      sex,
      birthday,
      nationality,
    })
    .returning();

  // Add teacher role
  await db.insert(employeeRoles).values({
    employeeId: user.id,
    role: "teacher",
    startDate: hireDate,
  });

  return { ...employee, name: user.name, email: user.email, image: user.image };
};

export type UpdateTeacherParams = {
  userId: string;
  employeeNumber?: string | null;
  hireDate?: Date | null;
  phone?: string | null;
  address?: string | null;
  sex?: "male" | "female";
  birthday?: Date | null;
  nationality?: string;
  // User fields
  name?: string | null;
  email?: string | null;
  image?: string | null;
};

export const updateTeacher = async (
  db: Database,
  params: UpdateTeacherParams,
) => {
  const { userId, name, email, image, ...employeeData } = params;

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

  // Update employee
  const [result] = await db
    .update(employees)
    .set(employeeData)
    .where(eq(employees.userId, userId))
    .returning();

  return result;
};

export type DeleteTeacherParams = {
  userId: string;
};

export const deleteTeacher = async (
  db: Database,
  params: DeleteTeacherParams,
) => {
  const { userId } = params;

  // Delete user (cascade will delete employee and roles)
  const [result] = await db
    .delete(users)
    .where(eq(users.id, userId))
    .returning();

  return result;
};

// Teacher Subjects

export type GetTeacherSubjectsParams = {
  teacherId: string;
};

export const getTeacherSubjects = async (
  db: Database,
  params: GetTeacherSubjectsParams,
) => {
  const { teacherId } = params;

  const data = await db
    .select({
      id: teacherSubjects.id,
      employeeId: teacherSubjects.employeeId,
      subjectId: teacherSubjects.subjectId,
      startYear: teacherSubjects.startYear,
      endYear: teacherSubjects.endYear,
      subject: {
        id: subjects.id,
        code: subjects.code,
        name: subjects.name,
        shortName: subjects.shortName,
        department: subjects.department,
      },
    })
    .from(teacherSubjects)
    .where(eq(teacherSubjects.employeeId, teacherId))
    .leftJoin(subjects, eq(subjects.id, teacherSubjects.subjectId))
    .orderBy(asc(subjects.name));

  return data;
};

export type AddTeacherSubjectParams = {
  employeeId: string;
  subjectId: number;
  startYear?: number | null;
  endYear?: number | null;
};

export const addTeacherSubject = async (
  db: Database,
  params: AddTeacherSubjectParams,
) => {
  const [result] = await db
    .insert(teacherSubjects)
    .values(params)
    .returning();

  return result;
};

export type RemoveTeacherSubjectParams = {
  id: number;
};

export const removeTeacherSubject = async (
  db: Database,
  params: RemoveTeacherSubjectParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(teacherSubjects)
    .where(eq(teacherSubjects.id, id))
    .returning();

  return result;
};

// Teacher Class Assignments

export type GetTeacherClassAssignmentsParams = {
  teacherId: string;
  academicYearId?: number | null;
};

export const getTeacherClassAssignments = async (
  db: Database,
  params: GetTeacherClassAssignmentsParams,
) => {
  const { teacherId, academicYearId } = params;

  const whereConditions: SQL[] = [
    eq(teacherClassSubjects.employeeId, teacherId),
  ];

  if (academicYearId) {
    whereConditions.push(eq(curriculumSubjects.academicYearId, academicYearId));
  }

  const data = await db
    .select({
      id: teacherClassSubjects.id,
      employeeId: teacherClassSubjects.employeeId,
      curriculumSubjectId: teacherClassSubjects.curriculumSubjectId,
      classId: teacherClassSubjects.classId,
      startDate: teacherClassSubjects.startDate,
      endDate: teacherClassSubjects.endDate,
      class: {
        id: classes.id,
        name: classes.name,
        code: classes.code,
      },
      subject: {
        id: subjects.id,
        code: subjects.code,
        name: subjects.name,
        shortName: subjects.shortName,
      },
      curriculumSubject: {
        id: curriculumSubjects.id,
        weeklyPeriods: curriculumSubjects.weeklyPeriods,
      },
    })
    .from(teacherClassSubjects)
    .where(and(...whereConditions))
    .leftJoin(classes, eq(classes.id, teacherClassSubjects.classId))
    .leftJoin(
      curriculumSubjects,
      eq(curriculumSubjects.id, teacherClassSubjects.curriculumSubjectId),
    )
    .leftJoin(subjects, eq(subjects.id, curriculumSubjects.subjectId))
    .orderBy(asc(classes.name), asc(subjects.name));

  return data;
};

export type AssignTeacherToClassParams = {
  employeeId: string;
  curriculumSubjectId: number;
  classId: number;
  classSubjectId?: number | null;
  teacherSubjectId?: number | null;
  startDate?: Date | null;
  endDate?: Date | null;
};

export const assignTeacherToClass = async (
  db: Database,
  params: AssignTeacherToClassParams,
) => {
  const [result] = await db
    .insert(teacherClassSubjects)
    .values(params)
    .returning();

  return result;
};

export type UnassignTeacherFromClassParams = {
  id: number;
};

export const unassignTeacherFromClass = async (
  db: Database,
  params: UnassignTeacherFromClassParams,
) => {
  const { id } = params;

  const [result] = await db
    .delete(teacherClassSubjects)
    .where(eq(teacherClassSubjects.id, id))
    .returning();

  return result;
};
