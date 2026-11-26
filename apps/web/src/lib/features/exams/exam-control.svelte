<script lang="ts">
import { periodOverrides } from "$lib/shared/utils/period-override.svelte";
import { gradePeriodOverride, isGradePeriodOverrideable } from "$lib/api/period-override";

import { onMount, untrack } from "svelte";
import * as XLSX from "xlsx";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { api, extractErrorMessage, type RecordGradeItemRequest } from "$lib/api";
import { academicContext, authStore, effectiveContext, uiStore } from "$lib/stores";
import { useUnsavedGuard } from "$lib/stores/unsaved-guard.svelte";
import { confirmStore } from "$lib/stores/confirm.svelte";
import { toast } from "@orbis/ui";
import type { GridRowsChangeEvent } from "@orbis/svelte-grid";
import { ControlGrid } from "../academics";
import { orgContext } from "$lib/stores/org-context.svelte";
import BulkGradeWorkbooks from "./components/bulk-grade-workbooks.svelte";
import { buildGradeWorkbook, downloadGradeWorkbook, isGradeWorkbook, reviewGradeWorkbook } from "./grade-workbook";
import {
  buildGradeGridColumns, isGradeEntryOpen, loadClassGradeSheet, makeGradeItem, parseSheetScore, savedScore, withSheetTotals,
  type SheetComponent, type SheetRow, type SheetScore, type SheetSubject,
} from "./class-grade-sheet";

let { orgId, bulk = false }: { orgId: string; bulk?: boolean } = $props();

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
let rosterPeriodId = $state<string | null>(null);
let scopeCanEnter = $state<boolean | null>(null);
let bulkBusy = $state(false);
let reviewingBulkWorkbook = $state(false);
const guard = useUnsavedGuard({ title: m.common_unsaved_changes() });
const period = $derived(academicContext.allPeriods.find((item) => item.id === periodId));
const entryOpen = $derived(scopeCanEnter ?? isGradeEntryOpen(period));
const canWriteGrades = $derived(effectiveContext.hasPermission("grade:write") && !academicContext.isReadOnly);
const canEdit = $derived(entryOpen && canWriteGrades);
const arabic = $derived(getLocale() === "ar");
const visibleSubjects = $derived(subjects.filter((subject) =>
  subject.components.length > 0 && (!subjectId || subjectId === "all" || subject.id === subjectId)));
const canRequestOverride = $derived(
  canWriteGrades && effectiveContext.hasPermission("workflow:write") &&
  isGradePeriodOverrideable(period) && visibleSubjects.length > 0,
);
function requestGradeOverride() {
  const activePeriodId = periodId;
  if (activePeriodId) {
    periodOverrides.open(orgId, visibleSubjects.map((subject) => gradePeriodOverride(activePeriodId, subject.id)));
  }
}
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
        items.push(makeGradeItem(periodId, subject, component, row, value));
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

const columns = $derived(buildGradeGridColumns(visibleSubjects, {
  arabic,
  validate: canEdit ? validateScore : undefined,
  showTotals: !subjectId || subjectId === "all",
  showGrade: academicContext.gradeScaleItems.length > 0,
}));

onMount(() => {
  classId = academicContext.selectedClassId || null;
  periodId = uiStore.getPreference(
    { userId: authStore.userId, organizationId: orgId },
    "exams.selectedPeriodId", "", (value): value is string => typeof value === "string",
  ) || null;
});

