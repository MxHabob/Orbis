import type { OrbisColumnDef } from "@orbis/svelte-grid";
import * as m from "$lib/paraglide/messages";

// ── Assessment Column Config ───────────────────────────────────────────
// Describes a single gradable column derived from an AssessmentScheme component.
// This replaces all hardcoded "monthly"/"final" assessment type assumptions.

export interface AssessmentColumnConfig {
  /** Stable component code from the scheme (e.g. "assignments", "oral", "written_test") */
  code: string;
  /** Display name (English) */
  name: string;
  /** Display name (Arabic) */
  name_ar?: string | null;
  /** Maximum possible score for this component */
  max_score: number;
  /** Sort order within the scheme */
  sort_order: number;
}

export interface GradeCell {
  grade_id?: string | null;
  assessment_id?: string | null;
  /** Component code from scheme (used as column key) */
  exam_type: string;
  score?: number | null;
  max_score: number;
  /** Links this grade to a specific assessment component for backend storage */
  component_id?: string | null;
}

export interface GradeMatrixStudent {
  enrollment_id: string;
  student_id: string;
  student_name: string;
  student_name_ar?: string | null;
  grades: GradeCell[];
  total: number;
  max_total: number;
  percentage: number;
  grade_letter: string;
}

export interface GradeMatrixResponse {
  period_id: string;
  section_subject_id: string;
  students: GradeMatrixStudent[];
  assessment_types: string[];
  /** Typed column configs derived from the assessment scheme. When present, these drive column rendering. */
  components?: AssessmentColumnConfig[];
}

export interface GradeScaleItem {
  id: string;
  scale_id: string;
  grade_label: string;
  min_score: number;
  max_score: number;
  gpa_points: number;
  is_passing: number | boolean;
}

/**
 * Resolve a grade letter from scaleItems if provided.
 * When no scaleItems exist, returns `fallbackLabel` (typically the
 * backend-provided `grade_letter`) so we never silently fabricate
 * a grading policy on the client.
 */
export function scoreToGradeLetter(
  percentage: number,
  scaleItems?: GradeScaleItem[],
  fallbackLabel?: string,
): string {
  if (scaleItems && scaleItems.length > 0) {
    const sorted = [...scaleItems].sort((a, b) => b.min_score - a.min_score);
    for (const item of sorted) {
      if (percentage >= item.min_score && percentage <= item.max_score) {
        return item.grade_label;
      }
    }
    return sorted[sorted.length - 1]?.grade_label ?? "—";
  }
  // No local scale → trust whatever the backend already computed
  return fallbackLabel || "—";
}

/**
 * Return a CSS class for the grade badge colour.
 * Falls back to a generic passing/failing distinction when no scaleItems
 * are configured — never uses hardcoded percentage thresholds.
 */
export function scoreToGradeClass(
  percentage: number,
  scaleItems?: GradeScaleItem[],
  passingThreshold = 50,
): string {
  if (scaleItems && scaleItems.length > 0) {
    const sorted = [...scaleItems].sort((a, b) => b.min_score - a.min_score);
    for (let i = 0; i < sorted.length; i++) {
      const item = sorted[i];
      if (percentage >= item.min_score && percentage <= item.max_score) {
        return item.is_passing ? "grade-passing" : "grade-fail";
      }
    }
    return "grade-fail";
  }
  return percentage >= passingThreshold ? "grade-passing" : "grade-fail";
}

export function buildGradeColumns(
  matrix: GradeMatrixResponse,
  opts: {
    readonly?: boolean;
    locale?: string;
    scaleItems?: GradeScaleItem[];
    labels?: {
      rowIndex?: string;
      studentName?: string;
      total?: string;
      percentage?: string;
      grade?: string;
      gradeLetter?: string;
    };
  } = {},
): OrbisColumnDef[] {
  const { readonly = false, locale = "ar", labels = {} } = opts;

  const cols: OrbisColumnDef[] = [
    {
      id: "__rowIndex",
      prop: "__rowIndex",
      header: labels.rowIndex || "#",
      name: labels.rowIndex || "#",
      width: 48,
      size: 48,
      readonly: true,
      pin: "left",
      align: "center",
      sort: false,
    },
    {
      id: locale === "ar" ? "student_name_ar" : "student_name",
      prop: locale === "ar" ? "student_name_ar" : "student_name",
      header: labels.studentName || m.common_grade_column_student(),
      name: labels.studentName || m.common_grade_column_student(),
      width: 200,
      size: 200,
      type: "avatar",
      readonly: true,
      pin: "left",
      sort: true,
    },
  ];

  // ── Dynamic columns from scheme components ─────────────────
  if (matrix.components && matrix.components.length > 0) {
    // Use typed component configs from the assessment scheme
    for (const comp of matrix.components) {
      const displayName =
        locale === "ar" && comp.name_ar ? comp.name_ar : comp.name;
      const typeLabel = `${displayName} (${comp.max_score})`;

      cols.push({
        id: `exam_${comp.code}`,
        prop: `exam_${comp.code}`,
        header: typeLabel,
        name: typeLabel,
        width: 105,
        size: 105,
        type: "number",
        align: "end",
        editor: readonly
          ? undefined
          : {
              type: "text",
              config: { min: 0, max: comp.max_score },
            },
        sort: true,
      });
    }
  } else {
    // Legacy fallback: use assessment_types strings
    for (const type of matrix.assessment_types || []) {
      const sample = matrix.students?.[0]?.grades?.find(
        (g) => g.exam_type === type,
      );
      const max = sample?.max_score || (type === "final" ? 30 : 20);
      const typeLabel =
        type === "monthly"
          ? m.exams_grade_work_label({ max })
          : type === "final"
            ? m.exams_grade_exam_label({ max })
            : `${type} (${max})`;

      cols.push({
        id: `exam_${type}`,
        prop: `exam_${type}`,
        header: typeLabel,
        name: typeLabel,
        width: 105,
        size: 105,
        type: "number",
        align: "end",
        editor: readonly
          ? undefined
          : {
              type: "text",
              config: { min: 0, max },
            },
        sort: true,
      });
    }
  }

  cols.push(
    {
      id: "total",
      prop: "total",
      header: labels.total || m.common_grades_col_total(),
      name: labels.total || m.common_grades_col_total(),
      width: 85,
      size: 85,
      type: "number",
      align: "end",
      readonly: true,
      sort: true,
    },
    {
      id: "percentage",
      prop: "percentage",
      header: labels.percentage || "%",
      name: labels.percentage || "%",
      width: 75,
      size: 75,
      type: "percentage",
      align: "end",
      readonly: true,
      sort: true,
    },
    {
      id: "grade_letter",
      prop: "grade_letter",
      header: labels.grade || labels.gradeLetter || m.common_grade(),
      name: labels.grade || labels.gradeLetter || m.common_grade(),
      width: 85,
      size: 85,
      type: "badge",
      align: "center",
      readonly: true,
      sort: true,
    },
  );

  return cols;
}

