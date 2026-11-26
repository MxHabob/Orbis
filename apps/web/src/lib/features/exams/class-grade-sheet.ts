import type { OrbisColumnDef } from "@orbis/svelte-grid";
﻿import type { AcademicPeriodDto, GradeDto, RecordGradeItemRequest, StudentDetailDto } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import {
  extractSchemeComponents,
  scoreToGradeLetter,
  type AssessmentColumnConfig,
  type GradeScaleItem,
} from "./grade-helpers";

export type SheetScore = number | string | null;

export interface SheetComponent extends AssessmentColumnConfig {
  field: string;
  historical?: boolean;
}

export interface SheetSubject {
  id: string;
  schemeId?: string;
  schemeRevision?: string;
  name: string;
  name_ar?: string | null;
  components: SheetComponent[];
}

export interface SheetRow extends Record<string, unknown> {
  student_id: string;
  enrollment_id: string | undefined;
  student_name: string;
  student_name_ar: string;
  savedGrades: Record<string, GradeDto | undefined>;
}

export const gradeStatus = {
  absent: () => m.exams_grade_sheet_absent(),
  excused: () => m.student_grades_state_excused(),
  exempt: () => m.student_grades_state_exempt(),
};

function gradeCode(grade: GradeDto) {
  return grade.component_code || grade.component_id || (grade.is_final ? "final" : "monthly");
}

function legacyLabel(code: string, max: number) {
  if (code === "monthly") return m.exams_grade_work_label({ max }).replace(/\s*\([^)]*\)\s*$/, "");
  if (code === "final") return m.exams_grade_exam_label({ max }).replace(/\s*\([^)]*\)\s*$/, "");
  return code;
}

export function savedScore(grade: GradeDto | undefined): SheetScore {
  if (!grade) return null;
  if (grade.is_absent) return gradeStatus.absent();
  if (grade.is_excused) return gradeStatus.excused();
  if (grade.is_exempt) return gradeStatus.exempt();
  return Number(grade.score);
}

export function parseSheetScore(raw: unknown, max: number): SheetScore | undefined {
  const value = String(raw ?? "").trim();
  if (!value) return null;
  if (
    [
      "absent",
      gradeStatus.absent(),
      m.exams_grade_sheet_absent({}, { locale: "ar" }),
    ].includes(value.toLowerCase())
  ) {
    return gradeStatus.absent();
  }
  if (
    value.toLowerCase() === "excused" ||
    value === m.student_grades_state_excused({}, { locale: "ar" }) ||
    value === m.exams_grade_sheet_excused_arabic_input({}, { locale: "ar" }) ||
    value === gradeStatus.excused()
  ) {
    return gradeStatus.excused();
  }
  if (
    value.toLowerCase() === "exempt" ||
    value === m.student_grades_state_exempt({}, { locale: "ar" }) ||
    value === gradeStatus.exempt()
  ) {
    return gradeStatus.exempt();
  }
  const normalized = value
    .replace(/[٠-٩]/g, (digit) => String(digit.charCodeAt(0) - 0x660))
    .replace(/[۰-۹]/g, (digit) => String(digit.charCodeAt(0) - 0x6f0))
    .replace(/[٫,]/g, ".");
  if (!/^\d+(?:\.\d+)?$/.test(normalized)) return undefined;
  const score = Number(normalized);
  return score <= max ? score : undefined;
}

export function withSheetTotals(row: SheetRow, subjects: SheetSubject[], scale: GradeScaleItem[]): SheetRow {
  let total = 0;
  let maximum = 0;
  let hasScore = false;
  for (const subject of subjects) {
    let subtotal = 0;
    let hasSubjectScore = false;
    for (const component of subject.components) {
      const value = row[component.field];
      if (value === gradeStatus.exempt() || value === gradeStatus.excused()) continue;
      if (component.historical && value == null) continue;
      maximum += component.max_score;
      if (typeof value === "number") subtotal += value;
      if (value != null) hasSubjectScore = true;
    }
    row[`total_${subject.id}`] = hasSubjectScore ? subtotal : null;
    total += subtotal;
    hasScore ||= hasSubjectScore;
  }
  const percentage = hasScore && maximum ? Math.round(100 * total / maximum) : null;
  return {
    ...row,
    total: hasScore ? total : null,
    percentage,
    grade_letter: percentage == null || !scale.length ? null : scoreToGradeLetter(percentage, scale),
  };
}

