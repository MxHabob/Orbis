<!--
@component AcademicsAssessmentsPage
Manages assessments (exams, quizzes, homework) for a selected class and subject.
Allows teachers to review assessment listings, weights, max scores, and delete records.
Triggers AssessmentCreateSheet.
-->
<script lang="ts">
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import {
  Plus,
  Award,
  Percent,
  Calendar,
  CheckSquare,
  Trash2,
  AlertCircle,
  FileText,
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import AssessmentCreateSheet from "$lib/components/sheets/assessment-create-sheet.svelte";
import * as m from "$lib/paraglide/messages";

let { data }: { data: { orgId: string } } = $props();

// State
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);
let cycles = $state<any[]>([]);
let activeCycleId = $state<string>("");

// Semester & Month sub-cycles
let semesters = $state<any[]>([]);
let selectedSemesterId = $state<string>("");
let months = $state<any[]>([]);
let selectedMonthId = $state<string>("all");

let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");

let assessments = $state<any[]>([]);
let isLoading = $state(true);
let isAssessmentsLoading = $state(false);
let error = $state<string | null>(null);

// Sheet trigger state
let isModalOpen = $state(false);

// Get 1-based chronological month index
function getMonthNumber(monthId: string): number | undefined {
  const sortedMonths = cycles
    .filter((c) => c.type === "month")
    .sort((a, b) => (a.start_date || "").localeCompare(b.start_date || ""));
  const idx = sortedMonths.findIndex((m) => m.id === monthId);
  return idx !== -1 ? idx + 1 : undefined;
}

// Reactively load child months of the selected semester
$effect(() => {
  if (selectedSemesterId) {
    months = cycles.filter(
      (c) => c.parent_id === selectedSemesterId && c.type === "month",
    );
    selectedMonthId = "all";
  } else {
    months = [];
    selectedMonthId = "all";
  }
});

// Load basic filters
async function loadFilters() {
  isLoading = true;
  error = null;
  try {
    const [classList, subjectList, cycleList] = await Promise.all([
      commands.classes.getAll(data.orgId),
      commands.subjects.getAll(data.orgId),
      commands.academics.getCycles(data.orgId),
    ]);
    classes = classList;
    subjects = subjectList;
    cycles = cycleList;

    const activeYear =
      cycleList.find(
        (c) =>
          c.type === "year" && (c.is_current === 1 || c.is_current === true),
      ) ||
      cycleList.find((c) => c.type === "year") ||
      cycleList[0];

    if (activeYear) {
      activeCycleId = String(activeYear.id);

      // Filter direct child semesters/terms of this year
      semesters = cycleList.filter(
        (c) => c.parent_id === activeYear.id && c.type === "term",
      );
      if (semesters.length > 0) {
        selectedSemesterId = semesters[0].id;
      }
    }

    if (classList.length > 0) {
      selectedClassId = String(classList[0].id);
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
  if (!selectedClassId) return;
  isAssessmentsLoading = true;
  try {
    const list = await commands.assessments.getByClass({
      classId: selectedClassId,
      subjectId: selectedSubjectId || undefined,
      termId: selectedSemesterId || activeCycleId || undefined,
      month:
        selectedMonthId !== "all" ? getMonthNumber(selectedMonthId) : undefined,
    });
    assessments = list;
  } catch (e) {
    console.error("Error loading assessments:", e);
    // Fallback to empty
    assessments = [];
  } finally {
    isAssessmentsLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadFilters();
  }
});

// Reactively load assessments when selections change
$effect(() => {
  if (
    selectedClassId ||
    selectedSubjectId ||
    activeCycleId ||
    selectedSemesterId ||
    selectedMonthId
  ) {
    loadAssessments();
  }
});

// Compute KPI weights
const totalWeight = $derived(
  assessments.reduce((sum, ass) => sum + Number(ass.weight || 0), 0),
);

// Delete assessment
async function handleDeleteAssessment(id: string) {
  if (!confirm(m.assessments_page_confirm_delete())) return;
  try {
    await commands.assessments.delete(id);
    toast.success(m.assessments_page_success_delete());
    await loadAssessments();
  } catch (e) {
    toast.error(m.assessments_page_error_delete());
  }
}

function getAssessmentTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "exam") return m.assessments_type_exam();
  if (t === "quiz") return m.assessments_type_quiz();
  if (t === "homework") return m.assessments_type_homework();
  if (t === "classwork") return m.assessments_type_classwork();
  return typeStr;
}

