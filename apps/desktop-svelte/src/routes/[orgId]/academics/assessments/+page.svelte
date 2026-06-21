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
  Plus, Award, Percent, Calendar, CheckSquare, 
  Trash2, AlertCircle, FileText
} from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import AssessmentCreateSheet from "$lib/components/academics/assessment-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();
const isAr = $derived(getLocale() === "ar");

// State
let classes = $state<any[]>([]);
let subjects = $state<any[]>([]);
let cycles = $state<any[]>([]);
let activeCycleId = $state<string>("");

let selectedClassId = $state<string>("");
let selectedSubjectId = $state<string>("");

let assessments = $state<any[]>([]);
let isLoading = $state(true);
let isAssessmentsLoading = $state(false);
let error = $state<string | null>(null);

// Sheet trigger state
let isModalOpen = $state(false);

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

    const active = cycleList.find((c) => c.is_current === 1 || c.is_current === true) || cycleList[0];
    if (active) {
      activeCycleId = String(active.id);
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
      termId: activeCycleId || undefined,
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
  if (selectedClassId || selectedSubjectId || activeCycleId) {
    loadAssessments();
  }
});

// Compute KPI weights
const totalWeight = $derived(
  assessments.reduce((sum, ass) => sum + Number(ass.weight || 0), 0)
);

// Delete assessment
async function handleDeleteAssessment(id: string) {
  if (!confirm(isAr ? "هل أنت متأكد من حذف هذا التقييم؟" : "Are you sure you want to delete this assessment?")) return;
  try {
    await commands.assessments.delete(id);
    toast.success(isAr ? "تم حذف التقييم" : "Assessment deleted");
    await loadAssessments();
  } catch (e) {
    toast.error(isAr ? "فشل حذف التقييم" : "Failed to delete assessment");
  }
}

function getAssessmentTypeLabel(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "exam") return isAr ? "اختبار نهائي" : "Final Exam";
  if (t === "quiz") return isAr ? "اختبار قصير" : "Quiz";
  if (t === "homework") return isAr ? "واجب منزلي" : "Homework";
  if (t === "classwork") return isAr ? "مشاركة صفية" : "Classwork";
  return typeStr;
}

function getAssessmentTypeColor(typeStr: string) {
  const t = String(typeStr).toLowerCase();
  if (t === "exam") return "bg-rose-500/10 text-rose-500 border-rose-500/20";
  if (t === "quiz") return "bg-purple-500/10 text-purple-500 border-purple-500/20";
  if (t === "homework") return "bg-blue-500/10 text-blue-500 border-blue-500/20";
  return "bg-emerald-500/10 text-emerald-500 border-emerald-500/20";
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6 font-sans">
	<!-- Filter & Selector Header -->
	<div class="flex flex-col md:flex-row md:items-center md:justify-between gap-4 bg-card/25 p-4 rounded-xl border border-border/80 shrink-0 select-none">
		<div class="grid grid-cols-1 sm:grid-cols-2 md:flex items-center gap-4 w-full md:w-auto">
			<!-- Class Selector -->
			<div class="flex flex-col sm:flex-row sm:items-center gap-2 text-start">
				<span class="text-xs font-bold text-muted-foreground uppercase tracking-wider whitespace-nowrap">
					{isAr ? "الفصل الدراسي:" : "Class:"}
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
					{isAr ? "المادة الدراسية:" : "Subject:"}
				</span>
				<select 
					bind:value={selectedSubjectId} 
					class="bg-background text-foreground border border-border rounded-md px-3 py-1.5 text-xs focus:ring-1 focus:ring-ring outline-none min-w-44"
				>
					<option value="">{isAr ? "جميع المواد" : "All Subjects"}</option>
					{#each subjects as sub (sub.id)}
						<option value={String(sub.id)}>{sub.name_ar || sub.name}</option>
					{/each}
				</select>
			</div>
		</div>

		<!-- Action Add Button -->
		<Button 
			variant="lime" 
			class="text-xs h-9 px-3 shrink-0" 
			onclick={() => isModalOpen = true}
			disabled={classes.length === 0}
		>
			<Plus size={14} class={isAr ? 'ml-1.5' : 'mr-1.5'} />
			{isAr ? "تقييم جديد" : "New Assessment"}
		</Button>
	</div>

	<!-- KPI Summary Weight indicators -->
	{#if !isAssessmentsLoading && assessments.length > 0}
		<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4" in:fade={{ duration: 150 }}>
			<!-- Total Assessments -->
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4 h-20 select-none">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[9px] font-bold text-muted-foreground uppercase tracking-wider">
						{isAr ? "عدد التقييمات" : "Total Assessments"}
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
						{isAr ? "إجمالي وزن الدرجات" : "Total Term Weight"}
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
						<span class="text-[10px] font-bold text-emerald-600 dark:text-emerald-400 uppercase tracking-wider">{isAr ? "الدرجات متطابقة" : "Weight Complete"}</span>
						<span class="text-[9px] text-muted-foreground line-clamp-1">{isAr ? "مجموع الأوزان يعادل 100% للترم" : "Weights sum perfectly to 100%"}</span>
					</div>
				{:else}
					<div class="h-9 w-9 rounded-lg bg-amber-500/10 text-amber-500 flex items-center justify-center shrink-0">
						<AlertCircle size={16} />
					</div>
					<div class="flex flex-col min-w-0">
						<span class="text-[10px] font-bold text-amber-600 dark:text-amber-400 uppercase tracking-wider">{isAr ? "تنبيه توزيع الأوزان" : "Weight Discrepancy"}</span>
						<span class="text-[9px] text-muted-foreground line-clamp-1">
							{isAr 
								? `مجموع الأوزان ${totalWeight}% ولا يعادل 100%` 
								: `Current sum is ${totalWeight}% instead of 100%`}
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
				<p class="text-xs font-semibold">{isAr ? "لا توجد تقييمات مسجلة" : "No assessments listed"}</p>
				<p class="text-[10px] text-muted-foreground/80">
					{isAr ? "لا يوجد اختبار أو واجب مضاف لهذا التحديد حالياً." : "No exams, homework, or classwork recorded for this selection."}
				</p>
			</div>
			<Button variant="lime" class="text-xs mt-2" onclick={() => isModalOpen = true} disabled={classes.length === 0}>
				{isAr ? "تقييم جديد" : "New Assessment"}
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
							title={isAr ? "حذف التقييم" : "Delete Assessment"}
						>
							<Trash2 size={11} />
						</button>
					</div>

					<!-- Details parameters -->
					<div class="flex items-center gap-3 my-2.5">
						<!-- Max score badge -->
						<div class="flex flex-col text-start">
							<span class="text-[8px] uppercase tracking-wider text-muted-foreground">{isAr ? "الدرجة القصوى" : "Max Score"}</span>
							<span class="font-bold text-foreground font-mono">{ass.max_score || 100}</span>
						</div>

						<!-- Vertical Divider -->
						<div class="h-6 w-px bg-border"></div>

						<!-- Weight badge -->
						<div class="flex flex-col text-start">
							<span class="text-[8px] uppercase tracking-wider text-muted-foreground">{isAr ? "الوزن النسبي" : "Weight"}</span>
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
	cycleId={activeCycleId}
	onSuccess={loadAssessments}
/>

