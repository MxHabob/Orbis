/**
 * @orbis/data-grid — Grade-specific helpers
 *
 * Utility functions for the grade entry grid:
 * - Score → grade letter mapping (matches Rust backend)
 * - Grade → CSS class mapping
 * - Column builder for grade matrix
 */

import type { OrbisColumnDef, GradeMatrixResponse } from "./types";
import { createInitials } from "./renderers";

// ── Grade Letter Conversion (mirrors Rust: Service::score_to_grade_letter) ──

export function scoreToGradeLetter(percentage: number): string {
  if (percentage >= 90) return "ممتاز";
  if (percentage >= 80) return "جيد جداً";
  if (percentage >= 70) return "جيد";
  if (percentage >= 60) return "مقبول";
  if (percentage >= 50) return "ضعيف";
  return "راسب";
}

// ── Grade Color (oklch — S.C.R.A.P.S. aligned) ──────────────────────────────

export function scoreToGradeColor(percentage: number): string {
  if (percentage >= 90)
    return "color: oklch(0.72 0.17 160)"; /* emerald — ممتاز */
  if (percentage >= 80)
    return "color: oklch(0.62 0.15 260)"; /* blue — جيد جداً */
  if (percentage >= 70) return "color: oklch(0.68 0.14 230)"; /* sky — جيد */
  if (percentage >= 60) return "color: oklch(0.78 0.16 75)"; /* amber — مقبول */
  if (percentage >= 50) return "color: oklch(0.72 0.17 50)"; /* orange — ضعيف */
  return "color: oklch(0.62 0.22 25)"; /* red — راسب */
}

// ── Grade CSS Class (for RevoGrid cells) ─────────────────────────────────────

export function scoreToGradeClass(percentage: number): string {
  if (percentage >= 90) return "grade-excellent";
  if (percentage >= 80) return "grade-very-good";
  if (percentage >= 70) return "grade-good";
  if (percentage >= 60) return "grade-acceptable";
  if (percentage >= 50) return "grade-weak";
  return "grade-fail";
}

// ── Build Grade Matrix Columns ───────────────────────────────────────────────

/**
 * Builds column definitions for the grade entry matrix.
 * Creates: Row# | Student Name | [Assessment 1] | [Assessment 2] | ... | Total | % | Grade
 */
export function buildGradeColumns(
  matrixData: GradeMatrixResponse,
  options?: {
    readonly?: boolean;
    locale?: string;
    labels?: {
      rowIndex?: string;
      studentName?: string;
      total?: string;
      percentage?: string;
      gradeLetter?: string;
    };
  },
): OrbisColumnDef[] {
  const isRtl = options?.locale === "ar";
  const labels = options?.labels ?? {};
  const isReadonly = options?.readonly ?? false;

  const columns: OrbisColumnDef[] = [];

  // 1. Row index column
  columns.push({
    prop: "__rowIndex",
    name: labels.rowIndex || "#",
    size: 50,
    pin: "colPinStart",
    readonly: true,
    sortable: false,
    resizable: false,
    align: "center",
    cellClass: "cell-row-index",
  });

  // 2. Student name column (frozen/pinned)
  columns.push({
    prop: "student_name_display",
    name: labels.studentName || (isRtl ? "الطالب" : "Student"),
    size: 180,
    minSize: 140,
    pin: "colPinStart",
    readonly: true,
    sortable: true,
    cellClass: "cell-student-name",
    cellRenderer: ({ h, row }) => {
      const name = String(row.student_name_display || "");
      const src = row.avatar as string | undefined;
      const initials = createInitials(name);

      const innerEl = src
        ? h("img", {
            class: "orbis-r-avatar__img",
            src,
            alt: name,
            loading: "lazy",
          })
        : h("span", { class: "orbis-r-avatar__fallback-text" }, initials);

      const avatarEl = h(
        "span",
        {
          class: `orbis-r-avatar__root orbis-r-avatar--sm ${
            src
              ? "orbis-r-avatar__root--has-img"
              : "orbis-r-avatar__root--has-fallback"
          }`,
        },
        [innerEl],
      );

      return h(
        "div",
        {
          class: "flex items-center gap-2 cursor-pointer w-full h-full px-2 text-start",
        },
        [
          avatarEl,
          h("span", { class: "text-xs font-medium text-foreground truncate" }, name),
        ],
      );
    },
  });

  // 3. Assessment columns (editable)
  if (matrixData.students.length > 0) {
    const firstStudent = matrixData.students[0];
    if (firstStudent) {
      for (const grade of firstStudent.grades) {
        const assessmentKey = grade.assessment_id || grade.exam_type;
        columns.push({
          prop: `score_${assessmentKey}`,
          name: grade.exam_type,
          size: 100,
          minSize: 70,
          readonly: isReadonly,
          sortable: false,
          align: "center",
          cellType: "number",
          editor: isReadonly ? undefined : "number",
          meta: {
            min: 0,
            max: grade.max_score,
            step: 0.5,
            assessmentId: grade.assessment_id,
            examType: grade.exam_type,
            maxScore: grade.max_score,
          },
          cellClass: "cell-number-input",
        });
      }
    }
  }

  // 4. Total column (pinned end, computed)
  columns.push({
    prop: "__total",
    name: labels.total || (isRtl ? "المجموع" : "Total"),
    size: 85,
    pin: "colPinEnd",
    readonly: true,
    sortable: true,
    align: "center",
    cellClass: "cell-total",
  });

  // 5. Percentage column (computed)
  columns.push({
    prop: "__percentage",
    name: labels.percentage || (isRtl ? "النسبة" : "%"),
    size: 70,
    pin: "colPinEnd",
    readonly: true,
    sortable: true,
    align: "center",
    cellClass: "cell-percentage",
  });

  // 6. Grade letter column (computed)
  columns.push({
    prop: "__gradeLetter",
    name: labels.gradeLetter || (isRtl ? "التقدير" : "Grade"),
    size: 80,
    pin: "colPinEnd",
    readonly: true,
    sortable: true,
    align: "center",
  });

  return columns;
}

