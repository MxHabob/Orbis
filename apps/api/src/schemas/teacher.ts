import { z } from "zod";
import { userSexSchema } from "./student";

/**
 * Staff role enum
 */
export const staffRoleSchema = z.enum([
  "teacher",
  "admin",
  "accountant",
  "nurse",
  "security",
  "other",
]);

/**
 * Schema for creating a teacher
 */
export const createTeacherSchema = z.object({
  userId: z.string().uuid().optional(),
  employeeNumber: z.string().max(50).optional().nullable(),
  hireDate: z.coerce.date().optional().nullable(),
  phone: z.string().max(50).optional().nullable(),
  address: z.string().optional().nullable(),
  sex: userSexSchema,
  birthday: z.coerce.date().optional().nullable(),
  nationality: z.string().max(100).optional().default("Yemeni"),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for updating a teacher
 */
export const updateTeacherSchema = z.object({
  employeeNumber: z.string().max(50).optional().nullable(),
  hireDate: z.coerce.date().optional().nullable(),
  phone: z.string().max(50).optional().nullable(),
  address: z.string().optional().nullable(),
  sex: userSexSchema.optional(),
  birthday: z.coerce.date().optional().nullable(),
  nationality: z.string().max(100).optional(),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for getting teachers
 */
export const getTeachersSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  q: z.string().optional().nullable(),
  sort: z.array(z.string()).length(2).optional(),
  subjectId: z.coerce.number().int().positive().optional().nullable(),
  sex: userSexSchema.optional().nullable(),
});

/**
 * Schema for adding a teacher subject
 */
export const addTeacherSubjectSchema = z.object({
  employeeId: z.string().uuid(),
  subjectId: z.coerce.number().int().positive(),
  startYear: z.coerce.number().int().optional().nullable(),
  endYear: z.coerce.number().int().optional().nullable(),
});

/**
 * Schema for assigning teacher to class
 */
export const assignTeacherToClassSchema = z.object({
  employeeId: z.string().uuid(),
  curriculumSubjectId: z.coerce.number().int().positive(),
  classId: z.coerce.number().int().positive(),
  classSubjectId: z.coerce.number().int().positive().optional().nullable(),
  teacherSubjectId: z.coerce.number().int().positive().optional().nullable(),
  startDate: z.coerce.date().optional().nullable(),
  endDate: z.coerce.date().optional().nullable(),
});

/**
 * Schema for getting teacher class assignments
 */
export const getTeacherClassAssignmentsSchema = z.object({
  teacherId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});
