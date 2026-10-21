<!--
  @component SectionSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
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
  scoreToGradeLetter,
  transformMatrixToSource,
  type GradeCell,
  type GradeMatrixResponse,
  type GradeMatrixStudent,
} from "$lib/features/exams/grade-helpers";

import {
  createClassStudentsQuery,
  createGradesQuery,
} from "$lib/features/academics/queries";

let { orgId, classId, SectionSubjectId }: { orgId: string; classId: string; SectionSubjectId: string } = $props();

// Matrix Data (Coursework)
const selectedPeriodId = $derived(academicContext.effectivePeriodId || "");
const canEnterGrades = $derived(!academicContext.isReadOnly);

const studentsQuery = createClassStudentsQuery(() => classId);
const gradesQuery = createGradesQuery(() => SectionSubjectId);
const isMatrixLoading = $derived(
  studentsQuery.isLoading || gradesQuery.isLoading,
);
const error = $derived(studentsQuery.error || gradesQuery.error);

let isSaving = $state(false);
let isDirty = $state(false);

const matrixData = $derived.by((): GradeMatrixResponse => {
  const roster = studentsQuery.data ?? [];
  const grades = gradesQuery.data ?? [];
  const assessmentConfigs = [
    { type: "monthly", isFinal: false, max: 20 },
    { type: "final", isFinal: true, max: 30 },
  ];
  const students: GradeMatrixStudent[] = roster.map((student: any) => {
    const studentId = student.student?.id ?? student.id;
    const studentGrades = grades.filter(
      (grade) =>
        String(grade.student_id) === String(studentId) &&
        String(grade.period_id) === String(selectedPeriodId),
    );
    const cells: GradeCell[] = assessmentConfigs.map(
      ({ type, isFinal, max }) => {
        const grade = studentGrades.find(
          (item) => item.is_final === isFinal,
        );
        const gradeMax = Number(grade?.max_score);
        return {
          grade_id: grade?.id ?? null,
          assessment_id: grade?.component_id ?? null,
          exam_type: type,
          score: grade ? Number(grade.score) : null,
          max_score: Number.isFinite(gradeMax) && gradeMax > 0 ? gradeMax : max,
        };
      },
    );
    const total = cells.reduce((sum, cell) => sum + (cell.score || 0), 0);
    const maxTotal = cells.reduce((sum, cell) => sum + cell.max_score, 0);
    const percentage =
      maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;

    return {
      enrollment_id:
        student.enrollment?.id ??
        studentGrades.find((grade) => grade.enrollment_id)?.enrollment_id ??
        "",
      student_id: studentId,
      student_name:
        student.person?.full_name ||
        student.person?.display_name ||
        student.student?.student_number ||
        "",
      student_name_ar:
        student.person?.full_name_ar ||
        student.person?.full_name ||
        student.person?.display_name ||
        student.student?.student_number ||
        null,
      grades: cells,
      total,
      max_total: maxTotal,
      percentage,
      grade_letter: scoreToGradeLetter(
        percentage,
        academicContext.gradeScaleItems,
      ),
    };
  });

  return {
    period_id: selectedPeriodId,
    section_subject_id: SectionSubjectId,
    students,
    assessment_types: assessmentConfigs.map(({ type }) => type),
  };
});

let gridSource = $derived.by(() =>
  transformMatrixToSource(
    matrixData,
    getLocale(),
    academicContext.gradeScaleItems,
  ),
);
const assessmentKeys = $derived(matrixData.assessment_types);

const gridColumns = $derived.by(() => {
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
  });
});

function handleRefresh() {
  studentsQuery.refetch();
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
        const score = row[`exam_${key}`];
        if (score !== null && score !== undefined && score !== "") {
          studentsToSave.push({
            student_id: row.student_id,
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
        student_id: String(student.student_id),
        enrollment_id: student.enrollment_id || undefined,
        section_subject_id: SectionSubjectId,
        score: String(student.score),
        max_score: String(student.exam_type === "final" ? 30 : 20),
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
      {#if error}
        <div class="flex flex-1 flex-col items-center justify-center gap-2 p-6 text-center text-destructive">
          <p class="text-xs font-semibold">{m.common_load_error()}</p>
          <p class="text-[10px] text-muted-foreground">
            {extractErrorMessage(error)}
          </p>
          <Button variant="outline" size="sm" class="h-8 mt-2 text-xs" onclick={handleRefresh}>
            {m.common_retry()}
          </Button>
        </div>
      {:else if !gridColumns.length && !isMatrixLoading}
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
