<script lang="ts">
import { onMount, untrack } from "svelte";
import * as XLSX from "xlsx";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { api, extractErrorMessage, type RecordGradeItemRequest } from "$lib/api";
import { academicContext, authStore, uiStore } from "$lib/stores";
import { useUnsavedGuard } from "$lib/stores/unsaved-guard.svelte";
import { confirmStore } from "$lib/stores/confirm.svelte";
import { toast } from "@orbis/ui";
import type { GridRowsChangeEvent, OrbisColumnDef } from "@orbis/svelte-grid";
import { ControlGrid } from "../academics";
import {
  gradeStatus, loadClassGradeSheet, parseSheetScore, savedScore, withSheetTotals,
  type SheetComponent, type SheetRow, type SheetScore, type SheetSubject,
} from "./class-grade-sheet";

let { orgId }: { orgId: string } = $props();

let classId = $state<string | null>(null);
let periodId = $state<string | null>(null);
let subjectId = $state<string | null>("all");
let studentId = $state<string | null>("all");
let subjects = $state.raw<SheetSubject[]>([]);
let rows = $state.raw<SheetRow[]>([]);
let isLoading = $state(false);
let loadError = $state(false);
let isSaving = $state(false);
let isImporting = $state(false);
let isExporting = $state(false);
let isCalculating = $state(false);
let importInput: HTMLInputElement;
let loadVersion = 0;
const guard = useUnsavedGuard({ title: m.common_unsaved_changes() });
const period = $derived(academicContext.allPeriods.find((item) => item.id === periodId));
const canEdit = $derived(Boolean(period?.is_grade_entry_open && !period.is_locked && !academicContext.isReadOnly));
const arabic = $derived(getLocale() === "ar");
const subjectLabel = (subject: SheetSubject) => (arabic && subject.name_ar) || subject.name;
const studentLabel = (row: SheetRow) => arabic ? row.student_name_ar : row.student_name;
const visibleSubjects = $derived(subjects.filter((subject) =>
  subject.components.length > 0 && (!subjectId || subjectId === "all" || subject.id === subjectId)));
const visibleRows = $derived(!studentId || studentId === "all" ? rows : rows.filter((row) => row.student_id === studentId));
const selectedComponents = $derived(visibleSubjects.flatMap((subject) =>
  subject.components.map((component) => ({
    subject, component, key: visibleSubjects.length === 1 ? component.code : component.field,
  }))));
const pending = $derived.by(() => {
  const items: RecordGradeItemRequest[] = [];
  if (!periodId) return items;
  for (const row of rows) {
    for (const subject of subjects) {
      for (const component of subject.components) {
        const value = row[component.field] as SheetScore | undefined;
        const saved = row.savedGrades[component.field];
        if (value == null || value === savedScore(saved) || component.historical) continue;
        items.push({
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
        });
      }
    }
  }
  return items;
});
const busy = $derived(isSaving || isImporting || isCalculating);
const isDirty = $derived(pending.length > 0);
$effect(() => guard.setDirty(isDirty));

function validateScore(raw: unknown, row: SheetRow, component: SheetComponent) {
  const saved = row.savedGrades[component.field];
  const value = parseSheetScore(raw, component.max_score);
  if (saved && !saved.component_code && !saved.component_id) return m.exams_grade_sheet_historical();
  if (value === null && saved) return m.exams_grade_sheet_keep_saved();
  if (value === undefined) return m.exams_grade_sheet_score_range({ max: component.max_score });
  return { ok: true, value };
}

const columns = $derived.by((): OrbisColumnDef[] => {
  const result: OrbisColumnDef[] = [
    { id: "__rowIndex", header: "#", width: 48, pin: "colPinStart", readonly: true, align: "center", sort: false },
    { id: arabic ? "student_name_ar" : "student_name", header: m.common_word_student(), width: 210, pin: "colPinStart", readonly: true, sort: true },
  ];
  for (const subject of visibleSubjects) {
    const group = subjectLabel(subject);
    for (const component of subject.components) {
      result.push({
        id: component.field,
        group,
        header: (arabic && component.name_ar) || component.name || component.code,
        subHeader: String(component.max_score),
        align: "center",
        width: 118,
        editor: "text",
        readonly: !canEdit || component.historical,
        sort: true,
        validate: (value, row) => validateScore(value, row as SheetRow, component),
      });
    }
    result.push({
      id: `total_${subject.id}`, group, header: m.common_grades_col_total(),
      subHeader: String(subject.components.reduce((sum, item) => sum + item.max_score, 0)),
      width: 84, readonly: true, align: "center", emphasis: true, type: "number",
    });
  }
  if (!subjectId || subjectId === "all") {
    result.push(
      { id: "total", header: m.exams_grade_sheet_grand_total(), width: 122, readonly: true, align: "center", emphasis: true, type: "number" },
      { id: "percentage", header: m.common_control_col_percentage(), width: 80, readonly: true, align: "center", type: "percentage" },
    );
    if (academicContext.gradeScaleItems.length) {
      result.push({ id: "grade_letter", header: m.common_control_col_grade(), width: 96, readonly: true, align: "center", type: "badge" });
    }
  }
  return result;
});