async function loadStudents(classId: string, periodId: string) {
  const { api } = await import("$lib/api");
  const students: StudentDetailDto[] = [];
  let page = 1;
  let totalPages = 1;
  do {
    const result = await api.listStudents({ class_id: classId, period_id: periodId, page, page_size: 100 });
    students.push(...result.items);
    totalPages = result.total_pages;
    page++;
  } while (page <= totalPages);
  return students;
}

export { isGradeEntryOpen } from "$lib/api/period-override";
import { isGradeEntryOpen } from "$lib/api/period-override";

export function gradePeriodChain(periodId: string, periods: AcademicPeriodDto[]): AcademicPeriodDto[] {
  const byId = new Map(periods.map((period) => [period.id, period]));
  const first = byId.get(periodId);
  const chain: AcademicPeriodDto[] = [];
  const seen = new Set<string>();
  let current = first;
  while (current && current.year_id === first?.year_id && !seen.has(current.id)) {
    chain.push(current);
    seen.add(current.id);
    current = current.parent_id ? byId.get(current.parent_id) : undefined;
  }
  return chain;
}

export function makeGradeItem(
  periodId: string,
  subject: SheetSubject,
  component: SheetComponent,
  row: SheetRow,
  value: Exclude<SheetScore, null>,
): RecordGradeItemRequest {
  const saved = row.savedGrades[component.field];
  return {
    student_id: row.student_id,
    enrollment_id: row.enrollment_id ?? saved?.enrollment_id,
    period_id: periodId,
    section_subject_id: subject.id,
    component_code: component.code,
    score: String(typeof value === "number" ? value : 0),
    max_score: String(component.max_score),
    is_absent: value === gradeStatus.absent(),
    is_excused: value === gradeStatus.excused(),
    is_exempt: value === gradeStatus.exempt(),
    notes: saved?.notes ?? null,
  };
}

export async function loadGradeSheetReferences() {
  const { api } = await import("$lib/api");
  return Promise.all([api.listSubjects(), api.listAssessmentSchemes()]);
}

