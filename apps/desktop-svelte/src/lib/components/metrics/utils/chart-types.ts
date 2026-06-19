// Chart type identifiers for the school dashboard
export type ChartId =
  | "attendance-trends"
  | "grade-distribution"
  | "fee-collection"
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
  "grade-distribution",
  "fee-collection",
  "behavior-analysis",
];

// Default responsive card layouts
export const DEFAULT_CHART_LAYOUT: ChartLayoutItem[] = [
  { id: "attendance-trends", colSpan: 12 },
  { id: "grade-distribution", colSpan: 6 },
  { id: "fee-collection", colSpan: 6 },
  { id: "behavior-analysis", colSpan: 12 },
];

export const VALID_COL_SPANS: ColSpan[] = [4, 6, 8, 12];

// Display names for educational report types
export const chartDisplayNames: Record<ChartId, string> = {
  "attendance-trends": "Attendance Trends",
  "grade-distribution": "Grade Distribution",
  "fee-collection": "Tuition Fee Collection",
  "behavior-analysis": "Behavioral Incident Severity",
};