function getAssessmentTypeColor(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "exam") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  if (t === "quiz")
    return "bg-purple-500/10 text-purple-500 border-purple-500/20";
  if (t === "homework")
    return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Filter & Selector Header -->
	<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0 select-none">
		<div class="grid grid-cols-1 sm:grid-cols-2 md:flex items-center gap-4 w-full md:w-auto flex-wrap">
			<!-- Class Selector -->
			<div class="flex flex-col sm:flex-row sm:items-center gap-2 text-start">
				<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
					{m.assessments_page_class()}
				</span>
				<select 
					bind:value={selectedClassId} 
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-44"
				>
					{#each classes as cls (cls.id)}
						<option value={String(cls.id)}>{cls.name_ar || cls.name}</option>
					{/each}
				</select>
			</div>

			<!-- Subject Selector -->
			<div class="flex flex-col sm:flex-row sm:items-center gap-2 text-start">
				<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
					{m.assessments_page_subject()}
				</span>
				<select 
					bind:value={selectedSubjectId} 
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-44"
				>
					<option value="">{m.assessments_page_all_subjects()}</option>
					{#each subjects as sub (sub.id)}
						<option value={String(sub.id)}>{sub.name_ar || sub.name}</option>
					{/each}
				</select>
			</div>

			<!-- Semester Selector -->
			{#if semesters.length > 0}
				<div class="flex flex-col sm:flex-row sm:items-center gap-2 text-start">
					<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
						{getLocale() === "ar" ? "الفصل الدراسي" : "Semester"}
					</span>
					<select 
						bind:value={selectedSemesterId} 
						class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-44"
					>
						{#each semesters as sem (sem.id)}
							<option value={String(sem.id)}>{sem.name_ar || sem.name}</option>
						{/each}
					</select>
				</div>
			{/if}

			<!-- Month Selector -->
			{#if months.length > 0}
				<div class="flex flex-col sm:flex-row sm:items-center gap-2 text-start">
					<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
						{getLocale() === "ar" ? "الشهر الهجري" : "Hijri Month"}
					</span>
					<select 
						bind:value={selectedMonthId} 
						class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-44"
					>
						<option value="all">{getLocale() === "ar" ? "كل التقييمات الشهرية" : "All Monthly Exams"}</option>
						{#each months as m (m.id)}
							<option value={String(m.id)}>{m.name_ar || m.name}</option>
						{/each}
					</select>
				</div>
			{/if}
		</div>

		<!-- Action Add Button -->
		<Button 
			variant="lime" 
			class="text-xs h-9 px-3 shrink-0" 
			onclick={() => isModalOpen = true}
			disabled={classes.length === 0 || !selectedSubjectId}
		>
			<Plus size={14} class={getLocale() === "ar" ? 'ml-1.5' : 'mr-1.5'} />
			{m.assessments_page_new_button()}
		</Button>
	</div>

	<!-- KPI Summary Weight indicators -->
	{#if !isAssessmentsLoading && assessments.length > 0}
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4" in:fade={{ duration: 150 }}>
			<!-- Total Assessments -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.assessments_page_total_count()}
					</span>
					<span class="text-xl font-bold text-foreground font-mono">{assessments.length}</span>
				</div>
				<div class="h-9 w-9 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
					<FileText size={18} />
				</div>
			</div>

			<!-- Total Weight Card -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.assessments_page_total_weight()}
					</span>
					<span class="text-xl font-bold font-mono {totalWeight > 100 ? 'text-destructive' : 'text-foreground'}">
						{totalWeight}% / 100%
					</span>
				</div>
				<div class="h-9 w-9 rounded-lg bg-brand-lime/10 flex items-center justify-center text-brand-lime">
					<Percent size={16} />
				</div>
			</div>

			<!-- System Alert if weight doesn't sum to 100% -->
			<div class="card-tactile bg-card/40 flex items-center gap-3 p-4 h-20 text-start select-none">
				{#if totalWeight === 100}
					<div class="h-9 w-9 rounded-lg bg-emerald-500/10 text-emerald-500 flex items-center justify-center shrink-0">
						<CheckSquare size={16} />
					</div>
					<div class="flex flex-col min-w-0">
						<span class="text-[10px] font-bold text-emerald-600 dark:text-emerald-400 uppercase tracking-wider">{m.assessments_page_weight_complete_title()}</span>
						<span class="text-[9px] text-muted-foreground line-clamp-1">{m.assessments_page_weight_complete_desc()}</span>
					</div>
				{:else}
					<div class="h-9 w-9 rounded-lg bg-amber-500/10 text-amber-500 flex items-center justify-center shrink-0">
						<AlertCircle size={16} />
					</div>
					<div class="flex flex-col min-w-0">
						<span class="text-[10px] font-bold text-amber-600 dark:text-amber-400 uppercase tracking-wider">{m.assessments_page_weight_alert_title()}</span>
						<span class="text-[9px] text-muted-foreground line-clamp-1">
							{m.assessments_page_weight_alert_desc({ weight: totalWeight })}
						</span>
					</div>
				{/if}
			</div>
		</div>
	{/if}

	<!-- Main Assessment Directory Content -->
	{#if isAssessmentsLoading}
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
			<Skeleton class="h-32 w-full" />
			<Skeleton class="h-32 w-full" />
		</div>
	{:else if assessments.length === 0}
		<div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4">
			<Award size={48} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{m.assessments_page_empty_title()}</p>
				<p class="text-[10px] text-muted-foreground/80">
					{m.assessments_page_empty_desc()}
				</p>
			</div>
			<Button variant="lime" class="text-xs mt-2" onclick={() => isModalOpen = true} disabled={classes.length === 0}>
				{m.assessments_page_new_button()}
			</Button>
		</div>
	{:else}
		<!-- Grid List of Assessments -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4" in:fade={{ duration: 200 }}>
			{#each assessments as ass (ass.id)}
				{@const typeColor = getAssessmentTypeColor(ass.type || "exam")}
				<div class="card-tactile bg-card flex flex-col justify-between h-36 p-4 select-none group relative transition-all duration-150">
					<!-- Header -->
					<div class="flex items-start justify-between">
						<div class="flex flex-col text-start min-w-0 flex-1">
							<span class="text-xs font-bold text-foreground truncate group-hover:text-primary transition-colors">
								{ass.name_ar || ass.name || ass.id}
							</span>
							{#if ass.name && ass.name_ar}
								<span class="text-[9px] text-muted-foreground font-mono truncate">{ass.name}</span>
							{/if}
						</div>

						<button 
							onclick={() => handleDeleteAssessment(ass.id)}
							class="text-muted-foreground/40 hover:text-destructive h-5 w-5 rounded flex items-center justify-center hover:bg-destructive/5 duration-100 shrink-0"
							title={m.assessments_page_delete_title()}
						>
							<Trash2 size={11} />
						</button>
					</div>

					<!-- Details parameters -->
					<div class="flex items-center gap-3 my-2.5">
						<!-- Max score badge -->
						<div class="flex flex-col text-start">
							<span class="text-[8px] uppercase tracking-wider text-muted-foreground">{m.assessments_page_max_score()}</span>
							<span class="font-bold text-foreground font-mono">{ass.max_score || 100}</span>
						</div>

						<!-- Vertical Divider -->
						<div class="h-6 w-px bg-border"></div>

						<!-- Weight badge -->
						<div class="flex flex-col text-start">
							<span class="text-[8px] uppercase tracking-wider text-muted-foreground">{m.assessments_page_weight()}</span>
							<span class="font-bold text-foreground font-mono">{ass.weight || 15}%</span>
						</div>
					</div>

					<!-- Footer Status tags -->
					<div class="flex items-center justify-between border-t border-border/60 pt-2 text-[9px]">
						<span class="px-2 py-0.5 rounded-full border text-[8px] font-bold uppercase tracking-wider {typeColor}">
							{getAssessmentTypeLabel(ass.type || "exam")}
						</span>

						<span class="text-muted-foreground font-mono text-[8px] flex items-center gap-0.5">
							<Calendar size={9} />
							<span>{ass.created_at ? String(ass.created_at).slice(0, 10) : "-"}</span>
						</span>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<!-- Assessment Creation Side Sheet -->
<AssessmentCreateSheet
	bind:open={isModalOpen}
	classId={selectedClassId}
	subjectId={selectedSubjectId}
	cycleId={selectedSemesterId || activeCycleId}
	month={selectedMonthId !== "all" ? getMonthNumber(selectedMonthId) : undefined}
	onSuccess={loadAssessments}
/>