export async function loadClassGradeSheet(
  classId: string,
  periodId: string,
  periods: AcademicPeriodDto[] = [],
  references?: Awaited<ReturnType<typeof loadGradeSheetReferences>>,
) {
  const { api } = await import("$lib/api");
  const chain = gradePeriodChain(periodId, periods);
  let rosterPeriodId = periodId;
  let offerings = await api.listSectionSubjects(classId, rosterPeriodId);
  for (const ancestor of chain.slice(1)) {
    if (offerings.length) break;
    const inherited = await api.listSectionSubjects(classId, ancestor.id);
    if (!inherited.length) continue;
    rosterPeriodId = ancestor.id;
    offerings = inherited;
  }
  const [students, [subjects, schemes]] = await Promise.all([
    loadStudents(classId, rosterPeriodId),
    references ?? loadGradeSheetReferences(),
  ]);
  const subjectById = new Map(subjects.map((subject) => [subject.id, subject]));
  const schemeById = new Map(schemes.map((scheme) => [scheme.id, scheme]));
  const defaultScheme = schemes.find((scheme) => scheme.is_default);
  const rows: SheetRow[] = students.map(({ student, person, enrollment }, index) => ({
    __rowIndex: index + 1,
    student_id: student.id,
    enrollment_id: enrollment?.id,
    student_name: person.full_name || person.display_name || student.student_number || student.id,
    student_name_ar: person.full_name_ar || person.full_name || person.display_name || student.student_number || student.id,
    savedGrades: {},
  }));
  const rowByStudent = new Map(rows.map((row) => [row.student_id, row]));

  const groups = await Promise.all(offerings.map(async (offering): Promise<SheetSubject> => {
    const subject = subjectById.get(offering.subject_id);
    const scheme = offering.scheme_id ? schemeById.get(offering.scheme_id) : defaultScheme;
    const grades = (await api.listGrades({ SectionSubjectId: offering.id }))
      .filter((grade) => grade.period_id === periodId);
    const configs = scheme ? extractSchemeComponents(scheme as Record<string, unknown>) : [];
    const components = new Map<string, SheetComponent>();
    for (const config of configs) {
      components.set(config.code.toLowerCase(), {
        ...config,
        field: `exam_${offering.id}:${config.code}`,
        historical: !offering.is_active,
      });
    }

    for (const grade of grades) {
      const code = gradeCode(grade);
      const key = code.toLowerCase();
      let component = components.get(key);
      if (!component) {
        const max = Number(grade.max_score);
        component = {
          code,
          field: `exam_${offering.id}:${code}`,
          name: legacyLabel(code, max),
          max_score: max,
          sort_order: components.size,
          historical: Boolean(offering.scheme_id) || !offering.is_active ||
            (!grade.component_code && !grade.component_id),
        };
        components.set(key, component);
      }
      const row = rowByStudent.get(grade.student_id);
      if (!row) continue;
      row[component.field] = savedScore(grade);
      row.savedGrades[component.field] = grade;
    }
    return {
      id: offering.id,
      schemeId: scheme?.id,
      schemeRevision: scheme?.updated_at,
      name: subject?.name || subject?.name_ar || offering.subject_id,
      name_ar: subject?.name_ar,
      components: [...components.values()],
    };
  }));
  const rosterIndex = chain.findIndex((period) => period.id === rosterPeriodId);
  const canEnter = isGradeEntryOpen(chain[0]) && !chain.slice(1, rosterIndex + 1).some((period) => period.is_locked);
  return { rows, subjects: groups, rosterPeriodId, canEnter };
}

export function buildGradeGridColumns(
  visibleSubjects: SheetSubject[],
  { arabic, validate, showTotals = true, showGrade = false }: {
    arabic: boolean;
    validate?: (value: unknown, row: SheetRow, component: SheetComponent) => ReturnType<NonNullable<OrbisColumnDef["validate"]>>;
    showTotals?: boolean;
    showGrade?: boolean;
  },
): OrbisColumnDef[] {
  const result: OrbisColumnDef[] = [
    { id: "__rowIndex", header: "#", width: 48, pin: "colPinStart", readonly: true, align: "center", sort: false },
    { id: arabic ? "student_name_ar" : "student_name", header: m.common_word_student(), width: 210, pin: "colPinStart", readonly: true, sort: true },
  ];
  for (const subject of visibleSubjects) {
    const group = (arabic && subject.name_ar) || subject.name;
    for (const component of subject.components) {
      result.push({
        id: component.field,
        group,
        header: (arabic && component.name_ar) || component.name || component.code,
        subHeader: String(component.max_score),
        align: "center",
        width: 118,
        editor: "text",
        readonly: !validate || component.historical,
        sort: true,
        validate: validate ? (value, row) => validate(value, row as SheetRow, component) : undefined,
      });
    }
    result.push({
      id: `total_${subject.id}`, group, header: m.common_grades_col_total(),
      subHeader: String(subject.components.reduce((sum, item) => sum + item.max_score, 0)),
      width: 84, readonly: true, align: "center", emphasis: true, type: "number",
    });
  }
  if (showTotals) {
    result.push(
      { id: "total", header: m.exams_grade_sheet_grand_total(), width: 122, readonly: true, align: "center", emphasis: true, type: "number" },
      { id: "percentage", header: m.common_control_col_percentage(), width: 80, readonly: true, align: "center", type: "percentage" },
    );
    if (showGrade) {
      result.push({ id: "grade_letter", header: m.common_control_col_grade(), width: 96, readonly: true, align: "center", type: "badge" });
    }
  }
  return result;
}
