import * as m from "$lib/paraglide/messages";
<!--
@component ClassAssessmentsPage
Manages assessments for the current class (classId from URL context).
No more class filter needed — subject & period filters only.
Moved from academics/assessments/ with context-first navigation.
-->
<script lang="ts">
import { page } from "$app/state";
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
import {
  Plus,
  Award,
  Percent,
  Calendar,
  Trash2,
  AlertCircle,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { fade } from "svelte/transition";
import AssessmentCreateSheet from "$lib/features/academics/components/assessment-create-sheet.svelte";
import * as common from "$lib/paraglide/messages/common";

const orgId = $derived(page.params.orgId);
const classId = $derived(page.params.classId);

// State
let subjects = $state<any[]>([]);
let periods = $state<any[]>([]);
let activePeriodId = $state<string>("");

// Semester & Month sub-periods
let semesters = $state<any[]>([]);
let selectedSemesterId = $state<string>("");
let months = $state<any[]>([]);
let selectedMonthId = $state<string>("all");

let selectedSubjectId = $state<string>("");

let assessments = $state<any[]>([]);
let isLoading = $state(true);
let isAssessmentsLoading = $state(false);
let error = $state<string | null>(null);

// Sheet trigger state
let isModalOpen = $state(false);

// Get 1-based chronological month index
function getMonthNumber(monthId: string): number | undefined {
  const sortedMonths = periods
    .filter((c) => c.type === "month")
    .sort((a, b) => (a.start_date || "").localeCompare(b.start_date || ""));
  const idx = sortedMonths.findIndex((m) => m.id === monthId);
  return idx !== -1 ? idx + 1 : undefined;
}

// Reactively load child months of the selected semester
$effect(() => {
  if (selectedSemesterId) {
    months = periods.filter(
      (c) => c.parent_id === selectedSemesterId && c.type === "month",
    );
    selectedMonthId = "all";
  } else {
    months = [];
    selectedMonthId = "all";
  }
});

// Load filters (subjects, periods — class is already from URL)
async function loadFilters() {
  isLoading = true;
  error = null;
  try {
    const [subjectList, periodList] = await Promise.all([
      commands.academic.listSubjects(orgId),
      commands.academic.listPeriods(orgId),
    ]);
    subjects = subjectList;
    periods = periodList;

    const activeYear =
      periodList.find(
        (c) =>
          c.type === "year" && (c.is_current === 1 || c.is_current === true),
      ) ||
      periodList.find((c) => c.type === "year") ||
      periodList[0];

    if (activeYear) {
      activePeriodId = String(activeYear.id);
      semesters = periodList.filter(
        (c) => c.parent_id === activeYear.id && c.type === "term",
      );
      if (semesters.length > 0) {
        selectedSemesterId = semesters[0].id;
      }
    }

    if (subjectList.length > 0) {
      selectedSubjectId = String(subjectList[0].id);
    }
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
  } finally {
    isLoading = false;
  }
}

// Load assessments based on selections
async function loadAssessments() {
  if (!classId) return;
  isAssessmentsLoading = true;
  try {
    const list = await commands.assessment.listAssessments({
      classId,
      subjectId: selectedSubjectId || undefined,
      termId: selectedSemesterId || activePeriodId || "",
      month:
        selectedMonthId !== "all" ? getMonthNumber(selectedMonthId) : undefined,
    });
    assessments = list;
  } catch (e) {
    console.error("Error loading assessments:", e);
    assessments = [];
  } finally {
    isAssessmentsLoading = false;
  }
}

onMount(() => {
  if (orgId) loadFilters();
});

// Reactively load assessments when selections change
$effect(() => {
  if (classId && selectedSubjectId) {
    loadAssessments();
  }
});

async function handleDelete(id: string) {
  if (
    !confirm(
      m.assessments_delete_confirm
        ? m.assessments_delete_confirm()
        : "هل تريد حذف هذا التقييم؟",
    )
  )
    return;
  try {
    await commands.assessment.deleteAssessment(id);
    assessments = assessments.filter((a) => String(a.id) !== id);
    toast.success(
      m.assessments_delete_success
        ? m.assessments_delete_success()
        : "تم الحذف",
    );
  } catch (e: unknown) {
    toast.error((e as any)?.message || "فشل الحذف");
  }
}

// Weight total for current view
const totalWeight = $derived(
  assessments.reduce((sum, a) => sum + (Number(a.weight) || 0), 0),
);
</script>

<div class="flex flex-col gap-5 text-start">
  <!-- Filters Bar -->
  <div class="flex items-center gap-3 flex-wrap">
    <!-- Subject Filter -->
    <div class="flex items-center gap-2">
      <span class="text-xs text-muted-foreground font-medium">
        {m.assessments_subject_filter ? m.assessments_subject_filter() : "المادة"}:
      </span>
      <Select
        type="single"
        value={selectedSubjectId}
        onValueChange={(v) => (selectedSubjectId = v ?? "")}
      >
        <SelectTrigger class="h-7 text-xs w-40 border-border/50">
          {selectedSubjectId
            ? String(
                subjects.find((s) => String(s.id) === selectedSubjectId)
                  ?.name_ar ??
                  subjects.find((s) => String(s.id) === selectedSubjectId)
                    ?.name ??
                  selectedSubjectId,
              )
            : "اختر مادة"}
        </SelectTrigger>
        <SelectContent>
          {#each subjects as subj (subj.id)}
            <SelectItem value={String(subj.id)}>
              {subj.name_ar || subj.name || subj.id}
            </SelectItem>
          {/each}
        </SelectContent>
      </Select>
    </div>

    <!-- Semester Filter -->
    {#if semesters.length > 0}
      <div class="flex items-center gap-2">
        <span class="text-xs text-muted-foreground font-medium">
          {m.assessments_semester_filter ? m.assessments_semester_filter() : "الفصل الدراسي"}:
        </span>
        <Select
          type="single"
          value={selectedSemesterId}
          onValueChange={(v) => (selectedSemesterId = v ?? "")}
        >
          <SelectTrigger class="h-7 text-xs w-40 border-border/50">
            {selectedSemesterId
              ? String(
                  semesters.find((s) => s.id === selectedSemesterId)?.name_ar ??
                    semesters.find((s) => s.id === selectedSemesterId)?.name ??
                    selectedSemesterId,
                )
              : "اختر فترة"}
          </SelectTrigger>
          <SelectContent>
            {#each semesters as sem (sem.id)}
              <SelectItem value={String(sem.id)}>
                {sem.name_ar || sem.name || sem.id}
              </SelectItem>
            {/each}
          </SelectContent>
        </Select>
      </div>
    {/if}

    <div class="flex-1"></div>

    <!-- Add Assessment Button -->
    <Button
      variant="lime"
      size="sm"
      class="h-8 text-xs gap-1.5"
      onclick={() => (isModalOpen = true)}
    >
      <Plus size={13} />
      {m.assessments_add ? m.assessments_add() : "إضافة تقييم"}
    </Button>
  </div>

  <!-- Weight Summary -->
  {#if assessments.length > 0}
    <div
      class="flex items-center gap-2 text-xs text-muted-foreground"
      in:fade={{ duration: 150 }}
    >
      <Percent size={12} />
      <span>
        {m.assessments_total_weight
          ? m.assessments_total_weight()
          : "إجمالي الأوزان"}: <strong
          class={totalWeight === 100
            ? "text-primary"
            : "text-destructive"}>{totalWeight}%</strong
        >
      </span>
    </div>
  {/if}

  <!-- Assessments List -->
  {#if isLoading}
    <div class="flex flex-col gap-3">
      {#each Array(3) as _}
        <Skeleton class="h-16 w-full rounded-lg" />
      {/each}
    </div>
  {:else if assessments.length === 0}
    <div
      class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-border border-dashed rounded-xl gap-3"
    >
      <Award size={40} class="text-muted-foreground/30" />
      <p class="text-xs font-semibold">
        {common.assessments_empty ? common.assessments_empty() : "لا توجد تقييمات"}
      </p>
    </div>
  {:else}
    <div class="flex flex-col gap-2" in:fade={{ duration: 200 }}>
      {#each assessments as assessment (assessment.id)}
        <div
          class="flex items-center justify-between p-4 bg-card border border-border rounded-lg hover:border-primary/20 transition-colors"
        >
          <div class="flex flex-col gap-1 text-start">
            <span class="text-xs font-bold text-foreground">
              {assessment.name || assessment.name_ar || assessment.type}
            </span>
            <div class="flex items-center gap-3 text-[10px] text-muted-foreground">
              <span class="flex items-center gap-1">
                <Percent size={10} />
                {common.assessments_weight
                  ? common.assessments_weight()
                  : "الوزن"}: {assessment.weight}%
              </span>
              <span class="flex items-center gap-1">
                <Award size={10} />
                {common.assessments_max_score
                  ? common.assessments_max_score()
                  : "أقصى درجة"}: {assessment.max_score}
              </span>
              {#if assessment.date}
                <span class="flex items-center gap-1">
                  <Calendar size={10} />
                  {assessment.date}
                </span>
              {/if}
            </div>
          </div>
          <Button
            variant="ghost"
            size="icon-sm"
            class="text-destructive/60 hover:text-destructive hover:bg-destructive/10"
            onclick={() => handleDelete(String(assessment.id))}
          >
            <Trash2 size={13} />
          </Button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<!-- Assessment Creation Sheet -->
<AssessmentCreateSheet
  bind:open={isModalOpen}
  orgId={orgId}
  preselectedClassId={classId}
  onSuccess={loadAssessments}
/>
