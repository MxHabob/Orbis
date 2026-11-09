export type DatabaseManagementSection =
  | "overview"
  | "connection"
  | "backup"
  | "restore"
  | "maintenance"
  | "performance";

export interface DatabaseNavigationItem {
  id: DatabaseManagementSection;
  title: string;
}

export interface DatabaseManagementMetric {
  label: string;
  value: string;
  sub?: string;
  fullTitle?: string;
}
