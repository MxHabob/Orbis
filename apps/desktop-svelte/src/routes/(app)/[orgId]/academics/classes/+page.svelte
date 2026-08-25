<!--
@component AcademicsClassesPage
Lists all academic classes with student counts, capacity utilization progress bars,
and triggers the class creation side sheet. Each card links to the class detail page.
-->
<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { onMount } from "svelte";
import { createQuery } from "@tanstack/svelte-query";
import { commands } from "$lib/core/ipc";
import { Button } from "@orbis/ui-svelte/button";
import { Skeleton } from "@orbis/ui-svelte/skeleton";
import { MetricProgress } from "@orbis/ui-svelte";
import { Plus, Users, GraduationCap } from "@lucide/svelte";
import { toast } from "svelte-sonner";
import { localizeHref } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n/locale";
import { fade } from "svelte/transition";
import ClassCreateSheet from "$lib/features/academics/components/class-create-sheet.svelte";

let { data }: { data: { orgId: string } } = $props();

// State
let classes = $state<any[]>([]);
let isLoading = $state(true);
let error = $state<string | null>(null);
let activePeriodId = $state("");

const periodsQuery = createQuery(() => ({
	queryKey: ["classes-periods", data.orgId],
	queryFn: () => commands.calendar.listAcademicPeriods(),
	enabled: !!data.orgId,
	staleTime: 10 * 60 * 1000,
}));

const classMetricsQuery = createQuery(() => ({
	queryKey: ["classes-metrics", data.orgId, activePeriodId],
	queryFn: () => commands.assessment.classMetrics(activePeriodId),
	enabled: !!data.orgId && !!activePeriodId,
	staleTime: 2 * 60 * 1000,
}));

const periods = $derived(periodsQuery.data ?? []);
const classMetrics = $derived(classMetricsQuery.data ?? []);

$effect(() => {
	if (periods.length > 0 && !activePeriodId) {
		const active = periods.find((period: any) => period.status === "active") ?? periods[0];
		activePeriodId = String(active.id);
	}
});

// Sheet trigger state
let isModalOpen = $state(false);

// Load data
async function loadData() {
  isLoading = true;
  error = null;
  try {
		const classData = await commands.academic.listClasses();
    classes = classData;
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

function getClassMetric(classId: string): any {
	return classMetrics.find((metric: any) => String(metric.class_id) === String(classId));
}

// Compute statistics
const totalClassesCount = $derived(classes.length);
const totalStudentsCount = $derived(
	classMetrics.reduce((total: number, metric: any) => total + Number(metric.active_students || 0), 0),
);
const averageClassSize = $derived(
  totalClassesCount > 0
    ? Math.round(totalStudentsCount / totalClassesCount)
    : 0,
);
const pageLoading = $derived(
	isLoading || periodsQuery.isLoading || classMetricsQuery.isLoading,
);
</script>

<div class="h-full w-full min-w-0 overflow-y-auto overflow-x-hidden scrollbar-hide bg-background p-3 sm:p-4 flex flex-col gap-4">
	<!-- KPI Metrics Summary -->
	{#if !pageLoading && !error && classes.length > 0}
		<div class="grid grid-cols-1 gap-3 sm:grid-cols-3" in:fade={{ duration: 200 }}>
			<div class="rounded-lg border border-border/70 bg-card p-3">
		        <MetricProgress variant="single" value={totalClassesCount} showProgress={false} tone="primary">
					{#snippet title()}{m.common_classes_total_classes()}{/snippet}
							{#snippet metadata()}{m.common_classes_total_classes()}{/snippet}
				</MetricProgress>
			</div>
			<div class="rounded-lg border border-border/70 bg-card p-3">
				<MetricProgress variant="single" value={totalStudentsCount} showProgress={false} tone="success">
					{#snippet title()}{m.common_classes_enrolled_students()}{/snippet}
					{#snippet metadata()}{m.common_classes_enrolled_students()}{/snippet}
				</MetricProgress>
			</div>
			<div class="rounded-lg border border-border/70 bg-card p-3">
				<MetricProgress variant="single" value={averageClassSize} showProgress={false} tone="warning">
					{#snippet title()}{m.common_classes_avg_size()}{/snippet}
					{#snippet metadata()}{m.common_classes_avg_size()}{/snippet}
				</MetricProgress>
			</div>
		</div>
	{/if}

	<!-- Header Area -->
		<div class="flex min-w-0 flex-wrap items-center justify-between gap-3 border-b border-border/70 pb-3 font-sans">
			<h2 class="flex items-center gap-2 text-sm font-bold uppercase tracking-wider text-foreground">
			<span>{m.common_card_active_classes()}</span>
			{#if !pageLoading}
				<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted font-mono text-muted-foreground">
					{classes.length}
				</span>
			{/if}
		</h2>

		<Button variant="default" class="h-8 px-3 text-xs" onclick={() => isModalOpen = true}>
			<Plus size={14} class={m.academics_classes_mr_1_5()} />
			{m.common_classes_new_class()}
		</Button>
	</div>

	<!-- Main Directory Content -->
	{#if pageLoading}
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
			<Button variant="default" class="text-xs mt-2" onclick={() => isModalOpen = true}>
				{m.common_classes_add_class()}
			</Button>
		</div>
	{:else}
		<!-- Classes Grid — each card links to class detail -->
		<div class="grid min-w-0 grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4" in:fade={{ duration: 250 }}>
			{#each classes as cls (cls.id)}
				{@const metric = getClassMetric(String(cls.id))}
				{@const displayName = l(cls.name_ar, cls.name) || m.common_classes_value_unavailable()}
				{@const count = Number(metric?.active_students || 0)}
				{@const limit = Number(metric?.capacity || cls.capacity || 0)}
				{@const ratio = Number(metric?.utilization_rate || 0)}
				<a
					href={localizeHref(`/${data.orgId}/academics/classes/${cls.id}`)}
					class="group relative flex min-h-36 min-w-0 flex-col justify-between rounded-lg border border-border/70 bg-card p-3 no-underline transition-colors duration-150 hover:border-primary/40 select-none sm:p-4"
				>
					<!-- Class Header -->
					<div class="flex items-start justify-between">
						<div class="flex flex-col text-start">
							<span class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
								{displayName}
							</span>
							{#if cls.name && cls.name_ar}
								<span class="text-[9px] text-muted-foreground font-mono">{cls.name}</span>
							{/if}
						</div>
						<div class="px-1.5 py-0.5 rounded text-[9px] font-mono border border-border bg-background text-muted-foreground">
							{cls.code || m.common_classes_code_unavailable()}
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
					<div class="flex items-center justify-between text-[9px] text-muted-foreground">
						<span>{m.common_classes_attendance({ rate: Math.round(Number(metric?.attendance_rate || 0)) })}</span>
						<span>{m.common_classes_subjects_count({ count: Number(metric?.subject_count || 0) })}</span>
					</div>

					<!-- Class Footer -->
					<div class="flex items-center justify-between border-t border-border/60 pt-2 text-[9px] text-muted-foreground">
						<span>
							{m.common_classes_grade_level_label({ level: cls.level || m.common_classes_value_unavailable() })}
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
