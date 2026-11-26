<script lang="ts">
import { periodOverrides } from "$lib/shared/utils/period-override.svelte";
import { gradePeriodOverride, isGradePeriodOverrideable } from "$lib/api/period-override";

import * as XLSX from "xlsx";
import { onDestroy } from "svelte";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import { api, extractErrorMessage, type AcademicPeriodDto, type SectionDto } from "$lib/api";
import { academicContext, effectiveContext } from "$lib/stores";
import { orgContext } from "$lib/stores/org-context.svelte";
import { toast } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { OrbisGrid, type OrbisColumnDef, type GridRow } from "@orbis/svelte-grid";
import { createClassesQuery } from "../../academics/queries";
import { buildGradeGridColumns, loadClassGradeSheet, loadGradeSheetReferences, withSheetTotals } from "../class-grade-sheet";
import { buildGradeWorkbook, downloadGradeWorkbook, reviewGradeWorkbook, type GradeWorkbookPlan } from "../grade-workbook";

let { orgId, period, blocked, canEdit, onBusyChange, onReviewChange, onSaved }: {
  orgId: string;
  period: AcademicPeriodDto;
  blocked: boolean;
  canEdit: boolean;
  onBusyChange: (busy: boolean) => void;
  onReviewChange: (active: boolean) => void;
  onSaved: () => void;
} = $props();

const classes = createClassesQuery(() => orgId);
const arabic = $derived(getLocale() === "ar");
const label = (item: SectionDto | AcademicPeriodDto) => (arabic && item.name_ar) || item.name;
const sectionLabel = (section: SectionDto) => [label(section), section.code].filter(Boolean).join(" / ");
let selectedIds = $state<string[]>([]);
let filter = $state("");
const filtered = $derived((classes.data ?? []).filter((section) =>
  section.is_active && sectionLabel(section).toLocaleLowerCase().includes(filter.trim().toLocaleLowerCase()),
));
let busy = $state(false);
let fileInput: HTMLInputElement;
let stagedFile = $state.raw<{ workbook: XLSX.WorkBook; name: string } | null>(null);
let plan = $state.raw<GradeWorkbookPlan | null>(null);
let reviewClassId = $state("");
const reviewSheet = $derived(plan?.sheets.find((sheet) => sheet.classId === reviewClassId) ?? plan?.sheets[0]);
const newCount = $derived(plan?.changes.filter((change) => change.isNew).length ?? 0);
const editsByStudent = $derived.by(() => {
  const edits = new Map<string, Map<string, unknown>>();
  for (const change of plan?.changes ?? []) {
    if (change.classId !== reviewSheet?.classId) continue;
    const fields = edits.get(change.studentId) ?? new Map();
    fields.set(change.field, change.after);
    edits.set(change.studentId, fields);
  }
  return edits;
});
const previewRows = $derived(reviewSheet?.rows.map((row) => withSheetTotals(
  { ...row, ...Object.fromEntries(editsByStudent.get(row.student_id) ?? []) },
  reviewSheet.subjects, academicContext.gradeScaleItems,
)) ?? []);
const previewColumns = $derived(buildGradeGridColumns(reviewSheet?.subjects ?? [], {
  arabic,
  showGrade: Boolean(academicContext.gradeScaleItems.length),
}));

function highlightChange(row: GridRow, column: OrbisColumnDef) {
  return editsByStudent.get(String(row.student_id))?.has(String(column.id)) ? "bg-primary/15 font-semibold" : "";
}

function clearReview() {
  plan = null;
  stagedFile = null;
  reviewClassId = "";
  onReviewChange(false);
}

onDestroy(() => onReviewChange(false));

function toggle(id: string) {
  selectedIds = selectedIds.includes(id) ? selectedIds.filter((item) => item !== id) : [...selectedIds, id];
}

function setBusy(value: boolean) {
  busy = value;
  onBusyChange(value);
}

