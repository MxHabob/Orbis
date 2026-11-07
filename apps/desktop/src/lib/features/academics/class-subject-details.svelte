<!--
  @component SectionSubjectDetails
  Feature component for the Coursework Gradebook spreadsheet.
  Provides interactive cell editing, total recalculation, and batch grade saving inside a standardized Card container.
-->
<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import { api, extractErrorMessage } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { toast } from "@orbis/ui";
import SettingsBottomBar from "$lib/shared/components/settings-bottom-bar.svelte";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { academicContext } from "$lib/stores";
import { OrbisGrid, type CellEditEvent } from "@orbis/svelte-grid";
import { Refresh } from "@orbis/icons";
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

let {
  orgId,
  classId,
  SectionSubjectId,
}: { orgId: string; classId: string; SectionSubjectId: string } = $props();

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
        const grade = studentGrades.find((item) => item.is_final === isFinal);
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
    const percentage = maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;

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

<div class="h-full min-h-0 w-full font-sans">
  {#if error}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-6 text-center text-destructive">
      <p class="text-xs font-semibold">{m.common_load_error()}</p>
      <p class="text-[10px] text-muted-foreground">
        {extractErrorMessage(error)}
      </p>
      <Button variant="outline" size="sm" class="mt-2 h-8 text-xs" onclick={handleRefresh}>
        {m.common_retry()}
      </Button>
    </div>
  {:else if !gridColumns.length && !isMatrixLoading}
    <div class="flex h-full flex-col items-center justify-center gap-2 p-8 text-center">
      <p class="text-xs font-semibold text-foreground">
        {m.common_assessment_no_scheme()}
      </p>
      <p class="max-w-sm text-[11px] text-muted-foreground">
        {m.common_assessment_configure_hint()}
      </p>
    </div>
  {:else}
    <OrbisGrid
      columns={gridColumns}
      bind:source={gridSource}
      loading={isMatrixLoading}
      height="100%"
      rtl={getLocale() === "ar"}
      showAddRow={false}
      readonly={!canEnterGrades}
      oncelledit={handleCellEdit}
      emptyTitle={m.common_students_empty_title()}
    >
      {#snippet toolbarActions()}
        <Button
          variant="outline"
          size="sm"
          class="h-8 gap-1.5 px-2.5 text-xs"
          onclick={handleRefresh}
          disabled={isMatrixLoading}
          title={m.common_retry()}
        >
          {#if isMatrixLoading}<LoadingIndicator size={12} />{:else}<Refresh size={12} />{/if}
          <span>{m.common_retry()}</span>
        </Button>
      {/snippet}
    </OrbisGrid>
  {/if}
</div>

<SettingsBottomBar
  visible={canEnterGrades && isDirty}
  isDirty={isDirty}
  isSaving={isSaving}
  showReset={false}
  message={m.common_unsaved_changes()}
  saveLabel={m.common_save_changes()}
  savingLabel={m.common_state_saving()}
  onsave={handleSaveAll}
/>