onMount(() => {
  classId = academicContext.selectedClassId || null;
  periodId = uiStore.getPreference(
    { userId: authStore.userId, organizationId: orgId },
    "exams.selectedPeriodId", "", (value): value is string => typeof value === "string",
  ) || academicContext.effectivePeriodId || null;
});

$effect(() => {
  const periods = academicContext.allPeriods;
  if (periods.length && !periods.some((item) => item.id === periodId)) {
    periodId = academicContext.activePeriod?.id ?? periods[0]!.id;
  }
});

$effect(() => {
  const selectedClass = classId;
  const selectedPeriod = periodId;
  const hasPeriod = Boolean(period);
  const scope = orgId;
  untrack(() => {
    academicContext.selectedClassId = selectedClass ?? "";
    if (hasPeriod && selectedPeriod) {
      academicContext.selectedPeriodId = selectedPeriod;
      uiStore.setPreference({ userId: authStore.userId, organizationId: scope }, "exams.selectedPeriodId", selectedPeriod);
    }
    subjectId = "all";
    studentId = "all";
    void refresh(selectedClass, hasPeriod ? selectedPeriod : null);
  });
});

async function refresh(selectedClass = classId, selectedPeriod = periodId) {
  const version = ++loadVersion;
  subjects = [];
  rows = [];
  loadError = false;
  isLoading = Boolean(selectedClass && selectedPeriod);
  if (!selectedClass || !selectedPeriod) return;
  try {
    const sheet = await loadClassGradeSheet(selectedClass, selectedPeriod);
    if (version !== loadVersion) return;
    subjects = sheet.subjects;
    rows = sheet.rows.map((row) => withSheetTotals(row, sheet.subjects, academicContext.gradeScaleItems));
  } catch (error) {
    if (version !== loadVersion) return;
    loadError = true;
    toast.error(extractErrorMessage(error));
  } finally {
    if (version === loadVersion) isLoading = false;
  }
}

async function reload() {
  if (isDirty && !await confirmStore.prompt({
    title: m.common_unsaved_changes(),
    description: m.exams_grade_sheet_discard_prompt(),
    confirmText: m.exams_grade_sheet_discard(),
    cancelText: m.common_action_cancel(),
    variant: "destructive",
  })) return;
  await refresh();
}

function applyRows(event: GridRowsChangeEvent) {
  if (!canEdit || busy) return;
  const updated = new Map(event.changedRows.map((row) => [row.student_id, row as SheetRow]));
  rows = rows.map((row) => {
    const changed = updated.get(row.student_id);
    if (!changed) return row;
    return withSheetTotals({ ...changed }, subjects, academicContext.gradeScaleItems);
  });
}

async function save() {
  if (!canEdit || busy || pending.length === 0) return;
  isSaving = true;
  try {
    const result = await api.batchRecordGrades({ items: pending });
    const grades = new Map(result.grades.map((grade) => [
      `${grade.student_id}:${grade.section_subject_id}:${grade.component_code}`, grade,
    ]));
    rows = rows.map((row) => {
      const next: SheetRow = { ...row, savedGrades: { ...row.savedGrades } };
      for (const subject of subjects) {
        for (const component of subject.components) {
          const grade = grades.get(`${row.student_id}:${subject.id}:${component.code}`);
          if (!grade) continue;
          next[component.field] = savedScore(grade);
          next.savedGrades[component.field] = grade;
        }
      }
      return withSheetTotals(next, subjects, academicContext.gradeScaleItems);
    });
    toast.success(m.common_control_save_success({ total: String(result.count) }));
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isSaving = false;
  }
}

async function calculateRankings() {
  if (!canEdit || busy || isDirty || !rows.length) return;
  const field = subjectId && subjectId !== "all" ? `total_${subjectId}` : "total";
  const ranked = rows.filter((row) => row[field] != null);
  if (!ranked.length) return;
  isCalculating = true;
  try {
    await api.calculateRankings({
      items: ranked.map((row) => ({ id: row.student_id, score: String(row[field]) })),
    });
    toast.success(m.exams_control_rankings_calculated());
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isCalculating = false;
  }
}

