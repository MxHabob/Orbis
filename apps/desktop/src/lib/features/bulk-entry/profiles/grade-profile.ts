import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import {
  findDuplicateIndex,
  type BulkValidationContext,
} from "../engine/duplicate-index";
import type {
  BulkImportProfile,
  BulkReferenceData,
  ColumnDefinitionMetadata,
  BulkCellIssue,
  BulkPreviewSummary,
  GradePreviewDetails,
} from "../types";

export interface GradeRowData {
  rowNumber?: number;
  student_number: string;
  student_name?: string;
  subject_name: string;
  assessment_type: string;
  score: number | string;
  max_score?: number | string;
  is_excused?: boolean | string;
  notes?: string;
  [key: string]: unknown;
}

export const gradeColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "student_number",
    headerAr: "رقم قيد الطالب*",
    headerEn: "Student Number*",
    required: true,
    type: "text",
    descriptionAr: "الرقم التعريفي الفريد للطالب في المدرسة",
    example: "STU-2026-001",
  },
  {
    id: "student_name",
    headerAr: "اسم الطالب (مرجعي)",
    headerEn: "Student Name (Ref)",
    required: false,
    type: "text",
    descriptionAr: "اسم الطالب للمطابقة البصرية فقط",
    example: "أحمد محمد علي",
  },
  {
    id: "subject_name",
    headerAr: "المادة الدراسية*",
    headerEn: "Subject Name*",
    required: true,
    type: "reference",
    referenceSource: "subjects",
    descriptionAr: "اسم المادة المطابقة للائحة المواد الدراسية",
    example: "الرياضيات",
  },
  {
    id: "assessment_type",
    headerAr: "نوع التقييم / الاختبار*",
    headerEn: "Assessment Type*",
    required: true,
    type: "reference",
    referenceSource: "assessmentSchemes",
    descriptionAr: "نوع التقييم حسب لائحة التقييم المعتمدة",
    example: "اختبار الشهر الأول",
  },
  {
    id: "score",
    headerAr: "الدرجة المحرزة*",
    headerEn: "Score*",
    required: true,
    type: "decimal",
    descriptionAr:
      "الدرجة التي حصل عليها الطالب (أكبر أو تساوي 0 ولا تتجاوز الدرجة العظمى)",
    example: "18.5",
  },
  {
    id: "max_score",
    headerAr: "الدرجة العظمى",
    headerEn: "Max Score",
    required: false,
    type: "decimal",
    descriptionAr: "اتركها فارغة لاستخدام الدرجة العظمى المحددة في النظام",
    example: "20",
  },
  {
    id: "is_excused",
    headerAr: "غياب بعذر",
    headerEn: "Is Excused",
    required: false,
    type: "boolean",
    example: "لا",
  },
  {
    id: "notes",
    headerAr: "ملاحظات",
    headerEn: "Remarks",
    required: false,
    type: "text",
    example: "متميز في حل المسائل",
  },
];

export const gradeGridColumns: OrbisColumnDef[] = [
  {
    id: "__idx",
    header: m.bulk_entry_grid_row_number(),
    width: 50,
    pin: "left",
    readonly: true,
  },
  {
    id: "status",
    header: m.bulk_entry_grid_status(),
    width: 110,
    pin: "left",
    type: "badge",
    readonly: true,
  },
  {
    id: "student_number",
    header: m.bulk_entry_grade_header_student_number(),
    width: 130,
  },
  {
    id: "student_name",
    header: m.bulk_entry_grade_header_student_name(),
    width: 160,
    readonly: true,
  },
  {
    id: "subject_name",
    header: m.bulk_entry_grade_header_subject_name(),
    width: 150,
  },
  {
    id: "assessment_type",
    header: m.bulk_entry_grade_header_assessment_type(),
    width: 160,
  },
  {
    id: "score",
    header: m.bulk_entry_grade_header_score(),
    width: 100,
    type: "decimal",
  },
  {
    id: "max_score",
    header: m.bulk_entry_grade_header_max_score(),
    width: 110,
    type: "decimal",
  },
  {
    id: "is_excused",
    header: m.bulk_entry_grade_header_is_excused(),
    width: 90,
    type: "boolean",
  },
  { id: "notes", header: m.bulk_entry_grade_header_notes(), width: 180 },
];

