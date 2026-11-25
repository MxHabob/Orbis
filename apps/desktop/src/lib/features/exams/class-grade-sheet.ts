import { api, type GradeDto, type StudentDetailDto } from "$lib/api";
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
  if (["غ", "absent", gradeStatus.absent()].includes(value.toLowerCase())) return gradeStatus.absent();
  if (value === gradeStatus.excused() || value === gradeStatus.exempt()) return value;
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

export async function loadClassGradeSheet(classId: string, periodId: string) {
  const [students, offerings, subjects, schemes] = await Promise.all([
    loadStudents(classId, periodId),
    api.listSectionSubjects(classId, periodId),
    api.listSubjects(),
    api.listAssessmentSchemes(),
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
      name: subject?.name || subject?.name_ar || offering.subject_id,
      name_ar: subject?.name_ar,
      components: [...components.values()],
    };
  }));
  return { rows, subjects: groups };
}
