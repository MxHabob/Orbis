<!--
@component ControlPage
The Control Panel (الكنترول) — Central hub for grade entry management.
Rebuilt with OrbisGrid (RevoGrid) for enterprise-grade spreadsheet experience:
- Virtual scrolling (10,000+ students)
- Frozen columns (student name pinned)
- Excel-like keyboard navigation (Arrow, Tab, Enter)
- Copy/Paste from Excel
- RTL support
- S.C.R.A.P.S. design tokens
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Select,
  SelectTrigger,
  SelectContent,
  SelectItem,
} from "@orbis/ui-svelte/select";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  ShieldCheck,
  Lock,
  Unlock,
  CalendarClock,
  CalendarOff,
  AlertTriangle,
  CheckCircle2,
  XCircle,
  Save,
  BarChart3,
  Users,
  BookOpen,
  GraduationCap,
  Loader2,
  Info,
  RefreshCw,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { fade, slide } from "svelte/transition";
import { academicStore, authStore } from "$lib/stores";
import { getLocale } from "$lib/paraglide/runtime";
import * as m from "$lib/paraglide/messages";

// OrbisGrid
import { OrbisGrid } from "@orbis/data-grid";
import {
  buildGradeColumns,
  transformMatrixToSource,
  recalculateRowTotals,
  scoreToGradeLetter,
  scoreToGradeColor,
} from "@orbis/data-grid";
import type { CellEditEvent, GradeMatrixResponse } from "@orbis/data-grid";

let { data } = $props();
const orgId = $derived(data?.orgId || "");

// ── State ──────────────────────────────────────────────────────────────────
let cycles = $state<any[]>([]);
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);

let selectedCycleId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");

let isLoading = $state(true);
let isCompileLoading = $state(false);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isCycleActionLoading = $state(false);
let isDirty = $state(false);

// Grade matrix data
let matrixData = $state<GradeMatrixResponse | null>(null);
let gridSource = $state<Record<string, unknown>[]>([]);
let assessmentKeys = $state<string[]>([]);

// Sync to/from global academic state store
onMount(() => {
  if (academicStore.selectedCycleId)
    selectedCycleId = academicStore.selectedCycleId;
  if (academicStore.selectedClassId)
    selectedClassId = academicStore.selectedClassId;
  if (academicStore.selectedSubjectId)
    selectedSubjectId = academicStore.selectedSubjectId;
});

$effect(() => {
  if (selectedCycleId) academicStore.selectedCycleId = selectedCycleId;
});
$effect(() => {
  if (selectedClassId) academicStore.selectedClassId = selectedClassId;
});
$effect(() => {
  if (selectedSubjectId) academicStore.selectedSubjectId = selectedSubjectId;
});

// Selected cycle details
let selectedCycle = $derived(
  cycles.find((c) => String(c.id) === selectedCycleId),
);
let isCycleLocked = $derived(selectedCycle?.is_locked === 1);
let isGradeEntryOpen = $derived(selectedCycle?.is_grade_entry_open === 1);
let canEnterGrades = $derived(!isCycleLocked && isGradeEntryOpen);

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
async function loadInitialData() {
  isLoading = true;
  try {
    const [cycleList, classList, subjectList] = await Promise.all([
      commands.academics.getCycles(orgId),
      commands.classes.getAll(orgId),
      commands.subjects.getAll(orgId),
    ]);

    cycles = cycleList;
    classes = classList;
    subjects = subjectList;

    if (!selectedCycleId) {
      const activeCycle =
        cycleList.find((c: any) => c.is_current === 1) || cycleList[0];
      if (activeCycle) {
        selectedCycleId = String(activeCycle.id);
      }
    }
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Error loading data");
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
    toast.error((e as any)?.message || "Error");
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
    toast.error((e as any)?.message || "Error");
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
    toast.error((e as any)?.message || "Error");
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
    toast.error((e as any)?.message || "Error");
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
    toast.error((e as any)?.message || "Error");
  } finally {
    isCompileLoading = false;
  }
}

function updateCycleInList(updated: any) {
  cycles = cycles.map((c) => (c.id === updated.id ? updated : c));
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

    // Build assessment keys for recalculation
    if (result?.students?.length > 0) {
      const first = result.students[0];
      assessmentKeys = first
        ? first.grades.map((g) => g.assessment_id || g.exam_type)
        : [];
    }

    // Transform to flat grid source
    gridSource = transformMatrixToSource(result, getLocale());
  } catch (e: unknown) {
    toast.error((e as any)?.message || "Error loading grade matrix");
  } finally {
    isMatrixLoading = false;
  }
}

// Auto-load matrix when all filters are selected
$effect(() => {
  if (selectedCycleId && selectedClassId && selectedSubjectId) {
    loadGradeMatrix();
  }
});

