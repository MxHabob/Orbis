<script lang="ts">
import { onMount } from "svelte";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  ShieldCheck,
  Lock,
  AlertTriangle,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { slide } from "svelte/transition";
import { academicContext, authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";
import { ControlGrid } from "$lib/features/academics";
import {
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
} from "@orbis/data-grid";
import type { CellEditEvent, GradeMatrixResponse } from "@orbis/data-grid";

let { data } = $props();
const orgId = $derived(data?.orgId || "");
let cycles = $derived(academicContext.allCycles);
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);
let selectedCycleId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");
let isLoading = $state(true);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isDirty = $state(false);
let matrixData = $state<GradeMatrixResponse | null>(null);
let gridSource = $state<Record<string, unknown>[]>([]);
let assessmentKeys = $state<string[]>([]);

onMount(() => {
  if (academicContext.effectiveCycleId)
    selectedCycleId = academicContext.effectiveCycleId;
  if (academicContext.selectedClassId)
    selectedClassId = academicContext.selectedClassId;
  if (academicContext.selectedSubjectId)
    selectedSubjectId = academicContext.selectedSubjectId;
});

$effect(() => {
  if (selectedCycleId) academicContext.selectedCycleId = selectedCycleId;
});
$effect(() => {
  if (selectedClassId) academicContext.selectedClassId = selectedClassId;
});
$effect(() => {
  if (selectedSubjectId) academicContext.selectedSubjectId = selectedSubjectId;
});

let selectedCycle = $derived(
  cycles.find((c) => String(c.id) === selectedCycleId),
);
let isCycleLocked = $derived(
  selectedCycle?.is_locked === true || selectedCycle?.is_locked === 1,
);
let isGradeEntryOpen = $derived(
  selectedCycle?.is_grade_entry_open === true ||
    selectedCycle?.is_grade_entry_open === 1,
);
let canEnterGrades = $derived(
  !isCycleLocked && isGradeEntryOpen && !academicContext.isReadOnly,
);

let gridColumns = $derived.by(() => {
  if (!matrixData) return [];
  return buildGradeColumns(matrixData, {
    readonly: !canEnterGrades,
    locale: getLocale(),
    labels: {
      rowIndex: m.control_col_index(),
      studentName: m.control_col_student(),
      total: m.control_col_total(),
      percentage: m.control_col_percentage(),
      gradeLetter: m.control_col_grade(),
    },
  });
});


async function loadInitialData() {
  isLoading = true;
  try {
    const [classList, subjectList] = await Promise.all([
      commands.classes.getAll(orgId),
      commands.subjects.getAll(orgId),
    ]);

    classes = classList;
    subjects = subjectList;

    // If no cycle selected yet, use context's effective cycle
    if (!selectedCycleId && academicContext.effectiveCycleId) {
      selectedCycleId = academicContext.effectiveCycleId;
    }
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isLoading = false;
  }
}

// Reset subject when class changes
$effect(() => {
  if (selectedClassId) {
    selectedSubjectId = "";
  }
});

// ── Grade Matrix ───────────────────────────────────────────────────────────
async function loadGradeMatrix() {
  if (!selectedCycleId || !selectedClassId || !selectedSubjectId) return;
  isMatrixLoading = true;
  matrixData = null as any;
  gridSource = [];
  isDirty = false;
  try {
    const result = (await commands.grades.getGradeMatrix(
      orgId,
      selectedCycleId,
      selectedClassId,
      selectedSubjectId,
    )) as unknown as GradeMatrixResponse;

    matrixData = result;

    if (result?.students?.length > 0) {
      const first = result.students[0];
      assessmentKeys = first
        ? first.grades.map((g) => g.assessment_id || g.exam_type)
        : [];
    }

    gridSource = transformMatrixToSource(result, getLocale());
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Error loading grade matrix");
  } finally {
    isMatrixLoading = false;
  }
}

$effect(() => {
  if (selectedCycleId && selectedClassId && selectedSubjectId) {
    loadGradeMatrix();
  }
});

