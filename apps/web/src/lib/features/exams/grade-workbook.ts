import * as XLSX from "xlsx";
import type { RecordGradeItemRequest } from "$lib/api";
import * as m from "$lib/paraglide/messages";
import {
  gradeStatus, makeGradeItem, parseSheetScore, savedScore,
  type SheetComponent, type SheetRow, type SheetScore, type SheetSubject,
} from "./class-grade-sheet";

const manifestName = "_Orbis";
const manifestVersion = "grade-workbook-1";
const fieldRow = 4;
const firstStudentRow = 5;
const identityFields = ["student_name", "student_id", "enrollment_id", "baseline"];

interface GradeWorkbookContext {
  orgId: string;
  periodId: string;
}

export interface GradeWorkbookSheet {
  classId: string;
  className: string;
  rosterPeriodId: string;
  rows: SheetRow[];
  subjects: SheetSubject[];
  canEnter: boolean;
}

export interface GradeWorkbookChange {
  id: string;
  classId: string;
  className: string;
  studentId: string;
  student: string;
  subject: string;
  component: string;
  field: string;
  before: SheetScore;
  after: Exclude<SheetScore, null>;
  revision: string | null;
  item: RecordGradeItemRequest;
  isNew: boolean;
}

export interface GradeWorkbookPlan {
  sheets: GradeWorkbookSheet[];
  changes: GradeWorkbookChange[];
  errors: string[];
}

interface SheetManifest {
  name: string;
  classId: string;
  rosterPeriodId: string;
  schema: string;
}

function headerText(item: { name: string; name_ar?: string | null }, arabic: boolean) {
  return (arabic && item.name_ar) || item.name;
}

function studentText(row: SheetRow, arabic: boolean) {
  return arabic ? row.student_name_ar : row.student_name;
}

function componentColumns(subjects: SheetSubject[]) {
  return subjects.flatMap((subject) => subject.components.map((component) => ({ subject, component })));
}

function schemaOf(subjects: SheetSubject[]) {
  return JSON.stringify(componentColumns(subjects)
    .map(({ subject, component }) => [component.field, subject.schemeId ?? "", subject.schemeRevision ?? "", component.max_score, Boolean(component.historical)])
    .sort(([left], [right]) => String(left).localeCompare(String(right), "en")));
}

function fieldsInSnapshotOrder(subjects: SheetSubject[]) {
  return componentColumns(subjects).map(({ component }) => component.field).sort();
}

function canonicalScore(value: SheetScore | undefined): SheetScore {
  if (value === gradeStatus.absent()) return "absent";
  if (value === gradeStatus.excused()) return "excused";
  if (value === gradeStatus.exempt()) return "exempt";
  return value ?? null;
}

