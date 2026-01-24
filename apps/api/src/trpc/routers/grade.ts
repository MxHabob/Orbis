import {
  bulkCreateGradeEntriesSchema,
  createGradeEntrySchema,
  getClassGradesForAssessmentSchema,
  getGradesSchema,
  updateGradeEntrySchema,
} from "@api/schemas/grade";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { gradeCache } from "@midday/cache/grade-cache";
import {
  bulkCreateGradeEntries,
  createGradeEntry,
  deleteGradeEntry,
  getClassGradesForAssessment,
  getGradeById,
  getGrades,
  updateGradeEntry,
} from "@midday/db/queries/grade";
import { z } from "zod";

export const gradeRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getGradesSchema)
    .query(async ({ input, ctx: { db } }) => {
      // Optional: Cache based on common filters like class grades
      return getGrades(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getGradeById(db, { id: input.id });
    }),

  getClassGrades: protectedProcedure
    .input(getClassGradesForAssessmentSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await gradeCache.getClassGrades(input.classId, input.assessmentItemId);
      if (cached) return cached;

      const result = await getClassGradesForAssessment(db, input);
      if (result) {
        await gradeCache.setClassGrades(input.classId, input.assessmentItemId, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createGradeEntrySchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createGradeEntry(db, input);
      await gradeCache.invalidateStudent(input.studentId);
      await gradeCache.invalidateAssessment(input.assessmentItemId);
      return result;
    }),

  update: protectedProcedure
    .input(updateGradeEntrySchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const original = await getGradeById(db, { id });
      
      const result = await updateGradeEntry(db, { id, ...data });
      
      if (original) {
        await gradeCache.invalidateStudent(original.studentId);
        await gradeCache.invalidateAssessment(original.assessmentItemId);
      }
      
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const original = await getGradeById(db, { id: input.id });
      const result = await deleteGradeEntry(db, { id: input.id });
      
      if (original) {
        await gradeCache.invalidateStudent(original.studentId);
        await gradeCache.invalidateAssessment(original.assessmentItemId);
      }
      
      return result;
    }),

  bulkCreate: protectedProcedure
    .input(bulkCreateGradeEntriesSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await bulkCreateGradeEntries(db, input);
      // Invalidate cache for all affected students (potentially expensive, better to invalidate assessment cache)
      const assessmentItemId = input.grades[0]?.assessmentItemId;
      if (assessmentItemId) {
        await gradeCache.invalidateAssessment(assessmentItemId);
      }
      return result;
    }),
});
