<script lang="ts">
import { LoadingIndicator } from "@orbis/ui";
import * as m from "$lib/paraglide/messages";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { api, extractErrorMessage, type ClassDto, type AcademicPeriodDto, type EnrollmentDto } from "$lib/api";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
  CardFooter,
} from "@orbis/ui/card";
import { BookOpen, Plus, Users, Refresh } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n/locale";
import { fade } from "svelte/transition";
import ClassCreateSheet from "$lib/features/academics/components/class-create-sheet.svelte";

let { orgId }: { orgId: string } = $props();
const queryClient = useQueryClient();

let userSelectedPeriodId = $state<string | null>(null);
const activePeriodId = $derived(
  userSelectedPeriodId ||
    String(
      periods.find((period) => period.status === "active")?.id ||
        periods[0]?.id ||
        "",
    ),
);

const classesQuery = createQuery(() => ({
  queryKey: ["classes", orgId],
  queryFn: () => api.listClasses(),
  enabled: !!orgId,
  staleTime: 3 * 60 * 1000,
}));

const periodsQuery = createQuery(() => ({
  queryKey: ["classes-periods", orgId],
  queryFn: () => api.listAcademicPeriods(),
  enabled: !!orgId,
  staleTime: 10 * 60 * 1000,
}));

const periods = $derived(
  (periodsQuery.data as AcademicPeriodDto[] | undefined) ?? [],
);

const enrollmentsQuery = createQuery(() => ({
  queryKey: ["classes-enrollments", orgId, activePeriodId],
  queryFn: () => api.listEnrollments({ periodId: activePeriodId || undefined }),
  enabled: !!orgId,
  staleTime: 3 * 60 * 1000,
}));

const classes = $derived((classesQuery.data as ClassDto[] | undefined) ?? []);
const enrollments = $derived(
  (enrollmentsQuery.data as EnrollmentDto[] | undefined) ?? [],
);
const error = $derived(
  classesQuery.error ? extractErrorMessage(classesQuery.error) : null,
);

// Sheet trigger state
let isModalOpen = $state(false);

function refetchAll() {
  classesQuery.refetch();
  periodsQuery.refetch();
  enrollmentsQuery.refetch();
}

function getClassMetric(classId: string): { active_students: number } {
  const count = enrollments.filter(
    (e) => String(e.class_id) === String(classId),
  ).length;
  return { active_students: count };
}

// Compute statistics
const totalClassesCount = $derived(classes.length);
const totalStudentsCount = $derived(enrollments.length);
const averageClassSize = $derived(
  totalClassesCount > 0
    ? Math.round(totalStudentsCount / totalClassesCount)
    : 0,
);
const pageLoading = $derived(
  classesQuery.isLoading || periodsQuery.isLoading || enrollmentsQuery.isLoading,
);
</script>

<div
	class="h-full w-full min-w-0 overflow-y-auto overflow-x-hidden scrollbar-hide p-4 sm:p-6 flex flex-col gap-6 font-sans"
