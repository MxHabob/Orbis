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
  getStudentReportData,
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

  export: protectedProcedure
    .input(
      getGradeGridDataSchema.extend({
        format: z.enum(["xlsx", "csv"]).default("xlsx"),
      })
    )
    .mutation(async ({ input }) => {
      const data = await getGradeGridData(input);
      const { generateGradesExport, generateGradesFilename } = await import("@api/utils/excel-export");
      const { getClassesV3 } = await import("@midday/db/queries");

      const allClasses = await getClassesV3({
        flat: true,
        limit: 1,
      });
      const classData = allClasses.find(c => c.id === input.classId);

      const exportData = data.students.map((student) => {
        const studentGrades: Record<number, number | string | null> = {};
        data.items.forEach((item) => {
          const grade = data.grades.find(
            (g) => g.studentId === student.studentId && g.assessmentItemId === item.id
          );
          studentGrades[item.id] = grade?.score ?? null;
        });

        return {
          studentNumber: student.studentNumber,
          name: student.name || "N/A",
          grades: studentGrades,
        };
      });

      const buffer = await generateGradesExport(exportData, {
        className: classData?.name || "Class",
        academicYear: undefined,
        assessmentItems: data.items,
        format: input.format,
      });

      const filename = generateGradesFilename(classData?.name || "Class", input.format);

      return {
        data: buffer.toString("base64"),
        filename,
        mimeType:
          input.format === "xlsx"
            ? "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            : "text/csv",
      };
    }),
});