$effect(() => {
  const periods = academicContext.allPeriods;
  if (periodId && periods.length && !periods.some((item) => item.id === periodId)) periodId = null;
  if (!periodId) {
    const available = periods.filter(isGradeEntryOpen);
    if (available.length === 1) periodId = available[0]!.id;
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
  rosterPeriodId = null;
  scopeCanEnter = null;
  loadError = false;
  isLoading = Boolean(selectedClass && selectedPeriod);
  if (!selectedClass || !selectedPeriod) return;
  try {
    const periods = await api.listAcademicPeriods();
    const sheet = await loadClassGradeSheet(selectedClass, selectedPeriod, periods);
    if (version !== loadVersion) return;
    subjects = sheet.subjects;
    rosterPeriodId = sheet.rosterPeriodId;
    scopeCanEnter = sheet.canEnter;
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
    if (!periodOverrides.handleError(orgId, error)) toast.error(extractErrorMessage(error));
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
  if (!classId || !period || !rosterPeriodId || !subjects.length || !rows.length || bulkBusy) return;
  const scope = { classId, rosterPeriodId, rows, subjects, canEnter: canEdit };
  const selectedPeriod = period;
  isExporting = true;
  try {
    const section = await api.getSection(scope.classId);
    const workbook = buildGradeWorkbook({
      orgId, periodId: selectedPeriod.id, orgName: orgContext.displayName,
      periodLabel: (arabic && selectedPeriod.name_ar) || selectedPeriod.name,
      yearLabel: arabic ? academicContext.yearLabelAr : academicContext.yearLabel, arabic,
    }, [{ ...scope, className: (arabic && section.name_ar) || section.name }]);
    if (await downloadGradeWorkbook(workbook, "orbis-grades-" + section.id.slice(0, 8) + ".xlsx")) {
      toast.success(m.common_action_export());
    }
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    isExporting = false;
  }
}

async function stageClassWorkbook(workbook: XLSX.WorkBook) {
  const sectionId = classId;
  const selectedPeriodId = periodId;
  if (!sectionId || !selectedPeriodId) return;
  const [periods, section] = await Promise.all([api.listAcademicPeriods(), api.getSection(sectionId)]);
  const plan = await reviewGradeWorkbook(workbook, { orgId, periodId: selectedPeriodId }, async (scopeId) => {
    if (scopeId !== sectionId) throw new Error(m.exams_workbook_use_bulk());
    return {
      ...await loadClassGradeSheet(scopeId, selectedPeriodId, periods),
      classId: scopeId, className: (arabic && section.name_ar) || section.name,
    };
  }, arabic);
  if (plan.errors.length) {
    toast.error(plan.errors[0]);
    return;
  }
  const byId = new Map(rows.map((row) => [row.student_id, row]));
  const byStudent = new Map<string, Record<string, SheetScore>>();
  for (const change of plan.changes) {
    const row = byId.get(change.studentId);
    const saved = row?.savedGrades[change.field];
    const current = row?.[change.field] ?? null;
    if (!row || (saved?.updated_at ?? null) !== change.revision ||
        (current !== savedScore(saved) && current !== change.after)) {
      toast.error(m.exams_workbook_grade_changed());
      return;
    }
    const edits = byStudent.get(change.studentId) ?? {};
    edits[change.field] = change.after;
    byStudent.set(change.studentId, edits);
  }
  rows = rows.map((row) => {
    const edits = byStudent.get(row.student_id);
    return edits ? withSheetTotals({ ...row, ...edits }, subjects, academicContext.gradeScaleItems) : row;
  });
  if (plan.changes.length) {
    subjectId = "all";
    studentId = "all";
    toast.success(m.exams_workbook_staged({ count: plan.changes.length }));
  } else {
    toast.warning(m.exams_workbook_no_changes());
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
    if (isGradeWorkbook(workbook)) {
      await stageClassWorkbook(workbook);
      return;
    }
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
    const arabicStudentIdHeader = m.exams_grade_sheet_student_id_header(
      {},
      { locale: "ar" },
    );
    const imported = new Map(
      XLSX.utils
        .sheet_to_json<Record<string, unknown>>(sheet)
        .map((row) => [
          String(
            row.student_id ??
              row.studentId ??
              row.StudentId ??
              row[arabicStudentIdHeader] ??
              "",
          ).trim(),
          row,
        ]),
    );
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
  {entryOpen}
  {isDirty}
  busy={busy || bulkBusy}
  workbookReview={reviewingBulkWorkbook}
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
  onRequestOverride={canRequestOverride ? requestGradeOverride : undefined}
>
  {#snippet children()}
    {#if bulk && period}
      {#key orgId + ":" + period.id}
        <BulkGradeWorkbooks
          {orgId}
          {period}
          blocked={busy || isDirty || isExporting}
          canEdit={canWriteGrades}
          onBusyChange={(value) => (bulkBusy = value)}
          onReviewChange={(value) => (reviewingBulkWorkbook = value)}
          onSaved={() => { if (classId) void refresh(); }}
        />
      {/key}
    {/if}
  {/snippet}
</ControlGrid>
<input bind:this={importInput} type="file" accept=".xlsx,.xls" class="hidden" onchange={importWorkbook} />
