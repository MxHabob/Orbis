import { z } from "zod";

/**
 * Schema for creating an attendance record
 */
export const createAttendanceSchema = z.object({
  date: z.coerce.date(),
  present: z.boolean(),
  studentId: z.string().uuid(),
  lessonId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for updating an attendance record
 */
export const updateAttendanceSchema = z.object({
  date: z.coerce.date().optional(),
  present: z.boolean().optional(),
  lessonId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting attendance records
 */
export const getAttendancesSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(50),
  sort: z.array(z.string()).length(2).optional(),
  studentId: z.string().uuid().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  lessonId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  dateFrom: z.coerce.date().optional().nullable(),
  dateTo: z.coerce.date().optional().nullable(),
  present: z.boolean().optional().nullable(),
});

/**
 * Schema for bulk creating attendance records
 */
export const bulkCreateAttendancesSchema = z.object({
  attendances: z.array(
    z.object({
      date: z.coerce.date(),
      present: z.boolean(),
      studentId: z.string().uuid(),
      lessonId: z.coerce.number().int().positive().optional().nullable(),
      termId: z.coerce.number().int().positive().optional().nullable(),
      academicYearId: z.coerce.number().int().positive().optional().nullable(),
    }),
  ),
});

/**
 * Schema for getting student attendance summary
 */
export const getStudentAttendanceSummarySchema = z.object({
  studentId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  dateFrom: z.coerce.date().optional().nullable(),
  dateTo: z.coerce.date().optional().nullable(),
});

/**
 * Schema for getting class attendance for a specific date
 */
export const getClassAttendanceForDateSchema = z.object({
  classId: z.coerce.number().int().positive(),
  date: z.coerce.date(),
  lessonId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting class attendance statistics
 */
export const getClassAttendanceStatsSchema = z.object({
  classId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  dateFrom: z.coerce.date().optional().nullable(),
  dateTo: z.coerce.date().optional().nullable(),
});
