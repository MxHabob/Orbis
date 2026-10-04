<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { createQuery, useQueryClient } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui/button";
import { Input } from "@orbis/ui/input";
import { Skeleton } from "@orbis/ui/skeleton";
import { localizeHref } from "$lib/paraglide/runtime";
import { api } from "$lib/api";
import { fade } from "svelte/transition";
import { ArrowRight, BookOpen, Calendar, Plus, Search, TaskList, Users } from "@orbis/icons";

import ExamStatusBadge from "$lib/features/exams/components/status-badge.svelte";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";
import { createExamSessionsQuery } from "$lib/features/exams/queries";

let { orgId }: { orgId: string } = $props();

let searchQuery = $state("");
let isSheetOpen = $state(false);

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
const sessionsQuery = createExamSessionsQuery(() => activePeriodId);
const sessions = $derived((sessionsQuery.data || []) as any[]);
const isLoading = $derived(sessionsQuery.isLoading);

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

<div class="h-full w-full overflow-y-auto p-6 flex flex-col gap-4 select-none">
	<!-- Header -->
	<div class="flex items-center justify-between" in:fade={{ duration: 150 }}>
		<div class="flex flex-col">
			<h1 class="text-sm font-bold text-foreground">{m.common_exams_sessions_title()}</h1>
		</div>
		<Button
			variant="default"
			class="h-8 px-3 text-xs font-semibold flex items-center gap-1.5 cursor-pointer"
			onclick={() => isSheetOpen = true}
		>
			<Plus size={14} />
			<span>{m.common_exams_sessions_create()}</span>
		</Button>
	</div>

	<!-- Search -->
	<div class="relative">
		<Search size={14} class="absolute inset-s-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
		<Input
			placeholder={m.common_exams_sessions_search()}
			class="ps-9 h-9 text-xs"
			bind:value={searchQuery}
		/>
	</div>

	<!-- Loading -->
	{#if isLoading}
		<div class="flex flex-col gap-3">
			{#each Array(3) as _}
				<Skeleton class="h-32 w-full rounded-xl" />
			{/each}
		</div>
	{:else if filtered.length === 0}
		<!-- Empty State -->
		<div class="flex-1 flex flex-col items-center justify-center gap-4 py-16 text-center" in:fade={{ duration: 200 }}>
			<div class="h-16 w-16 rounded-2xl bg-muted/50 flex items-center justify-center">
				<TaskList size={28} class="text-muted-foreground" />
			</div>
			<p class="text-xs text-muted-foreground max-w-xs">
				{m.common_exams_sessions_empty()}
			</p>
			<Button
				variant="outline"
				class="h-8 px-4 text-xs cursor-pointer"
				onclick={() => isSheetOpen = true}
			>
				<Plus size={14} class="me-1.5" />
				{m.common_exams_sessions_create()}
			</Button>
		</div>
	{:else}
		<!-- Sessions Cards -->
		<div class="flex flex-col gap-3" in:fade={{ delay: 50, duration: 200 }}>
			{#each filtered as session (session.id)}
				<div class="card-tactile bg-card p-5 flex flex-col gap-3 transition-all duration-200 hover:-translate-y-0.5 group">
					<!-- Top Row: Name + Status -->
					<div class="flex items-start justify-between">
						<div class="flex flex-col gap-1">
							<h3 class="text-xs font-bold text-foreground group-hover:text-primary transition-colors">
								{session.name_ar || session.name}
							</h3>
							{#if session.name_ar && session.name}
								<span class="text-[9px] text-muted-foreground">{session.name}</span>
							{/if}
						</div>
						<ExamStatusBadge status={session.status} />
					</div>

					<!-- Info Row -->
					<div class="flex items-center gap-4 text-[10px] text-muted-foreground">
						<div class="flex items-center gap-1">
							<Calendar size={11} />
							<span>{session.start_date} → {session.end_date}</span>
						</div>
						<div class="flex items-center gap-1">
							<span class="px-1.5 py-0.5 rounded bg-muted/60 border border-border/40 text-[9px] font-medium">
								{getTypeLabel(session.session_type)}
							</span>
						</div>
					</div>

					<!-- Bottom Row: Counts + Action -->
					<div class="flex items-center justify-between border-t border-border/50 pt-3">
						<div class="flex items-center gap-3 text-[9px] text-muted-foreground">
							<div class="flex items-center gap-1">
								<BookOpen size={10} />
								<span>{session.subjects_count || 0} {m.common_word_type()}</span>
							</div>
							<div class="flex items-center gap-1">
								<Users size={10} />
								<span>{session.students_count || 0}</span>
							</div>
						</div>
						<Button
							href={localizeHref(`/${orgId}/academics/exams/sessions/${session.id}`)}
							variant="ghost"
							class="text-[10px] font-bold text-primary h-7 px-2 flex items-center gap-1 cursor-pointer"
						>
							<span>{m.common_exams_open_section()}</span>
							<ArrowRight size={10} class="transition-transform group-hover:translate-x-1" />
						</Button>
					</div>
				</div>
			{/each}
		</div>
	{/if}
</div>

<SessionCreateSheet
	bind:open={isSheetOpen}
	{orgId}
	onSuccess={() => {
		queryClient.invalidateQueries({ queryKey: ["exam-sessions"] });
	}}
/>
