import {
  createStudentEnrollmentSchema,
  createStudentSchema,
  getStudentsByClassSchema,
  getStudentsSchema,
  updateStudentEnrollmentSchema,
  updateStudentSchema,
} from "@api/schemas/student";
import { getStudentAttendanceSummarySchema } from "@api/schemas/attendance";
import { getStudentGradesSummarySchema } from "@api/schemas/grade";
import { getStudentPaymentSummarySchema } from "@api/schemas/payment";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { classCache } from "@midday/cache/class-cache";
import { studentCache } from "@midday/cache/student-cache";
import { getStudentAttendanceSummary } from "@midday/db/queries/attendance";
import { getStudentGradesSummary } from "@midday/db/queries/grade";
import { getStudentPaymentSummary } from "@midday/db/queries/payment";
import {
  createStudent,
  createStudentEnrollment,
  deleteStudent,
  getStudentById,
  getStudentByNumber,
  getStudentEnrollments,
  getStudents,
  getStudentsByClass,
  updateStudent,
  updateStudentEnrollment,
} from "@midday/db/queries/student";
import { z } from "zod";

export const studentRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getStudentsSchema)
    .query(async ({ input, ctx: { db } }) => {
      return getStudents(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await studentCache.getById(input.userId);
      if (cached) return cached;

      const result = await getStudentById(db, { userId: input.userId });
      if (result) {
        await studentCache.setById(input.userId, result);
      }
      return result;
    }),

  getByNumber: protectedProcedure
    .input(z.object({ studentNumber: z.string() }))
    .query(async ({ input, ctx: { db } }) => {
      const cached = await studentCache.getByNumber(input.studentNumber);
      if (cached) return cached;

      const result = await getStudentByNumber(db, input);
      if (result) {
        await studentCache.setByNumber(input.studentNumber, result);
      }
      return result;
    }),

  getByClass: protectedProcedure
    .input(getStudentsByClassSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await classCache.getStudents(input.classId);
      if (cached && !input.academicYearId) return cached; // Only leverage cache if no specific year filter or handle complex cache keys

      return getStudentsByClass(db, input);
    }),

  create: protectedProcedure
    .input(createStudentSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createStudent(db, input);
      if (input.currentClassId) {
        await classCache.invalidate(input.currentClassId);
      }
      // Can't cache result yet as we need ID which is generated inside, but createStudent returns it
      return result;
    }),

  update: protectedProcedure
    .input(updateStudentSchema.extend({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { userId, ...data } = input;
      const originalStudent = await getStudentById(db, { userId });
      const result = await updateStudent(db, { userId, ...data });

      await studentCache.invalidate(userId);
      
      // Invalidate class cache if class changed
      if (data.currentClassId && originalStudent?.currentClassId !== data.currentClassId) {
        await classCache.invalidate(data.currentClassId);
        if (originalStudent?.currentClassId) {
          await classCache.invalidate(originalStudent.currentClassId);
        }
      } else if (originalStudent?.currentClassId) {
        // Always invalidate current class list just in case
        await classCache.invalidate(originalStudent.currentClassId);
      }

      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ userId: z.string().uuid() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const student = await getStudentById(db, { userId: input.userId });
      const result = await deleteStudent(db, { userId: input.userId });
      
      await studentCache.invalidate(input.userId);
      if (student?.currentClassId) {
        await classCache.invalidate(student.currentClassId);
      }
      return result;
    }),

  // Enrollments
  getEnrollments: protectedProcedure
    .input(z.object({ studentId: z.string().uuid() }))
    .query(async ({ input, ctx: { db } }) => {
      return getStudentEnrollments(db, input);
    }),

  createEnrollment: protectedProcedure
    .input(createStudentEnrollmentSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      return createStudentEnrollment(db, input);
    }),

  updateEnrollment: protectedProcedure
    .input(updateStudentEnrollmentSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      return updateStudentEnrollment(db, { id, ...data });
    }),

  // Analytics
  getGradesSummary: protectedProcedure
    .input(getStudentGradesSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      return getStudentGradesSummary(db, input);
    }),

  getAttendanceSummary: protectedProcedure
    .input(getStudentAttendanceSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      return getStudentAttendanceSummary(db, input);
    }),

  getPaymentSummary: protectedProcedure
    .input(getStudentPaymentSummarySchema)
    .query(async ({ input, ctx: { db } }) => {
      return getStudentPaymentSummary(db, input);
    }),
});
