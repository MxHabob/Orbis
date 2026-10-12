<!--
  @component SectionSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { api } from "$lib/api";
import { Badge } from "@orbis/ui/badge";
import { Button } from "@orbis/ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
} from "@orbis/ui/card";
import { toast } from "@orbis/ui";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import { OrbisGrid, type CellEditEvent } from "@orbis/svelte-grid";
import { DocumentCheck, Refresh } from "@orbis/icons";
import {
  buildGradeColumns,
  recalculateRowTotals,
} from "$lib/features/exams/grade-helpers";

import { createGradesQuery } from "$lib/features/academics/queries";

let { orgId, classId, SectionSubjectId }: { orgId: string; classId: string; SectionSubjectId: string } = $props();

// Matrix Data (Coursework)
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const canEnterGrades = $derived(!academicContext.isReadOnly);

const gradesQuery = createGradesQuery(() => SectionSubjectId);
const isMatrixLoading = $derived(gradesQuery.isLoading);

let gridSource = $state<any[]>([]);
let assessmentKeys = $state<string[]>([]);
let isSaving = $state(false);
let isDirty = $state(false);

const matrixData = $derived.by(() => {
  const grades = gradesQuery.data ?? [];
  return { students: [] };
});

const gridColumns = $derived.by(() => {
  if (!matrixData) return [];
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
  });
});

function handleRefresh() {
  gradesQuery.refetch();
  isDirty = false;
}

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
        period_id: selectedPeriodId,
        student_id: student.student_id || student.enrollment_id,
        enrollment_id: student.enrollment_id,
        section_subject_id: SectionSubjectId,
        score: student.score,
        notes: student.notes ?? undefined,
      });
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

<div class="flex flex-col h-full w-full gap-2.5 font-sans">
  <Card class="flex-1 flex flex-col min-h-0 overflow-hidden">
    <!-- Top Action Toolbar in CardHeader -->
    <CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
      <div class="flex items-center gap-2.5">
        <div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
          <DocumentCheck class="w-4 h-4" />
        </div>
        <div class="flex flex-col text-start">
          <div class="flex items-center gap-2">
            <CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
              {m.shared_ui_coursework_gradebook_title()}
            </CardTitle>
            {#if isDirty}
              <Badge variant="outline" class="bg-amber-500/10 text-[9px] text-amber-600 border-amber-500/30 font-bold animate-pulse">
                {m.common_unsaved_changes()}
              </Badge>
            {:else if gridSource.length > 0}
              <Badge variant="outline" class="bg-emerald-500/10 text-[9px] text-emerald-600 border-emerald-500/20 font-medium">
                {m.common_all_saved()}
              </Badge>
            {/if}
          </div>
          <CardDescription class="text-[10px]">
            {gridSource.length} {m.common_students_count_label()}
          </CardDescription>
        </div>
      </div>

      <!-- Actions -->
      <CardAction>
        <div class="flex items-center gap-2">
          <Button 
            variant="outline" 
            size="sm" 
            class="h-8 px-2.5 text-xs font-semibold gap-1.5 cursor-pointer shadow-2xs" 
            onclick={handleRefresh} 
            disabled={isMatrixLoading}
            title={m.common_retry()}
          >
            {#if isMatrixLoading}<LoadingIndicator  size={12} />{:else}<Refresh size={12}   />{/if}
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
      </CardAction>
    </CardHeader>
    
    <!-- Main Grid Canvas -->
    <CardContent class="p-0 flex-1 min-h-0 flex flex-col overflow-hidden">
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
        <div class="flex-1 min-h-0 w-full h-full">
          <OrbisGrid
            columns={gridColumns}
            bind:source={gridSource}
            loading={isMatrixLoading}
            height="100%"
            readonly={!canEnterGrades}
            oncelledit={handleCellEdit}
            emptyTitle={m.common_students_empty_title()}
          />
        </div>
      {/if}
    </CardContent>
  </Card>
</div>
