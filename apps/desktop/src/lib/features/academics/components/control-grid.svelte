<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";
import type { AcademicPeriodDto } from "$lib/api";
import { LoadingIndicator } from "@orbis/ui";
import { Button } from "@orbis/ui/button";
import { OrbisGrid, type GridRowsChangeEvent, type OrbisColumnDef } from "@orbis/svelte-grid";
import SearchableSelect from "$lib/shared/components/searchable-select.svelte";
import ClassSelect from "./class-select.svelte";
import type { SheetRow, SheetSubject } from "../../exams/class-grade-sheet";

interface Props {
  classId: string | null;
  periodId: string | null;
  subjectId: string | null;
  studentId: string | null;
  periods: AcademicPeriodDto[];
  period: AcademicPeriodDto | undefined;
  subjects: SheetSubject[];
  rows: SheetRow[];
  visibleRows: SheetRow[];
  columns: OrbisColumnDef[];
  isLoading: boolean;
  loadError: boolean;
  canEdit: boolean;
  isDirty: boolean;
  busy: boolean;
  isSaving: boolean;
  isExporting: boolean;
  isImporting: boolean;
  isCalculating: boolean;
  onRefresh: () => void;
  onSave: () => void;
  onRowsChange: (event: GridRowsChangeEvent) => void;
  onCalculate: () => void;
  onExport: () => void;
  onImport: () => void;
}

let {
  classId = $bindable(),
  periodId = $bindable(),
  subjectId = $bindable(),
  studentId = $bindable(),
  periods, period, subjects, rows, visibleRows, columns,
  isLoading, loadError, canEdit, isDirty, busy, isSaving,
  isExporting, isImporting, isCalculating,
  onRefresh, onSave, onRowsChange, onCalculate, onExport, onImport,
}: Props = $props();

const arabic = $derived(getLocale() === "ar");
const configuredSubjects = $derived(subjects.filter((subject) => subject.components.length > 0));
const hasSheet = $derived(Boolean(configuredSubjects.length && rows.length));
</script>

<section class="flex h-full min-h-0 flex-col gap-3 overflow-hidden p-4" aria-label={m.academics_nav_grade_entry()}>
  <div class="grid shrink-0 grid-cols-2 gap-3 xl:grid-cols-4">
    <div class="flex min-w-0 flex-col gap-1.5">
      <label for="grade-sheet-period" class="text-xs font-medium">{m.common_lesson_label_period()}</label>
      <SearchableSelect
        id="grade-sheet-period"
        bind:value={periodId}
        items={periods}
        disabled={busy || isDirty}
        placeholder={m.common_lesson_label_period()}
        getItemValue={(item) => item.id}
        getItemLabel={(item) => (arabic && item.name_ar) || item.name}
      />
    </div>
    <div class="flex min-w-0 flex-col gap-1.5">
      <label for="grade-sheet-class" class="text-xs font-medium">{m.exams_grade_sheet_class()}</label>
      <ClassSelect id="grade-sheet-class" bind:value={classId} disabled={!periodId || busy || isDirty} class="w-full" />
    </div>
    <div class="flex min-w-0 flex-col gap-1.5">
      <label for="grade-sheet-subject" class="text-xs font-medium">{m.exams_grade_sheet_subject()}</label>
      <SearchableSelect
        id="grade-sheet-subject"
        bind:value={subjectId}
        items={configuredSubjects}
        disabled={!hasSheet || busy}
        showAllOption
        allOptionLabel={m.shared_ui_all_subjects()}
        getItemValue={(item) => item.id}
        getItemLabel={(item) => (arabic && item.name_ar) || item.name}
      />
    </div>
    <div class="flex min-w-0 flex-col gap-1.5">
      <label for="grade-sheet-student" class="text-xs font-medium">{m.common_word_student()}</label>
      <SearchableSelect
        id="grade-sheet-student"
        bind:value={studentId}
        items={rows}
        disabled={!hasSheet || busy}
        showAllOption
        allOptionLabel={m.exams_grade_sheet_all_students()}
        getItemValue={(item) => item.student_id}
        getItemLabel={(item) => arabic ? item.student_name_ar : item.student_name}
      />
    </div>
  </div>

  {#if period?.is_locked || (period && !period.is_grade_entry_open)}
    <p class="shrink-0 rounded-lg border bg-muted px-3 py-2 text-sm">
      {period.is_locked ? m.common_control_period_locked_title() : m.common_control_entry_closed_title()}
    </p>
  {/if}

  {#if hasSheet}
    <div class="flex shrink-0 flex-wrap items-center gap-2">
      <span class="me-auto text-xs text-muted-foreground" role="status">
        {isDirty ? m.common_unsaved_changes() : m.exams_grade_sheet_student_count({ count: visibleRows.length })}
      </span>
      <Button variant="outline" size="sm" onclick={onRefresh} disabled={busy}>
        {isDirty ? m.exams_grade_sheet_discard() : m.common_control_refresh()}
      </Button>
      <Button variant="outline" size="sm" onclick={onExport} disabled={isExporting || busy}>
        {m.common_action_export()}
      </Button>
      {#if canEdit}
        <Button variant="outline" size="sm" onclick={onImport} disabled={busy}>
          {#if isImporting}<LoadingIndicator size={12} />{/if}
          {m.common_action_import()}
        </Button>
        {#if subjectId && subjectId !== "all"}
          <Button variant="outline" size="sm" onclick={onCalculate} disabled={busy || isDirty}>
            {#if isCalculating}<LoadingIndicator size={12} />{/if}
            {m.common_control_calculate_ranking()}
          </Button>
        {/if}
        <Button variant="lime" size="sm" onclick={onSave} disabled={busy || !isDirty}>
          {#if isSaving}<LoadingIndicator size={12} />{/if}
          {m.common_action_save()}
        </Button>
      {/if}
    </div>
  {/if}

  <div class="min-h-0 flex-1 overflow-hidden rounded-lg border bg-card">
    {#if hasSheet}
      {#key `${classId}:${periodId}:${subjectId}:${studentId}`}
        <OrbisGrid
          {columns}
          source={visibleRows}
          rowIdKey="student_id"
          height="100%"
          rowSize={34}
          headerRowSize={32}
          rtl={arabic}
          readonly={!canEdit || busy}
          fixedRows
          toolbar={false}
          onrowschange={onRowsChange}
        />
      {/key}
    {:else}
      <div class="flex h-full flex-col items-center justify-center gap-3 p-6 text-center text-sm text-muted-foreground">
        {#if isLoading}
          <LoadingIndicator size={20} />
          <p role="status">{m.common_control_loading()}</p>
        {:else if loadError}
          <p role="alert">{m.exams_grade_sheet_load_error()}</p>
          <Button variant="outline" size="sm" onclick={onRefresh}>{m.common_action_retry()}</Button>
        {:else if !classId || !periodId}
          <p>{m.exams_grade_sheet_choose_class()}</p>
        {:else if !subjects.length}
          <p>{m.common_control_no_subjects_for_class()}</p>
        {:else if !rows.length}
          <p>{m.exams_grade_sheet_no_students()}</p>
        {:else}
          <p>{m.exams_grade_sheet_no_components()}</p>
        {/if}
      </div>
    {/if}
  </div>
</section>
