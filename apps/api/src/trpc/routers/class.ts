import {
  addClassSubjectSchema,
  createClassSchema,
  getClassesByYearAndLevelSchema,
  getClassesSchema,
  updateClassSchema,
} from "@api/schemas/class";
import { getClassAttendanceStatsSchema } from "@api/schemas/attendance";
import { getClassPerformanceSummarySchema } from "@api/schemas/report";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { classCache } from "@midday/cache/class-cache";
import { getClassAttendanceStats } from "@midday/db/queries/attendance";
import {
  addClassSubject,
  createClass,
  deleteClass,
  getClassById,
  getClassSubjects,
  getClasses,
  getClassesByYearAndLevel,
  removeClassSubject,
  updateClass,
} from "@midday/db/queries/class";
import { getClassPerformanceSummary } from "@midday/db/queries/report";
import { z } from "zod";

export const classRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getClassesSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getClasses(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await classCache.getDetails(input.id);
      if (cached) return cached;

      const result = await getClassById(db, { id: input.id });
      if (result) {
        await classCache.setDetails(input.id, result);
      }
      return result;
    }),

  getByYearAndLevel: protectedProcedure
    .input(getClassesByYearAndLevelSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getClassesByYearAndLevel(db, input);
    }),

  create: protectedProcedure
    .input(createClassSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      await classCache.invalidate(input.levelId); // Maybe invalidate level stats
      return createClass(db, input);
    }),

  update: protectedProcedure
    .input(updateClassSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const result = await updateClass(db, { id, ...data });
      await classCache.invalidate(id);
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteClass(db, { id: input.id });
      await classCache.invalidate(input.id);
      return result;
    }),

  // Subjects
  getSubjects: protectedProcedure
    .input(z.object({ classId: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await classCache.getSubjects(input.classId);
      if (cached) return cached;

      const result = await getClassSubjects(db, { classId: input.classId });
      if (result) {
        await classCache.setSubjects(input.classId, result);
      }
      return result;
    }),

  addSubject: protectedProcedure
    .input(addClassSubjectSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await addClassSubject(db, input);
      await classCache.invalidate(input.classId);
      return result;
    }),

  removeSubject: protectedProcedure
    .input(z.object({ id: z.number(), classId: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await removeClassSubject(db, { id: input.id });
      await classCache.invalidate(input.classId);
      return result;
    }),

  // Statistics
  getPerformance: protectedProcedure
    .input(getClassPerformanceSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      return getClassPerformanceSummary(db, input);
    }),

  getAttendanceStats: protectedProcedure
    .input(getClassAttendanceStatsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getClassAttendanceStats(db, input);
    }),
});
