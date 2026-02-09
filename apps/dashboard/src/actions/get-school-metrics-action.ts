
"use server";

import { authActionClient } from "@/actions/safe-action";
import { 
  getStudentCount, 
  getTeacherCount, 
  getAverageAttendanceRate, 
  getGenderDistribution,
  getClassDistribution,
  getRecentEnrolledStudents,
  getOverallPerformance
} from "@midday/db/queries/metrics";

export const getSchoolMetricsAction = authActionClient
  .metadata({
    name: "get-school-metrics",
  })
  .action(async () => {
    const [
      studentCount, 
      teacherCount, 
      attendanceRate,
      genderStats,
      classStats,
      recentStudents
    ] = await Promise.all([
      getStudentCount(),
      getTeacherCount(),
      getAverageAttendanceRate(),
      getGenderDistribution(),
      getClassDistribution(),
      getRecentEnrolledStudents(),
      getOverallPerformance()
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
  });
