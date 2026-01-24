import { z } from "zod";

/**
 * Day enum
 */
export const daySchema = z.enum([
  "monday",
  "tuesday",
  "wednesday",
  "thursday",
  "friday",
  "saturday",
  "sunday",
]);

/**
 * Schema for creating a lesson
 */
export const createLessonSchema = z.object({
  name: z.string().max(255).optional().nullable(),
  day: daySchema,
  startTime: z.coerce.date(),
  endTime: z.coerce.date(),
  teacherClassSubjectId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  teacherId: z.string().uuid().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for updating a lesson
 */
export const updateLessonSchema = z.object({
  name: z.string().max(255).optional().nullable(),
  day: daySchema.optional(),
  startTime: z.coerce.date().optional(),
  endTime: z.coerce.date().optional(),
  teacherClassSubjectId: z.coerce.number().int().positive().optional(),
});

/**
 * Schema for getting lessons
 */
export const getLessonsSchema = z.object({
  cursor: z.string().optional().nullable(),
  pageSize: z.coerce.number().min(1).max(100).optional().default(25),
  sort: z.array(z.string()).length(2).optional(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  teacherId: z.string().uuid().optional().nullable(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
  day: daySchema.optional().nullable(),
});

/**
 * Schema for getting class schedule
 */
export const getClassScheduleSchema = z.object({
  classId: z.coerce.number().int().positive(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for getting teacher schedule
 */
export const getTeacherScheduleSchema = z.object({
  teacherId: z.string().uuid(),
  academicYearId: z.coerce.number().int().positive().optional().nullable(),
});

/**
 * Schema for bulk creating lessons
 */
export const bulkCreateLessonsSchema = z.object({
  lessons: z.array(
    z.object({
      name: z.string().max(255).optional().nullable(),
      day: daySchema,
      startTime: z.coerce.date(),
      endTime: z.coerce.date(),
      teacherClassSubjectId: z.coerce.number().int().positive(),
      academicYearId: z.coerce.number().int().positive().optional().nullable(),
      teacherId: z.string().uuid().optional().nullable(),
      classId: z.coerce.number().int().positive().optional().nullable(),
    }),
  ),
});

/**
 * Schema for getting schedule conflicts
 */
export const getScheduleConflictsSchema = z.object({
  teacherId: z.string().uuid().optional().nullable(),
  classId: z.coerce.number().int().positive().optional().nullable(),
  day: daySchema,
  startTime: z.coerce.date(),
  endTime: z.coerce.date(),
  excludeLessonId: z.coerce.number().int().positive().optional().nullable(),
});
