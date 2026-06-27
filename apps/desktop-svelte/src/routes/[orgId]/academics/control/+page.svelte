<!--
@component ControlPage
The Control Panel (الكنترول) — Central hub for grade entry management.
Manages academic cycle lock/unlock, grade entry periods, and bulk grade entry.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  ShieldCheck,
  Lock,
  Unlock,
  CalendarClock,
  CalendarOff,
  ChevronDown,
  AlertTriangle,
  CheckCircle2,
  XCircle,
  Clock,
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

let { data }: { data: { orgId: string } } = $props();

// ── State ──────────────────────────────────────────────────────────────────
let cycles = $state<any[]>([]);
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);
let classSubjects = $state<any[]>([]);

let selectedCycleId = $state<string>("");
let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");

let isLoading = $state(true);
let isMatrixLoading = $state(false);
let isSaving = $state(false);
let isCycleActionLoading = $state(false);

// Grade matrix data
let matrixData = $state<any>(null);
let editedScores = $state<Record<string, number | null>>({});
let errorCells = $state<Set<string>>(new Set());

// Selected cycle details
let selectedCycle = $derived(cycles.find((c) => String(c.id) === selectedCycleId));
let isCycleLocked = $derived(selectedCycle?.is_locked === 1);
let isGradeEntryOpen = $derived(selectedCycle?.is_grade_entry_open === 1);
let canEnterGrades = $derived(!isCycleLocked && isGradeEntryOpen);

// ── Load initial data ──────────────────────────────────────────────────────
async function loadInitialData() {
  isLoading = true;
  try {
    const [cycleList, classList, subjectList, classSubjectList] = await Promise.all([
      commands.academics.getCycles(data.orgId),
      commands.classes.getAll(data.orgId),
      commands.subjects.getAll(data.orgId),
      commands.schedule.getClassSubjects(),
    ]);

    cycles = cycleList;
    classes = classList;
    subjects = subjectList;
    classSubjects = classSubjectList;

    // Default to current/active cycle
    const activeCycle =
      cycleList.find((c: any) => c.is_current === 1) || cycleList[0];
    if (activeCycle) {
      selectedCycleId = String(activeCycle.id);
    }
  } catch (e: any) {
    toast.error("خطأ في تحميل البيانات", { description: e?.message || String(e) });
  } finally {
    isLoading = false;
  }
}

// Reset subject selection when class changes
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
    toast.success("تم إقفال الدورة الأكاديمية");
  } catch (e: any) {
    toast.error("خطأ في إقفال الدورة", { description: e?.message || String(e) });
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
    toast.success("تم فتح قفل الدورة الأكاديمية");
  } catch (e: any) {
    toast.error("خطأ في فتح القفل", { description: e?.message || String(e) });
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
    toast.success("تم فتح فترة إدخال الدرجات");
  } catch (e: any) {
    toast.error("خطأ", { description: e?.message || String(e) });
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
    toast.success("تم إغلاق فترة إدخال الدرجات");
  } catch (e: any) {
    toast.error("خطأ", { description: e?.message || String(e) });
  } finally {
    isCycleActionLoading = false;
  }
}

function updateCycleInList(updated: any) {
  cycles = cycles.map((c) => (c.id === updated.id ? updated : c));
}

