import {
  createScheduledLessonSchema,
  createTimeSlotSchema,
  updateScheduledLessonSchema,
} from "@api/schemas/lessons";
import {
  adminProcedure,
  createTRPCRouter,
  protectedProcedure,
} from "@api/trpc/init";
import {
  createScheduledLesson,
  createTimeSlot,
  deleteScheduledLesson,
  getClassSchedule,
  getScheduledLessonById,
  getTeacherSchedule,
  getTimeSlots,
  updateScheduledLesson,
} from "@orbis/db/queries";
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

  createTimeSlot: adminProcedure
    .input(createTimeSlotSchema)
    .mutation(async ({ input }) => {
      return createTimeSlot(input);
    }),

  create: adminProcedure
    .input(createScheduledLessonSchema)
    .mutation(async ({ input }) => {
      return createScheduledLesson(input);
    }),

  update: adminProcedure
    .input(updateScheduledLessonSchema)
    .mutation(async ({ input }) => {
      const { id, ...data } = input;
      return updateScheduledLesson(id, data);
    }),

  delete: adminProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteScheduledLesson(input.id);
    }),
});
