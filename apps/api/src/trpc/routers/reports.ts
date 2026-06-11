import {
  getAverageAttendanceRate,
  getClassDistribution,
  getGenderDistribution,
  getOverallPerformance,
  getRecentEnrolledStudents,
  getStudentCount,
  getTeacherCount,
} from "@orbis/db/queries/metrics";
import { z } from "zod";
import {
  createTRPCRouter,
  protectedProcedure,
  teacherProcedure,
} from "../init";

export const reportsRouter = createTRPCRouter({
  getSchoolMetrics: teacherProcedure
    .input(
      z.object({
        from: z.string().optional(),
        to: z.string().optional(),
        currency: z.string().optional(),
      }),
    )
    .query(async ({ input }) => {
      console.log("Fetching school metrics with input:", input);

      const [
        studentCount,
        teacherCount,
        attendanceRate,
        genderStats,
        classStats,
        recentStudents,
        overallPerformance,
      ] = await Promise.all([
        getStudentCount(input),
        getTeacherCount(input),
        getAverageAttendanceRate(input),
        getGenderDistribution(input),
        getClassDistribution(input),
        getRecentEnrolledStudents(input),
        getOverallPerformance(input),
      ]);

      return {
        studentCount,
        teacherCount,
        attendanceRate,
        genderStats,
        classStats,
        recentStudents,
        performance: overallPerformance,
      };
    }),
});
