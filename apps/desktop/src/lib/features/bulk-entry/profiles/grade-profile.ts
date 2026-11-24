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
  ReferenceItem,
} from "../types";

export interface GradeRowData {
  rowNumber?: number;
  student_id: string;
  student_name?: string;
  period_id: string;
  section_subject_id: string;
  component_code: string;
  score: number | string;
  max_score: number | string;
  is_absent?: boolean | string;
  is_excused?: boolean | string;
  is_exempt?: boolean | string;
  notes?: string;
  [key: string]: unknown;
}

function normalizeToken(value: unknown): string {
  return String(value ?? "").trim().toLowerCase();
}

function parseBooleanLike(value: unknown): boolean | null {
  const s = normalizeToken(value);
  if (s === "") return null;
  if (["true", "yes", "1", "y", "نعم", "صح", "صحيح"].includes(s)) return true;
  if (["false", "no", "0", "n", "لا", "خطأ"].includes(s)) return false;
  return null;
}

function parseScoreInput(value: unknown): number | null {
  const raw = String(value ?? "").trim();
  if (raw === "") return null;
  const normalized = raw
    .replace(/[٠-٩]/g, (d) => String("٠١٢٣٤٥٦٧٨٩".indexOf(d)))
    .replace(/٫/g, ".")
    .replace(/,/g, ".");
  const n = Number(normalized);
  return Number.isFinite(n) ? n : null;
}

function resolveStateFlags(row: GradeRowData): {
  isAbsent: boolean;
  isExcused: boolean;
  isExempt: boolean;
  invalid: string[];
} {
  const flags = [
    ["is_absent", parseBooleanLike(row.is_absent)] as const,
    ["is_excused", parseBooleanLike(row.is_excused)] as const,
    ["is_exempt", parseBooleanLike(row.is_exempt)] as const,
  ];
  const invalid = flags
    .filter(([id, v]) => String(row[id] ?? "").trim() !== "" && v === null)
    .map(([id]) => id);
  return {
    isAbsent: flags[0][1] === true,
    isExcused: flags[1][1] === true,
    isExempt: flags[2][1] === true,
    invalid,
  };
}

function isNonScoredState(row: GradeRowData): boolean {
  const s = resolveStateFlags(row);
  return s.isAbsent || s.isExcused || s.isExempt;
}

export const gradeColumnMetadata: ColumnDefinitionMetadata[] = [
  {
    id: "student_id",
    headerAr: "معرّف الطالب*",
    headerEn: "Student ID*",
    required: true,
    type: "text",
    descriptionAr: "معرّف الطالب كما هو مسجل في النظام",
    example: "student-id",
  },
  {
    id: "student_name",
    headerAr: "اسم الطالب (مرجع)",
    headerEn: "Student Name (Ref)",
    required: false,
    type: "text",
    descriptionAr: "اسم الطالب للعرض المرجعي فقط (لا يُستخدم للحفظ)",
    example: "أحمد محمد",
  },
  {
    id: "period_id",
    headerAr: "معرّف الفترة الأكاديمية*",
    headerEn: "Academic Period ID*",
    required: true,
    type: "text",
    descriptionAr: "معرّف الفترة الأكاديمية التي يُسجَّل فيها التقييم",
    example: "period-id",
  },
  {
    id: "section_subject_id",
    headerAr: "معرّف مادة الشعبة*",
    headerEn: "Section Subject ID*",
    required: true,
    type: "text",
    descriptionAr: "معرّف ربط المادة بالشعبة (Section Subject)",
    example: "section-subject-id",
  },
  {
    id: "component_code",
    headerAr: "رمز مكوّن التقييم*",
    headerEn: "Assessment Component Code*",
    required: true,
    type: "text",
    descriptionAr: "رمز المكوّن من مخطط التقييم، مثل written_test أو oral",
    example: "written_test",
  },
  {
    id: "score",
    headerAr: "الدرجة*",
    headerEn: "Score*",
    required: true,
    type: "decimal",
    descriptionAr: "قيمة الدرجة (يمكن تركها فارغة/0 مع حالة غائب أو بعذر أو معفى)",
    example: "18.5",
  },
  {
    id: "max_score",
    headerAr: "الدرجة العظمى*",
    headerEn: "Maximum Score*",
    required: true,
    type: "decimal",
    descriptionAr: "الحد الأعلى للدرجة لهذا المكوّن",
    example: "20",
  },
  {
    id: "is_absent",
    headerAr: "غائب",
    headerEn: "Absent",
    required: false,
    type: "boolean",
    example: "نعم",
  },
  {
    id: "is_excused",
    headerAr: "غياب بعذر",
    headerEn: "Excused",
    required: false,
    type: "boolean",
    example: "لا",
  },
  {
    id: "is_exempt",
    headerAr: "معفى",
    headerEn: "Exempt",
    required: false,
    type: "boolean",
    example: "لا",
  },
  {
    id: "notes",
    headerAr: "ملاحظات",
    headerEn: "Notes",
    required: false,
    type: "text",
    example: "ملاحظة",
  },
];

