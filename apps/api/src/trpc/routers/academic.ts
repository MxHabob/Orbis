import {
  createAcademicTermSchema,
  createAcademicYearSchema,
  updateAcademicTermSchema,
  updateAcademicYearSchema,
} from "@api/schemas/academic";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createAcademicTerm,
  createAcademicYear,
  getAcademicYears,
  getTermsByYear,
  updateAcademicTerm,
  updateAcademicYear,
} from "@midday/db/queries";
import { z } from "zod";

export const academicRouter = createTRPCRouter({
  getYears: protectedProcedure.query(async () => {
    return getAcademicYears();
  }),

  getTerms: protectedProcedure
    .input(z.object({ yearId: z.number() }))
    .query(async ({ input }) => {
      return getTermsByYear(input.yearId);
    }),

  createYear: protectedProcedure
    .input(createAcademicYearSchema)
    .mutation(async ({ input }) => {
      return createAcademicYear(input);
    }),

  updateYear: protectedProcedure
    .input(updateAcademicYearSchema)
    .mutation(async ({ input }) => {
      return updateAcademicYear(input.id, input);
    }),

  createTerm: protectedProcedure
    .input(createAcademicTermSchema)
    .mutation(async ({ input }) => {
      return createAcademicTerm(input);
    }),

  updateTerm: protectedProcedure
    .input(updateAcademicTermSchema)
    .mutation(async ({ input }) => {
      return updateAcademicTerm(input.id, input);
    }),
});
