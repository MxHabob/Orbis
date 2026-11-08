import type { OrbisColumnDef } from "@orbis/svelte-grid";
import { api, type RecordGradeItemRequest } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import {
  type BulkValidationContext,
  findDuplicateIndex,
} from "../engine/duplicate-index";
import type {
  BulkCellIssue,
  BulkImportProfile,
  BulkPreviewSummary,
  BulkReferenceData,
  ColumnDefinitionMetadata,
  GradePreviewDetails,
} from "../types";

export interface GradeRowData {
  rowNumber?: number;
  student_id: string;
  student_name?: string;
  period_id: string;
  section_subject_id: string;
  component_id: string;
  score: number | string;
  max_score: number | string;
  is_excused?: boolean | string;
  notes?: string;
  [key: string]: unknown;
}

export const gradeColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "student_id",
    headerAr: "معرف الطالب*",
    headerEn: "Student ID*",
    required: true,
    type: "text",
    descriptionAr: "معرف الطالب كما هو مسجل في النظام",
    example: "student-id",
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
    id: "period_id",
    headerAr: "معرف الفترة الأكاديمية*",
    headerEn: "Academic Period ID*",
    required: true,
    type: "text",
    descriptionAr: "معرف الفترة الأكاديمية التي ينتمي إليها التقييم",
    example: "period-id",
  },
  {
    id: "section_subject_id",
    headerAr: "معرف عرض المادة*",
    headerEn: "Section Subject ID*",
    required: true,
    type: "text",
    descriptionAr: "معرف عرض المادة المرتبط بتسجيل الطالب",
    example: "section-subject-id",
  },
  {
    id: "component_id",
    headerAr: "معرف مكون التقييم*",
    headerEn: "Assessment Component ID*",
    required: true,
    type: "text",
    descriptionAr: "معرف مكون التقييم كما هو معرف في إعداد المؤسسة",
    example: "assessment-component-id",
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
    headerAr: "الدرجة العظمى*",
    headerEn: "Maximum Score*",
    required: true,
    type: "decimal",
    descriptionAr: "الحد الأعلى الذي ينتمي إليه score في هذا المقياس",
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
    id: "student_id",
    header: "Student ID",
    width: 130,
  },
  {
    id: "student_name",
    header: m.bulk_entry_grade_header_student_name(),
    width: 160,
    readonly: true,
  },
  {
    id: "period_id",
    header: "Academic Period ID",
    width: 150,
  },
  {
    id: "section_subject_id",
    header: "Section Subject ID",
    width: 180,
  },
  {
    id: "component_id",
    header: "Assessment Component ID",
    width: 190,
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
    ar: "استيراد الدرجات باستخدام معرفات الطالب والفترة وعرض المادة ومكون التقييم والحد الأعلى الصريح",
    en: "Import grades using explicit student, period, section-subject, and assessment-component IDs with an explicit maximum score",
  },
  templateVersion: "ORBIS-TPL-GRD-2026.2",
  columnMetadata: gradeColumnMetadata,
  gridColumns: gradeGridColumns,

  async buildReferenceData(): Promise<BulkReferenceData> {
    return {
      classes: [],
      teachers: [],
      sections: [],
      levels: [],
      subjects: [],
      feeTemplates: [],
      discountTemplates: [],
      userGroups: [],
      assessmentSchemes: [],
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
    _refData: BulkReferenceData,
  ): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();

    if (
      [
        "student_id",
        "period_id",
        "section_subject_id",
        "component_id",
        "score",
        "max_score",
      ].includes(colId)
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
      if (!Number.isFinite(num)) {
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

      const maximum = Number(row.max_score);
      if (
        row.max_score === undefined ||
        row.max_score === null ||
        String(row.max_score).trim() === "" ||
        !Number.isFinite(maximum) ||
        maximum <= 0
      ) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: "max_score",
          message: m.bulk_entry_grade_validation_max_positive(),
        };
      }
      if (num > maximum) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_exceeds_max({
            score: num,
            maximum,
          }),
        };
      }
    }

    if (colId === "max_score" && strVal) {
      const maximum = Number(strVal);
      if (!Number.isFinite(maximum) || maximum <= 0) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: "max_score",
          message: m.bulk_entry_grade_validation_max_positive(),
        };
      }
      const score = Number(row.score);
      if (
        String(row.score ?? "").trim() !== "" &&
        Number.isFinite(score) &&
        score > maximum
      ) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: "score",
          message: m.bulk_entry_grade_validation_exceeds_max({
            score,
            maximum,
          }),
        };
      }
    }

    if (colId === "is_excused" && strVal) {
      const normalized = strVal.toLocaleLowerCase();
      if (
        !["true", "false", "yes", "no", "نعم", "لا", "1", "0"].includes(
          normalized,
        )
      ) {
        return {
          severity: "BLOCKING_ERROR",
          columnId: colId,
          message: m.bulk_entry_grade_validation_boolean(),
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

    if (
      row.student_id &&
      row.period_id &&
      row.section_subject_id &&
      row.component_id
    ) {
      const key = JSON.stringify([
        row.student_id.trim().toLocaleLowerCase(),
        row.period_id.trim().toLocaleLowerCase(),
        row.section_subject_id.trim().toLocaleLowerCase(),
        row.component_id.trim().toLocaleLowerCase(),
      ]);
      const duplicateIndex = context
        ? (findDuplicateIndex(context, "grade_assessment", key, rowIndex) ?? -1)
        : allRows.findIndex((other, index) => {
            if (index === rowIndex) return false;
            const otherKey = JSON.stringify([
              other.student_id?.trim().toLocaleLowerCase(),
              other.period_id?.trim().toLocaleLowerCase(),
              other.section_subject_id?.trim().toLocaleLowerCase(),
              other.component_id?.trim().toLocaleLowerCase(),
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
    const sectionSubjectsSummary: Record<string, number> = {};
    const academicPeriodsSummary: Record<string, number> = {};
    const assessmentComponentsSummary: Record<string, number> = {};
    const seen = new Set<string>();

    for (const row of rows) {
      const num = Number(row.score);
      const maximum = Number(row.max_score);
      const key = JSON.stringify([
        row.student_id?.trim().toLocaleLowerCase(),
        row.period_id?.trim().toLocaleLowerCase(),
        row.section_subject_id?.trim().toLocaleLowerCase(),
        row.component_id?.trim().toLocaleLowerCase(),
      ]);
      const hasContext =
        Boolean(row.student_id?.trim()) &&
        Boolean(row.period_id?.trim()) &&
        Boolean(row.section_subject_id?.trim()) &&
        Boolean(row.component_id?.trim());
      const isInvalid =
        !hasContext ||
        String(row.score ?? "").trim() === "" ||
        String(row.max_score ?? "").trim() === "" ||
        !Number.isFinite(num) ||
        !Number.isFinite(maximum) ||
        maximum <= 0 ||
        num < 0 ||
        num > maximum ||
        (String(row.is_excused ?? "").trim() !== "" &&
          !["true", "false", "yes", "no", "نعم", "لا", "1", "0"].includes(
            String(row.is_excused).trim().toLocaleLowerCase(),
          )) ||
        seen.has(key);
      if (hasContext) seen.add(key);
      if (isInvalid) {
        errorCount++;
      } else {
        validCount++;
        sectionSubjectsSummary[row.section_subject_id] =
          (sectionSubjectsSummary[row.section_subject_id] || 0) + 1;
        academicPeriodsSummary[row.period_id] =
          (academicPeriodsSummary[row.period_id] || 0) + 1;
        assessmentComponentsSummary[row.component_id] =
          (assessmentComponentsSummary[row.component_id] || 0) + 1;
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
        sectionSubjectsSummary,
        academicPeriodsSummary,
        assessmentComponentsSummary,
      },
    };
  },

  async commitChunk(
    rows: GradeRowData[],
    _orgId: string,
    _refData: BulkReferenceData,
  ): Promise<{ success: boolean; errors?: string[] }> {
    const items = buildGradeBatchItems(rows);

    try {
      const result = await api.batchRecordGrades({ items });
      if (result.count !== items.length) {
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

export function buildGradeBatchItems(
  rows: GradeRowData[],
): RecordGradeItemRequest[] {
  return rows.map((row) => {
    const requiredIds = [
      row.student_id,
      row.period_id,
      row.section_subject_id,
      row.component_id,
    ];
    if (requiredIds.some((id) => typeof id !== "string" || !id.trim())) {
      throw new TypeError(
        "Grade rows require student, period, offering, and component IDs.",
      );
    }

    const score = Number(row.score);
    const maximum = Number(row.max_score);
    if (
      String(row.score ?? "").trim() === "" ||
      String(row.max_score ?? "").trim() === "" ||
      !Number.isFinite(score) ||
      !Number.isFinite(maximum) ||
      maximum <= 0 ||
      score < 0 ||
      score > maximum
    ) {
      throw new RangeError("Grade score must be within its explicit maximum.");
    }

    const excusedInput = String(row.is_excused ?? "")
      .trim()
      .toLocaleLowerCase();
    const isExcused =
      row.is_excused === true ||
      ["true", "yes", "نعم", "1"].includes(excusedInput);
    if (
      excusedInput &&
      !["true", "false", "yes", "no", "نعم", "لا", "1", "0"].includes(
        excusedInput,
      )
    ) {
      throw new TypeError("Grade rows require a valid excused value.");
    }

    return {
      period_id: row.period_id.trim(),
      student_id: row.student_id.trim(),
      section_subject_id: row.section_subject_id.trim(),
      component_id: row.component_id.trim(),
      score: String(score),
      max_score: String(maximum),
      ...(excusedInput ? { is_excused: isExcused } : {}),
      notes: row.notes?.trim() || undefined,
    };
  });
}