function safeSheetName(name: string, index: number) {
  const prefix = String(index + 1) + " - ";
  return (prefix + name.replace(/[\\/?*\[\]:']/g, " ").trim()).slice(0, 31);
}

export function buildGradeWorkbook(
  context: GradeWorkbookContext & { orgName: string; periodLabel: string; yearLabel?: string; arabic: boolean },
  scopes: GradeWorkbookSheet[],
) {
  const workbook = XLSX.utils.book_new();
  workbook.Workbook = { Views: [{ RTL: context.arabic }] };
  const manifest = [[manifestVersion, context.orgId, context.periodId]];
  for (const scope of scopes) {
    const name = safeSheetName(scope.className, workbook.SheetNames.length);
    const subjects = scope.subjects.filter((subject) => subject.components.length);
    const layout = subjects.flatMap((subject) => [
      ...subject.components.map((component) => ({
        key: component.field,
        label: headerText(component, context.arabic),
        maximum: component.max_score,
      })),
      {
        key: `total_${subject.id}`,
        label: m.common_grades_col_total(),
        maximum: subject.components.reduce((total, component) => total + component.max_score, 0),
      },
    ]);
    const groupLabels = identityFields.map(() => "");
    const merges: XLSX.Range[] = [];
    let col = identityFields.length;
    const subtotalColumns: number[] = [];
    for (const subject of subjects) {
      groupLabels[col] = headerText(subject, context.arabic);
      merges.push({ s: { r: 1, c: col }, e: { r: 1, c: col + subject.components.length } });
      subtotalColumns.push(col + subject.components.length);
      col += subject.components.length + 1;
    }
    if (subtotalColumns.length) {
      layout.push({ key: "total", label: m.exams_grade_sheet_grand_total(),
        maximum: subjects.reduce((sum, subject) => sum + subject.components.reduce((maximum, component) => maximum + component.max_score, 0), 0) });
    }
    merges.push({ s: { r: 0, c: 0 }, e: { r: 0, c: Math.max(3, col) } });
    // Start the heading at the first visible (student name) column.
    const snapshotFields = fieldsInSnapshotOrder(scope.subjects);
    const headers = [
      [[context.orgName, scope.className, context.yearLabel, context.periodLabel].filter(Boolean).join(" / ")],
      groupLabels,
      [m.common_word_student(), "", "", "", ...layout.map((column) => column.label)],
      ["", "", "", "", ...layout.map((column) => column.maximum)],
      [...identityFields, ...layout.map((column) => column.key)],
    ];
    const values = scope.rows.map((row) => [
      studentText(row, context.arabic),
      row.student_id,
      row.enrollment_id ?? "",
      JSON.stringify(snapshotFields.map((field) => {
        const grade = row.savedGrades[field];
        return [canonicalScore(savedScore(grade)), grade?.updated_at ?? null];
      })),
      ...layout.map((column) => row[column.key] ?? ""),
    ]);
    const sheet = XLSX.utils.aoa_to_sheet([...headers, ...values]);
    sheet["!rows"] = [{ hpt: 25 }, { hpt: 22 }, { hpt: 21 }, {}, { hidden: true }];
    sheet["!cols"] = [{ wch: 29 }, { hidden: true }, { hidden: true }, { hidden: true },
      ...layout.map((column) => ({ wch: Math.max(13, Math.min(22, column.label.length + 4)) }))];
    sheet["!merges"] = merges;
    let start = identityFields.length;
    for (const subject of subjects) {
      const end = start + subject.components.length;
      for (let index = 0; index < values.length; index++) {
        const r = firstStudentRow + index;
        const range = XLSX.utils.encode_range({ s: { r, c: start }, e: { r, c: end - 1 } });
        const subtotal = values[index]?.[end];
        sheet[XLSX.utils.encode_cell({ r, c: end })] = {
          t: typeof subtotal === "number" ? "n" : "s",
          v: typeof subtotal === "number" ? subtotal : "",
          f: "IF(COUNT(" + range + ")+COUNTIF(" + range + ' ,"absent")+COUNTIF(' + range + ',"\u063a")=0,"",SUM(' + range + "))",
        };
      }
      start = end + 1;
    }
    if (subtotalColumns.length) {
      for (let index = 0; index < values.length; index++) {
        const r = firstStudentRow + index;
        const cells = subtotalColumns.map((c) => XLSX.utils.encode_cell({ r, c })).join(",");
        const total = scope.rows[index]?.total;
        sheet[XLSX.utils.encode_cell({ r, c: col })] = {
          t: typeof total === "number" ? "n" : "s",
          v: typeof total === "number" ? total : "",
          f: "IF(COUNT(" + cells + ')=0,"",SUM(' + cells + "))",
        };
      }
    }
    XLSX.utils.book_append_sheet(workbook, sheet, name);
    manifest.push([name, scope.classId, scope.rosterPeriodId, schemaOf(scope.subjects)]);
  }
  const guide = XLSX.utils.aoa_to_sheet([
    [m.exams_workbook_guide()],
    [m.exams_workbook_guide_scores()],
    [m.exams_workbook_guide_scope()],
  ]);
  guide["!cols"] = [{ wch: 100 }];
  XLSX.utils.book_append_sheet(workbook, guide, m.exams_workbook_guide());
  XLSX.utils.book_append_sheet(workbook, XLSX.utils.aoa_to_sheet(manifest), manifestName);
  workbook.Workbook.Sheets = workbook.SheetNames.map((sheetName) => ({ name: sheetName, Hidden: sheetName === manifestName ? 2 : 0 }));
  return workbook;
}

export function isGradeWorkbook(workbook: XLSX.WorkBook) {
  return Boolean(workbook.Sheets[manifestName]);
}

export async function downloadGradeWorkbook(workbook: XLSX.WorkBook, filename: string) {
  const { isTauri } = await import("@tauri-apps/api/core");
  if (!isTauri()) {
    XLSX.writeFile(workbook, filename);
    return true;
  }
  const [{ save }, { writeFile }] = await Promise.all([import("@tauri-apps/plugin-dialog"), import("@tauri-apps/plugin-fs")]);
  const path = await save({ defaultPath: filename, filters: [{ name: "Excel", extensions: ["xlsx"] }] });
  if (!path) return false;
  await writeFile(path, new Uint8Array(XLSX.write(workbook, { bookType: "xlsx", type: "array" })));
  return true;
}

function readManifest(workbook: XLSX.WorkBook, context: GradeWorkbookContext): SheetManifest[] {
  const manifest = workbook.Sheets[manifestName];
  if (!manifest) throw new Error(m.exams_workbook_template_required());
  const [[version, orgId, periodId] = [], ...entries] = XLSX.utils.sheet_to_json<string[]>(manifest, {
    header: 1, raw: false, blankrows: false,
  });
  if (version !== manifestVersion || orgId !== context.orgId || periodId !== context.periodId || !entries.length) {
    throw new Error(m.exams_grade_sheet_file_context());
  }
  const classIds = new Set<string>();
  const names = new Set<string>();
  return entries.map(([name, classId, rosterPeriodId, schema]) => {
    if (!name || !classId || !rosterPeriodId || !schema || names.has(name) || classIds.has(classId) || !workbook.Sheets[name]) {
      throw new Error(m.exams_grade_sheet_file_context());
    }
    names.add(name);
    classIds.add(classId);
    return { name, classId, rosterPeriodId, schema };
  });
}

function readSnapshot(value: unknown): [SheetScore, string | null][] | null {
  try {
    const parsed: unknown = JSON.parse(String(value));
    if (!Array.isArray(parsed) || parsed.some((entry) =>
      !Array.isArray(entry) || entry.length !== 2 ||
      ![null, "number", "string"].includes(entry[0] === null ? null : typeof entry[0]) ||
      (entry[1] !== null && typeof entry[1] !== "string"),
    )) return null;
    return parsed;
  } catch {
    return null;
  }
}

export async function reviewGradeWorkbook(
  workbook: XLSX.WorkBook,
  context: GradeWorkbookContext,
  loadSheet: (classId: string) => Promise<GradeWorkbookSheet>,
  arabic: boolean,
): Promise<GradeWorkbookPlan> {
  const plan: GradeWorkbookPlan = { sheets: [], changes: [], errors: [] };
  for (const entry of readManifest(workbook, context)) {
    const scope = await loadSheet(entry.classId);
    plan.sheets.push(scope);
    const addError = (message: string, row?: number, col?: number) => {
      const address = row === undefined ? "" : col === undefined ? ` · ${row + 1}` : ` · ${XLSX.utils.encode_cell({ r: row, c: col })}`;
      plan.errors.push(`${scope.className}${address} — ${message}`);
    };
    if (entry.rosterPeriodId !== scope.rosterPeriodId || entry.schema !== schemaOf(scope.subjects)) {
      addError(m.exams_workbook_schema_changed());
      continue;
    }
    const fields = fieldsInSnapshotOrder(scope.subjects);
    const sheet = workbook.Sheets[entry.name]!;
    const data = XLSX.utils.sheet_to_json<unknown[]>(sheet, { header: 1, raw: true, defval: "", range: 0 });
    const keys = (data[fieldRow] ?? []).map(String);
    const indexByKey = new Map<string, number>();
    for (const [index, key] of keys.entries()) {
      if (!key) continue;
      if (indexByKey.has(key)) {
        addError(m.exams_workbook_schema_changed());
        break;
      }
      indexByKey.set(key, index);
    }
    if (indexByKey.size !== keys.filter(Boolean).length) continue;
    if (![...identityFields, ...fields].every((key) => indexByKey.has(key))) {
      addError(m.exams_workbook_schema_changed());
      continue;
    }
    const students = new Map(scope.rows.map((row) => [row.student_id, row]));
    const components = new Map(componentColumns(scope.subjects).map((item) => [item.component.field, item]));
    const seen = new Set<string>();
    for (let index = firstStudentRow; index < data.length; index++) {
      const cells = data[index] ?? [];
      if (cells.every((cell) => !String(cell ?? "").trim())) continue;
      const studentId = String(cells[indexByKey.get("student_id")!] ?? "");
      const row = students.get(studentId);
      if (!row || seen.has(studentId) || String(cells[indexByKey.get("enrollment_id")!] ?? "") !== (row.enrollment_id ?? "")) {
        addError(m.exams_workbook_student_changed(), index);
        continue;
      }
      seen.add(studentId);
      const snapshot = readSnapshot(cells[indexByKey.get("baseline")!]);
      if (!snapshot || snapshot.length !== fields.length) {
        addError(m.exams_workbook_template_required(), index);
        continue;
      }
      for (const [snapshotIndex, field] of fields.entries()) {
        const col = indexByKey.get(field)!;
        const raw = cells[col];
        if (raw == null || String(raw).trim() === "") continue;
        const { subject, component } = components.get(field)!;
        const value = parseSheetScore(raw, component.max_score);
        if (value == null) {
          addError(m.exams_grade_sheet_score_range({ max: component.max_score }), index, col);
          continue;
        }
        const saved = row.savedGrades[field];
        const before = savedScore(saved);
        const [original, revision] = snapshot[snapshotIndex]!;
        if (canonicalScore(value) === original || value === before) continue;
        if (component.historical || (saved && !saved.component_code && !saved.component_id)) {
          addError(m.exams_grade_sheet_historical(), index, col);
          continue;
        }
        if (!row.enrollment_id || canonicalScore(before) !== original || (saved?.updated_at ?? null) !== revision) {
          addError(m.exams_workbook_grade_changed(), index, col);
          continue;
        }
        plan.changes.push({
          id: `${scope.classId}:${studentId}:${field}`,
          classId: scope.classId, className: scope.className, studentId, student: studentText(row, arabic),
          subject: headerText(subject, arabic), component: headerText(component, arabic), field, before,
          after: value, revision, item: makeGradeItem(context.periodId, subject, component, row, value), isNew: !saved,
        });
      }
    }
  }
  return plan;
}
