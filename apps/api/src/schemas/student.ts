import { z } from "zod";

/**
 * Student status enum
 */
export const studentStatusSchema = z.enum([
  "active",
  "graduated",
  "transferred",
  "suspended",
  "dropped",
  "repeating",
]);

/**
 * User sex enum
 */
export const userSexSchema = z.enum(["male", "female"]);

/**
 * Schema for creating a student
 */
export const createStudentSchema = z.object({
  userId: z.string().uuid().optional(),
  studentNumber: z.string().min(1).max(50),
  currentClassId: z.coerce.number().int().positive().optional().nullable(),
  currentAcademicYearId: z.coerce.number().int().positive().optional().nullable(),
  enrollmentDate: z.coerce.date().optional().nullable(),
  status: studentStatusSchema.optional().default("active"),
  surname: z.string().min(1).max(255),
  phone: z.string().max(50).optional().nullable(),
  address: z.string().optional().nullable(),
  sex: userSexSchema,
  birthday: z.coerce.date(),
  nationality: z.string().max(100).optional().default("Yemeni"),
  religion: z.string().max(50).optional().default("Muslim"),
  parentId: z.string().uuid().optional().nullable(),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for updating a student
 */
export const updateStudentSchema = z.object({
  studentNumber: z.string().min(1).max(50).optional(),
  currentClassId: z.coerce.number().int().positive().optional().nullable(),
  currentAcademicYearId: z.coerce.number().int().positive().optional().nullable(),
  enrollmentDate: z.coerce.date().optional().nullable(),
  status: studentStatusSchema.optional(),
  surname: z.string().min(1).max(255).optional(),
  phone: z.string().max(50).optional().nullable(),
  address: z.string().optional().nullable(),
  sex: userSexSchema.optional(),
  birthday: z.coerce.date().optional(),
  nationality: z.string().max(100).optional(),
  religion: z.string().max(50).optional(),
  parentId: z.string().uuid().optional().nullable(),
  // User fields
  name: z.string().min(1).max(255).optional().nullable(),
  email: z.string().email().optional().nullable(),
  image: z.string().url().optional().nullable(),
});

/**
 * Schema for getting students
 */
export const getStudentsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  q: z.string().optional().nullable(),
  sort: z.array(z.string()).length(2).optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  status: studentStatusSchema.optional().nullable(),
  sex: userSexSchema.optional().nullable(),
});

/**
 * Schema for creating a student enrollment
 */
export const createStudentEnrollmentSchema = z.object({
  studentId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive(),
  classId: z.coerce.number().int().positive(),
  status: z.string().max(100).optional().nullable(),
});

/**
 * Schema for updating a student enrollment
 */
export const updateStudentEnrollmentSchema = z.object({
  status: z.string().max(100).optional().nullable(),
  finalAverage: z.coerce.number().min(0).max(100).optional().nullable(),
  ministerialTotalScore: z.coerce.number().min(0).optional().nullable(),
  ministerialAverage: z.coerce.number().min(0).max(100).optional().nullable(),
  promotionDecision: z
    .enum(["passed", "failed", "repeat", "promoted", "supplementary"])
    .optional()
    .nullable(),
});

/**
 * Schema for getting students by class
 */
export const getStudentsByClassSchema = z.object({
  classId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});
