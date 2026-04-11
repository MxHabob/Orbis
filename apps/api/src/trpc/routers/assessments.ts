import {
  createAssessmentItemSchema,
  createGradeEntrySchema,
  updateAssessmentItemSchema,
  updateGradeEntrySchema,
} from "@api/schemas/assessments";
import { createTRPCRouter, protectedProcedure, adminProcedure, teacherProcedure } from "@api/trpc/init";
import {
  createAssessment,
  createGradeEntry,
  deleteAssessment,
  getAssessmentsForClass,
  getStudentGrades,
  updateAssessment,
  updateGradeEntry,
} from "@midday/db/queries";
import { z } from "zod";

export const assessmentsRouter = createTRPCRouter({
  // Assessments
  getByClass: protectedProcedure
    .input(
      z.object({
        classId: z.number(),
        subjectId: z.number().optional(),
        termId: z.number().optional(),
        month: z.number().optional(),
      })
    )
    .query(async ({ input }) => {
      return getAssessmentsForClass(
        input.classId,
        input.subjectId,
        input.termId,
        input.month
      );
    }),

  create: adminProcedure
    .input(createAssessmentItemSchema)
    .mutation(async ({ input }) => {
      return createAssessment({
        ...input,
        maxScore: String(input.maxScore),
        weight: input.weight !== null ? String(input.weight) : null,
      });
    }),

  update: adminProcedure
    .input(updateAssessmentItemSchema)
    .mutation(async ({ input }) => {
      const { id, ...data } = input;
      return updateAssessment(id, {
        ...data,
        maxScore:
          data.maxScore !== undefined ? String(data.maxScore) : undefined,
        weight:
          data.weight !== undefined && data.weight !== null
            ? String(data.weight)
            : data.weight,
      });
    }),

  delete: adminProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input }) => {
      return deleteAssessment(input.id);
    }),

  // Grades
  getGrades: protectedProcedure
    .input(
      z.object({
        studentId: z.string().uuid(),
        termId: z.number().optional(),
      })
    )
    .query(async ({ input }) => {
      return getStudentGrades(input.studentId, input.termId);
    }),

  addGrade: teacherProcedure
    .input(createGradeEntrySchema)
    .mutation(async ({ input }) => {
      return createGradeEntry({
        ...input,
        score: input.score !== null ? String(input.score) : null,
      });
    }),

  updateGrade: teacherProcedure
    .input(updateGradeEntrySchema)
    .mutation(async ({ input }) => {
      const { id, ...data } = input;
      return updateGradeEntry(id, {
        ...data,
        score:
          data.score !== undefined && data.score !== null
            ? String(data.score)
            : data.score,
      });
    }),

  importGrades: teacherProcedure
    .input(z.object({
      assessmentItemId: z.number(),
      month: z.number().optional(),
      identifierType: z.enum(["studentNumber", "secretNumber"]).default("studentNumber"),
      grades: z.array(z.object({
        identifier: z.string(),
        score: z.number().nullable(),
        isAbsent: z.boolean().optional(),
        comments: z.string().optional(),
      }))
    }))
    .mutation(async ({ input, ctx }) => {
      const { bulkImportGrades, bulkImportGradesBySecretNumber } = await import("@midday/db/queries");
      
      if (input.identifierType === "secretNumber") {
        return bulkImportGradesBySecretNumber(
          input.assessmentItemId,
          input.grades.map(g => ({ ...g, secretNumber: g.identifier })),
          ctx.session.user.id
        );
      }

      return bulkImportGrades(
        input.assessmentItemId, 
        input.grades.map(g => ({ ...g, studentNumber: g.identifier })), 
        ctx.session.user.id
      );
    }),
});

