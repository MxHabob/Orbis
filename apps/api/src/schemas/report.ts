import { z } from "zod";

/**
 * Report type enum
 */
export const reportTypeSchema = z.enum([
  "ministerial_grade9",
  "ministerial_grade12",
]);

/**
 * Schema for creating a report
 */
export const createReportSchema = z.object({
  title: z.string().min(1).max(255),
  reportDate: z.coerce.date(),
  content: z.string(),
  generatedBy: z.string().max(255),
  levelId: z.coerce.number().int().positive().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  studentId: z.string().uuid().optional().nullable(),
  totalScore: z.coerce.number().min(0).optional().nullable(),
  average: z.coerce.number().min(0).max(100).optional().nullable(),
  passStatus: z.string().max(100).optional().nullable(),
  reportType: reportTypeSchema.optional().nullable(),
});

/**
 * Schema for updating a report
 */
export const updateReportSchema = z.object({
  title: z.string().min(1).max(255).optional(),
  content: z.string().optional(),
  totalScore: z.coerce.number().min(0).optional().nullable(),
  average: z.coerce.number().min(0).max(100).optional().nullable(),
  passStatus: z.string().max(100).optional().nullable(),
});

/**
 * Schema for getting reports
 */
export const getReportsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  levelId: z.coerce.number().int().positive().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  studentId: z.string().uuid().optional().nullable(),
  reportType: reportTypeSchema.optional().nullable(),
});

/**
 * Schema for generating a student ministerial report
 */
export const generateStudentMinisterialReportSchema = z.object({
  studentId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive(),
  reportType: reportTypeSchema,
  generatedBy: z.string().max(255),
});

/**
 * Schema for getting class performance summary
 */
export const getClassPerformanceSummarySchema = z.object({
  classId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive(),
});

/**
 * Schema for getting subject performance analysis
 */
export const getSubjectPerformanceAnalysisSchema = z.object({
  classId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive(),
});