// ── Cell Edit Handler ──────────────────────────────────────────────────────
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= gridSource.length) return;

  const updatedSource = [...gridSource];
  const row = { ...updatedSource[rowIndex]! };
  row[prop] = newValue;

  recalculateRowTotals(row, assessmentKeys);

  updatedSource[rowIndex] = row;
  gridSource = updatedSource;
  isDirty = true;
}

// ── Save All Grades ────────────────────────────────────────────────────────
async function handleSaveAll() {
  if (!matrixData || !canEnterGrades) return;
  isSaving = true;

  try {
    const sub = subjects.find((s: any) => String(s.id) === selectedSubjectId);

    const students: any[] = [];
    for (const row of gridSource) {
      for (const key of assessmentKeys) {
        const score = row[`score_${key}`];
        if (score !== null && score !== undefined) {
          students.push({
            enrollment_id: row.enrollment_id,
            score: Number(score),
            notes: null,
          });
        }
      }
    }

    if (students.length === 0) {
      toast.warning(m.control_no_grades_to_save());
      isSaving = false;
      return;
    }

    const request = {
      cycle_id: selectedCycleId,
      class_id: selectedClassId,
      subject_id: selectedSubjectId,
      subject_name: sub?.name || "",
      subject_name_ar: sub?.name_ar || null,
      assessment_id: null,
      class_subject_id: null,
      exam_type: "final",
      semester: "1",
      max_score: 100,
      weight: 1.0,
      is_final: 1,
      students,
    };

    const result = await commands.grades.bulkSubjectGradeEntry(
      orgId,
      authStore.userId || "admin",
      request,
    );

    const created = (result as any)?.created || 0;
    const updated = (result as any)?.updated || 0;
    const failed = (result as any)?.failed || 0;
    const total = created + updated;

    if (failed > 0) {
      toast.warning(
        m.control_save_partial({
          success: String(total),
          failed: String(failed),
        }),
        {
          description: ((result as any)?.errors || []).join("\n"),
        },
      );
    } else {
      toast.success(m.control_save_success({ total: String(total) }), {
        description: m.control_save_detail({
          created: String(created),
          updated: String(updated),
        }),
      });
    }

    isDirty = false;
    await loadGradeMatrix();
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Error saving grades");
  } finally {
    isSaving = false;
  }
}

onMount(() => {
  if (orgId) loadInitialData();
});
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-5 select-none" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  {#if isLoading}
    <div class="flex flex-col gap-4">
      <Skeleton class="h-64 w-full rounded-xl" />
    </div>
  {:else}
    <!-- Warnings -->
    {#if selectedCycle && isCycleLocked}
      <div
        class="bg-destructive/5 border border-destructive/20 rounded-xl p-4 flex items-center gap-3"
        in:slide={{ duration: 200 }}
      >
        <div class="h-8 w-8 rounded-lg bg-destructive/10 flex items-center justify-center shrink-0">
          <Lock size={16} class="text-destructive" />
        </div>
        <div class="text-start">
          <p class="text-xs font-bold text-destructive">{m.control_cycle_locked_title()}</p>
          <p class="text-[10px] text-destructive/80 mt-0.5">{m.control_cycle_locked_desc()}</p>
        </div>
      </div>
    {:else if selectedCycle && !isGradeEntryOpen}
      <div
        class="bg-amber-500/5 border border-amber-500/20 rounded-xl p-4 flex items-center gap-3"
        in:slide={{ duration: 200 }}
      >
        <div class="h-8 w-8 rounded-lg bg-amber-500/10 flex items-center justify-center shrink-0">
          <AlertTriangle size={16} class="text-amber-500" />
        </div>
        <div class="text-start">
          <p class="text-xs font-bold text-amber-500">{m.control_entry_closed_title()}</p>
          <p class="text-[10px] text-amber-400 mt-0.5">{m.control_entry_closed_desc()}</p>
        </div>
      </div>
    {/if}

    <ControlGrid
      bind:selectedClassId
      bind:selectedSubjectId
      {classes}
      {subjects}
      {matrixData}
      {gridSource}
      {gridColumns}
      {isMatrixLoading}
      {isSaving}
      {isDirty}
      {canEnterGrades}
      {selectedCycleId}
      onRefresh={loadGradeMatrix}
      onSaveAll={handleSaveAll}
      onCellEdit={handleCellEdit}
    />
  {/if}
</div>
