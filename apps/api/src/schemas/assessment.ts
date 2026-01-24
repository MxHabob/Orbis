import { z } from "zod";

/**
 * Assessment type enum
 */
export const assessmentTypeSchema = z.enum([
  "exam",
  "quiz",
  "assignment",
  "project",
  "participation",
  "oral",
  "practical",
  "ministerial",
]);

/**
 * Schema for creating an assessment category
 */
export const createAssessmentCategorySchema = z.object({
  name: z.string().min(1).max(100),
  weight: z.coerce.number().min(0),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  curriculumSubjectId: z.coerce.number().int().positive().optional().nullable(),
  order: z.coerce.number().int().optional().nullable(),
});

/**
 * Schema for updating an assessment category
 */
export const updateAssessmentCategorySchema = z.object({
  name: z.string().min(1).max(100).optional(),
  weight: z.coerce.number().min(0).optional(),
  order: z.coerce.number().int().optional().nullable(),
});

/**
 * Schema for getting assessment categories
 */
export const getAssessmentCategoriesSchema = z.object({
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  curriculumSubjectId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for creating an assessment item
 */
export const createAssessmentItemSchema = z.object({
  title: z.string().min(1).max(255),
  type: assessmentTypeSchema,
  maxScore: z.coerce.number().min(0).optional().default(100),
  weight: z.coerce.number().min(0).optional().nullable(),
  dueDate: z.coerce.date().optional().nullable(),
  assessmentCategoryId: z.coerce.number().int().positive().optional().nullable(),
  curriculumSubjectId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  isMinisterial: z.boolean().optional().default(false),
});

/**
 * Schema for updating an assessment item
 */
export const updateAssessmentItemSchema = z.object({
  title: z.string().min(1).max(255).optional(),
  type: assessmentTypeSchema.optional(),
  maxScore: z.coerce.number().min(0).optional(),
  weight: z.coerce.number().min(0).optional().nullable(),
  dueDate: z.coerce.date().optional().nullable(),
  assessmentCategoryId: z.coerce.number().int().positive().optional().nullable(),
  isMinisterial: z.boolean().optional(),
});

/**
 * Schema for getting assessment items
 */
export const getAssessmentItemsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  curriculumSubjectId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  assessmentCategoryId: z.coerce.number().int().positive().optional().nullable(),
  type: assessmentTypeSchema.optional().nullable(),
  isMinisterial: z.boolean().optional().nullable(),
});