async function exportWorkbook() {
  if (!selectedComponents.length || !visibleRows.length) return;
  isExporting = true;
  try {
    const workbook = XLSX.utils.book_new();
    const data = visibleRows.map((row) => ({
      student_id: row.student_id,
      enrollment_id: row.enrollment_id ?? "",
      student_name: studentLabel(row),
      ...Object.fromEntries(selectedComponents.map(({ key, component }) => [key, row[component.field] ?? ""])),
      total: visibleSubjects.length === 1 ? row[`total_${visibleSubjects[0]!.id}`] : row.total,
      percentage: row.percentage,
      grade_letter: row.grade_letter,
    }));
    XLSX.utils.book_append_sheet(workbook, XLSX.utils.json_to_sheet(data), "Grades");
    XLSX.utils.book_append_sheet(workbook, XLSX.utils.json_to_sheet([
      { key: "period_id", value: periodId },
      { key: "class_id", value: classId },
      { key: "section_subject_id", value: visibleSubjects.length === 1 ? visibleSubjects[0]!.id : "" },
      ...selectedComponents.map(({ subject, component, key }) => ({
        key: `component:${key}`,
        value: `${subjectLabel(subject)} / ${(arabic && component.name_ar) || component.name} (${component.max_score})`,
      })),
    ]), "Context");
    const filename = `grades-${classId?.slice(0, 8)}.xlsx`;
    if ("__TAURI_INTERNALS__" in window) {
      const [{ save: chooseFile }, { writeFile }] = await Promise.all([
        import("@tauri-apps/plugin-dialog"), import("@tauri-apps/plugin-fs"),
      ]);
      const path = await chooseFile({ defaultPath: filename, filters: [{ name: "Excel", extensions: ["xlsx"] }] });
      if (!path) return;
      await writeFile(path, new Uint8Array(XLSX.write(workbook, { bookType: "xlsx", type: "array" })));
    } else {
      XLSX.writeFile(workbook, filename);
    }
    toast.success(m.common_action_export());
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isExporting = false;
  }
}

async function importWorkbook(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  input.value = "";
  if (!file || !canEdit || busy) return;
  isImporting = true;
  try {
    const workbook = XLSX.read(await file.arrayBuffer(), { type: "array" });
    const sheet = workbook.Sheets.Grades ?? workbook.Sheets[workbook.SheetNames[0] ?? ""];
    if (!sheet) throw new Error(m.common_control_no_data());
    const context = workbook.Sheets.Context;
    if (context) {
      const metadata = new Map(XLSX.utils.sheet_to_json<{ key: string; value: string }>(context)
        .map(({ key, value }) => [key, value]));
      const offering = metadata.get("section_subject_id");
      if ((metadata.get("period_id") && metadata.get("period_id") !== periodId) ||
          (metadata.get("class_id") && metadata.get("class_id") !== classId) ||
          (offering && !visibleSubjects.some((subject) => subject.id === offering))) {
        throw new Error(m.exams_grade_sheet_file_context());
      }
    }
    const imported = new Map(XLSX.utils.sheet_to_json<Record<string, unknown>>(sheet)
      .map((row) => [String(row.student_id ?? row.studentId ?? row.StudentId ?? row["معرف الطالب"] ?? "").trim(), row]));
    const visibleIds = new Set(visibleRows.map((row) => row.student_id));
    let changed = 0;
    let rejected = 0;
    rows = rows.map((row) => {
      const data = imported.get(row.student_id);
      if (!data || !visibleIds.has(row.student_id)) return row;
      const next = { ...row };
      for (const { component, key } of selectedComponents) {
        if (component.historical) continue;
        const raw = data[component.field] ?? data[key] ??
          (visibleSubjects.length === 1 ? data[`exam_${key}`] ?? data[`comp:${key}`] : undefined);
        if (raw == null || String(raw).trim() === "") continue;
        const validation = validateScore(raw, row, component);
        if (typeof validation === "string") {
          rejected++;
          continue;
        }
        if (validation.value === row[component.field]) continue;
        next[component.field] = validation.value;
        changed++;
      }
      return withSheetTotals(next, subjects, academicContext.gradeScaleItems);
    });
    const message = m.exams_grade_sheet_import_result({ changed, rejected });
    if (changed) toast.success(message);
    else toast.warning(message);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isImporting = false;
  }
}
</script>

<ControlGrid
  bind:classId
  bind:periodId
  bind:subjectId
  bind:studentId
  periods={academicContext.allPeriods}
  {period}
  {subjects}
  {rows}
  {visibleRows}
  {columns}
  {isLoading}
  {loadError}
  {canEdit}
  {isDirty}
  {busy}
  {isSaving}
  {isExporting}
  {isImporting}
  {isCalculating}
  onRefresh={reload}
  onSave={save}
  onRowsChange={applyRows}
  onCalculate={calculateRankings}
  onExport={exportWorkbook}
  onImport={() => importInput?.click()}
/>
<input bind:this={importInput} type="file" accept=".xlsx,.xls" class="hidden" onchange={importWorkbook} />