export const gradeGridColumns: OrbisColumnDef[] = [
  { id: "__idx", header: m.bulk_entry_grid_row_number(), width: 50, pin: "left", readonly: true },
  { id: "status", header: m.bulk_entry_grid_status(), width: 110, pin: "left", type: "badge", readonly: true },
  { id: "student_id", header: "Student ID", width: 150 },
  { id: "student_name", header: m.bulk_entry_grade_header_student_name(), width: 180, readonly: true },
  { id: "period_id", header: "Period ID", width: 150 },
  { id: "section_subject_id", header: "Section Subject ID", width: 180 },
  { id: "component_code", header: "Component Code", width: 160 },
  { id: "score", header: m.bulk_entry_grade_header_score(), width: 100, type: "decimal", align: "center" },
  { id: "max_score", header: m.bulk_entry_grade_header_max_score(), width: 110, type: "decimal", align: "center" },
  { id: "is_absent", header: "Absent", width: 90, type: "boolean", align: "center" },
  { id: "is_excused", header: m.bulk_entry_grade_header_is_excused(), width: 100, type: "boolean", align: "center" },
  { id: "is_exempt", header: "Exempt", width: 90, type: "boolean", align: "center" },
  { id: "notes", header: m.bulk_entry_grade_header_notes(), width: 180 },
];

function asRefItems(items: any[], nameKeys: string[] = ["name", "title", "label"], codeKeys: string[] = ["code"]): ReferenceItem[] {
  return (items || [])
    .map((it) => {
      if (!it || typeof it !== "object") return null;
      const id = String(it.id ?? it.value ?? "");
      if (!id) return null;
      const name = String(nameKeys.map((k) => it[k]).find((v) => typeof v === "string" && v) ?? it.name_ar ?? it.nameAr ?? id);
      const nameAr = typeof it.name_ar === "string" ? it.name_ar : (typeof it.nameAr === "string" ? it.nameAr : undefined);
      const codeVal = codeKeys.map((k) => it[k]).find((v) => typeof v === "string" && v);
      return { id, code: codeVal ? String(codeVal) : undefined, name, nameAr, meta: it as Record<string, unknown> } as ReferenceItem;
    })
    .filter(Boolean) as ReferenceItem[];
}

