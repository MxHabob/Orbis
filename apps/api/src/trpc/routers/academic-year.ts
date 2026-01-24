import {
  createAcademicTermSchema,
  createAcademicYearSchema,
  getAcademicYearsSchema,
  getTermsByYearSchema,
  updateAcademicTermSchema,
  updateAcademicYearSchema,
} from "@api/schemas/academic-year";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { academicYearCache } from "@midday/cache/academic-year-cache";
import {
  createAcademicTerm,
  createAcademicYear,
  deleteAcademicTerm,
  deleteAcademicYear,
  getAcademicTermsByYear,
  getAcademicYearById,
  getAcademicYears,
  getCurrentAcademicYear,
  updateAcademicTerm,
  updateAcademicYear,
} from "@midday/db/queries/academic-year";
import { z } from "zod";

export const academicYearRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getAcademicYearsSchema)
    .query(async ({ input, ctx: { db } }) => {
      // Create a cache key based on input params
      const cacheKey = JSON.stringify(input);
      // Try to get from cache (implementation depends on cache strategy)
      // For lists, we might cache based on query params

      return getAcademicYears(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await academicYearCache.getById(input.id);
      if (cached) return cached;

      const result = await getAcademicYearById(db, { id: input.id });
      if (result) {
        await academicYearCache.setById(input.id, result);
      }
      return result;
    }),

  getCurrent: protectedProcedure.query(async ({ ctx: { db } }) => {
    const cached = await academicYearCache.getCurrent();
    if (cached) return cached;

    const result = await getCurrentAcademicYear(db);
    if (result) {
      await academicYearCache.setCurrent(result);
    }
    return result;
  }),

  create: protectedProcedure
    .input(createAcademicYearSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createAcademicYear(db, input);
      if (input.isCurrent) {
        await academicYearCache.invalidateCurrent();
      }
      return result;
    }),

  update: protectedProcedure
    .input(updateAcademicYearSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const result = await updateAcademicYear(db, { id, ...data });
      
      await academicYearCache.invalidate(id);
      if (data.isCurrent) {
        await academicYearCache.invalidateCurrent();
      }
      
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await deleteAcademicYear(db, { id: input.id });
      await academicYearCache.invalidate(input.id);
      await academicYearCache.invalidateCurrent(); // Just in case it was current
      return result;
    }),

  // Terms
  getTerms: protectedProcedure
    .input(getTermsByYearSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getAcademicTermsByYear(db, { academicYearId: input.academicYearId });
    }),

  createTerm: protectedProcedure
    .input(createAcademicTermSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createAcademicTerm(db, input);
      await academicYearCache.invalidate(input.academicYearId); // Invalidate year cache to refresh term counts
      return result;
    }),

  updateTerm: protectedProcedure
    .input(updateAcademicTermSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateAcademicTerm(db, { id, ...data });
    }),

  deleteTerm: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteAcademicTerm(db, { id: input.id });
    }),
});
