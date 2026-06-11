"use server";

import {
  getAverageAttendanceRate,
  getClassDistribution,
  getGenderDistribution,
  getOverallPerformance,
  getRecentEnrolledStudents,
  getStudentCount,
  getTeacherCount,
} from "@orbis/db/queries/metrics";
import { authActionClient } from "@/actions/safe-action";

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
      recentStudents,
      overallPerformance,
    ] = await Promise.all([
      getStudentCount(),
      getTeacherCount(),
      getAverageAttendanceRate(),
      getGenderDistribution(),
      getClassDistribution(),
      getRecentEnrolledStudents(),
      getOverallPerformance(),
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
  });