// ── Transform Matrix Data to Grid Source ──────────────────────────────────────

/**
 * Transforms GradeMatrixResponse into flat row objects for RevoGrid.
 * Each row has: student info + score_{assessmentId} for each grade + computed totals.
 */
export function transformMatrixToSource(
  matrixData: GradeMatrixResponse,
  locale?: string,
): Record<string, unknown>[] {
  const isAr = locale === "ar";

  return matrixData.students.map((student, index) => {
    const row: Record<string, unknown> = {
      __rowIndex: index + 1,
      enrollment_id: student.enrollment_id,
      student_id: student.student_id,
      student_name: student.student_name,
      student_name_ar: student.student_name_ar,
      avatar: (student as any).avatar || null,
      student_name_display:
        isAr && student.student_name_ar
          ? student.student_name_ar
          : student.student_name,
      __total: student.total.toFixed(1),
      __maxTotal: student.max_total,
      __percentage: `${student.percentage.toFixed(1)}%`,
      __percentageRaw: student.percentage,
      __gradeLetter:
        student.grade_letter || scoreToGradeLetter(student.percentage),
    };

    // Flatten grade cells into row
    for (const grade of student.grades) {
      const key = `score_${grade.assessment_id || grade.exam_type}`;
      row[key] = grade.score ?? null;
      row[`__gradeId_${grade.assessment_id || grade.exam_type}`] =
        grade.grade_id;
      row[`__maxScore_${grade.assessment_id || grade.exam_type}`] =
        grade.max_score;
    }

    return row;
  });
}

// ── Recalculate Row Totals ───────────────────────────────────────────────────

/**
 * Recalculates total, percentage, and grade letter for a single row
 * after a cell edit. Updates the row object in-place and returns it.
 */
export function recalculateRowTotals(
  row: Record<string, unknown>,
  assessmentKeys: string[],
): Record<string, unknown> {
  let total = 0;
  let maxTotal = 0;

  for (const key of assessmentKeys) {
    const score = row[`score_${key}`];
    const maxScore = row[`__maxScore_${key}`];

    if (score != null && typeof score === "number" && !isNaN(score)) {
      total += score;
    }
    if (maxScore != null && typeof maxScore === "number") {
      maxTotal += maxScore;
    }
  }

  const percentage = maxTotal > 0 ? (total / maxTotal) * 100 : 0;
  const roundedPercentage = Math.round(percentage * 100) / 100;

  row.__total = total.toFixed(1);
  row.__maxTotal = maxTotal;
  row.__percentage = `${roundedPercentage.toFixed(1)}%`;
  row.__percentageRaw = roundedPercentage;
  row.__gradeLetter = scoreToGradeLetter(roundedPercentage);

  return row;
}
