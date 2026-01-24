import { z } from "zod";

/**
 * Schema for creating an academic year
 */
export const createAcademicYearSchema = z.object({
  year: z.string().min(1).max(20),
  startDate: z.coerce.date(),
  endDate: z.coerce.date().optional().nullable(),
  isCurrent: z.boolean().optional().default(false),
  isActive: z.boolean().optional().default(true),
});

/**
 * Schema for updating an academic year
 */
export const updateAcademicYearSchema = z.object({
  year: z.string().min(1).max(20).optional(),
  startDate: z.coerce.date().optional(),
  endDate: z.coerce.date().optional().nullable(),
  isCurrent: z.boolean().optional(),
  isActive: z.boolean().optional(),
});

/**
 * Schema for getting academic years
 */
export const getAcademicYearsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  activeOnly: z.boolean().optional().default(false),
});

/**
 * Schema for creating an academic term
 */
export const createAcademicTermSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
  name: z.string().min(1).max(100),
  startDate: z.coerce.date(),
  endDate: z.coerce.date().optional().nullable(),
  order: z.coerce.number().int().positive(),
});

/**
 * Schema for updating an academic term
 */
export const updateAcademicTermSchema = z.object({
  name: z.string().min(1).max(100).optional(),
  startDate: z.coerce.date().optional(),
  endDate: z.coerce.date().optional().nullable(),
  order: z.coerce.number().int().positive().optional(),
});

/**
 * Schema for getting terms by academic year
 */
export const getTermsByYearSchema = z.object({
  academicYearId: z.coerce.number().int().positive(),
});
