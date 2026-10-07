import type { OrbisColumnDef } from "@orbis/svelte-grid";
import * as m from "$lib/paraglide/messages";

export interface GradeCell {
  grade_id?: string | null;
  assessment_id?: string | null;
  exam_type: string;
  score?: number | null;
  max_score: number;
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
  class_subject_id: string;
  students: GradeMatrixStudent[];
  assessment_types: string[];
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

export function scoreToGradeLetter(
  percentage: number,
  scaleItems?: GradeScaleItem[],
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
  if (percentage >= 90) return m.operations_ui_grade_excellent();
  if (percentage >= 80) return m.operations_ui_grade_very_good();
  if (percentage >= 70) return m.operations_ui_grade_good();
  if (percentage >= 60) return m.operations_ui_grade_acceptable();
  if (percentage >= 50) return m.operations_ui_grade_weak();
  return m.operations_ui_grade_failing();
}

export function scoreToGradeClass(
  percentage: number,
  scaleItems?: GradeScaleItem[],
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
  if (percentage >= 90) return "grade-excellent";
  if (percentage >= 80) return "grade-very-good";
  if (percentage >= 70) return "grade-good";
  if (percentage >= 60) return "grade-acceptable";
  if (percentage >= 50) return "grade-weak";
  return "grade-fail";
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

  for (const type of matrix.assessment_types || []) {
    const sample = matrix.students?.[0]?.grades?.find(
      (g) => g.exam_type === type,
    );
    const max = sample?.max_score || (type === "final" ? 30 : 20);
    const typeLabel =
      type === "monthly"
        ? (locale === "ar" ? `المحصلة (${max})` : `Work (${max})`)
        : type === "final"
          ? (locale === "ar" ? `الاختبار (${max})` : `Exam (${max})`)
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
      header: labels.grade || m.common_grade(),
      name: labels.grade || m.common_grade(),
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
          : student.grade_letter,
    };

    for (const g of student.grades || []) {
      row[`exam_${g.exam_type}`] = g.score ?? null;
      row[`__meta_exam_${g.exam_type}`] = {
        grade_id: g.grade_id,
        assessment_id: g.assessment_id,
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
    grade_letter: scoreToGradeLetter(percentage, scaleItems),
  };
}
