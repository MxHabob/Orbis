import {
  bulkCreateLessonsSchema,
  createLessonSchema,
  getClassScheduleSchema,
  getLessonsSchema,
  getScheduleConflictsSchema,
  updateLessonSchema,
} from "@api/schemas/lesson";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { lessonCache } from "@midday/cache/lesson-cache";
import {
  bulkCreateLessons,
  createLesson,
  deleteLesson,
  getClassSchedule,
  getLessonById,
  getLessons,
  getScheduleConflicts,
  updateLesson,
} from "@midday/db/queries/lesson";
import { z } from "zod";

export const lessonRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getLessonsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getLessons(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await lessonCache.getById(input.id);
      if (cached) return cached;

      const result = await getLessonById(db, { id: input.id });
      if (result) {
        await lessonCache.setById(input.id, result);
      }
      return result;
    }),

  getClassSchedule: protectedProcedure
    .input(getClassScheduleSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await lessonCache.getClassSchedule(input.classId, input.academicYearId ?? undefined);
      if (cached) return cached;

      const result = await getClassSchedule(db, input);
      if (result) {
        await lessonCache.setClassSchedule(input.classId, result, input.academicYearId ?? undefined);
      }
      return result;
    }),

  getConflicts: protectedProcedure
    .input(getScheduleConflictsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getScheduleConflicts(db, input);
    }),

  create: protectedProcedure
    .input(createLessonSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      // Check conflicts first? Maybe on frontend or optional here
      const result = await createLesson(db, input);
      if (input.classId) {
        await lessonCache.invalidateClassSchedule(input.classId);
      }
      if (input.teacherId) {
        await lessonCache.invalidateTeacherSchedule(input.teacherId);
      }
      return result;
    }),

  update: protectedProcedure
    .input(updateLessonSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const original = await getLessonById(db, { id });
      
      const result = await updateLesson(db, { id, ...data });
      
      await lessonCache.invalidate(id);
      if (original) {
        if (original.classId) await lessonCache.invalidateClassSchedule(original.classId);
        if (original.teacherId) await lessonCache.invalidateTeacherSchedule(original.teacherId);
      }
      
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const original = await getLessonById(db, { id: input.id });
      const result = await deleteLesson(db, { id: input.id });
      
      await lessonCache.invalidate(input.id);
      if (original) {
        if (original.classId) await lessonCache.invalidateClassSchedule(original.classId);
        if (original.teacherId) await lessonCache.invalidateTeacherSchedule(original.teacherId);
      }
      return result;
    }),

  bulkCreate: protectedProcedure
    .input(bulkCreateLessonsSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await bulkCreateLessons(db, input);
      
      const classIds = new Set(input.lessons.map(l => l.classId).filter(Boolean) as number[]);
      const teacherIds = new Set(input.lessons.map(l => l.teacherId).filter(Boolean) as string[]);
      
      await Promise.all([
        ...Array.from(classIds).map(id => lessonCache.invalidateClassSchedule(id)),
        ...Array.from(teacherIds).map(id => lessonCache.invalidateTeacherSchedule(id))
      ]);
      
      return result;
    }),
});
