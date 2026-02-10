import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { 
  getTimeSlots, 
  getClassSchedule, 
  getTeacherSchedule, 
  createTimeSlot, 
  createScheduledLesson, 
  updateScheduledLesson,
  deleteScheduledLesson,
  getScheduledLessonById
} from "@midday/db/queries";

import { 
  createTimeSlotSchema, 
  createScheduledLessonSchema,
  updateScheduledLessonSchema 
} from "@api/schemas/lessons";
import { z } from "zod";

export const lessonsRouter = createTRPCRouter({
  getTimeSlots: protectedProcedure
    .input(z.object({ yearId: z.number() }))
    .query(async ({ input }) => {
      return getTimeSlots(input.yearId);
    }),

  getClassSchedule: protectedProcedure
    .input(z.object({ classId: z.number(), yearId: z.number() }))
    .query(async ({ input }) => {
      return getClassSchedule(input.classId, input.yearId);
    }),

  getTeacherSchedule: protectedProcedure
    .input(z.object({ teacherId: z.string().uuid(), yearId: z.number() }))
    .query(async ({ input }) => {
      return getTeacherSchedule(input.teacherId, input.yearId);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input }) => {
      return getScheduledLessonById(input.id);
    }),

  createTimeSlot: protectedProcedure
    .input(createTimeSlotSchema)
    .mutation(async ({ input }) => {
      return createTimeSlot(input);
    }),

  create: protectedProcedure
    .input(createScheduledLessonSchema)
    .mutation(async ({ input }) => {
      return createScheduledLesson(input);
    }),

  update: protectedProcedure
    .input(updateScheduledLessonSchema)
    .mutation(async ({ input }) => {
      const { id, ...data } = input;
      return updateScheduledLesson(id, data);
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteScheduledLesson(input.id);
    }),
});

