import { z } from "zod";

/**
 * Section enum
 */
export const sectionSchema = z.enum(["male", "female", "mixed"]);

/**
 * Track enum
 */
export const trackSchema = z.enum(["scientific", "literary", "none"]);

/**
 * Schema for creating a class
 */
export const createClassSchema = z.object({
  name: z.string().min(1).max(100),
  code: z.string().max(50).optional().nullable(),
  division: z.string().max(10).optional().nullable(),
  capacity: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive(),
  levelId: z.coerce.number().int().positive(),
  section: sectionSchema.optional().nullable(),
  track: trackSchema.optional().nullable(),
  supervisorId: z.string().uuid().optional().nullable(),
  current: z.boolean().optional().default(true),
});

/**
 * Schema for updating a class
 */
export const updateClassSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  code: z.string().max(50).optional().nullable(),
  division: z.string().max(10).optional().nullable(),
  capacity: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional(),
  levelId: z.coerce.number().int().positive().optional(),
  section: sectionSchema.optional().nullable(),
  track: trackSchema.optional().nullable(),
  supervisorId: z.string().uuid().optional().nullable(),
  current: z.boolean().optional(),
});

/**
 * Schema for getting classes
 */
export const getClassesSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  q: z.string().optional().nullable(),
  sort: z.array(z.string()).length(2).optional(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  levelId: z.coerce.number().int().positive().optional().nullable(),
  section: sectionSchema.optional().nullable(),
  track: trackSchema.optional().nullable(),
  currentOnly: z.boolean().optional().default(false),
});

/**
 * Schema for adding a class subject
 */
export const addClassSubjectSchema = z.object({
  classId: z.coerce.number().int().positive(),
  curriculumSubjectId: z.coerce.number().int().positive(),
});

/**
 * Schema for getting classes by year and level
 */
export const getClassesByYearAndLevelSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
  levelId: z.coerce.number().int().positive().optional().nullable(),
});
