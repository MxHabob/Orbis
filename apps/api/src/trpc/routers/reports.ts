
import { z } from "zod";
import { createTRPCRouter, protectedProcedure } from "../init";
import { 
  getStudentCount, 
  getTeacherCount, 
  getAverageAttendanceRate, 
  getGenderDistribution,
  getClassDistribution,
  getRecentEnrolledStudents,
  getOverallPerformance
} from "@midday/db/queries/metrics";

export const reportsRouter = createTRPCRouter({
  getSchoolMetrics: protectedProcedure
    .input(
      z.object({
        from: z.string().optional(),
        to: z.string().optional(),
        currency: z.string().optional(),
      })
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
        overallPerformance
      ] = await Promise.all([
        getStudentCount(input),
        getTeacherCount(input),
        getAverageAttendanceRate(input),
        getGenderDistribution(input),
        getClassDistribution(input),
        getRecentEnrolledStudents(input),
        getOverallPerformance(input)
      ]);

      return {
        studentCount,
        teacherCount,
        attendanceRate,
        genderStats,
        classStats,
        recentStudents,
        performance: overallPerformance
      };
    }),
});