async function exportSections() {
  if (blocked || busy || !selectedIds.length) return;
  setBusy(true);
  try {
    const context = { orgId, periodId: period.id };
    const [periods, references] = await Promise.all([api.listAcademicPeriods(), loadGradeSheetReferences()]);
    const scopes = [];
    const empty: string[] = [];
    for (const classId of selectedIds) {
      const section = classes.data?.find((item) => item.id === classId);
      if (!section) continue;
      const scope = await loadClassGradeSheet(classId, period.id, periods, references);
      if (!scope.rows.length || !scope.subjects.some((subject) => subject.components.length)) {
        empty.push(sectionLabel(section));
        continue;
      }
      scopes.push({
        ...scope, classId, className: sectionLabel(section),
        rows: scope.rows.map((row) => withSheetTotals(row, scope.subjects, [])),
      });
    }
    if (empty.length || !scopes.length) {
      toast.warning(
        m.exams_workbook_empty_sections({
          sections: empty.join(m.common_list_separator()),
        }),
      );
      if (!scopes.length) return;
    }
    const workbook = buildGradeWorkbook({
      ...context, orgName: orgContext.displayName, periodLabel: label(period), yearLabel: arabic ? academicContext.yearLabelAr : academicContext.yearLabel, arabic,
    }, scopes);
    if (await downloadGradeWorkbook(workbook, `orbis-grades-${period.code || period.id.slice(0, 8)}.xlsx`)) {
      toast.success(m.common_action_export());
    }
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    setBusy(false);
  }
}

async function createPlan(workbook: XLSX.WorkBook) {
  const [periods, availableSections, references] = await Promise.all([api.listAcademicPeriods(), api.listSections(), loadGradeSheetReferences()]);
  const sections = new Map(availableSections.map((item) => [item.id, item]));
  return reviewGradeWorkbook(workbook, { orgId, periodId: period.id }, async (classId) => {
    const section = sections.get(classId);
    if (!section) throw new Error(m.exams_grade_sheet_file_context());
    const scope = await loadClassGradeSheet(classId, period.id, periods, references);
    return { ...scope, classId, className: sectionLabel(section) };
  }, arabic);
}

async function importFile(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];
  input.value = "";
  if (!file || blocked || busy || !canEdit) return;
  setBusy(true);
  clearReview();
  try {
    const workbook = XLSX.read(await file.arrayBuffer(), { type: "array" });
    plan = await createPlan(workbook);
    stagedFile = { workbook, name: file.name };
    onReviewChange(true);
  } catch (error) {
    toast.error(extractErrorMessage(error));
  } finally {
    setBusy(false);
  }
}

function fingerprint(review: GradeWorkbookPlan) {
  return JSON.stringify(review.changes.map(({ id, revision, item }) => [id, revision, item]));
}

async function saveReview() {
  if (!plan?.changes.length || plan.errors.length || !stagedFile || !canEdit || blocked || busy) return;
  setBusy(true);
  try {
    const refreshed = await createPlan(stagedFile.workbook);
    if (refreshed.errors.length || fingerprint(refreshed) !== fingerprint(plan)) {
      plan = refreshed;
      toast.warning(m.exams_workbook_review_again());
      return;
    }
    const result = await api.batchRecordGrades({ items: refreshed.changes.map((change) => change.item) });
    clearReview();
    toast.success(m.common_control_save_success({ total: String(result.count) }));
    onSaved();
  } catch (error) {
    if (!periodOverrides.handleError(orgId, error)) toast.error(extractErrorMessage(error));
  } finally {
    setBusy(false);
  }
}

const closedSheets = $derived(new Set(plan?.sheets.filter((sheet) => !sheet.canEnter).map((sheet) => sheet.classId)));
const closedChanges = $derived(plan?.changes.filter((change) => closedSheets.has(change.classId)) ?? []);
const canRequestOverride = $derived(canEdit && effectiveContext.hasPermission("workflow:write") &&
  isGradePeriodOverrideable(period) && closedChanges.length > 0);

function requestOverride() {
  periodOverrides.open(orgId, closedChanges.map(({ item }) =>
    gradePeriodOverride(item.period_id, item.section_subject_id),
  ));
}
</script>