// ── Grade Matrix ───────────────────────────────────────────────────────────
async function loadGradeMatrix() {
  if (!selectedCycleId || !selectedClassId || !selectedSubjectId) return;
  isMatrixLoading = true;
  matrixData = null;
  editedScores = {};
  errorCells = new Set();
  try {
    const result = await commands.grades.getGradeMatrix(
      data.orgId,
      selectedCycleId,
      selectedClassId,
      selectedSubjectId
    );
    matrixData = result;
    // Initialize edited scores from existing data
    if (result?.students) {
      for (const student of result.students as any[]) {
        for (const grade of student.grades as any[]) {
          const key = `${student.enrollment_id}_${grade.assessment_id || grade.exam_type}`;
          editedScores[key] = grade.score ?? null;
        }
      }
    }
  } catch (e: any) {
    toast.error("خطأ في تحميل مصفوفة الدرجات", {
      description: e?.message || String(e),
    });
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

function handleScoreChange(enrollmentId: string, assessmentKey: string, value: string, maxScore: number) {
  const key = `${enrollmentId}_${assessmentKey}`;
  const numValue = value === "" ? null : parseFloat(value);

  if (numValue !== null && (isNaN(numValue) || numValue < 0 || numValue > maxScore)) {
    errorCells = new Set([...errorCells, key]);
  } else {
    const newErrors = new Set(errorCells);
    newErrors.delete(key);
    errorCells = newErrors;
  }

  editedScores = { ...editedScores, [key]: numValue };
}

// ── Save Bulk Grades ───────────────────────────────────────────────────────
async function handleSaveAll() {
  if (!matrixData || errorCells.size > 0) return;
  isSaving = true;

  try {
    const sub = subjects.find((s: any) => String(s.id) === selectedSubjectId);
    const cs = classSubjects.find(
      (c: any) => String(c.class_id) === selectedClassId && String(c.subject_id) === selectedSubjectId
    );

    // Build student scores from edited values
    const students: any[] = [];
    for (const student of matrixData.students as any[]) {
      for (const grade of student.grades as any[]) {
        const key = `${student.enrollment_id}_${grade.assessment_id || grade.exam_type}`;
        const score = editedScores[key];
        if (score !== null && score !== undefined) {
          students.push({
            enrollment_id: student.enrollment_id,
            score,
            notes: null,
          });
        }
      }
    }

    if (students.length === 0) {
      toast.warning("لا توجد درجات للحفظ");
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
      class_subject_id: cs?.id || null,
      exam_type: "final",
      semester: "1",
      max_score: 100,
      weight: 1.0,
      is_final: 1,
      students,
    };

    const result = await commands.grades.bulkSubjectGradeEntry(
      data.orgId,
      "admin",
      request
    );

    const created = (result as any)?.created || 0;
    const updated = (result as any)?.updated || 0;
    const failed = (result as any)?.failed || 0;

    if (failed > 0) {
      toast.warning(
        `تم حفظ ${created + updated} درجة مع ${failed} أخطاء`,
        { description: ((result as any)?.errors || []).join("\n") }
      );
    } else {
      toast.success(`تم حفظ ${created + updated} درجة بنجاح`, {
        description: `${created} جديدة، ${updated} محدّثة`,
      });
    }

    // Reload matrix
    await loadGradeMatrix();
  } catch (e: any) {
    toast.error("خطأ في حفظ الدرجات", { description: e?.message || String(e) });
  } finally {
    isSaving = false;
  }
}

// ── Grade Letter Helper ────────────────────────────────────────────────────
function getGradeLetter(percentage: number): string {
  if (percentage >= 90) return "ممتاز";
  if (percentage >= 80) return "جيد جداً";
  if (percentage >= 70) return "جيد";
  if (percentage >= 60) return "مقبول";
  if (percentage >= 50) return "ضعيف";
  return "راسب";
}

function getGradeColor(percentage: number): string {
  if (percentage >= 90) return "text-emerald-500";
  if (percentage >= 80) return "text-blue-500";
  if (percentage >= 70) return "text-sky-500";
  if (percentage >= 60) return "text-amber-500";
  if (percentage >= 50) return "text-orange-500";
  return "text-red-500";
}

function getCycleStatusColor(cycle: any): string {
  if (cycle.is_locked === 1) return "bg-red-500/10 text-red-500 border-red-500/30";
  if (cycle.is_grade_entry_open === 1) return "bg-emerald-500/10 text-emerald-500 border-emerald-500/30";
  return "bg-amber-500/10 text-amber-500 border-amber-500/30";
}

function getCycleStatusLabel(cycle: any): string {
  if (cycle.is_locked === 1) return "مقفلة";
  if (cycle.is_grade_entry_open === 1) return "الإدخال مفتوح";
  return "الإدخال مغلق";
}

onMount(() => {
  if (data.orgId) loadInitialData();
});
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 select-none" dir="rtl">
  <!-- ═══════════════════════════════════════════════════════════════════════
       Header
       ═══════════════════════════════════════════════════════════════════════ -->
  <div
    class="card-tactile bg-gradient-to-br from-card to-card/50 p-5 flex items-center justify-between"
    in:fade={{ duration: 150 }}
  >
    <div class="flex items-center gap-3">
      <div class="h-10 w-10 rounded-xl bg-primary/10 text-primary flex items-center justify-center shrink-0">
        <ShieldCheck size={22} />
      </div>
      <div>
        <h1 class="text-sm font-bold text-foreground">الكنترول</h1>
        <p class="text-[10px] text-muted-foreground mt-0.5">
          إدارة الدورات الأكاديمية وإدخال الدرجات
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
    <div class="card-tactile bg-card p-5 flex flex-col gap-4" in:fade={{ delay: 50, duration: 200 }}>
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
          <CalendarClock size={14} class="text-primary" />
          إدارة الدورات الأكاديمية
        </h2>
      </div>

      <!-- Cycle Selector -->
      <div class="flex items-center gap-3 flex-wrap">
        <div class="relative">
          <select
            bind:value={selectedCycleId}
            class="appearance-none bg-background border border-border rounded-lg px-3 py-2 pe-8 text-xs text-foreground focus:outline-none focus:ring-2 focus:ring-primary/30 cursor-pointer min-w-[200px]"
          >
            <option value="" disabled>اختر الدورة الأكاديمية...</option>
            {#each cycles as cycle}
              <option value={cycle.id}>{cycle.name_ar || cycle.name}</option>
            {/each}
          </select>
          <ChevronDown size={12} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        </div>

        {#if selectedCycle}
          <span class="px-2.5 py-1 rounded-md border text-[10px] font-bold {getCycleStatusColor(selectedCycle)}">
            {getCycleStatusLabel(selectedCycle)}
          </span>
        {/if}
      </div>

      <!-- Cycle Status & Actions -->
      {#if selectedCycle}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3" in:slide={{ duration: 200 }}>
          <!-- Status Card -->
          <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
            <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">حالة الدورة</h3>
            <div class="flex flex-col gap-2 text-xs">
              <div class="flex items-center justify-between">
                <span class="text-muted-foreground">القفل:</span>
                <span class="flex items-center gap-1 font-semibold {isCycleLocked ? 'text-red-500' : 'text-emerald-500'}">
                  {#if isCycleLocked}
                    <Lock size={12} /> مقفلة
                  {:else}
                    <Unlock size={12} /> مفتوحة
                  {/if}
                </span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-muted-foreground">إدخال الدرجات:</span>
                <span class="flex items-center gap-1 font-semibold {isGradeEntryOpen ? 'text-emerald-500' : 'text-amber-500'}">
                  {#if isGradeEntryOpen}
                    <CheckCircle2 size={12} /> مفتوح
                  {:else}
                    <XCircle size={12} /> مغلق
                  {/if}
                </span>
              </div>
              {#if selectedCycle.grade_entry_start}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">بداية الإدخال:</span>
                  <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_start).toLocaleDateString("ar")}</span>
                </div>
              {/if}
              {#if selectedCycle.grade_entry_end}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">نهاية الإدخال:</span>
                  <span class="font-mono text-[10px]">{new Date(selectedCycle.grade_entry_end).toLocaleDateString("ar")}</span>
                </div>
              {/if}
              {#if selectedCycle.locked_by}
                <div class="flex items-center justify-between">
                  <span class="text-muted-foreground">تم الإقفال بواسطة:</span>
                  <span class="text-[10px]">{selectedCycle.locked_by}</span>
                </div>
              {/if}
            </div>
          </div>

          <!-- Action Buttons Card -->
          <div class="bg-background/50 border border-border rounded-lg p-4 flex flex-col gap-3">
            <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">الإجراءات</h3>
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
                  فتح قفل الدورة
                </Button>
              {:else}
                <Button
                  variant="outline"
                  class="w-full justify-start gap-2 text-xs h-9 cursor-pointer border-red-500/30 text-red-500 hover:bg-red-500/10"
                  onclick={handleLockCycle}
                  disabled={isCycleActionLoading}
                >
                  {#if isCycleActionLoading}
                    <Loader2 size={14} class="animate-spin" />
                  {:else}
                    <Lock size={14} />
                  {/if}
                  إقفال الدورة
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
                    إغلاق فترة الإدخال
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
                    فتح فترة الإدخال
                  </Button>
                {/if}
              {/if}
            </div>
          </div>
        </div>
      {/if}
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════════
         Lock Warning Banner
         ═══════════════════════════════════════════════════════════════════════ -->
    {#if selectedCycle && isCycleLocked}
      <div
        class="bg-red-500/5 border border-red-500/20 rounded-xl p-4 flex items-center gap-3"
        in:slide={{ duration: 200 }}
      >
        <div class="h-8 w-8 rounded-lg bg-red-500/10 flex items-center justify-center shrink-0">
          <Lock size={16} class="text-red-500" />
        </div>
        <div>
          <p class="text-xs font-bold text-red-500">الدورة الأكاديمية مقفلة</p>
          <p class="text-[10px] text-red-400 mt-0.5">
            لا يمكن إدخال أو تعديل أي درجات. تواصل مع الإدارة لفتح القفل.
          </p>
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
        <div>
          <p class="text-xs font-bold text-amber-500">فترة إدخال الدرجات مغلقة</p>
          <p class="text-[10px] text-amber-400 mt-0.5">
            يجب فتح فترة الإدخال أولاً قبل البدء بإدخال الدرجات.
          </p>
        </div>
      </div>
    {/if}

    <!-- ═══════════════════════════════════════════════════════════════════════
         Section 2: Bulk Grade Entry
         ═══════════════════════════════════════════════════════════════════════ -->
    <div class="card-tactile bg-card p-5 flex flex-col gap-4" in:fade={{ delay: 100, duration: 200 }}>
      <div class="flex items-center justify-between">
        <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
          <Users size={14} class="text-primary" />
          إدخال الدرجات الجماعي
        </h2>
        {#if matrixData && canEnterGrades}
          <div class="flex items-center gap-2">
            <Button
              variant="outline"
              class="text-[10px] h-7 px-2.5 gap-1.5 cursor-pointer"
              onclick={loadGradeMatrix}
              disabled={isMatrixLoading}
            >
              <RefreshCw size={11} class={isMatrixLoading ? "animate-spin" : ""} />
              تحديث
            </Button>
            <Button
              class="text-[10px] h-7 px-3 gap-1.5 cursor-pointer bg-primary text-primary-foreground hover:bg-primary/90"
              onclick={handleSaveAll}
              disabled={isSaving || errorCells.size > 0}
            >
              {#if isSaving}
                <Loader2 size={11} class="animate-spin" />
              {:else}
                <Save size={11} />
              {/if}
              حفظ الكل
            </Button>
          </div>
        {/if}
      </div>

      <!-- Filters Row -->
      <div class="flex items-center gap-3 flex-wrap">
        <!-- Class Selector -->
        <div class="relative">
          <select
            bind:value={selectedClassId}
            class="appearance-none bg-background border border-border rounded-lg px-3 py-2 pe-8 text-xs text-foreground focus:outline-none focus:ring-2 focus:ring-primary/30 cursor-pointer min-w-[160px]"
            disabled={!selectedCycleId}
          >
            <option value="" disabled>اختر الفصل...</option>
            {#each classes as cls}
              <option value={cls.id}>{cls.name_ar || cls.name}</option>
            {/each}
          </select>
          <GraduationCap size={12} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        </div>

        <!-- Subject Selector -->
        <div class="relative">
          <select
            bind:value={selectedSubjectId}
            class="appearance-none bg-background border border-border rounded-lg px-3 py-2 pe-8 text-xs text-foreground focus:outline-none focus:ring-2 focus:ring-primary/30 cursor-pointer min-w-[160px]"
            disabled={!selectedClassId || subjects.length === 0}
          >
            <option value="" disabled>اختر المادة...</option>
            {#each subjects as sub}
              <option value={String(sub.id)}>{sub.name_ar || sub.name}</option>
            {/each}
          </select>
          <BookOpen size={12} class="absolute start-2 top-1/2 -translate-y-1/2 text-muted-foreground pointer-events-none" />
        </div>
      </div>

      <!-- Grade Matrix Table -->
      {#if isMatrixLoading}
        <div class="flex flex-col gap-2">
          {#each Array(6) as _, i}
            <Skeleton class="h-10 w-full rounded-lg" />
          {/each}
        </div>
      {:else if matrixData && matrixData.students && matrixData.students.length > 0}
        <div class="border border-border rounded-xl overflow-hidden" in:fade={{ duration: 200 }}>
          <div class="overflow-x-auto">
            <table class="w-full text-xs">
              <thead>
                <tr class="bg-muted/30 border-b border-border">
                  <th class="px-3 py-2.5 text-start font-bold text-muted-foreground sticky start-0 bg-muted/30 z-10 min-w-[40px]">#</th>
                  <th class="px-3 py-2.5 text-start font-bold text-muted-foreground sticky start-[40px] bg-muted/30 z-10 min-w-[160px]">الطالب</th>
                  {#each (matrixData.assessment_types || []) as examType, i}
                    <th class="px-3 py-2.5 text-center font-bold text-muted-foreground min-w-[90px]">{examType}</th>
                  {/each}
                  <th class="px-3 py-2.5 text-center font-bold text-muted-foreground min-w-[70px]">المجموع</th>
                  <th class="px-3 py-2.5 text-center font-bold text-muted-foreground min-w-[60px]">النسبة</th>
                  <th class="px-3 py-2.5 text-center font-bold text-muted-foreground min-w-[70px]">التقدير</th>
                </tr>
              </thead>
              <tbody>
                {#each matrixData.students as student, idx}
                  <tr class="border-b border-border/50 hover:bg-muted/10 transition-colors">
                    <td class="px-3 py-2 text-muted-foreground font-mono sticky start-0 bg-card z-10">{idx + 1}</td>
                    <td class="px-3 py-2 font-semibold text-foreground sticky start-[40px] bg-card z-10">
                      {student.student_name_ar || student.student_name}
                    </td>
                    {#each student.grades as grade, gi}
                      {@const cellKey = `${student.enrollment_id}_${grade.assessment_id || grade.exam_type}`}
                      {@const hasError = errorCells.has(cellKey)}
                      <td class="px-1.5 py-1.5 text-center">
                        {#if canEnterGrades}
                          <input
                            type="number"
                            min="0"
                            max={grade.max_score}
                            step="0.5"
                            value={editedScores[cellKey] ?? grade.score ?? ""}
                            oninput={(e) => handleScoreChange(student.enrollment_id, grade.assessment_id || grade.exam_type, (e.target as HTMLInputElement).value, grade.max_score)}
                            class="w-full text-center bg-background border rounded-md px-2 py-1.5 text-xs font-mono
                              focus:outline-none focus:ring-2 transition-all
                              {hasError
                                ? 'border-red-500 focus:ring-red-500/30 text-red-500'
                                : 'border-border focus:ring-primary/30 text-foreground'}"
                          />
                        {:else}
                          <span class="font-mono text-muted-foreground">{grade.score ?? "—"}</span>
                        {/if}
                      </td>
                    {/each}
                    <td class="px-3 py-2 text-center font-bold font-mono">{student.total.toFixed(1)}</td>
                    <td class="px-3 py-2 text-center font-mono">{student.percentage.toFixed(1)}%</td>
                    <td class="px-3 py-2 text-center">
                      <span class="font-bold text-[10px] {getGradeColor(student.percentage)}">
                        {student.grade_letter || getGradeLetter(student.percentage)}
                      </span>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>

        <!-- Error Count -->
        {#if errorCells.size > 0}
          <div class="flex items-center gap-2 text-red-500 text-[10px]">
            <AlertTriangle size={12} />
            <span>{errorCells.size} خلايا تحتوي على أخطاء — يجب تصحيحها قبل الحفظ</span>
          </div>
        {/if}

      {:else if selectedCycleId && selectedClassId && selectedSubjectId && !isMatrixLoading}
        <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
          <Info size={32} class="opacity-40" />
          <p class="text-xs">لا توجد بيانات لعرضها</p>
          <p class="text-[10px]">تأكد من وجود تقييمات وطلاب مسجلين في هذا الفصل</p>
        </div>
      {:else}
        <div class="flex flex-col items-center justify-center py-12 gap-3 text-muted-foreground">
          <BarChart3 size={32} class="opacity-40" />
          <p class="text-xs">اختر الدورة والفصل والمادة لبدء إدخال الدرجات</p>
        </div>
      {/if}
    </div>

    <!-- ═══════════════════════════════════════════════════════════════════════
         Section 3: Statistics Summary
         ═══════════════════════════════════════════════════════════════════════ -->
    {#if matrixData && matrixData.students && matrixData.students.length > 0}
      {@const students = matrixData.students as any[]}
      {@const percentages = students.map((s: any) => s.percentage)}
      {@const avg = percentages.reduce((a: number, b: number) => a + b, 0) / percentages.length}
      {@const max = Math.max(...percentages)}
      {@const min = Math.min(...percentages)}
      {@const passCount = percentages.filter((p: number) => p >= 50).length}
      {@const passRate = (passCount / percentages.length) * 100}
      {@const excellent = percentages.filter((p: number) => p >= 90).length}
      {@const veryGood = percentages.filter((p: number) => p >= 80 && p < 90).length}
      {@const good = percentages.filter((p: number) => p >= 70 && p < 80).length}
      {@const acceptable = percentages.filter((p: number) => p >= 60 && p < 70).length}
      {@const weak = percentages.filter((p: number) => p >= 50 && p < 60).length}
      {@const fail = percentages.filter((p: number) => p < 50).length}
      {@const total = percentages.length}

      <div class="card-tactile bg-card p-5 flex flex-col gap-4" in:fade={{ delay: 150, duration: 200 }}>
        <h2 class="text-xs font-bold text-foreground flex items-center gap-2">
          <BarChart3 size={14} class="text-primary" />
          ملخص الإحصائيات
        </h2>

        <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
          <!-- Average -->
          <div class="bg-background/50 border border-border rounded-lg p-3 flex flex-col items-center gap-1">
            <span class="text-[10px] text-muted-foreground">المعدل</span>
            <span class="text-lg font-bold {getGradeColor(avg)}">{avg.toFixed(1)}%</span>
            <span class="text-[9px] {getGradeColor(avg)}">{getGradeLetter(avg)}</span>
          </div>
          <!-- Highest -->
          <div class="bg-background/50 border border-border rounded-lg p-3 flex flex-col items-center gap-1">
            <span class="text-[10px] text-muted-foreground">أعلى درجة</span>
            <span class="text-lg font-bold text-emerald-500">{max.toFixed(1)}%</span>
          </div>
          <!-- Lowest -->
          <div class="bg-background/50 border border-border rounded-lg p-3 flex flex-col items-center gap-1">
            <span class="text-[10px] text-muted-foreground">أدنى درجة</span>
            <span class="text-lg font-bold text-red-500">{min.toFixed(1)}%</span>
          </div>
          <!-- Pass Rate -->
          <div class="bg-background/50 border border-border rounded-lg p-3 flex flex-col items-center gap-1">
            <span class="text-[10px] text-muted-foreground">معدل النجاح</span>
            <span class="text-lg font-bold {passRate >= 70 ? 'text-emerald-500' : passRate >= 50 ? 'text-amber-500' : 'text-red-500'}">{passRate.toFixed(0)}%</span>
            <span class="text-[9px] text-muted-foreground">{passCount} / {percentages.length}</span>
          </div>
        </div>

        <!-- Grade Distribution Bar -->

        <div class="flex flex-col gap-2">
          <h3 class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">توزيع التقديرات</h3>
          <div class="flex flex-col gap-1.5">
            {#each [
              { label: "ممتاز", count: excellent, color: "bg-emerald-500", textColor: "text-emerald-500" },
              { label: "جيد جداً", count: veryGood, color: "bg-blue-500", textColor: "text-blue-500" },
              { label: "جيد", count: good, color: "bg-sky-500", textColor: "text-sky-500" },
              { label: "مقبول", count: acceptable, color: "bg-amber-500", textColor: "text-amber-500" },
              { label: "ضعيف", count: weak, color: "bg-orange-500", textColor: "text-orange-500" },
              { label: "راسب", count: fail, color: "bg-red-500", textColor: "text-red-500" },
            ] as dist}
              <div class="flex items-center gap-2">
                <span class="text-[10px] w-14 text-end {dist.textColor} font-semibold">{dist.label}</span>
                <div class="flex-1 h-4 bg-muted/30 rounded-full overflow-hidden">
                  <div
                    class="{dist.color} h-full rounded-full transition-all duration-500"
                    style="width: {total > 0 ? (dist.count / total) * 100 : 0}%"
                  ></div>
                </div>
                <span class="text-[10px] font-mono text-muted-foreground w-6 text-end">{dist.count}</span>
              </div>
            {/each}
          </div>
        </div>
      </div>
    {/if}
  {/if}
</div>
