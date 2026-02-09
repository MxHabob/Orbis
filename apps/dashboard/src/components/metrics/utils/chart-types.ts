
// Chart type identifiers
export type ChartId =
  | "student-count"
  | "teacher-count"
  | "attendance-rate"
  | "gender-distribution"
  | "class-distribution"
  | "overall-performance";

// Report types for database storage (if needed, otherwise just mirror ChartId)
export type ReportType =
  | "student_count"
  | "teacher_count"
  | "attendance_rate"
  | "gender_distribution"
  | "class_distribution"
  | "overall_performance";

// Default chart order matching current layout
export const DEFAULT_CHART_ORDER: ChartId[] = [
  "student-count",
  "teacher-count",
  "attendance-rate",
  "gender-distribution",
  "class-distribution",
  "overall-performance",
];

// Mapping from ChartId to ReportType
const chartToReportMap: Record<ChartId, ReportType> = {
  "student-count": "student_count",
  "teacher-count": "teacher_count",
  "attendance-rate": "attendance_rate",
  "gender-distribution": "gender_distribution",
  "class-distribution": "class_distribution",
  "overall-performance": "overall_performance",
};

// Mapping from ReportType to ChartId
const reportToChartMap: Record<ReportType, ChartId> = {
  student_count: "student-count",
  teacher_count: "teacher-count",
  attendance_rate: "attendance-rate",
  gender_distribution: "gender-distribution",
  class_distribution: "class-distribution",
  overall_performance: "overall-performance",
};

// Display names for chart types
const chartDisplayNames: Record<ReportType, string> = {
  student_count: "Total Students",
  teacher_count: "Total Teachers",
  attendance_rate: "Attendance Rate",
  gender_distribution: "Gender Distribution",
  class_distribution: "Class Distribution",
  overall_performance: "Overall Performance",
};

export function chartTypeToReportType(chartId: ChartId): ReportType {
  return chartToReportMap[chartId];
}

export function reportTypeToChartType(reportType: ReportType): ChartId {
  return reportToChartMap[reportType];
}

export function getChartDisplayName(reportType: ReportType): string {
  return chartDisplayNames[reportType];
}