>
	<!-- KPI Metrics Summary Row using standard Cards -->
	{#if !pageLoading && !error && classes.length > 0}
		<div
			class="grid grid-cols-1 gap-4 sm:grid-cols-3"
			in:fade={{ duration: 200 }}
		>
			<Card class="flex flex-col justify-between">
				<CardHeader class="py-3 px-4">
					<CardTitle class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">
						{m.common_classes_total_classes()}
					</CardTitle>
				</CardHeader>
				<CardContent class="p-4 pt-1">
					<div class="text-2xl font-black font-mono text-foreground">
						{totalClassesCount}
					</div>
					<p class="text-[10px] text-muted-foreground mt-1">
						{m.shared_ui_registered_classes()}
					</p>
				</CardContent>
			</Card>

			<Card class="flex flex-col justify-between">
				<CardHeader class="py-3 px-4">
					<CardTitle class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">
						{m.common_classes_enrolled_students()}
					</CardTitle>
				</CardHeader>
				<CardContent class="p-4 pt-1">
					<div class="text-2xl font-black font-mono text-emerald-600 dark:text-emerald-400">
						{totalStudentsCount}
					</div>
					<p class="text-[10px] text-muted-foreground mt-1">
						{m.shared_ui_enrolled_students_total()}
					</p>
				</CardContent>
			</Card>

			<Card class="flex flex-col justify-between">
				<CardHeader class="py-3 px-4">
					<CardTitle class="text-[10px] font-bold uppercase tracking-wider text-muted-foreground">
						{m.common_classes_avg_size()}
					</CardTitle>
				</CardHeader>
				<CardContent class="p-4 pt-1">
					<div class="text-2xl font-black font-mono text-amber-600 dark:text-amber-400">
						{averageClassSize}
					</div>
					<p class="text-[10px] text-muted-foreground mt-1">
						{m.shared_ui_average_class_size()}
					</p>
				</CardContent>
			</Card>
		</div>
	{/if}

	<!-- Header Area Card -->
	<Card>
		<CardHeader class="py-3 px-4 flex flex-row items-center justify-between">
			<div class="flex items-center gap-2.5">
				<div class="w-8 h-8 rounded-lg bg-primary/10 flex items-center justify-center text-primary shrink-0">
					<BookOpen class="w-4 h-4" />
				</div>
				<div class="flex flex-col text-start">
					<CardTitle class="text-xs font-bold uppercase tracking-wider text-foreground">
						{m.common_card_active_classes()}
					</CardTitle>
					<CardDescription class="text-[10px]">
						{m.shared_ui_classes_directory_description()}
					</CardDescription>
				</div>
			</div>

			<CardAction>
				<div class="flex items-center gap-2">
					<Button
						variant="ghost"
						size="icon-sm"
						onclick={refetchAll}
						class="text-muted-foreground"
					>
						{#if pageLoading}<LoadingIndicator  size={14} />{:else}<Refresh size={14}   />{/if}
					</Button>

					<Button
						variant="default"
						size="sm"
						class="h-8 px-3 text-xs font-bold gap-1.5 shadow-sm"
						onclick={() => (isModalOpen = true)}
					>
						<Plus size={14} />
						{m.common_classes_new_class()}
					</Button>
				</div>
			</CardAction>
		</CardHeader>

		<!-- Main Directory Content -->
		<CardContent class="p-4 pt-2">
			{#if pageLoading}
				<div
					class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4"
				>
					<Skeleton class="h-40 w-full rounded-xl" />
					<Skeleton class="h-40 w-full rounded-xl" />
					<Skeleton class="h-40 w-full rounded-xl" />
					<Skeleton class="h-40 w-full rounded-xl" />
				</div>
			{:else if error}
				<div
					class="flex flex-col items-center justify-center p-8 border border-destructive/20 bg-destructive/5 rounded-xl text-center gap-3"
				>
					<p class="text-xs text-destructive font-semibold">
						{m.common_classes_load_error()}
					</p>
					<p class="text-xs text-muted-foreground">{error}</p>
					<Button variant="outline" class="text-xs" onclick={refetchAll}>
						{m.common_retry()}
					</Button>
				</div>
			{:else if classes.length === 0}
				<div
					class="flex flex-col items-center justify-center py-16 text-muted-foreground border border-border border-dashed rounded-xl gap-4"
				>
					<BookOpen size={48} class="text-muted-foreground/30" />
					<div class="flex flex-col gap-1 text-center">
						<p class="text-xs font-semibold">
							{m.common_classes_empty_title()}
						</p>
						<p class="text-[10px] text-muted-foreground/80">
							{m.common_classes_empty_desc()}
						</p>
					</div>
					<Button
						variant="default"
						class="text-xs mt-2"
						onclick={() => (isModalOpen = true)}
					>
						{m.common_classes_add_class()}
					</Button>
				</div>
			{:else}
				<!-- Classes Grid — each card is a structured Card component linking to class detail -->
				<div
					class="grid min-w-0 grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
					in:fade={{ duration: 250 }}
				>
					{#each classes as cls (cls.id)}
						{@const metric = getClassMetric(String(cls.id))}
						{@const displayName =
							l(cls.name_ar, cls.name) ||
							m.common_classes_value_unavailable()}
						{@const count = Number(metric?.active_students || 0)}
						{@const limit = Number(metric?.capacity || cls.capacity || 0)}
						{@const ratio = Number(metric?.utilization_rate || 0)}
						<a
							href={localizeHref(
								`/${orgId}/academics/classes/${cls.id}`,
							)}
							class="no-underline group select-none block"
						>
							<Card class="flex min-h-40 min-w-0 flex-col justify-between hover:border-primary/50 transition-colors h-full">
								<!-- Class Header -->
								<CardHeader class="py-2.5 px-3.5 flex flex-row items-start justify-between">
									<div class="flex flex-col text-start">
										<CardTitle class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
											{displayName}
										</CardTitle>
										{#if cls.name && cls.name_ar}
											<CardDescription class="text-[9px] font-mono">
												{cls.name}
											</CardDescription>
										{/if}
									</div>
									<div
										class="px-1.5 py-0.5 rounded text-[9px] font-mono border border-border bg-muted/50 text-muted-foreground shrink-0"
									>
										{cls.code || m.common_classes_code_unavailable()}
									</div>
								</CardHeader>

								<!-- Class Capacity Bar & Metrics -->
								<CardContent class="p-3.5 py-2 flex flex-col gap-2">
									<div class="flex flex-col gap-1.5">
										<div
											class="flex items-center justify-between text-[10px]"
										>
											<span
												class="text-muted-foreground flex items-center gap-1"
											>
												<Users size={11} />
												<span>
													{m.common_insight_students_link({ count })}
												</span>
											</span>
											<span
												class="font-mono {ratio >= 100
													? 'text-destructive font-bold'
													: 'text-muted-foreground'}"
											>
												{count}/{limit} ({ratio}%)
											</span>
										</div>
										<div
											class="h-1.5 w-full bg-muted rounded-full overflow-hidden"
										>
											<div
												class="h-full rounded-full transition-all duration-300 {ratio >=
												100
													? 'bg-destructive'
													: ratio >= 80
														? 'bg-amber-500'
														: 'bg-primary'}"
												style="width: {ratio}%"
											></div>
										</div>
									</div>
									<div
										class="flex items-center justify-between text-[9px] text-muted-foreground"
									>
										<span>
											{m.common_classes_attendance({
												rate: Math.round(
													Number(metric?.attendance_rate || 0),
												),
											})}
										</span>
										<span>
											{m.common_classes_subjects_count({
												count: Number(metric?.subject_count || 0),
											})}
										</span>
									</div>
								</CardContent>

								<!-- Class Footer -->
								<CardFooter
									class="flex items-center justify-between border-t border-border/60 py-2 px-3.5 text-[9px] text-muted-foreground"
								>
									<span>
										{m.common_classes_grade_level_label({
											level:
												cls.level ||
												m.common_classes_value_unavailable(),
										})}
									</span>
									{#if ratio >= 100}
										<span
											class="text-destructive font-bold uppercase text-[8px] tracking-wider px-1 py-0.2 rounded border border-destructive/20 bg-destructive/5"
										>
											{m.common_classes_status_full()}
										</span>
									{/if}
								</CardFooter>
							</Card>
						</a>
					{/each}
				</div>
			{/if}
		</CardContent>
	</Card>
</div>

<!-- Class Creation Side Sheet Component -->
<ClassCreateSheet
	bind:open={isModalOpen}
	{orgId}
	onSuccess={() => queryClient.invalidateQueries({ queryKey: ["classes"] })}
/>
