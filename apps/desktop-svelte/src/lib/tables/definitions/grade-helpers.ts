import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { AvatarCell } from "..";

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
  cycle_id: string;
  class_subject_id: string;
  students: GradeMatrixStudent[];
  assessment_types: string[];
}

export function scoreToGradeLetter(percentage: number): string {
  if (percentage >= 90) return "ممتاز";
  if (percentage >= 80) return "جيد جداً";
  if (percentage >= 70) return "جيد";
  if (percentage >= 60) return "مقبول";
  if (percentage >= 50) return "ضعيف";
  return "راسب";
}

export function scoreToGradeClass(percentage: number): string {
  if (percentage >= 90) return "grade-excellent";
  if (percentage >= 80) return "grade-very-good";
  if (percentage >= 70) return "grade-good";
  if (percentage >= 60) return "grade-acceptable";
  if (percentage >= 50) return "grade-weak";
  return "grade-fail";
}

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

  columns.push({
    id: "__rowIndex",
    prop: "__rowIndex",
    header: labels.rowIndex || "#",
    name: labels.rowIndex || "#",
    width: 50,
    readonly: true,
    sort: false,
    align: "center",
    cellClass: "cell-row-index",
  });

  columns.push({
    id: "student_name_display",
    prop: "student_name_display",
    header: labels.studentName || (isRtl ? "الطالب" : "Student"),
    name: labels.studentName || (isRtl ? "الطالب" : "Student"),
    width: 180,
    readonly: true,
    sort: true,
    cellClass: "cell-student-name",
    cell: AvatarCell
  });

  if (matrixData.students.length > 0) {
    const firstStudent = matrixData.students[0];
    if (firstStudent) {
      for (const grade of firstStudent.grades) {
        const assessmentKey = grade.assessment_id || grade.exam_type;
        columns.push({
          id: `score_${assessmentKey}`,
          prop: `score_${assessmentKey}`,
          header: grade.exam_type,
          name: grade.exam_type,
          width: 100,
          readonly: isReadonly,
          sort: false,
          align: "center",
          editor: isReadonly ? undefined : {
            type: "text",
            config: {
              min: 0,
              max: grade.max_score
            }
          },
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

  columns.push({
    id: "__total",
    prop: "__total",
    header: labels.total || (isRtl ? "المجموع" : "Total"),
    name: labels.total || (isRtl ? "المجموع" : "Total"),
    width: 85,
    readonly: true,
    sort: true,
    align: "center",
    cellClass: "cell-total",
  });

  columns.push({
    id: "__percentage",
    prop: "__percentage",
    header: labels.percentage || (isRtl ? "النسبة" : "%"),
    name: labels.percentage || (isRtl ? "النسبة" : "%"),
    width: 70,
    readonly: true,
    sort: true,
    align: "center",
    cellClass: "cell-percentage",
  });

  columns.push({
    id: "__gradeLetter",
    prop: "__gradeLetter",
    header: labels.gradeLetter || (isRtl ? "التقدير" : "Grade"),
    name: labels.gradeLetter || (isRtl ? "التقدير" : "Grade"),
    width: 80,
    readonly: true,
    sort: true,
    align: "center",
  });

  return columns;
}

export function transformMatrixToSource(
  matrixData: GradeMatrixResponse,
  locale?: string,
): Record<string, unknown>[] {
  const isAr = locale === "ar";

  return matrixData.students.map((student, index) => {
    const row: Record<string, unknown> = {
      id: student.student_id,
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

    for (const grade of student.grades) {
      const key = `score_${grade.assessment_id || grade.exam_type}`;
      row[key] = grade.score ?? "";
      row[`__gradeId_${grade.assessment_id || grade.exam_type}`] =
        grade.grade_id;
      row[`__maxScore_${grade.assessment_id || grade.exam_type}`] =
        grade.max_score;
    }

    return row;
  });
}

export function recalculateRowTotals(
  row: Record<string, unknown>,
  assessmentKeys: string[],
): Record<string, unknown> {
  let total = 0;
  let maxTotal = 0;

  for (const key of assessmentKeys) {
    const scoreVal = row[`score_${key}`];
    const score = (scoreVal !== "" && scoreVal !== null) ? Number(scoreVal) : null;
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
