<!--
@component AcademicsClassesPage
Lists all academic classes with student counts, capacity utilization progress bars,
and triggers the class creation side sheet. Each card links to the class detail page.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { Plus, Users, GraduationCap, Percent } from "lucide-svelte";
import { toast } from "svelte-sonner";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import { fade } from "svelte/transition";
import ClassCreateSheet from "$lib/features/academics/components/class-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

// State
let classes = $state<any[]>([]);
let students = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);

// Sheet trigger state
let isModalOpen = $state(false);

// Load data
async function loadData() {
  isLoading = true;
  error = null;
  try {
    const [classData, studentData] = await Promise.all([
      commands.academic.listClasses(),
      commands.academic.listEnrollments(),
    ]);
    classes = classData;
    students = studentData;
  } catch (e) {
    error = e instanceof Error ? e.message : String(e);
    toast.error(m.common_classes_load_error());
  } finally {
    isLoading = false;
  }
}

onMount(() => {
  if (data.orgId) {
    loadData();
  }
});

// Calculate student counts per class
function getStudentCount(classId: string): number {
  return students.filter((s) => String(s.class_id) === String(classId)).length;
}

// Compute statistics
const totalClassesCount = $derived(classes.length);
const totalStudentsCount = $derived(students.length);
const averageClassSize = $derived(
  totalClassesCount > 0
    ? Math.round(totalStudentsCount / totalClassesCount)
    : 0,
);
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-6">
	<!-- KPI Metrics Summary -->
	{#if !isLoading && !error && classes.length > 0}
		<div class="grid grid-cols-1 md:grid-cols-3 gap-4" in:fade={{ duration: 200 }}>
			<div class="card-tactile bg-card/40 flex items-center justify-between p-4">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.common_classes_total_classes()}
					</span>
					<span class="text-2xl font-bold text-foreground font-mono">{totalClassesCount}</span>
				</div>
				<div class="h-10 w-10 rounded-lg bg-primary/10 flex items-center justify-center text-primary">
					<GraduationCap size={20} />
				</div>
			</div>

			<div class="card-tactile bg-card/40 flex items-center justify-between p-4">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.common_classes_enrolled_students()}
					</span>
					<span class="text-2xl font-bold text-foreground font-mono">{totalStudentsCount}</span>
				</div>
				<div class="h-10 w-10 rounded-lg bg-brand-lime/10 flex items-center justify-center text-brand-lime">
					<Users size={20} />
				</div>
			</div>

			<div class="card-tactile bg-card/40 flex items-center justify-between p-4">
				<div class="flex flex-col gap-1 text-start">
					<span class="text-[10px] font-bold text-muted-foreground uppercase tracking-wider">
						{m.common_classes_avg_size()}
					</span>
					<span class="text-2xl font-bold text-foreground font-mono">{averageClassSize}</span>
				</div>
				<div class="h-10 w-10 rounded-lg bg-brand-pink/10 flex items-center justify-center text-brand-pink">
					<Percent size={18} />
				</div>
			</div>
		</div>
	{/if}

	<!-- Header Area -->
	<div class="flex items-center justify-between font-sans">
		<h2 class="text-sm font-bold text-foreground uppercase tracking-wider flex items-center gap-2">
			<span>{m.common_classes_active_classes()}</span>
			{#if !isLoading}
				<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground">
					{classes.length}
				</span>
			{/if}
		</h2>

		<Button variant="lime" class="text-xs h-9 px-3" onclick={() => isModalOpen = true}>
			<Plus size={14} class={m.academics_classes_mr_1_5()} />
			{m.common_classes_new_class()}
		</Button>
	</div>

	<!-- Main Directory Content -->
	{#if isLoading}
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
			<Skeleton class="h-32 w-full" />
			<Skeleton class="h-32 w-full" />
			<Skeleton class="h-32 w-full" />
			<Skeleton class="h-32 w-full" />
		</div>
	{:else if error}
		<div class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3">
			<p class="text-xs text-destructive font-semibold">{m.common_classes_load_error()}</p>
			<p class="text-xs text-muted-foreground">{error}</p>
			<Button variant="outline" class="text-xs" onclick={loadData}>{m.common_retry()}</Button>
		</div>
	{:else if classes.length === 0}
		<div class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4">
			<GraduationCap size={48} class="text-muted-foreground/30" />
			<div class="flex flex-col gap-1 text-center">
				<p class="text-xs font-semibold">{m.common_classes_empty_title()}</p>
				<p class="text-[10px] text-muted-foreground/80">{m.common_classes_empty_desc()}</p>
			</div>
			<Button variant="lime" class="text-xs mt-2" onclick={() => isModalOpen = true}>
				{m.common_classes_add_class()}
			</Button>
		</div>
	{:else}
		<!-- Classes Grid — each card links to class detail -->
		<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4" in:fade={{ duration: 250 }}>
			{#each classes as cls (cls.id)}
				{@const count = getStudentCount(String(cls.id))}
				{@const limit = Number(cls.capacity || 30)}
				{@const ratio = Math.min(100, Math.round((count / limit) * 100))}
				<a
					href={localizeHref(`/${data.orgId}/academics/classes/${cls.id}`)}
					class="card-tactile bg-card flex flex-col justify-between h-36 p-4 select-none relative group transition-all duration-150 hover:border-primary/30 cursor-pointer no-underline"
				>
					<!-- Class Header -->
					<div class="flex items-start justify-between">
						<div class="flex flex-col text-start">
							<span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
								{cls.name_ar || cls.name || cls.id}
							</span>
							{#if cls.name && cls.name_ar}
								<span class="text-[9px] text-muted-foreground font-mono">{cls.name}</span>
							{/if}
						</div>
						<div class="px-1.5 py-0.5 rounded text-[9px] font-mono border border-border bg-background text-muted-foreground">
							{cls.code || `CLASS-${String(cls.id).slice(0, 3).toUpperCase()}`}
						</div>
					</div>

					<!-- Class Capacity Bar -->
					<div class="flex flex-col gap-1.5 my-3">
						<div class="flex items-center justify-between text-[10px]">
							<span class="text-muted-foreground flex items-center gap-1">
								<Users size={10} />
								<span>{m.common_insight_students_link({ count })}</span>
							</span>
							<span class="font-mono {ratio >= 100 ? 'text-destructive font-bold' : 'text-muted-foreground'}">
								{count}/{limit} ({ratio}%)
							</span>
						</div>
						<div class="h-1.5 w-full bg-muted rounded-full overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-300 {ratio >= 100 ? 'bg-destructive' : ratio >= 80 ? 'bg-amber-500' : 'bg-primary'}"
								style="width: {ratio}%"
							></div>
						</div>
					</div>

					<!-- Class Footer -->
					<div class="flex items-center justify-between border-t border-border/60 pt-2 text-[9px] text-muted-foreground">
						<span>
							{m.common_classes_grade_level_label({ level: cls.grade_level || 'N/A' })}
						</span>
						{#if ratio >= 100}
							<span class="text-destructive font-bold uppercase text-[8px] tracking-wider px-1 py-0.2 rounded border border-destructive/20 bg-destructive/5">
								{m.common_classes_status_full()}
							</span>
						{/if}
					</div>
				</a>
			{/each}
		</div>
	{/if}
</div>

<!-- Class Creation Side Sheet Component -->
<ClassCreateSheet bind:open={isModalOpen} orgId={data.orgId} onSuccess={loadData} />
