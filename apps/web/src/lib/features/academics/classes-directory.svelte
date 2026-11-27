<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { useQueryClient } from "@tanstack/svelte-query";
import {
  api,
  extractErrorMessage,
  queryKeys,
  type SectionDto,
  type SectionSummaryItemDto,
} from "$lib/api";
import { createSectionsSummaryQuery } from "./queries";
import { academicContext } from "$lib/stores";
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
import { BookOpen, Plus, Users, Refresh, Pencil, Trash } from "@orbis/icons";
import { localizeHref } from "$lib/paraglide/runtime";
import { l } from "$lib/core/i18n/locale";
import { toast } from "@orbis/ui";
import ClassCreateSheet from "$lib/features/academics/components/class-create-sheet.svelte";
import SplitActionMenu from "$lib/shared/components/split-action-menu.svelte";
import { confirmStore } from "$lib/stores";

let { orgId }: { orgId: string } = $props();
const queryClient = useQueryClient();

const summaryQuery = createSectionsSummaryQuery(
  () => orgId,
  () => academicContext.effectivePeriodId || undefined,
);

const summary = $derived(summaryQuery.data);
const sectionItems = $derived<SectionSummaryItemDto[]>(summary?.items ?? []);
const classes = $derived(sectionItems.map((item) => item.section));
const error = $derived(
  summaryQuery.error ? extractErrorMessage(summaryQuery.error) : null,
);

// Sheet trigger state
let isModalOpen = $state(false);
let editingSection = $state<SectionDto | null>(null);

async function handleDeleteSection(cls: SectionDto) {
  const displayName =
    l(cls.name_ar, cls.name) ||
    cls.code ||
    m.common_classes_value_unavailable();
  const confirmed = await confirmStore.prompt({
    title: m.common_action_delete(),
    description: m.academics_class_delete_confirm({ name: displayName }),
    confirmText: m.common_action_delete(),
    variant: "destructive",
  });
  if (!confirmed) return;

  try {
    await api.deleteSection(cls.id);
    toast.success(m.academics_class_delete_success());
    await queryClient.invalidateQueries({
      queryKey: queryKeys.academic.classes(orgId),
    });
  } catch (e) {
    toast.error(extractErrorMessage(e));
  }
}

function refetchAll() {
  summaryQuery.refetch();
}

function getSectionMetric(item: SectionSummaryItemDto): {
  active_students: number;
  capacity: number;
  utilization_rate: number;
  attendance_rate: number;
  subject_count: number;
} {
  const count = Number(item.active_students || 0);
  const capacity = Number(item.section.capacity ?? 0);
  return {
    active_students: count,
    capacity,
    utilization_rate: capacity > 0 ? Math.round((count / capacity) * 100) : 0,
    attendance_rate: 0,
    subject_count: Number(item.subject_count || 0),
  };
}

// Server-computed KPI statistics
const totalClassesCount = $derived(
  Number(summary?.total_sections ?? classes.length),
);
const totalStudentsCount = $derived(
  Number(summary?.total_enrolled_students ?? 0),
);
const averageClassSize = $derived(Number(summary?.average_section_size ?? 0));
const pageLoading = $derived(summaryQuery.isLoading);
const classMenuActions = $derived([
  {
    id: "refresh",
    label: m.operations_ui_refresh(),
    icon: Refresh,
    disabled: pageLoading,
    onSelect: refetchAll,
  },
]);
</script>

<div
	class="h-full min-h-0 w-full min-w-0 overflow-hidden p-3 @lg/workspace:p-6 flex flex-col gap-6 font-sans"
