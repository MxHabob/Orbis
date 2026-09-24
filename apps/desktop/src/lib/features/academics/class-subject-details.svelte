<!--
  @component ClassSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving.
-->
<script lang="ts">
import { api } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import type { CellEditEvent } from "@orbis/svelte-grid";
import {
  DataTable,
  buildGradeColumns,
  recalculateRowTotals,
} from "$lib/tables";

let { orgId, classId, classSubjectId }: { orgId: string; classId: string; classSubjectId: string } = $props();

// Matrix Data (Coursework)
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const canEnterGrades = $derived(!academicContext.isReadOnly);

let matrixData = $state<any>(null);
let gridSource = $state<any[]>([]);
let assessmentKeys = $state<string[]>([]);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isDirty = $state(false);

const gridColumns = $derived.by(() => {
  if (!matrixData) return [];
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
  });
});

async function loadGradeMatrix() {
  isMatrixLoading = true;
  try {
    const grades = await api.listGrades({ classSubjectId }).catch(() => []);
    matrixData = { students: [] };
    assessmentKeys = [];
    gridSource = [];
    isDirty = false;
  } catch (e: any) {
    toast.error(e?.message || m.common_error_occurred());
  } finally {
    isMatrixLoading = false;
  }
}

$effect(() => {
  if (selectedPeriodId && classSubjectId) {
    loadGradeMatrix();
  }
});

function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, columnId, newValue } = event;
  const field = columnId;
  if (rowIndex < 0 || rowIndex >= gridSource.length) return;

  const updatedSource = [...gridSource];
  const row = { ...updatedSource[rowIndex]! };
  row[field] = newValue;
  recalculateRowTotals(row, assessmentKeys, academicContext.gradeScaleItems);

  updatedSource[rowIndex] = row;
  gridSource = updatedSource;
  isDirty = true;
}

async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;
  try {
    const studentsToSave: any[] = [];
    for (const row of gridSource) {
      for (const key of assessmentKeys) {
        const score = row[`score_${key}`];
        if (score !== null && score !== undefined) {
          studentsToSave.push({
            enrollment_id: row.enrollment_id,
            score: Number(score),
            notes: null,
            exam_type: key,
          });
        }
      }
    }

    if (studentsToSave.length === 0) {
      toast.info(m.common_no_changes_to_save());
      return;
    }

    for (const student of studentsToSave) {
      await api.recordGrade({
        enrollment_id: student.enrollment_id,
        class_subject_id: classSubjectId,
        score: student.score,
        assessment_type: student.exam_type || "exam",
      } as any);
    }

    toast.success(m.common_grades_save_success());
    isDirty = false;
  } catch (e: any) {
    toast.error(e?.message || m.common_grades_save_error());
  } finally {
    isSaving = false;
  }
}
</script>

<div class="flex flex-col h-full w-full gap-2.5">
  <!-- Top Action Toolbar -->
  <div class="flex items-center justify-between shrink-0 px-0.5">
    <div class="flex items-center gap-2">
      <div class="flex items-center gap-1.5 text-xs text-muted-foreground font-medium">
        <span>
          {gridSource.length} {m.common_students_count_label()}
        </span>
      </div>

      {#if isDirty}
        <Badge variant="outline" class="bg-amber-500/10 text-[10px] text-amber-600 border-amber-500/30 font-bold animate-pulse">
          {m.common_unsaved_changes()}
        </Badge>
      {:else if gridSource.length > 0}
        <Badge variant="outline" class="bg-emerald-500/10 text-[10px] text-emerald-600 border-emerald-500/20 font-medium">
          {m.common_all_saved()}
        </Badge>
      {/if}
    </div>

    <!-- Actions -->
    <div class="flex items-center gap-2">
      <Button 
        variant="outline" 
        size="sm" 
        class="h-8 px-2.5 text-xs font-semibold gap-1.5 cursor-pointer shadow-2xs" 
        onclick={loadGradeMatrix} 
        disabled={isMatrixLoading}
        title={m.common_retry()}
      >
        <span>{m.common_retry()}</span>
      </Button>

      <Button 
        variant={isDirty ? "default" : "secondary"} 
        size="sm" 
        class="h-8 px-3.5 gap-1.5 text-xs font-bold cursor-pointer shadow-2xs transition-all" 
        disabled={!isDirty || isSaving || !canEnterGrades} 
        onclick={handleSaveAll}
      >
        <span>{m.common_save_changes()}</span>
      </Button>
    </div>
  </div>
  
  <!-- Main Grid Canvas -->
  <div class="flex-1 w-full overflow-hidden border border-border/70 rounded-xl shadow-xs bg-card">
    {#if !gridColumns.length && !isMatrixLoading}
      <div class="flex flex-col h-full items-center justify-center p-8 text-center bg-muted/5 gap-2">
        <p class="text-xs font-bold text-foreground">
          {m.common_assessment_no_scheme()}
        </p>
        <p class="text-[11px] text-muted-foreground max-w-sm">
          {m.common_assessment_configure_hint()}
        </p>
      </div>
    {:else}
      <DataTable
        columns={gridColumns}
        source={gridSource}
        loading={isMatrixLoading}
        height="100%"
        readonly={!canEnterGrades}
        oncelledit={handleCellEdit}
        emptyTitle={m.common_students_empty_title()}
      />
    {/if}
  </div>
</div>
