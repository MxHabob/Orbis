import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { 
  getTimeSlots, 
  getClassSchedule, 
  getTeacherSchedule, 
  createTimeSlot, 
  createScheduledLesson, 
  deleteScheduledLesson 
} from "@midday/db/queries";
import { 
  createTimeSlotSchema, 
  createScheduledLessonSchema 
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

  createTimeSlot: protectedProcedure
    .input(createTimeSlotSchema)
    .mutation(async ({ input }) => {
      return createTimeSlot(input);
    }),

  createScheduledLesson: protectedProcedure
    .input(createScheduledLessonSchema)
    .mutation(async ({ input }) => {
      return createScheduledLesson(input);
    }),

  deleteScheduledLesson: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteScheduledLesson(input.id);
    }),
});
