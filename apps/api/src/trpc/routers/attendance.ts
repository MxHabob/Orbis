import {
  bulkCreateAttendancesSchema,
  createAttendanceSchema,
  getClassAttendanceForDateSchema,
  getAttendancesSchema,
  updateAttendanceSchema,
} from "@api/schemas/attendance";
import { createTRPCRouter, protectedProcedure } from "@api/trpc/init";
import { attendanceCache } from "@midday/cache/attendance-cache";
import {
  bulkCreateAttendances,
  createAttendance,
  deleteAttendance,
  getAttendanceById,
  getAttendances,
  getClassAttendanceForDate,
  updateAttendance,
} from "@midday/db/queries/attendance";
import { z } from "zod";

export const attendanceRouter = createTRPCRouter({
  get: protectedProcedure
    .input(getAttendancesSchema)
    .query(async ({ input, ctx: { db } }) => {
      // For single student attendance, we might cache
      if (input.studentId && input.academicYearId && input.termId) {
        const cached = await attendanceCache.getStudentAttendance(
            input.studentId, 
            input.academicYearId, 
            input.termId
        );
        if (cached && !input.cursor) return cached; // Only return cached if full list requested (simplified)
      }
      return getAttendances(db, input);
    }),

  getById: protectedProcedure
    .input(z.object({ id: z.number() }))
    .query(async ({ input, ctx: { db } }) => {
      return getAttendanceById(db, { id: input.id });
    }),

  getClassAttendance: protectedProcedure
    .input(getClassAttendanceForDateSchema)
    .query(async ({ input, ctx: { db } }) => {
      const cached = await attendanceCache.getClassAttendance(input.classId, input.date);
      if (cached) return cached;

      const result = await getClassAttendanceForDate(db, input);
      if (result) {
        await attendanceCache.setClassAttendance(input.classId, input.date, result);
      }
      return result;
    }),

  create: protectedProcedure
    .input(createAttendanceSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await createAttendance(db, input);
      await attendanceCache.invalidateStudent(input.studentId);
      if (input.lessonId) { // If tied to a class session
          // We can't easily invalidate class cache here without resolving classId from student/lesson
          // For now rely on TTL or specific invalidation if classId is passed or resolved
      }
      return result;
    }),

  update: protectedProcedure
    .input(updateAttendanceSchema.extend({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const { id, ...data } = input;
      const original = await getAttendanceById(db, { id });
      
      const result = await updateAttendance(db, { id, ...data });
      
      if (original) {
        await attendanceCache.invalidateStudent(original.studentId);
      }
      return result;
    }),

  delete: protectedProcedure
    .input(z.object({ id: z.number() }))
    .mutation(async ({ input, ctx: { db } }) => {
      const original = await getAttendanceById(db, { id: input.id });
      const result = await deleteAttendance(db, { id: input.id });
      
      if (original) {
        await attendanceCache.invalidateStudent(original.studentId);
      }
      return result;
    }),

  bulkCreate: protectedProcedure
    .input(bulkCreateAttendancesSchema)
    .mutation(async ({ input, ctx: { db } }) => {
      const result = await bulkCreateAttendances(db, input);
      
      // Invalidate relevant caches
      const studentIds = new Set(input.attendances.map(a => a.studentId));
      await Promise.all(Array.from(studentIds).map(id => attendanceCache.invalidateStudent(id)));
      
      return result;
    }),
});
