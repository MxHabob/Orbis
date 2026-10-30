<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { api, extractErrorMessage } from "$lib/api";
import { Skeleton } from "@orbis/ui/skeleton";
import { toast } from "@orbis/ui";
import { slide } from "svelte/transition";
import { academicContext, authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";

import {
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
  scoreToGradeLetter,
  type GradeMatrixResponse,
  type GradeMatrixStudent,
  type GradeCell,
} from "./grade-helpers";
import type { CellEditEvent } from "@orbis/svelte-grid";
import { ControlGrid } from "../academics";

let { orgId }: { orgId: string } = $props();

let periods = $derived(academicContext.allPeriods);
let subjects = $state<any[]>([]);
let selectedPeriodId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");
let isLoading = $state(true);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isDirty = $state(false);
let matrixData = $state<GradeMatrixResponse | null>(null);
let gridSource = $state<Record<string, unknown>[]>([]);
let assessmentKeys = $state<string[]>([]);
let previousSelectedClassId: string | null = null;

onMount(() => {
  if (academicContext.effectivePeriodId)
    selectedPeriodId = academicContext.effectivePeriodId;
  if (academicContext.selectedClassId)
    selectedClassId = academicContext.selectedClassId;
  if (academicContext.selectedSubjectId)
    selectedSubjectId = academicContext.selectedSubjectId;
  previousSelectedClassId = selectedClassId;
});

$effect(() => {
  if (selectedPeriodId) academicContext.selectedPeriodId = selectedPeriodId;
});
$effect(() => {
  if (selectedClassId) academicContext.selectedClassId = selectedClassId;
});
$effect(() => {
  if (selectedSubjectId) academicContext.selectedSubjectId = selectedSubjectId;
});

let selectedPeriod = $derived(
  periods.find((c) => String(c.id) === selectedPeriodId),
);
let isPeriodLocked = $derived(Boolean(selectedPeriod?.is_locked));
let isGradeEntryOpen = $derived(Boolean(selectedPeriod?.is_grade_entry_open));
let canEnterGrades = $derived(
  !isPeriodLocked && isGradeEntryOpen && !academicContext.isReadOnly,
);

let gridColumns = $derived.by(() => {
  if (!matrixData) return [];
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
    labels: {
      rowIndex: m.common_control_col_index(),
      studentName: m.common_word_student(),
      total: m.common_grades_col_total(),
      percentage: m.common_control_col_percentage(),
      gradeLetter: m.common_control_col_grade(),
    },
  });
});

async function loadInitialData() {
  isLoading = true;
  try {
    subjects = await api.listSubjects();

    // If no period selected yet, use context's effective period
    if (!selectedPeriodId && academicContext.effectivePeriodId) {
      selectedPeriodId = academicContext.effectivePeriodId;
    }
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }
}

// Reset subject when class changes
$effect(() => {
  const currentClassId = selectedClassId;
  if (
    previousSelectedClassId !== null &&
    currentClassId !== previousSelectedClassId
  ) {
    selectedSubjectId = "";
    academicContext.selectedSubjectId = "";
  }
  previousSelectedClassId = currentClassId;
});

