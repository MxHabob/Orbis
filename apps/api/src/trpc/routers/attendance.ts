import { createTRPCRouter, protectedProcedure, teacherProcedure } from "@api/trpc/init";
import { 
  getAttendanceByLesson, 
  getStudentAttendance, 
  markAttendance, 
  bulkMarkAttendance 
} from "@midday/db/queries";
import { 
  markAttendanceSchema, 
  bulkMarkAttendanceSchema 
} from "@api/schemas/attendance";
import { z } from "zod";

export const attendanceRouter = createTRPCRouter({
  getByLesson: protectedProcedure
    .input(z.object({ lessonId: z.number(), date: z.string() }))
    .query(async ({ input }) => {
      return getAttendanceByLesson(input.lessonId, input.date);
    }),

  getStudentAttendance: protectedProcedure
    .input(z.object({ 
      studentId: z.string().uuid(), 
      startDate: z.string().optional(), 
      endDate: z.string().optional() 
    }))
    .query(async ({ input }) => {
      return getStudentAttendance(input.studentId, input.startDate, input.endDate);
    }),

  mark: teacherProcedure
    .input(markAttendanceSchema)
    .mutation(async ({ input, ctx }) => {
      // Automatically set the marker ID if not provided, assuming marker is the logged in user
      const data = {
          ...input,
          markedById: input.markedById || ctx.session.user.id
      };
      return markAttendance(data);
    }),

  bulkMark: teacherProcedure
    .input(bulkMarkAttendanceSchema)
    .mutation(async ({ input, ctx }) => {
      const data = input.map(record => ({
          ...record,
          markedById: record.markedById || ctx.session.user.id
      }));
      return bulkMarkAttendance(data);
    }),
});
