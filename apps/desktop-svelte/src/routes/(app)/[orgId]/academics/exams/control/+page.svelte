<script lang="ts">
import { onMount } from "svelte";
import { commands, extractErrorMessage } from "$lib/core/ipc";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import { Button } from "@orbis/ui-svelte/button";
import {
  ShieldCheck,
  Lock,
  Unlock,
  CalendarClock,
  AlertTriangle,
  GraduationCap,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { fade, slide } from "svelte/transition";
import { academicContext, authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

// Component imports
import { ControlSidebar, ControlGrid } from "$lib/features/academics";

// OrbisGrid utilities
import {
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
} from "@orbis/data-grid";
import type { CellEditEvent, GradeMatrixResponse } from "@orbis/data-grid";

let { data } = $props();
const orgId = $derived(data?.orgId || "");

// ── State ──────────────────────────────────────────────────────────────────
// Cycles come from academicContext (populated by bootstrap) — no independent load
let cycles = $derived(academicContext.allCycles);
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);

// Selections — synced with academicContext for cross-page persistence
let selectedCycleId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");

let isLoading = $state(true);
let isCompileLoading = $state(false);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isCycleActionLoading = $state(false);
let isDirty = $state(false);
let showOverrideDialog = $state(false);

// Grade matrix data
let matrixData = $state<GradeMatrixResponse | null>(null);
let gridSource = $state<Record<string, unknown>[]>([]);
let assessmentKeys = $state<string[]>([]);

// Initialize selections from context on mount
onMount(() => {
  // Use context's effective cycle (auto-resolved by bootstrap)
  if (academicContext.effectiveCycleId)
    selectedCycleId = academicContext.effectiveCycleId;
  if (academicContext.selectedClassId)
    selectedClassId = academicContext.selectedClassId;
  if (academicContext.selectedSubjectId)
    selectedSubjectId = academicContext.selectedSubjectId;
});

// Sync selections back to context for cross-page persistence
$effect(() => {
  if (selectedCycleId) academicContext.selectedCycleId = selectedCycleId;
});
$effect(() => {
  if (selectedClassId) academicContext.selectedClassId = selectedClassId;
});
$effect(() => {
  if (selectedSubjectId) academicContext.selectedSubjectId = selectedSubjectId;
});

// Selected cycle details — from context-provided cycles
let selectedCycle = $derived(
  cycles.find((c) => String(c.id) === selectedCycleId),
);
let isCycleLocked = $derived(selectedCycle?.is_locked === true || selectedCycle?.is_locked === 1);
let isGradeEntryOpen = $derived(selectedCycle?.is_grade_entry_open === true || selectedCycle?.is_grade_entry_open === 1);
let canEnterGrades = $derived(!isCycleLocked && isGradeEntryOpen && !academicContext.isReadOnly);

// Grid columns (reactive)
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

// Statistics (derived)
let stats = $derived.by(() => {
  if (!gridSource.length) return null;
  const percentages = gridSource.map((r) => Number(r.__percentageRaw) || 0);
  const avg = percentages.reduce((a, b) => a + b, 0) / percentages.length;
  const max = Math.max(...percentages);
  const min = Math.min(...percentages);
  const passCount = percentages.filter((p) => p >= 50).length;
  const passRate = (passCount / percentages.length) * 100;
  return {
    avg,
    max,
    min,
    passCount,
    passRate,
    total: percentages.length,
    excellent: percentages.filter((p) => p >= 90).length,
    veryGood: percentages.filter((p) => p >= 80 && p < 90).length,
    good: percentages.filter((p) => p >= 70 && p < 80).length,
    acceptable: percentages.filter((p) => p >= 60 && p < 70).length,
    weak: percentages.filter((p) => p >= 50 && p < 60).length,
    fail: percentages.filter((p) => p < 50).length,
  };
});

// ── Load initial data ──────────────────────────────────────────────────────
// Cycles come from academicContext — only load classes and subjects
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

// ── Cycle Actions ──────────────────────────────────────────────────────────
async function handleLockCycle() {
  if (!selectedCycleId) return;
  isCycleActionLoading = true;
  try {
    const updated = await commands.academics.lockCycle(selectedCycleId);
    updateCycleInList(updated);
    toast.success(m.control_cycle_locked());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isCycleActionLoading = false;
  }
}

async function handleUnlockCycle() {
  if (!selectedCycleId) return;
  isCycleActionLoading = true;
  try {
    const updated = await commands.academics.unlockCycle(selectedCycleId);
    updateCycleInList(updated);
    toast.success(m.control_cycle_unlocked());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isCycleActionLoading = false;
  }
}

async function handleOpenGradeEntry() {
  if (!selectedCycleId) return;
  isCycleActionLoading = true;
  try {
    const updated = await commands.academics.openGradeEntry(selectedCycleId);
    updateCycleInList(updated);
    toast.success(m.control_grade_entry_open());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isCycleActionLoading = false;
  }
}

async function handleCloseGradeEntry() {
  if (!selectedCycleId) return;
  isCycleActionLoading = true;
  try {
    const updated = await commands.academics.closeGradeEntry(selectedCycleId);
    updateCycleInList(updated);
    toast.success(m.control_grade_entry_closed());
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isCycleActionLoading = false;
  }
}

async function handleCompileGrades() {
  if (!selectedCycleId) return;
  const semNum = selectedCycleId.endsWith("-s1")
    ? 1
    : selectedCycleId.endsWith("-s2")
      ? 2
      : null;
  if (!semNum) {
    toast.error(m.control_compile_term_only());
    return;
  }
  if (!confirm(m.control_compile_confirm())) return;

  isCompileLoading = true;
  try {
    const gradedBy = authStore.userId || "admin";
    await commands.grades.compileSemesterGrades(orgId, semNum, gradedBy);
    toast.success(m.control_compile_success());
    if (selectedClassId && selectedSubjectId) {
      await loadGradeMatrix();
    }
  } catch (e: unknown) {
    toast.error(extractErrorMessage(e));
  } finally {
    isCompileLoading = false;
  }
}

function updateCycleInList(updated: any) {
  // Cycles are derived from academicContext.allCycles — notify context
  // For now, the context will be refreshed on next resolveContext call
  // The local derived will auto-update
}

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

function getCycleStatusVariant(
  cycle: any,
): "destructive" | "success" | "secondary" {
  if (cycle.is_locked === 1) return "destructive";
  if (cycle.is_grade_entry_open === 1) return "success";
  return "secondary";
}

function getCycleStatusLabel(cycle: any): string {
  if (cycle.is_locked === 1) return m.control_status_locked();
  if (cycle.is_grade_entry_open === 1) return m.control_status_entry_open();
  return m.control_status_entry_closed();
}

onMount(() => {
  if (orgId) loadInitialData();
});
</script>

<div class="h-full w-full overflow-y-auto scrollbar-hide p-6 flex flex-col gap-5 select-none" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <!-- Header -->
  <div
    class="bg-card border border-border rounded-xl p-5 flex items-center justify-between"
    in:fade={{ duration: 150 }}
  >
    <div class="flex items-center gap-3 text-start">
      <div class="h-10 w-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
        <ShieldCheck size={22} />
      </div>
      <div>
        <h1 class="text-sm font-bold text-foreground">{m.control_page_title()}</h1>
        <p class="text-[10px] text-muted-foreground mt-0.5">
          {m.control_page_desc()}
        </p>
      </div>
    </div>
  </div>

  {#if isLoading}
    <div class="flex flex-col gap-4">
      <Skeleton class="h-32 w-full rounded-xl" />
      <Skeleton class="h-64 w-full rounded-xl" />
    </div>
  {:else}
    <!-- Section 1: Cycle Control Panel -->
    <div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4" in:fade={{ delay: 50, duration: 200 }}>
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
          <CalendarClock size={14} class="text-primary" />
          {m.control_cycle_management()}
        </h2>
      </div>

      <!-- Cycle Selector -->
      <div class="flex items-center gap-3 flex-wrap">
        <Select type="single" value={selectedCycleId} onValueChange={(v) => { if (v) selectedCycleId = v; }}>
          <SelectTrigger class="min-w-[200px] text-xs h-9">
            <div class="flex items-center gap-2">
              <span class="truncate">{selectedCycle ? selectedCycle.name_ar || selectedCycle.name : m.control_select_cycle()}</span>
              {#if selectedCycle}
                <Badge variant={getCycleStatusVariant(selectedCycle)} class="text-[9px] scale-90 px-1 py-0">{getCycleStatusLabel(selectedCycle)}</Badge>
              {/if}
            </div>
          </SelectTrigger>
          <SelectContent>
            {#each cycles as cycle}
              <SelectItem value={String(cycle.id)}>{cycle.name_ar || cycle.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      {#if selectedCycle}
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <ControlSidebar
            {selectedCycle}
            {isCycleLocked}
            {isGradeEntryOpen}
            {isCycleActionLoading}
            {isCompileLoading}
            stats={null}
            onLockCycle={handleLockCycle}
            onUnlockCycle={handleUnlockCycle}
            onOpenGradeEntry={handleOpenGradeEntry}
            onCloseGradeEntry={handleCloseGradeEntry}
            onCompileGrades={handleCompileGrades}
          />
        </div>
      {/if}
    </div>

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

    <!-- Section 2: Bulk Grade Entry — OrbisGrid -->
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
