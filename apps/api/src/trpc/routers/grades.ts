import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { 
  getStudentGrades, 
  createGradeEntry, 
  updateGradeEntry, 
  bulkImportGrades,
  batchUpsertGrades,
  getGradeGridData,
  getClassStatistics,
  getCertificateData,
} from "@midday/db/queries";
import { 
  createGradeEntrySchema, 
  updateGradeEntrySchema, 
  bulkImportGradesSchema,
  batchUpsertGradesSchema,
  getGradeGridDataSchema
} from "@api/schemas/grades";
import { getCertificateDataSchema } from "@api/schemas/certificates";
import { z } from "zod";

export const gradesRouter = createTRPCRouter({
  getGridData: protectedProcedure
    .input(getGradeGridDataSchema)
    .query(async ({ input }) => {
      return getGradeGridData(input);
    }),

  getStudentGrades: protectedProcedure
    .input(z.object({ studentId: z.string().uuid(), termId: z.number().optional() }))
    .query(async ({ input }) => {
      return getStudentGrades(input.studentId, input.termId);
    }),

  createEntry: protectedProcedure
    .input(createGradeEntrySchema)
    .mutation(async ({ input, ctx }) => {
      const data = {
          ...input,
          gradedById: input.gradedById || ctx.session.user.id
      };
      return createGradeEntry(data);
    }),

  updateEntry: protectedProcedure
    .input(updateGradeEntrySchema)
    .mutation(async ({ input, ctx }) => {
      const data = {
          ...input,
          gradedById: input.gradedById || ctx.session.user.id
      };
      return updateGradeEntry(input.id, data);
    }),

  batchUpsert: protectedProcedure
    .input(batchUpsertGradesSchema)
    .mutation(async ({ input, ctx }) => {
      const grades = input.grades.map(g => ({
        ...g,
        score: g.score === null ? null : g.score.toString()
      }));
      return batchUpsertGrades(grades, ctx.session.user.id);
    }),

  bulkImport: protectedProcedure
    .input(bulkImportGradesSchema)
    .mutation(async ({ input, ctx }) => {
      return bulkImportGrades(
          input.assessmentItemId, 
          input.gradeData, 
          ctx.session.user.id
      );
    }),

  getClassStatistics: protectedProcedure
    .input(z.object({ classId: z.string() }))
    .query(async ({ input }) => {
      return getClassStatistics(input.classId);
    }),

  getCertificateData: protectedProcedure
    .input(getCertificateDataSchema)
    .query(async ({ input }) => {
      return getCertificateData(input);
    }),
});