// ── Grade Matrix ───────────────────────────────────────────────────────────
async function loadGradeMatrix() {
  if (!selectedPeriodId || !selectedClassId || !selectedSubjectId) return;
  isMatrixLoading = true;
  matrixData = null;
  gridSource = [];
  isDirty = false;
  try {
    const [studentRes, sectionSubjects] = await Promise.all([
      api.listStudents({
        page: 1,
        page_size: 200,
        class_id: selectedClassId,
        period_id: selectedPeriodId,
      }),
      api.listSectionSubjects(selectedClassId, selectedPeriodId),
    ]);
    const studentItems = studentRes.items || [];
    const sectionSubject = sectionSubjects.find(
      (offering) => String(offering.subject_id) === String(selectedSubjectId),
    );
    const gradeItems = sectionSubject
      ? await api.listGrades({ SectionSubjectId: sectionSubject.id })
      : [];
    const assessmentConfigs = [
      { type: "monthly", max: 20 },
      { type: "final", max: 30 },
    ];
    const assessmentTypes = assessmentConfigs.map((c) => c.type);

    const students: GradeMatrixStudent[] = studentItems.map((st) => {
      const studentId = st.student?.id ?? st.id;
      const matchingGrades = gradeItems.filter(
        (grade) =>
          String(grade.student_id) === String(studentId) &&
          String(grade.period_id) === String(selectedPeriodId),
      );
      const cells: GradeCell[] = assessmentConfigs.map(({ type, max }) => {
        const grade = matchingGrades.find(
          (item) => Boolean(item.is_final) === (type === "final"),
        );
        const gradeMax = Number(grade?.max_score);
        return {
          grade_id: grade?.id ?? null,
          exam_type: type,
          score: grade ? Number(grade.score) : null,
          max_score: Number.isFinite(gradeMax) && gradeMax > 0 ? gradeMax : max,
        };
      });
      const total = cells.reduce((acc, cell) => acc + (cell.score || 0), 0);
      const maxTotal = cells.reduce((acc, cell) => acc + cell.max_score, 0);
      const percentage =
        maxTotal > 0 ? Math.round((total / maxTotal) * 100) : 0;
      return {
        enrollment_id:
          st.enrollment?.id ??
          matchingGrades.find((grade) => grade.enrollment_id)?.enrollment_id ??
          studentId,
        student_id: studentId,
        student_name:
          st.person?.full_name ||
          st.person?.display_name ||
          st.student?.student_number ||
          "",
        student_name_ar:
          st.person?.full_name_ar ||
          st.person?.full_name ||
          st.person?.display_name ||
          st.student?.student_number ||
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

    const result: GradeMatrixResponse = {
      period_id: selectedPeriodId,
      section_subject_id: sectionSubject?.id ?? "",
      students,
      assessment_types: assessmentTypes,
    };

    matrixData = result;
    assessmentKeys = assessmentTypes;

    gridSource = transformMatrixToSource(
      result,
      getLocale(),
      academicContext.gradeScaleItems,
    );
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Error loading grade matrix");
  } finally {
    isMatrixLoading = false;
  }
}

$effect(() => {
  if (selectedPeriodId && selectedClassId && selectedSubjectId) {
    loadGradeMatrix();
  }
});

// ── Cell Pencil Handler ──────────────────────────────────────────────────────
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

// ── Save All Grades ────────────────────────────────────────────────────────
async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;

  try {
    const items: any[] = [];
    for (const row of gridSource) {
      for (const key of assessmentKeys) {
        const score = row[`exam_${key}`];
        if (score !== null && score !== undefined && score !== "") {
          const max = key === "final" ? 30 : 20;
          items.push({
            student_id: row.student_id,
            period_id: selectedPeriodId,
            section_subject_id: matrixData.section_subject_id,
            score: String(Number(score)),
            max_score: max,
            enrollment_id: row.enrollment_id || undefined,
            notes: null,
          });
        }
      }
    }

    if (items.length === 0) {
      toast.warning(m.common_action_save());
      isSaving = false;
      return;
    }

    await api.batchRecordGrades({ items });
    toast.success(
      m.common_control_save_success({ total: String(items.length) }),
    );

    isDirty = false;
    await loadGradeMatrix();
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e) || "Error saving grades");
  } finally {
    isSaving = false;
  }
}

let isCalculatingRankings = $state(false);

async function handleCalculateRankings() {
  if (!selectedPeriodId) return;
  isCalculatingRankings = true;
  try {
    await api.calculateRankings({
      period_id: selectedPeriodId,
      class_id: selectedClassId || undefined,
    });
    toast.success(m.exams_control_rankings_calculated());
    await loadGradeMatrix();
  } catch (err: unknown) {
    toast.error(extractErrorMessage(err));
  } finally {
    isCalculatingRankings = false;
  }
}

onMount(() => {
  if (orgId) loadInitialData();
});
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-4 select-none text-start">
  {#if isLoading}
    <div class="flex flex-col gap-4">
      <Skeleton class="h-64 w-full rounded-xl" />
    </div>
  {:else}
    <!-- Warnings -->
    {#if selectedPeriod && isPeriodLocked}
      <div
        class="rounded-xl border border-destructive/20 bg-card p-4"
        in:slide={{ duration: 200 }}
      >
        <div class="text-start">
          <p class="text-xs text-destructive">{m.common_control_period_locked_title()}</p>
          <p class="mt-1 text-xs text-muted-foreground">{m.common_control_period_locked_desc()}</p>
        </div>
      </div>
    {:else if selectedPeriod && !isGradeEntryOpen}
      <div
        class="rounded-xl border border-amber-500/20 bg-card p-4"
        in:slide={{ duration: 200 }}
      >
        <div class="text-start">
          <p class="text-xs text-amber-600">{m.common_control_entry_closed_title()}</p>
          <p class="mt-1 text-xs text-muted-foreground">{m.common_control_entry_closed_desc()}</p>
        </div>
      </div>
    {/if}

    <ControlGrid
      bind:selectedClassId
      bind:selectedSubjectId
      {matrixData}
      {gridSource}
      {gridColumns}
      {isMatrixLoading}
      {isSaving}
      {isDirty}
      {canEnterGrades}
      {selectedPeriodId}
      onRefresh={loadGradeMatrix}
      onSaveAll={handleSaveAll}
      onCellEdit={handleCellEdit}
      onCalculateRankings={handleCalculateRankings}
      isCalculating={isCalculatingRankings}
    />
  {/if}
</div>
