import { z } from "zod";

/**
 * Exam type enum
 */
export const examTypeSchema = z.enum([
  "monthly",
  "semester1",
  "semester2",
  "final_year",
  "ministerial",
]);

/**
 * Schema for creating an exam
 */
export const createExamSchema = z.object({
  title: z.string().min(1).max(255),
  startTime: z.coerce.date(),
  endTime: z.coerce.date(),
  type: examTypeSchema,
  termId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for updating an exam
 */
export const updateExamSchema = z.object({
  title: z.string().min(1).max(255).optional(),
  startTime: z.coerce.date().optional(),
  endTime: z.coerce.date().optional(),
  type: examTypeSchema.optional(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting exams
 */
export const getExamsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  termId: z.coerce.number().int().positive().optional().nullable(),
  type: examTypeSchema.optional().nullable(),
});

/**
 * Schema for creating an exam committee
 */
export const createExamCommitteeSchema = z.object({
  name: z.string().min(1).max(255),
  examId: z.coerce.number().int().positive(),
  capacity: z.coerce.number().int().positive().optional().nullable(),
  location: z.string().max(255).optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for updating an exam committee
 */
export const updateExamCommitteeSchema = z.object({
  name: z.string().min(1).max(255).optional(),
  capacity: z.coerce.number().int().positive().optional().nullable(),
  location: z.string().max(255).optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for creating an exam seating
 */
export const createExamSeatingSchema = z.object({
  studentId: z.string().uuid(),
  committeeId: z.coerce.number().int().positive(),
  seatingNumber: z.string().min(1).max(50),
});

/**
 * Schema for bulk creating exam seatings
 */
export const bulkCreateExamSeatingsSchema = z.object({
  seatings: z.array(
    z.object({
      studentId: z.string().uuid(),
      committeeId: z.coerce.number().int().positive(),
      seatingNumber: z.string().min(1).max(50),
    }),
  ),
});

/**
 * Schema for getting student exam seating
 */
export const getStudentExamSeatingSchema = z.object({
  studentId: z.string().uuid(),
  examId: z.coerce.number().int().positive(),
});