export const gradeImportProfile: BulkImportProfile<GradeRowData, GradePreviewDetails> = {
  id: "grades",
  displayName: {
    ar: "استيراد درجات وتقييمات بالجملة",
    en: "Grades & Assessments Bulk Entry",
  },
  description: {
    ar: "استيراد ذكي للدرجات باستخدام الطالب والفترة ومادة الشعبة ورمز مكوّن التقييم من المخططات الموجودة، مع دعم الحالات (غائب/بعذر/معفى).",
    en: "Smart grade import using student, period, section-subject, and assessment component codes from existing schemes, with status support (absent/excused/exempt).",
  },
  templateVersion: "ORBIS-TPL-GRD-2026.3",
  columnMetadata: gradeColumnMetadata,
  gridColumns: gradeGridColumns,

  async buildReferenceData(_orgId: string): Promise<BulkReferenceData> {
    const [periods, schemes, students, sections, subjects, sectionSubjects] = await Promise.all([
      (api as any).listAcademicPeriods?.().catch(() => []) ?? Promise.resolve([]),
      (api as any).listAssessmentSchemes?.().catch(() => []) ?? Promise.resolve([]),
      (api as any).listStudents?.({ limit: 500 }).catch(() => (api as any).listStudents?.().catch(() => [])) ?? Promise.resolve([]),
      (api as any).listSections?.().catch(() => []) ?? Promise.resolve([]),
      (api as any).listSubjects?.().catch(() => []) ?? Promise.resolve([]),
      (api as any).listSectionSubjects?.().catch(() => []) ?? Promise.resolve([]),
    ]);

    const studentItems = Array.isArray(students) ? students : (students?.items ?? students?.data ?? []);
    const sectionSubjectItems = Array.isArray(sectionSubjects) ? sectionSubjects : (sectionSubjects?.items ?? sectionSubjects?.data ?? []);

    return {
      classes: [],
      teachers: [],
      sections: asRefItems(Array.isArray(sections) ? sections : [], ["name", "title"], ["code"]),
      levels: [],
      subjects: asRefItems(Array.isArray(subjects) ? subjects : [], ["name", "title"], ["code"]),
      feeTemplates: [],
      discountTemplates: [],
      userGroups: [],
      assessmentSchemes: asRefItems(Array.isArray(schemes) ? schemes : [], ["name", "title"], ["code", "level"]),
      academicPeriods: asRefItems(Array.isArray(periods) ? periods : [], ["name", "title"], ["code"]),
      existingSectionSubjectAssignments: sectionSubjectItems.map((it: any) => ({
        classId: String(it?.section_id ?? it?.sectionId ?? ""),
        subjectId: String(it?.subject_id ?? it?.subjectId ?? ""),
        periodId: String(it?.period_id ?? it?.periodId ?? ""),
        teacherId: String(it?.teacher_id ?? it?.teacherId ?? ""),
      })),
      enums: {
        genders: [],
        studyModes: [],
        employeeTypes: [],
        employmentTypes: [],
        bloodTypes: [],
        relationships: [],
      },
      // expose extra refs via meta consumers can read if needed
      ...({
        students: asRefItems(studentItems, ["full_name", "name", "student_number"], ["student_number", "code"]),
        sectionSubjects: asRefItems(sectionSubjectItems, ["name", "subject_name", "title"], ["code"]),
      } as any),
    } as BulkReferenceData;
  },

  validateCell(colId: string, value: unknown, row: GradeRowData, _refData: BulkReferenceData): BulkCellIssue | null {
    const strVal = String(value ?? "").trim();

    if (["student_id", "period_id", "section_subject_id", "component_code", "max_score"].includes(colId)) {
      if (!strVal) {
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_required() };
      }
    }

    if (colId === "score") {
      const state = resolveStateFlags(row);
      const hasStatus = state.isAbsent || state.isExcused || state.isExempt;
      if (!strVal) {
        if (hasStatus) return null; // allow empty score when a status flag is set
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_required() };
      }
      const num = parseScoreInput(strVal);
      if (num === null) {
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_numeric() };
      }
      if (num < 0) {
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_nonnegative() };
      }
      const maximum = parseScoreInput(row.max_score);
      if (maximum === null || maximum <= 0) {
        return { severity: "BLOCKING_ERROR", columnId: "max_score", message: m.bulk_entry_grade_validation_max_positive() };
      }
      if (num > maximum) {
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_exceeds_max({ score: num, maximum }) };
      }
    }

    if (colId === "max_score" && strVal) {
      const maximum = parseScoreInput(strVal);
      if (maximum === null || maximum <= 0) {
        return { severity: "BLOCKING_ERROR", columnId: "max_score", message: m.bulk_entry_grade_validation_max_positive() };
      }
      const scoreStr = String(row.score ?? "").trim();
      if (scoreStr !== "") {
        const score = parseScoreInput(scoreStr);
        if (score !== null && score > maximum) {
          return { severity: "BLOCKING_ERROR", columnId: "score", message: m.bulk_entry_grade_validation_exceeds_max({ score, maximum }) };
        }
      }
    }

    if (["is_absent", "is_excused", "is_exempt"].includes(colId) && strVal) {
      if (parseBooleanLike(strVal) === null) {
        return { severity: "BLOCKING_ERROR", columnId: colId, message: m.bulk_entry_grade_validation_boolean() };
      }
    }

    return null;
  },

  validateRow(row: GradeRowData, allRows: GradeRowData[], rowIndex: number, _refData: BulkReferenceData, context?: BulkValidationContext): BulkCellIssue[] {
    const issues: BulkCellIssue[] = [];

    // Conflicting or invalid status flags
    const flags = resolveStateFlags(row);
    if (flags.invalid.length > 0) {
      for (const colId of flags.invalid) {
        issues.push({ severity: "BLOCKING_ERROR", columnId, message: m.bulk_entry_grade_validation_boolean() });
      }
    }
    const activeCount = [flags.isAbsent, flags.isExcused, flags.isExempt].filter(Boolean).length;
    if (activeCount > 1) {
      issues.push({ severity: "BLOCKING_ERROR", columnId: "is_absent", message: m.bulk_entry_grade_validation_boolean() });
    }

    if (row.student_id && row.period_id && row.section_subject_id && row.component_code) {
      const key = JSON.stringify([
        row.student_id.trim().toLowerCase(),
        row.period_id.trim().toLowerCase(),
        row.section_subject_id.trim().toLowerCase(),
        row.component_code.trim().toLowerCase(),
      ]);
      const duplicateIndex = context
        ? (findDuplicateIndex(context, "grade_assessment", key, rowIndex) ?? -1)
        : allRows.findIndex((other, index) => {
            if (index === rowIndex) return false;
            const otherKey = JSON.stringify([
              other.student_id?.trim().toLowerCase(),
              other.period_id?.trim().toLowerCase(),
              other.section_subject_id?.trim().toLowerCase(),
              other.component_code?.trim().toLowerCase(),
            ]);
            return key === otherKey;
          });

      if (duplicateIndex !== -1) {
        issues.push({ severity: "BLOCKING_ERROR", columnId: "score", message: m.bulk_entry_grade_validation_duplicate({ row: duplicateIndex + 1 }) });
      }
    }

    // Cross-field: if no status and score empty, score is required (cell validator also enforces, but keep row consistent)
    if (!isNonScoredState(row) && String(row.score ?? "").trim() === "") {
      issues.push({ severity: "BLOCKING_ERROR", columnId: "score", message: m.bulk_entry_grade_validation_required() });
    }

    return issues;
  },

  async generatePreview(rows: GradeRowData[], _refData: BulkReferenceData): Promise<BulkPreviewSummary<GradePreviewDetails>> {
    let validCount = 0;
    let errorCount = 0;
    const sectionSubjectsSummary: Record<string, number> = {};
    const academicPeriodsSummary: Record<string, number> = {};
    const assessmentComponentsSummary: Record<string, number> = {};
    const seen = new Set<string>();

    for (const row of rows) {
      const key = JSON.stringify([
        row.student_id?.trim().toLowerCase(),
        row.period_id?.trim().toLowerCase(),
        row.section_subject_id?.trim().toLowerCase(),
        row.component_code?.trim().toLowerCase(),
      ]);
      const hasContext = Boolean(row.student_id?.trim() && row.period_id?.trim() && row.section_subject_id?.trim() && row.component_code?.trim());
      const max = parseScoreInput(row.max_score);
      const flags = resolveStateFlags(row);
      const hasStatus = flags.isAbsent || flags.isExcused || flags.isExempt;
      const scoreStr = String(row.score ?? "").trim();
      const score = scoreStr === "" && hasStatus ? 0 : parseScoreInput(row.score);
      const badFlags = flags.invalid.length > 0 || [flags.isAbsent, flags.isExcused, flags.isExempt].filter(Boolean).length > 1;
      const isInvalid =
        !hasContext ||
        (scoreStr === "" && !hasStatus) ||
        score === null ||
        max === null || max <= 0 ||
        score < 0 || score > max ||
        badFlags ||
        seen.has(key);
      if (hasContext) seen.add(key);
      if (isInvalid) {
        errorCount++;
      } else {
        validCount++;
        if (row.section_subject_id) sectionSubjectsSummary[row.section_subject_id] = (sectionSubjectsSummary[row.section_subject_id] || 0) + 1;
        if (row.period_id) academicPeriodsSummary[row.period_id] = (academicPeriodsSummary[row.period_id] || 0) + 1;
        if (row.component_code) assessmentComponentsSummary[row.component_code] = (assessmentComponentsSummary[row.component_code] || 0) + 1;
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

  async commitChunk(rows: GradeRowData[], _orgId: string, _refData: BulkReferenceData, onProgress?: (progress: import("../types").ChunkProgress) => void): Promise<{ success: boolean; errors?: string[] }> {
    const items = buildGradeBatchItems(rows);
    try {
      onProgress?.({ totalChunks: 1, currentChunk: 1, chunkSize: rows.length, totalRows: rows.length, processedRows: 0, committedRows: 0, failedRows: 0, isFinished: false, hasErrors: false });
      const result = await api.batchRecordGrades({ items });
      if (result.count !== items.length) {
        return { success: false, errors: [m.bulk_entry_grade_commit_failed()] };
      }
      onProgress?.({ totalChunks: 1, currentChunk: 1, chunkSize: rows.length, totalRows: rows.length, processedRows: rows.length, committedRows: rows.length, failedRows: 0, isFinished: true, hasErrors: false });
      return { success: true };
    } catch (err: any) {
      return { success: false, errors: [err.message || String(err)] };
    }
  },
};

export function buildGradeBatchItems(rows: GradeRowData[]): RecordGradeItemRequest[] {
  return rows.map((row) => {
    const requiredIds = [row.student_id, row.period_id, row.section_subject_id, row.component_code];
    if (requiredIds.some((id) => typeof id !== "string" || !id.trim())) {
      throw new TypeError("Grade rows require student, period, offering, and component IDs.");
    }

    const flags = resolveStateFlags(row);
    if (flags.invalid.length > 0 || [flags.isAbsent, flags.isExcused, flags.isExempt].filter(Boolean).length > 1) {
      throw new TypeError("Grade rows require valid, non-conflicting status flags.");
    }
    const hasStatus = flags.isAbsent || flags.isExcused || flags.isExempt;

    const maximum = parseScoreInput(row.max_score);
    if (maximum === null || maximum <= 0) {
      throw new RangeError("Grade rows require a valid positive maximum score.");
    }

    const scoreStr = String(row.score ?? "").trim();
    let score = parseScoreInput(row.score);
    if (score === null) {
      if (scoreStr === "" && hasStatus) score = 0;
      else throw new RangeError("Grade score must be within its explicit maximum.");
    }
    if (!Number.isFinite(score) || score < 0 || score > maximum) {
      throw new RangeError("Grade score must be within its explicit maximum.");
    }

    return {
      period_id: row.period_id.trim(),
      student_id: row.student_id.trim(),
      section_subject_id: row.section_subject_id.trim(),
      component_code: row.component_code.trim(),
      score: String(score),
      max_score: String(maximum),
      is_absent: flags.isAbsent,
      is_excused: flags.isExcused,
      is_exempt: flags.isExempt,
      notes: row.notes?.trim() || undefined,
    };
  });
}
