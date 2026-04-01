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
  getTopStudents,
} from "@midday/db/queries";
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
    .input(z.object({ classId: z.string(), academicYearId: z.number().optional() }))
    .query(async ({ input }) => {
      return getClassStatistics(input.classId, input.academicYearId);
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
      const { getClassesV4 } = await import("@midday/db/queries");

      const allClasses = await getClassesV4({
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

  exportClassReport: protectedProcedure
    .input(
      z.object({
        classId: z.number(),
        type: z.enum(["monthly", "half-yearly", "final", "attendance_sheet"]),
        month: z.number().optional(),
        monthText: z.string().optional(),
        format: z.enum(["xlsx", "csv"]).default("xlsx"),
      })
    )
    .mutation(async ({ input }) => {
      const { getClassBulkReportData } = await import("@midday/db/queries");
      const { generateClassBulkReportExport, generateAttendanceSheetExport } = await import("@api/utils/excel-export");
      const { getClassesV4 } = await import("@midday/db/queries");
      const { db } = await import("@db/client");
      const { users, students, classes } = await import("@db/schema");
      const { eq, and, asc } = await import("drizzle-orm");

      const allClasses = await getClassesV4({ flat: true, limit: 0 }); // change limit to get all
      const classData = allClasses.find(c => c.id === input.classId);

      let buffer: Buffer;

      if (input.type === "attendance_sheet") {
        const studentsData = await db
          .select({
            name: users.name,
            phone: users.phone,
          })
          .from(students)
          .innerJoin(users, eq(students.userId, users.id))
          .where(and(eq(students.currentClassId, input.classId), eq(students.status, 'active')))
          .orderBy(asc(users.name));

        buffer = await generateAttendanceSheetExport(studentsData, {
          className: classData?.name || "Class",
          format: input.format,
        });
      } else {
        const bulkData = await getClassBulkReportData(input.classId, input.type, input.month);

        buffer = await generateClassBulkReportExport(bulkData as any, {
          className: classData?.name || "Class",
          reportType: input.type,
          month: input.month,
          monthText: input.monthText,
          format: input.format,
        });
      }

      const timestamp = new Date().toISOString().replace(/[:.]/g, "-").slice(0, -5);
      // Replace illegal characters for filenames
      const safeName = (classData?.name || "Class").replace(/[\/\\?%*:|"<>]/g, '_');
      const monthPart = input.monthText ? `_${input.monthText.replace(/[\/\\?%*:|"<>]/g, '_')}` : "";
      const filename = `${safeName}_${input.type}${monthPart}_report_${timestamp}.${input.format}`;

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
