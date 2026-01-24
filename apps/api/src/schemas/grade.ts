import { z } from "zod";

/**
 * Schema for creating a grade entry
 */
export const createGradeEntrySchema = z.object({
  studentId: z.string().uuid(),
  assessmentItemId: z.coerce.number().int().positive(),
  score: z.coerce.number().min(0).optional().nullable(),
  isAbsent: z.boolean().optional().default(false),
  comments: z.string().optional().nullable(),
  gradedById: z.string().uuid().optional().nullable(),
  academicYearId: z.coerce.number().int().positive(),
  termId: z.coerce.number().int().positive(),
  isFinal: z.boolean().optional().default(false),
});

/**
 * Schema for updating a grade entry
 */
export const updateGradeEntrySchema = z.object({
  score: z.coerce.number().min(0).optional().nullable(),
  isAbsent: z.boolean().optional(),
  comments: z.string().optional().nullable(),
  gradedById: z.string().uuid().optional().nullable(),
  isFinal: z.boolean().optional(),
});

/**
 * Schema for getting grades
 */
export const getGradesSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(50),
  sort: z.array(z.string()).length(2).optional(),
  studentId: z.string().uuid().optional().nullable(),
  assessmentItemId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  subjectId: z.coerce.number().int().positive().optional().nullable(),
  isFinal: z.boolean().optional().nullable(),
  isMinisterial: z.boolean().optional().nullable(),
});

/**
 * Schema for bulk creating grade entries
 */
export const bulkCreateGradeEntriesSchema = z.object({
  grades: z.array(
    z.object({
      studentId: z.string().uuid(),
      assessmentItemId: z.coerce.number().int().positive(),
      score: z.coerce.number().min(0).optional().nullable(),
      isAbsent: z.boolean().optional().default(false),
      comments: z.string().optional().nullable(),
    }),
  ),
  gradedById: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive(),
  termId: z.coerce.number().int().positive(),
});

/**
 * Schema for getting student grades summary
 */
export const getStudentGradesSummarySchema = z.object({
  studentId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive(),
  termId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting class grades for assessment
 */
export const getClassGradesForAssessmentSchema = z.object({
  assessmentItemId: z.coerce.number().int().positive(),
  classId: z.coerce.number().int().positive(),
});
