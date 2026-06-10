import { createTRPCRouter, protectedProcedure, teacherProcedure } from "@api/trpc/init";
import { 
  getStudentGrades, 
  createGradeEntry, 
  updateGradeEntry, 
  bulkImportGrades,
  batchUpsertGrades,
  getGradeGridData,
  getClassStatistics,
  getCertificateData,
  getStudentReportData,
  getTopStudents,
} from "@orbis/db/queries";
import {
  createGradeEntrySchema,
  updateGradeEntrySchema,
  bulkImportGradesSchema,
  batchUpsertGradesSchema,
  getGradeGridDataSchema,
  getTopStudentsSchema,
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

  getReportData: protectedProcedure
    .input(z.object({ 
      studentId: z.string().uuid(),
      type: z.string().optional(),
      month: z.number().optional(),
      classId: z.number().optional(),
    }))
    .query(async ({ input }) => {
      return getStudentReportData(input.studentId, input.type, input.month, input.classId);
    }),

  getTopStudents: protectedProcedure
    .input(getTopStudentsSchema)
    .query(async ({ input }) => {
      return getTopStudents(input);
    }),

  createEntry: teacherProcedure
    .input(createGradeEntrySchema)
    .mutation(async ({ input, ctx }) => {
      const data = {
          ...input,
          gradedById: input.gradedById || ctx.session.user.id
      };
      return createGradeEntry(data);
    }),

  updateEntry: teacherProcedure
    .input(updateGradeEntrySchema)
    .mutation(async ({ input, ctx }) => {
      const data = {
          ...input,
          gradedById: input.gradedById || ctx.session.user.id
      };
      return updateGradeEntry(input.id, data);
    }),

  batchUpsert: teacherProcedure
    .input(batchUpsertGradesSchema)
    .mutation(async ({ input, ctx }) => {
      const grades = input.grades.map(g => ({
        ...g,
        score: g.score === null ? null : g.score.toString()
      }));
      return batchUpsertGrades(grades, ctx.session.user.id);
    }),

  bulkImport: teacherProcedure
    .input(bulkImportGradesSchema)
    .mutation(async ({ input, ctx }) => {
      return bulkImportGrades(
          input.assessmentItemId, 
          input.gradeData, 
          ctx.session.user.id
      );
    }),

  bulkImportBySecret: teacherProcedure
    .input(z.object({
        assessmentItemId: z.number(),
        gradeData: z.array(z.object({
            secretNumber: z.string(),
            score: z.number().nullable(),
            isAbsent: z.boolean().optional(),
            comments: z.string().optional(),
        })),
    }))
    .mutation(async ({ input, ctx }) => {
      const { bulkImportGradesBySecretNumber } = await import("@orbis/db/queries");
      return bulkImportGradesBySecretNumber(
          input.assessmentItemId, 
          input.gradeData, 
          ctx.session.user.id
      );
    }),

  getClassStatistics: protectedProcedure
    .input(z.object({ classId: z.string(), academicYearId: z.number().optional() }))
    .query(async ({ input }) => {
      return getClassStatistics(input.classId, input.academicYearId);
    }),

  getCertificateData: protectedProcedure
    .input(getCertificateDataSchema)
    .query(async ({ input }) => {
      return getCertificateData(input);
    }),

  export: teacherProcedure
    .input(
      getGradeGridDataSchema.extend({
        format: z.enum(["xlsx", "csv"]).default("xlsx"),
      })
    )
    .mutation(async ({ input, ctx }) => {
      const { jobQueue } = await import("@orbis/jobs/queue");
      const classIdNum = typeof input.classId === "string" ? parseInt(input.classId) : input.classId;
      const subjectIdNum = typeof input.subjectId === "string" ? parseInt(input.subjectId) : input.subjectId;

      const job = await jobQueue.add("report-generation", {
        userId: ctx.session.user.id,
        classId: classIdNum,
        type: "grades_grid",
        month: input.month,
        subjectId: subjectIdNum,
        termId: input.termId,
        academicYearId: input.academicYearId,
        format: input.format,
      });

      return {
        jobId: job.id,
        status: "queued"
      };
    }),

  exportClassReport: teacherProcedure
    .input(
      z.object({
        classId: z.number(),
        type: z.enum(["monthly", "half-yearly", "final", "attendance_sheet", "students_parents_data", "control_staff", "teacher_grading", "control_comparison"]),
        month: z.number().optional(),
        monthText: z.string().optional(),
        format: z.enum(["xlsx", "csv"]).default("xlsx"),
      })
    )
    .mutation(async ({ input, ctx }) => {
      const { jobQueue } = await import("@orbis/jobs/queue");

      const job = await jobQueue.add("report-generation", {
        userId: ctx.session.user.id,
        classId: input.classId,
        type: input.type,
        month: input.month,
        monthText: input.monthText,
        format: input.format,
      });

      return {
        jobId: job.id,
        status: "queued"
      };
    }),
});
