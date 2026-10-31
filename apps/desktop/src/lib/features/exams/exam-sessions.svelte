<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card } from "@orbis/ui/card";
import { formatDate } from "$lib/core/i18n";
import { localizeHref } from "$lib/paraglide/runtime";
import { api, extractErrorMessage, queryKeys } from "$lib/api";
import { fade } from "svelte/transition";
import { authStore, academicContext, uiStore } from "$lib/stores";

import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";
import { createExamSessionsQuery } from "$lib/features/exams/queries";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";

let { orgId }: { orgId: string } = $props();

let searchQuery = $state("");
let isSheetOpen = $state(false);
let selectedPeriodId = $state("");

const queryClient = useQueryClient();
const periodsQuery = createQuery(() => ({
  queryKey: ["exam-session-periods", orgId],
  queryFn: () => api.listAcademicPeriods(),
  enabled: !!orgId,
  staleTime: 5 * 60 * 1000,
}));
const periods = $derived((periodsQuery.data || []) as any[]);
const activePeriodId = $derived.by(() => {
  const active = periods.find(
    (period) => period.status === "active" || period.is_active,
  );
  return String(active?.id || periods[0]?.id || "");
});
const listPeriodId = $derived(selectedPeriodId || activePeriodId);
const sessionsQuery = createExamSessionsQuery(() => listPeriodId);
const sessions = $derived((sessionsQuery.data || []) as any[]);
const isLoading = $derived(sessionsQuery.isLoading);
const preferenceScope = () => ({
  userId: authStore.userId,
  organizationId: orgId,
});

$effect(() => {
  if (periodsQuery.isLoading || periods.length === 0) return;

  const isAvailable = (periodId: string) =>
    periods.some((period) => String(period.id) === periodId);
  const rememberedPeriodId = uiStore.getPreference(
    preferenceScope(),
    "exams.selectedPeriodId",
    "",
    (value): value is string => typeof value === "string",
  );
  const nextPeriodId = isAvailable(selectedPeriodId)
    ? selectedPeriodId
    : isAvailable(rememberedPeriodId)
      ? rememberedPeriodId
      : isAvailable(academicContext.effectivePeriodId)
        ? academicContext.effectivePeriodId
        : activePeriodId;

  if (nextPeriodId !== selectedPeriodId) selectedPeriodId = nextPeriodId;
  if (nextPeriodId) {
    academicContext.selectedPeriodId = nextPeriodId;
    uiStore.setPreference(
      preferenceScope(),
      "exams.selectedPeriodId",
      nextPeriodId,
    );
  }
});

const filtered = $derived(
  sessions.filter((s: any) => {
    const q = searchQuery.toLowerCase();
    return (
      (s.name || "").toLowerCase().includes(q) ||
      (s.name_ar || "").toLowerCase().includes(q) ||
      (s.session_type || "").toLowerCase().includes(q)
    );
  }),
);

function getTypeLabel(type: string): string {
  const map: Record<string, string> = {
    final: m.operations_ui_exam_type_final(),
    midterm: m.operations_ui_exam_type_midterm(),
    quiz: m.operations_ui_exam_type_quiz(),
    practical: m.operations_ui_exam_type_practical(),
  };
  return map[type] || type;
}
</script>

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 select-none text-start">
	<div class="flex flex-col gap-2 sm:flex-row sm:items-center" in:fade={{ duration: 150 }}>
		<div class="w-full shrink-0 sm:w-64">
			<PeriodSelect
				value={listPeriodId || null}
				placeholder={m.operations_ui_academic_session()}
				disabled={periodsQuery.isLoading}
				onchange={(periodId) => selectedPeriodId = periodId || ""}
			/>
		</div>
		<div class="relative min-w-0 flex-1">
			<Input
				placeholder={m.common_exams_sessions_search()}
				class="h-9 text-xs"
				bind:value={searchQuery}
			/>
		</div>
		<Button
			variant="lime"
			class="h-9 shrink-0 px-4 text-xs font-semibold"
			onclick={() => isSheetOpen = true}
		>
			{m.common_exams_sessions_create()}
		</Button>
	</div>

	<!-- Loading -->
	{#if isLoading}
		<div class="flex flex-col gap-3">
			{#each Array(3) as _, index (index)}
				<Skeleton class="h-32 w-full rounded-xl" />
			{/each}
		</div>
	{:else if sessionsQuery.error}
		<div class="flex flex-1 flex-col items-center justify-center gap-3 py-16 text-center text-destructive">
			<p class="text-xs font-semibold">{m.common_load_error()}</p>
			<p class="text-[10px] text-muted-foreground">
				{extractErrorMessage(sessionsQuery.error)}
			</p>
			<Button
				variant="outline"
				class="h-9 px-4 text-xs"
				onclick={() => sessionsQuery.refetch()}
			>
				{m.common_retry()}
			</Button>
		</div>
	{:else if filtered.length === 0}
		<!-- Empty State -->
		<Card class="flex flex-1 flex-col items-center justify-center gap-4 rounded-xl border-border/60 bg-card px-6 py-16 text-center">
			<p class="max-w-xs text-sm text-muted-foreground">
				{m.common_exams_sessions_empty()}
			</p>
			<Button
				variant="outline"
				class="h-9 px-4 text-xs"
				onclick={() => isSheetOpen = true}
			>
				{m.common_exams_sessions_create()}
			</Button>
		</Card>
	{:else}
		<!-- Sessions Cards -->
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3" in:fade={{ delay: 50, duration: 200 }}>
			{#each filtered as session (session.id)}
				<Card class="group gap-4 rounded-xl border-border/60 bg-card p-5 shadow-xs transition-colors hover:border-primary/30">
					<!-- Top Row: Name + Status -->
					<div class="flex items-start justify-between">
						<div class="flex flex-col gap-1">
							<span class="text-sm font-semibold text-foreground transition-colors group-hover:text-primary">
								{session.name_ar || session.name}
							</span>
						</div>
						<ExamStatusBadge status={session.status} />
					</div>

					<!-- Info Row -->
					<div class="flex flex-wrap items-center gap-x-4 gap-y-2 text-xs text-muted-foreground">
						<span>
							{formatDate(session.start_date, "date") || session.start_date}
							–
							{formatDate(session.end_date, "date") || session.end_date}
						</span>
						<div class="flex items-center gap-1">
							<span class="rounded-md border border-border/60 bg-muted/40 px-2 py-1 text-[10px] font-medium text-foreground">
								{getTypeLabel(session.session_type)}
							</span>
						</div>
					</div>

					<!-- Bottom Row: Counts + Action -->
					<div class="flex items-center justify-between border-t border-border/50 pt-3">
						<div class="flex items-center gap-3 text-xs text-muted-foreground">
							<span>{session.subjects_count || 0} {m.common_word_type()}</span>
							<span>{session.students_count || 0}</span>
						</div>
						<Button
							href={localizeHref(`/${orgId}/academics/exams/sessions/${session.id}`)}
							variant="ghost"
							class="h-8 px-2 text-xs font-semibold text-primary"
						>
							{m.common_exams_open_section()}
						</Button>
					</div>
				</Card>
			{/each}
		</div>
	{/if}
</div>

<SessionCreateSheet
	bind:open={isSheetOpen}
	{orgId}
	onSuccess={(periodId) => {
		if (periodId) selectedPeriodId = periodId;
		queryClient.invalidateQueries({ queryKey: queryKeys.exam.all });
	}}
/>
