import { z } from "zod";

/**
 * Department enum
 */
export const departmentSchema = z.enum([
  "sciences",
  "literary",
  "mathematics",
  "languages",
  "religious",
  "social",
  "other",
]);

/**
 * Schema for creating a subject
 */
export const createSubjectSchema = z.object({
  name: z.string().min(1).max(255),
  code: z.string().max(50).optional().nullable(),
  shortName: z.string().max(50).optional().nullable(),
  department: departmentSchema.optional().nullable(),
  description: z.string().optional().nullable(),
});

/**
 * Schema for updating a subject
 */
export const updateSubjectSchema = z.object({
  name: z.string().min(1).max(255).optional(),
  code: z.string().max(50).optional().nullable(),
  shortName: z.string().max(50).optional().nullable(),
  department: departmentSchema.optional().nullable(),
  description: z.string().optional().nullable(),
});

/**
 * Schema for getting subjects
 */
export const getSubjectsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  q: z.string().optional().nullable(),
  sort: z.array(z.string()).length(2).optional(),
  department: departmentSchema.optional().nullable(),
});

/**
 * Schema for creating a curriculum subject
 */
export const createCurriculumSubjectSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
  levelId: z.coerce.number().int().positive().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  subjectId: z.coerce.number().int().positive(),
  isMandatory: z.boolean().optional().default(true),
  weeklyPeriods: z.coerce.number().min(0).optional().nullable(),
  orderInReport: z.coerce.number().int().optional().nullable(),
  maxScore: z.coerce.number().min(0).optional().default(100),
});

/**
 * Schema for updating a curriculum subject
 */
export const updateCurriculumSubjectSchema = z.object({
  isMandatory: z.boolean().optional(),
  weeklyPeriods: z.coerce.number().min(0).optional().nullable(),
  orderInReport: z.coerce.number().int().optional().nullable(),
  maxScore: z.coerce.number().min(0).optional().nullable(),
});

/**
 * Schema for getting curriculum subjects
 */
export const getCurriculumSubjectsSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
  levelId: z.coerce.number().int().positive().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
});
