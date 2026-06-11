import { z } from "@hono/zod-openapi";

export const timeSlotSchema = z.object({
  id: z.number().openapi({ example: 1 }),
  slotNumber: z.number().openapi({ example: 1 }),
  startTime: z.string().openapi({ example: "08:00:00" }),
  endTime: z.string().openapi({ example: "08:45:00" }),
  durationMinutes: z.number().default(40).openapi({ example: 40 }),
  isBreak: z.boolean().default(false).openapi({ example: false }),
  academicYearId: z.number().openapi({ example: 1 }),
});

export const createTimeSlotSchema = timeSlotSchema.omit({ id: true });

export const scheduledLessonSchema = z.object({
  id: z.number().openapi({ example: 1 }),
  classId: z.number().openapi({ example: 1 }),
  subjectId: z.number().openapi({ example: 1 }),
  teacherId: z
    .string()
    .uuid()
    .openapi({ example: "550e8400-e29b-41d4-a716-446655440000" }),
  timeSlotId: z.number().openapi({ example: 1 }),
  dayOfWeek: z.number().min(0).max(6).openapi({ example: 1 }),
  academicYearId: z.number().openapi({ example: 1 }),
  academicTermId: z.number().nullable().openapi({ example: 1 }),
  roomId: z.number().nullable().openapi({ example: 1 }),
  isActive: z.boolean().default(true).openapi({ example: true }),
  notes: z.string().nullable().openapi({ example: "Lesson notes" }),
});

export const createScheduledLessonSchema = scheduledLessonSchema.omit({
  id: true,
});
export const updateScheduledLessonSchema = scheduledLessonSchema
  .partial()
  .extend({ id: z.number() });
