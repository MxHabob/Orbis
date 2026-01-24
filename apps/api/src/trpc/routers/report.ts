import {
  createReportSchema,
  generateStudentMinisterialReportSchema,
  getClassPerformanceSummarySchema,
  getReportsSchema,
  getSubjectPerformanceAnalysisSchema,
  updateReportSchema,
} from "@api/schemas/report";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import {
  createReport,
  deleteReport,
  generateStudentMinisterialReport,
  getClassPerformanceSummary,
  getReportById,
  getReports,
  getSubjectPerformanceAnalysis,
  updateReport,
} from "@midday/db/queries/report";
import { z } from "zod";

export const reportRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getReportsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getReports(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getReportById(db, { id: input.id });
    }),

  create: protectedProcedure
    .input(createReportSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createReport(db, input);
    }),

  update: protectedProcedure
    .input(updateReportSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateReport(db, { id, ...data });
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      return deleteReport(db, { id: input.id });
    }),

  generateMinisterial: protectedProcedure
    .input(generateStudentMinisterialReportSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return generateStudentMinisterialReport(db, input);
    }),

  // Analytics
  getClassPerformance: protectedProcedure
    .input(getClassPerformanceSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      return getClassPerformanceSummary(db, input);
    }),

  getSubjectAnalysis: protectedProcedure
    .input(getSubjectPerformanceAnalysisSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getSubjectPerformanceAnalysis(db, input);
    }),
});