// ── Cell Edit Handler ──────────────────────────────────────────────────────
function handleCellEdit(event: CellEditEvent) {
  const { rowIndex, prop, newValue } = event;
  if (rowIndex < 0 || rowIndex >= gridSource.length) return;

  // Update grid source
  const updatedSource = [...gridSource];
  const row = { ...updatedSource[rowIndex]! };
  row[prop] = newValue;

  // Recalculate totals
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

    // For each assessment, build a bulk request
    // Collect all students with scores
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

// ── Helpers ────────────────────────────────────────────────────────────────
function getCycleStatusVariant(
  cycle: any,
): "destructive" | "lime" | "secondary" {
  if (cycle.is_locked === 1) return "destructive";
  if (cycle.is_grade_entry_open === 1) return "lime";
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

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-5 select-none" dir={getLocale() === "ar" ? "rtl" : "ltr"}>
  <!-- ═══════════════════════════════════════════════════════════════════════
       Header
       ═══════════════════════════════════════════════════════════════════════ -->
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
    <!-- ═══════════════════════════════════════════════════════════════════════
         Section 1: Cycle Control Panel
         ═══════════════════════════════════════════════════════════════════════ -->
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
          <SelectTrigger class="min-w-[220px] text-xs h-9">{selectedCycle ? (selectedCycle.name_ar || selectedCycle.name) : m.control_select_cycle()}</SelectTrigger>
          <SelectContent>
            {#each cycles as cycle}
              <SelectItem value={String(cycle.id)}>{cycle.name_ar || cycle.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        {#if selectedCycle}
          <Badge variant={getCycleStatusVariant(selectedCycle)}>
            {getCycleStatusLabel(selectedCycle)}
          </Badge>
        {/if}
      </div>

      <!-- Cycle Status & Actions -->
      {#if selectedCycle}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3" in:slide={{ duration: 200 }}>
          <!-- Status Card -->
          <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
            <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.control_cycle_status()}</h3>
            <div class="flex flex-col gap-2 text-xs">
              <div class="flex items-center justify-between">
                <span class="text-muted-foreground">{m.control_cycle_lock()}</span>
                <span class="flex items-center gap-1 font-semibold {isCycleLocked ? 'text-destructive' : 'text-emerald-500'}">
                  {#if isCycleLocked}
                    <Lock size={12} /> {m.control_cycle_locked()}
                  {:else}
                    <Unlock size={12} /> {m.control_cycle_unlocked()}
                  {/if}
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-muted-foreground">{m.control_grade_entry()}</span>
                <span class="flex items-center gap-1 font-semibold {isGradeEntryOpen ? 'text-emerald-500' : 'text-amber-500'}">
                  {#if isGradeEntryOpen}
                    <CheckCircle2 size={12} /> {m.control_grade_entry_open()}
                  {:else}
                    <XCircle size={12} /> {m.control_grade_entry_closed()}
                  {/if}
                </span>
              </div>
              {#if selectedCycle.grade_entry_start}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">{m.control_grade_entry_start()}</span>
                  <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_start).toLocaleDateString(getLocale())}</span>
                </div>
              {/if}
              {#if selectedCycle.grade_entry_end}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">{m.control_grade_entry_end()}</span>
                  <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_end).toLocaleDateString(getLocale())}</span>
                </div>
              {/if}
              {#if selectedCycle.locked_by}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">{m.control_locked_by()}</span>
                  <span class="text-[10px]">{selectedCycle.locked_by}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Action Buttons Card -->
          <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
            <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">{m.control_actions()}</h3>
            <div class="flex flex-col gap-2">
              {#if isCycleLocked}
                <Button
                  variant="outline"
                  class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
                  onclick={handleUnlockCycle}
                  disabled={isCycleActionLoading}
                >
                  {#if isCycleActionLoading}
                    <Loader2 size={14} class="animate-spin" />
                  {:else}
                    <Unlock size={14} />
                  {/if}
                  {m.control_unlock_cycle()}
                </Button>
              {:else}
                <Button
                  variant="outline"
                  class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-destructive/30 text-destructive hover:bg-destructive/10"
                  onclick={handleLockCycle}
                  disabled={isCycleActionLoading}
                >
                  {#if isCycleActionLoading}
                    <Loader2 size={14} class="animate-spin" />
                  {:else}
                    <Lock size={14} />
                  {/if}
                  {m.control_lock_cycle()}
                </Button>

                {#if isGradeEntryOpen}
                  <Button
                    variant="outline"
                    class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-amber-500/30 text-amber-600 hover:bg-amber-500/10"
                    onclick={handleCloseGradeEntry}
                    disabled={isCycleActionLoading}
                  >
                    {#if isCycleActionLoading}
                      <Loader2 size={14} class="animate-spin" />
                    {:else}
                      <CalendarOff size={14} />
                    {/if}
                    {m.control_close_entry()}
                  </Button>
                {:else}
                  <Button
                    variant="outline"
                    class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-emerald-500/30 text-emerald-600 hover:bg-emerald-500/10"
                    onclick={handleOpenGradeEntry}
                    disabled={isCycleActionLoading}
                  >
                    {#if isCycleActionLoading}
                      <Loader2 size={14} class="animate-spin" />
                    {:else}
                      <CalendarClock size={14} />
                    {/if}
                    {m.control_open_entry()}
                  </Button>
                {/if}
              {/if}

              {#if selectedCycle && (selectedCycle.type === 'term' || selectedCycle.id.includes('-s1') || selectedCycle.id.includes('-s2'))}
                <Button
                  variant="outline"
                  class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-blue-500/30 text-blue-600 hover:bg-blue-500/10"
                  onclick={handleCompileGrades}
                  disabled={isCompileLoading}
                >
                  {#if isCompileLoading}
                    <Loader2 size={14} class="animate-spin" />
                  {:else}
                    <RefreshCw size={14} />
                  {/if}
                  {m.control_compile_grades()}
                </Button>
              {/if}
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════════
         Lock / Entry Warning Banners
         ═══════════════════════════════════════════════════════════════════════ -->
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

    <!-- ═══════════════════════════════════════════════════════════════════════
         Section 2: Bulk Grade Entry — OrbisGrid
         ═══════════════════════════════════════════════════════════════════════ -->
    <div class="bg-card border border-border rounded-xl p-5 flex flex-col gap-4" in:fade={{ delay: 100, duration: 200 }}>
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
          <Users size={14} class="text-primary" />
          {m.control_bulk_entry_title()}
        </h2>
        {#if matrixData && canEnterGrades}
          <div class="flex items-center gap-2 flex-nowrap shrink-0">
            {#if isDirty}
              <Badge variant="secondary" class="text-[10px]">●</Badge>
            {/if}
            <Button
              variant="outline"
              size="sm"
              class="text-[10px] h-7 px-2.5 gap-1.5 cursor-pointer"
              onclick={loadGradeMatrix}
              disabled={isMatrixLoading}
            >
              <RefreshCw size={11} class={isMatrixLoading ? "animate-spin" : ""} />
              {m.control_refresh()}
            </Button>
            <Button
              variant="lime"
              size="sm"
              class="text-[10px] h-7 px-3 gap-1.5 cursor-pointer"
              onclick={handleSaveAll}
              disabled={isSaving || !isDirty}
            >
              {#if isSaving}
                <Loader2 size={11} class="animate-spin" />
              {:else}
                <Save size={11} />
              {/if}
              {m.control_save_all()}
            </Button>
          </div>
        {/if}
      </div>

      <!-- Filters Row -->
      <div class="flex items-center gap-3 flex-wrap">
        <!-- Class Selector -->
        <Select type="single" value={selectedClassId} onValueChange={(v) => { if (v) selectedClassId = v; }}>
          <SelectTrigger class="min-w-[180px] text-xs h-9" disabled={!selectedCycleId}>
            <div class="flex items-center gap-2">
              <GraduationCap size={12} class="text-muted-foreground shrink-0" />
              <span class="truncate">{classes.find((c) => String(c.id) === selectedClassId)?.name_ar || classes.find((c) => String(c.id) === selectedClassId)?.name || m.control_select_class()}</span>
            </div>
          </SelectTrigger>
          <SelectContent>
            {#each classes as cls}
              <SelectItem value={String(cls.id)}>{cls.name_ar || cls.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>

        <!-- Subject Selector -->
        <Select type="single" value={selectedSubjectId} onValueChange={(v) => { if (v) selectedSubjectId = v; }}>
          <SelectTrigger class="min-w-[180px] text-xs h-9" disabled={!selectedClassId || subjects.length === 0}>
            <div class="flex items-center gap-2">
              <BookOpen size={12} class="text-muted-foreground shrink-0" />
              <span class="truncate">{subjects.find((s) => String(s.id) === selectedSubjectId)?.name_ar || subjects.find((s) => String(s.id) === selectedSubjectId)?.name || m.control_select_subject()}</span>
            </div>
          </SelectTrigger>
          <SelectContent>
            {#each subjects as sub}
              <SelectItem value={String(sub.id)}>{sub.name_ar || sub.name}</SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>

      <!-- Grade Matrix Grid -->
      {#if isMatrixLoading}
        <div class="flex flex-col gap-2">
          {#each Array(6) as _}
            <Skeleton class="h-10 w-full rounded-lg" />
          {/each}
        </div>
      {:else if matrixData && gridSource.length > 0}
        <div in:fade={{ duration: 200 }}>
          <OrbisGrid
            columns={gridColumns}
            source={gridSource}
            height="calc(100vh - 420px)"
            rtl={getLocale() === "ar"}
            readonly={!canEnterGrades}
            theme="compact"
            range={true}
            clipboard={true}
            rowSize={42}
            oncelledit={handleCellEdit}
          />
        </div>
      {:else if selectedCycleId && selectedClassId && selectedSubjectId && !isMatrixLoading}
        <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
          <Info size={32} class="opacity-40" />
          <p class="text-xs">{m.control_no_data()}</p>
          <p class="text-[10px]">{m.control_no_data_hint()}</p>
        </div>
      {:else}
        <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
          <BarChart3 size={32} class="opacity-40" />
          <p class="text-xs">{m.control_select_filters()}</p>
        </div>
      {/if}
    </div>
  {/if}
</div>
