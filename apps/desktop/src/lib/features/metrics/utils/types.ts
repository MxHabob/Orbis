// Chart type identifiers for the school dashboard
export type ChartId =
  | "attendance-trends"
  | "total-students"
  | "gender-distribution"
  | "class-distribution"
  | "fee-collection"
  | "profit"
  | "grade-distribution"
  | "behavior-analysis";

// Column span options: 4 = 1/3, 6 = 1/2, 8 = 2/3, 12 = full width (12-column grid)
export type ColSpan = 4 | 6 | 8 | 12;

export interface ChartLayoutItem {
  id: ChartId;
  colSpan: ColSpan;
}

// All widgets in default order
export const DEFAULT_CHART_ORDER: ChartId[] = [
  "attendance-trends",
  "total-students",
  "gender-distribution",
  "class-distribution",
  "fee-collection",
  "profit",
  "grade-distribution",
  "behavior-analysis",
];

// Default responsive card layouts
export const DEFAULT_CHART_LAYOUT: ChartLayoutItem[] = [
  { id: "attendance-trends", colSpan: 6 },
  { id: "total-students", colSpan: 6 },
  { id: "gender-distribution", colSpan: 6 },
  { id: "class-distribution", colSpan: 6 },
  { id: "fee-collection", colSpan: 12 },
  { id: "profit", colSpan: 6 },
  { id: "grade-distribution", colSpan: 6 },
];

export const VALID_COL_SPANS: ColSpan[] = [4, 6, 8, 12];

// Display names for educational report types
export const chartDisplayNames: Record<ChartId, string> = {
  "attendance-trends": "Attendance Rate",
  "total-students": "Total Students",
  "gender-distribution": "Gender Distribution",
  "class-distribution": "Class Distribution",
  "fee-collection": "Monthly Revenue Trend",
  profit: "Profit",
  "grade-distribution": "Grade Distribution",
  "behavior-analysis": "Behavioral Incident Severity",
};
