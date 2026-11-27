import * as m from "$lib/paraglide/messages";
import type { ReportDefinitionDto } from "$lib/api/types";
import {
  AdjustmentsHorizontal,
  Banknotes,
  BookOpen,
  Calendar,
  DocumentPage,
  Users,
} from "@orbis/icons";

export const REPORT_CATEGORIES = [
  { id: "all", label: () => m.operations_ui_report_category_all() },
  { id: "academic", label: () => m.operations_ui_report_category_academic() },
  { id: "student", label: () => m.operations_ui_report_category_student() },
  {
    id: "attendance",
    label: () => m.operations_ui_report_category_attendance(),
  },
  { id: "exam", label: () => m.operations_ui_report_category_exams() },
  { id: "finance", label: () => m.operations_ui_report_category_finance() },
  { id: "hr", label: () => m.operations_ui_report_category_hr() },
  {
    id: "administration",
    label: () => m.operations_ui_report_category_administration(),
  },
] as const;

const FORMAT_LABELS: Record<string, () => string> = {
  all: () => m.operations_ui_all_export_formats(),
  PDF: () => m.operations_ui_pdf_documents(),
  XLSX: () => m.operations_ui_xlsx_spreadsheets(),
  CSV: () => m.operations_ui_csv_files(),
  HTML: () => m.operations_ui_interactive_html(),
};

export function getFormatFilterLabel(format: string): string {
  return (FORMAT_LABELS[format] ?? (() => m.operations_ui_export_format()))();
}

const CATEGORY_MATCHERS: Record<
  string,
  { domains: string[]; categories: string[] }
> = {
  student: {
    domains: ["student", "students"],
    categories: ["students", "admissions"],
  },
  exam: {
    domains: ["exam", "exams", "examinations"],
    categories: ["examinations", "seating", "control"],
  },
  academic: {
    domains: ["academic"],
    categories: ["grades", "transcripts", "scheduling", "curriculum"],
  },
  finance: {
    domains: ["finance"],
    categories: ["accounting", "billing", "cash", "discounts", "tax"],
  },
  attendance: {
    domains: ["attendance"],
    categories: ["daily", "monthly", "alerts", "behavior"],
  },
  hr: {
    domains: ["hr", "employees"],
    categories: ["employees", "staff", "payroll", "workload", "contracts"],
  },
  administration: {
    domains: ["administration", "system"],
    categories: ["facilities", "security", "autopilot"],
  },
};

export function matchesReportCategory(
  def: ReportDefinitionDto,
  selectedCategory: string,
): boolean {
  const selected = selectedCategory.toLowerCase();
  if (selected === "all") return true;

  const domain = (def.domain || "").toLowerCase();
  const category = (def.category || "").toLowerCase();
  if (domain === selected || category === selected) return true;

  const rule = CATEGORY_MATCHERS[selected];
  if (!rule) return false;

  return rule.domains.includes(domain) || rule.categories.includes(category);
}

const DOMAIN_ICON_RULES = [
  {
    icon: Banknotes,
    match: (d: string, c: string) =>
      d === "finance" ||
      c.includes("billing") ||
      c.includes("accounting") ||
      c.includes("cash"),
  },
  {
    icon: BookOpen,
    match: (d: string, c: string) =>
      d === "academic" ||
      c.includes("grades") ||
      c.includes("transcript") ||
      c.includes("curriculum"),
  },
  {
    icon: Users,
    match: (d: string, c: string) =>
      d === "students" ||
      d === "student" ||
      c.includes("admission") ||
      c.includes("guardians") ||
      d.includes("employee") ||
      d.includes("hr") ||
      c.includes("staff") ||
      c.includes("payroll"),
  },
  {
    icon: Calendar,
    match: (d: string, c: string) =>
      d === "attendance" ||
      c.includes("daily") ||
      c.includes("monthly") ||
      c.includes("absence"),
  },
  {
    icon: DocumentPage,
    match: (d: string, c: string) =>
      d.includes("exam") || c.includes("seating") || c.includes("control"),
  },
];

export function getReportDomainIcon(domain?: string, category?: string) {
  const d = (domain || "").toLowerCase();
  const c = (category || "").toLowerCase();
  return (
    DOMAIN_ICON_RULES.find((rule) => rule.match(d, c))?.icon ??
    AdjustmentsHorizontal
  );
}