>
	<!-- KPI Metrics Summary Row using standard Cards -->
	{#if pageLoading}
		<div class="grid grid-cols-1 gap-4 @min-[36rem]/workspace:grid-cols-3" aria-busy="true">
			{#each Array(3) as _, i (i)}
				<Card class="flex flex-col justify-between p-4 gap-3">
					<Skeleton class="h-3 w-24" />
					<Skeleton class="h-7 w-16" />
					<Skeleton class="h-2.5 w-32" />
				</Card>
			{/each}
		</div>
	{:else if !error && classes.length > 0}
		<div
			class="grid grid-cols-1 gap-4 @min-[36rem]/workspace:grid-cols-3"
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
	<Card class="min-h-0 flex-1 flex flex-col overflow-hidden">
		<CardHeader class="py-3 px-4 flex flex-row items-center justify-between shrink-0">
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
				<SplitActionMenu
					label={m.common_classes_new_class()}
					menuLabel={m.operations_ui_actions()}
					icon={Plus}
					actions={classMenuActions}
					onprimary={() => {
						editingSection = null;
						isModalOpen = true;
					}}
					disabled={pageLoading}
				/>
			</CardAction>
		</CardHeader>

		<!-- Main Directory Content -->
		<CardContent class="min-h-0 flex-1 overflow-y-auto overflow-x-hidden overscroll-contain p-4 pt-2 [scrollbar-gutter:stable]">
			{#if pageLoading}
				<div
					class="grid grid-cols-1 @min-[30rem]/workspace:grid-cols-2 @min-[46rem]/workspace:grid-cols-3 @min-[64rem]/workspace:grid-cols-4 gap-4"
					aria-busy="true"
				>
					{#each Array(8) as _, i (i)}
						<Card class="flex min-h-40 flex-col justify-between p-3.5 gap-3">
							<div class="flex items-start justify-between gap-2">
								<div class="flex flex-col gap-1.5 flex-1">
									<Skeleton class="h-3.5 w-2/3" />
									<Skeleton class="h-2.5 w-1/2" />
								</div>
								<Skeleton class="h-5 w-12" />
							</div>
							<div class="flex flex-col gap-2">
								<div class="flex justify-between">
									<Skeleton class="h-2.5 w-16" />
									<Skeleton class="h-2.5 w-12" />
								</div>
								<Skeleton class="h-1.5 w-full rounded-full" />
							</div>
							<div class="flex justify-between border-t border-border/50 pt-2">
								<Skeleton class="h-2.5 w-20" />
								<Skeleton class="h-2.5 w-12" />
							</div>
						</Card>
					{/each}
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
						onclick={() => {
							editingSection = null;
							isModalOpen = true;
						}}
					>
						{m.common_classes_add_class()}
					</Button>
				</div>
			{:else}
				<!-- Classes Grid — each card is a structured Card component linking to class detail -->
				<div
					class="grid min-w-0 grid-cols-1 gap-4 @min-[30rem]/workspace:grid-cols-2 @min-[46rem]/workspace:grid-cols-3 @min-[64rem]/workspace:grid-cols-4"
				>
					{#each sectionItems as item (item.section.id)}
						{@const cls = item.section}
						{@const metric = getSectionMetric(item)}
						{@const displayName =
							l(cls.name_ar, cls.name) ||
							m.common_classes_value_unavailable()}
						{@const count = Number(metric?.active_students || 0)}
						{@const limit = Number(metric?.capacity || cls.capacity || 0)}
						{@const ratio = Number(metric?.utilization_rate || 0)}
						<div class="relative group select-none">
							<a
								href={localizeHref(
									`/${orgId}/academics/sections/${cls.id}`,
								)}
								class="no-underline block h-full"
							>
							<Card class="flex min-h-40 min-w-0 flex-col justify-between hover:border-primary/50 transition-colors h-full">
								<!-- Class Header -->
								<CardHeader class="py-2.5 px-3.5 pe-10 flex flex-row items-start justify-between gap-2">
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
										class="me-16 px-1.5 py-0.5 rounded text-[9px] font-mono border border-border bg-muted/50 text-muted-foreground shrink-0"
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
							<div class="absolute end-2 top-2 z-10 flex items-center gap-1">
								<Button
									variant="outline"
									size="icon-xs"
									class="h-7 w-7 rounded-md border-border/70 bg-background/95 shadow-sm backdrop-blur-sm opacity-100 transition-colors hover:border-primary/50 hover:text-primary"
									onclick={(event) => {
										event.preventDefault();
										event.stopPropagation();
										editingSection = cls;
										isModalOpen = true;
									}}
									title={m.common_action_edit()}
									aria-label={m.common_action_edit()}
								>
									<Pencil size={12} />
								</Button>
								<Button
									variant="outline"
									size="icon-xs"
									class="h-7 w-7 rounded-md border-border/70 bg-background/95 shadow-sm backdrop-blur-sm opacity-100 transition-colors hover:border-destructive/50 hover:text-destructive"
									onclick={(event) => {
										event.preventDefault();
										event.stopPropagation();
										void handleDeleteSection(cls);
									}}
									title={m.common_action_delete()}
									aria-label={m.common_action_delete()}
								>
									<Trash size={12} />
								</Button>
							</div>
						</div>
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
	section={editingSection}
	onSuccess={() =>
		queryClient.invalidateQueries({
			queryKey: queryKeys.academic.classes(orgId),
		})}
/>