export function transformMatrixToSource(
  matrix: GradeMatrixResponse,
  locale = "ar",
  scaleItems?: GradeScaleItem[],
): Record<string, unknown>[] {
  return (matrix.students || []).map((student, i) => {
    const row: Record<string, unknown> = {
      __rowIndex: i + 1,
      __idx: i + 1,
      enrollment_id: student.enrollment_id,
      student_id: student.student_id,
      student_name: student.student_name,
      student_name_ar: student.student_name_ar || student.student_name,
      total: student.total,
      max_total: student.max_total,
      percentage: student.percentage,
      grade_letter:
        scaleItems && scaleItems.length > 0
          ? scoreToGradeLetter(student.percentage, scaleItems)
          : student.grade_letter || "—",
    };

    for (const g of student.grades || []) {
      row[`exam_${g.exam_type}`] = g.score ?? null;
      row[`__meta_exam_${g.exam_type}`] = {
        grade_id: g.grade_id,
        assessment_id: g.assessment_id,
        component_id: g.component_id ?? null,
        max_score: g.max_score,
      };
    }

    return row;
  });
}

export function recalculateRowTotals(
  row: Record<string, unknown>,
  assessmentTypes: string[],
  scaleItems?: GradeScaleItem[],
): Record<string, unknown> {
  let total = 0;
  let maxTotal = 0;

  for (const type of assessmentTypes) {
    const val = row[`exam_${type}`];
    const meta = row[`__meta_exam_${type}`] as
      | { max_score: number }
      | undefined;
    const max = meta?.max_score || 100;
    maxTotal += max;
    if (val !== null && val !== undefined && val !== "") {
      total += Number(val);
    }
  }

  const percentage = maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;
  return {
    ...row,
    total,
    max_total: maxTotal,
    percentage,
    grade_letter: scoreToGradeLetter(
      percentage,
      scaleItems,
      row.grade_letter as string,
    ),
  };
}

// ── Scheme Helpers ──────────────────────────────────────────────────────

/**
 * Extract AssessmentColumnConfig[] from an assessment scheme's components.
 * The scheme data may carry components under `components` (array of objects
 * with camelCase keys from the backend) or under `formula.components`.
 */
export function extractSchemeComponents(
  scheme: Record<string, unknown>,
): AssessmentColumnConfig[] {
  const raw: unknown[] =
    Array.isArray(scheme.components) && scheme.components.length > 0
      ? (scheme.components as unknown[])
      : Array.isArray(
            (scheme.formula as Record<string, unknown> | null)?.components,
          )
        ? ((scheme.formula as Record<string, unknown>).components as unknown[])
        : [];

  if (raw.length === 0) return [];

  return raw
    .map((c, i) => {
      const comp = (typeof c === "object" && c !== null ? c : {}) as Record<
        string,
        unknown
      >;
      return {
        code: String(comp.code ?? comp.id ?? `comp-${i}`),
        name: String(comp.name ?? ""),
        name_ar:
          typeof comp.name_ar === "string"
            ? comp.name_ar
            : typeof comp.nameAr === "string"
              ? comp.nameAr
              : null,
        max_score: Number(comp.max_score ?? comp.maxScore) || 0,
        sort_order: Number(comp.sort_order ?? comp.sortOrder) || i + 1,
      } satisfies AssessmentColumnConfig;
    })
    .sort((a, b) => a.sort_order - b.sort_order);
}

/**
 * Find the default monthly-period assessment scheme from a list of schemes.
 * Prioritises: (1) scheme with level "monthly_period", (2) scheme with
 * level "monthly", (3) the default scheme, (4) the first scheme.
 */
export function findMonthlyScheme(
  schemes: Record<string, unknown>[],
): Record<string, unknown> | null {
  if (!schemes.length) return null;
  return (
    schemes.find(
      (s) =>
        String(s.level) === "monthly_period" || String(s.level) === "monthly",
    ) ??
    schemes.find((s) => s.is_default === true) ??
    schemes[0] ??
    null
  );
}