<div class="flex min-h-0 flex-col gap-3 rounded-lg border bg-card p-3 {plan ? 'flex-1' : 'shrink-0'}">
  <div class="flex flex-wrap items-center gap-2">
    <details class="relative me-auto">
      <summary class="cursor-pointer rounded-md border px-3 py-1.5 text-sm font-medium">
        {m.exams_workbook_select_sections({ count: selectedIds.length })}
      </summary>
      <div class="absolute start-0 z-20 mt-1 w-72 max-w-[75vw] rounded-lg border bg-popover p-2 shadow-md">
        <input bind:value={filter} type="search" aria-label={m.common_control_select_class()}
          placeholder={m.common_control_select_class()} class="mb-2 w-full rounded-md border bg-background px-2 py-1 text-sm" />
        <div class="mb-1 flex flex-wrap items-center gap-1">
          <Button variant="ghost" size="sm" disabled={blocked || busy || !filtered.length}
            onclick={() => (selectedIds = Array.from(new Set([...selectedIds, ...filtered.map((item) => item.id)])))}>
            {m.exams_workbook_select_visible()}
          </Button>
          <Button variant="ghost" size="sm" disabled={blocked || busy || !selectedIds.length} onclick={() => (selectedIds = [])}>
            {m.operations_ui_clear_selection()}
          </Button>
        </div>
        <div class="max-h-44 overflow-y-auto">
          {#each filtered as section (section.id)}
            <label class="flex cursor-pointer items-center gap-2 rounded px-2 py-1 text-sm hover:bg-accent">
              <input type="checkbox" checked={selectedIds.includes(section.id)} disabled={blocked || busy}
                onchange={() => toggle(section.id)} />
              {label(section)} <span class="ms-auto text-xs text-muted-foreground">{section.code}</span>
            </label>
          {:else}
            <p class="px-2 py-1 text-xs text-muted-foreground">{classes.isLoading ? m.common_loading() : m.common_control_no_data()}</p>
          {/each}
        </div>
        {#if classes.isError}
          <Button variant="ghost" size="sm" onclick={() => classes.refetch()}>{m.common_action_retry()}</Button>
        {/if}
      </div>
    </details>
    <Button size="sm" variant="outline" disabled={blocked || busy || !selectedIds.length} onclick={exportSections}>
      {m.exams_workbook_export_sections()}
    </Button>
    <Button size="sm" variant="outline" disabled={blocked || busy || !canEdit} onclick={() => fileInput?.click()}>
      {m.exams_workbook_import_sections()}
    </Button>
    <input bind:this={fileInput} type="file" accept=".xlsx" class="hidden" onchange={importFile} />
  </div>
  {#if !plan && !busy}
    <p class="text-xs text-muted-foreground">{m.exams_workbook_prefilled()}</p>
  {/if}
  {#if busy}
    <p role="status" class="text-xs text-muted-foreground">{m.common_loading()}</p>
  {/if}
  {#if stagedFile && plan}
    <div class="flex min-h-0 flex-1 flex-col gap-2 border-t pt-2">
      <div class="flex flex-wrap items-center gap-2">
        <div class="me-auto text-sm">
          <p class="font-medium" dir="auto">{stagedFile.name}</p>
          <p class="text-xs text-muted-foreground" role="status">{m.exams_workbook_review_counts({ added: newCount, updated: plan.changes.length - newCount, errors: plan.errors.length })}</p>
        </div>
        <Button size="sm" variant="ghost" disabled={busy} onclick={clearReview}>
          {m.common_action_cancel()}
        </Button>
        {#if canRequestOverride}
          <Button size="sm" variant="outline" disabled={busy || plan.errors.length > 0} onclick={requestOverride}>
            {m.shared_ui_request_override()}
          </Button>
        {:else}
          <Button size="sm" variant="lime" disabled={blocked || busy || !canEdit || !plan.changes.length || plan.errors.length > 0 || closedChanges.length > 0} onclick={saveReview}>
            {m.common_action_save()}
          </Button>
        {/if}
      </div>
      {#if plan.errors.length}
        <ul class="max-h-24 shrink-0 list-inside list-disc overflow-y-auto text-xs text-destructive" aria-label={m.exams_workbook_review_errors()}>
          {#each plan.errors as error, index (index)}<li>{error}</li>{/each}
        </ul>
      {/if}
      {#if reviewSheet}
        <div class="flex shrink-0 flex-wrap items-center gap-2">
          <label for="workbook-review-section" class="text-sm font-medium">{m.exams_grade_sheet_class()}</label>
          <select id="workbook-review-section" class="rounded-md border bg-background px-2 py-1 text-sm"
            value={reviewSheet.classId} onchange={(event) => (reviewClassId = event.currentTarget.value)}>
            {#each plan.sheets as sheet (sheet.classId)}
              <option value={sheet.classId}>{sheet.className}</option>
            {/each}
          </select>
          {#if plan.changes.length}
            <span class="rounded bg-primary/15 px-2 py-1 text-xs">{m.exams_workbook_highlighted()}</span>
          {/if}
        </div>
        <div class="min-h-0 flex-1 overflow-hidden rounded border">
          {#key reviewSheet.classId}
            <OrbisGrid columns={previewColumns} source={previewRows} rowIdKey="student_id"
              readonly rtl={arabic} toolbar={false} cellStyle={highlightChange} rowSize={34} headerRowSize={32} height="100%" />
          {/key}
        </div>
      {/if}
    </div>
  {/if}
</div>
