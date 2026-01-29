import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { 
  getStudentGrades, 
  createGradeEntry, 
  updateGradeEntry, 
  bulkImportGrades 
} from "@midday/db/queries";
import { 
  createGradeEntrySchema, 
  updateGradeEntrySchema, 
  bulkImportGradesSchema 
} from "@api/schemas/grades";
import { z } from "zod";

export const gradesRouter = createTRPCRouter({
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

  bulkImport: protectedProcedure
    .input(bulkImportGradesSchema)
    .mutation(async ({ input, ctx }) => {
      return bulkImportGrades(
          input.assessmentItemId, 
          input.gradeData, 
          ctx.session.user.id
      );
    }),
});