export const gradeImportProfile: BulkImportProfile<
  GradeRowData,
  GradePreviewDetails
> = {
  id: "grades",
  displayName: {
    ar: "استيراد ورصد الدرجات والتقييمات",
    en: "Academic Grades & Assessments Bulk Entry",
  },
  description: {
    ar: "رصد درجات الطلاب لجميع المواد والتقييمات، مع التحقق من الحدود القصوى وحساب المعدلات التراكمية آلياً",
    en: "Bulk enter student grades across subjects and assessments with live score bounds checking and automatic GPA recalculation",
  },
  templateVersion: "ORBIS-TPL-GRD-2026.1",
  columnMetadata: gradeColumnMetadata,
  gridColumns: gradeGridColumns,

  async buildReferenceData(_orgId: string): Promise<BulkReferenceData> {
    const [subjectsRes, schemesRes] = await Promise.all([
      api.listSubjects().catch(() => []),
      api.listAssessmentSchemes().catch(() => []),
    ]);

    const subjects = (subjectsRes || []).map((s: any) => ({
      id: s.id,
      code: s.code || "",
      name: s.name || "",
      nameAr: s.name_ar || s.name || "",
    }));

    const assessmentSchemes = (schemesRes || []).map((s: any) => ({
      id: s.id,
      name: s.name || "",
      meta: { max_score: s.max_score, formula: s.formula },
    }));

    return {
      classes: [],
      teachers: [],
      sections: [],
      levels: [],
      subjects,
      feeTemplates: [],
      discountTemplates: [],
      userGroups: [],
      assessmentSchemes,
      academicPeriods: [],
      existingSectionSubjectAssignments: [],
      enums: {
        genders: [],
        studyModes: [],
        employeeTypes: [],
        employmentTypes: [],
        bloodTypes: [],
        relationships: [],
      },
    };
  },

  validateCell(
    colId: string,
    value: unknown,
    row: GradeRowData,
    refData: BulkReferenceData,
  ): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();

    if (
      ["student_number", "subject_name", "assessment_type", "score"].includes(
        colId,
      )
    ) {
      if (!strVal) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_required(),
        };
      }
    }

    if (colId === "score" && strVal) {
      const num = Number(strVal);
      if (isNaN(num)) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_numeric(),
        };
      }
      if (num < 0) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_nonnegative(),
        };
      }

      // Check max score bounds
      let limit = 100;
      if (row.max_score) {
        limit = Number(row.max_score);
      } else if (row.assessment_type) {
        const matchedScheme = refData.assessmentSchemes.find(
          (s) => s.name.toLowerCase() === row.assessment_type.toLowerCase(),
        );
        if (matchedScheme?.meta?.max_score) {
          limit = Number(matchedScheme.meta.max_score);
        }
      }

      if (num > limit) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_exceeds_max({
            score: num,
            maximum: limit,
          }),
        };
      }
    }

    if (colId === "subject_name" && strVal) {
      const hit = refData.subjects.find(
        (s) =>
          s.id.toLowerCase() === strVal.toLowerCase() ||
          s.code?.toLowerCase() === strVal.toLowerCase() ||
          s.name.toLowerCase() === strVal.toLowerCase() ||
          s.nameAr?.toLowerCase() === strVal.toLowerCase(),
      );
      if (!hit) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_subject_not_found({
            subject: strVal,
          }),
        };
      }
    }

    return null;
  },

  validateRow(
    row: GradeRowData,
    allRows: GradeRowData[],
    rowIndex: number,
    _refData: BulkReferenceData,
    context?: BulkValidationContext,
  ): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];

    // Check duplicate student + subject + assessment in same batch
    if (row.student_number && row.subject_name && row.assessment_type) {
      const key = JSON.stringify([
        row.student_number.trim().toLowerCase(),
        row.subject_name.trim().toLowerCase(),
        row.assessment_type.trim().toLowerCase(),
      ]);
      const duplicateIndex = context
        ? (findDuplicateIndex(context, "grade_assessment", key, rowIndex) ?? -1)
        : allRows.findIndex((other, index) => {
            if (index === rowIndex) return false;
            const otherKey = JSON.stringify([
              other.student_number?.trim().toLowerCase(),
              other.subject_name?.trim().toLowerCase(),
              other.assessment_type?.trim().toLowerCase(),
            ]);
            return key === otherKey;
          });

      if (duplicateIndex !== -1) {
        issues.push({
          severity: "BLOCKING_ERROR",
          columnId: "score",
          message: m.bulk_entry_grade_validation_duplicate({
            row: duplicateIndex + 1,
          }),
        });
      }
    }

    return issues;
  },

  async generatePreview(
    rows: GradeRowData[],
    _refData: BulkReferenceData,
  ): Promise<BulkPreviewSummary<GradePreviewDetails>> {
    let validCount = 0;
    let errorCount = 0;
    const subjectsMap: Record<string, number> = {};

    for (const row of rows) {
      const num = Number(row.score);
      const isInvalid =
        !row.student_number ||
        !row.subject_name ||
        !row.assessment_type ||
        isNaN(num) ||
        num < 0;
      if (isInvalid) {
        errorCount++;
      } else {
        validCount++;
        const s = row.subject_name || m.bulk_entry_general();
        subjectsMap[s] = (subjectsMap[s] || 0) + 1;
      }
    }

    return {
      totalRows: rows.length,
      validRowsCount: validCount,
      errorRowsCount: errorCount,
      warningRowsCount: 0,
      details: {
        gradesToRecord: validCount,
        gradesToUpdate: 0,
        subjectsSummary: subjectsMap,
        assessmentType:
          rows[0]?.assessment_type || m.bulk_entry_general_assessment(),
        academicPeriodName: m.bulk_entry_current_period(),
        maxScore: 100,
      },
    };
  },

  async commitChunk(
    rows: GradeRowData[],
    orgId: string,
    refData: BulkReferenceData,
  ): Promise<{ success: boolean; errors?: string[] }> {
    const items = rows.map((row) => {
      const matchedSubject = refData.subjects.find(
        (s) =>
          s.id.toLowerCase() === row.subject_name.toLowerCase() ||
          s.code?.toLowerCase() === row.subject_name.toLowerCase() ||
          s.name.toLowerCase() === row.subject_name.toLowerCase() ||
          s.nameAr?.toLowerCase() === row.subject_name.toLowerCase(),
      );

      const scoreNum = Number(row.score);
      const maxScoreNum = row.max_score ? Number(row.max_score) : 100;

      const isExcused =
        row.is_excused === true ||
        String(row.is_excused).toLowerCase() === "true" ||
        String(row.is_excused) === "نعم" ||
        String(row.is_excused) === "1";

      return {
        period_id: orgId,
        student_id: row.student_number,
        section_subject_id: matchedSubject?.id || orgId,
        score: String(scoreNum),
        max_score: String(maxScoreNum),
        is_excused: isExcused,
        notes: row.notes?.trim() || undefined,
      };
    });

    try {
      const result = await api.batchRecordGrades({ items });
      if (result.count === 0 && items.length > 0) {
        return {
          success: false,
          errors: [m.bulk_entry_grade_commit_failed()],
        };
      }
      return { success: true };
    } catch (err: any) {
      return {
        success: false,
        errors: [err.message || String(err)],
      };
    }
  },
};
