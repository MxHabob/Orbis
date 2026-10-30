<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Skeleton } from "@orbis/ui/skeleton";
import { Card } from "@orbis/ui/card";
import { localizeHref } from "$lib/paraglide/runtime";
import { createQuery } from "@tanstack/svelte-query";
import { api, extractErrorMessage } from "$lib/api";
import { createPeriodsQuery } from "$lib/features/academics";
import SessionCreateSheet from "$lib/features/exams/components/session-create-sheet.svelte";

let { orgId }: { orgId: string } = $props();

// Modal trigger state
let isCreateSheetOpen = $state(false);

// Queries
const periodsQuery = createPeriodsQuery(() => orgId);
const periods = $derived(periodsQuery.data ?? []);

// Active academic year & period
const activePeriod = $derived(
  periods.find(
    (p) =>
      p.status === "active" ||
      p.status === "active_teaching" ||
      p.status === "grade_submission",
  ) || periods[0],
);
const activePeriodId = $derived(activePeriod ? String(activePeriod.id) : "");
// Grade Entry Status
const isGradeEntryOpen = $derived(
  activePeriod
    ? activePeriod.status === "grade_submission" ||
        activePeriod.status === "active_teaching" ||
        activePeriod.status === "open"
    : false,
);

// Exam Sessions Query
const examSessionsQuery = createQuery(() => ({
  queryKey: ["exam-sessions", orgId, activePeriodId],
  queryFn: () => api.listExamSessions(activePeriodId),
  enabled: !!orgId && !!activePeriodId,
  staleTime: 2 * 60 * 1000,
}));

const examSessions = $derived(examSessionsQuery.data ?? []);

const isLoading = $derived(
  periodsQuery.isLoading || examSessionsQuery.isLoading,
);

const error = $derived(periodsQuery.error || examSessionsQuery.error);

function refetchAll() {
  periodsQuery.refetch();
  examSessionsQuery.refetch();
}

const directoryItems = $derived([
  {
    path: `/${orgId}/academics/exams/sessions`,
    title: m.common_exams_sessions_title(),
    detail: String(examSessions.length),
  },
  {
    path: `/${orgId}/academics/exams/control`,
    title: m.academics_exams_center_control_title(),
    detail: isGradeEntryOpen
      ? m.academics_exams_center_open()
      : m.academics_exams_center_locked(),
  },
  {
    path: `/${orgId}/academics/exams/sessions`,
    title: m.academics_exams_center_halls_title(),
    detail: m.academics_exams_center_halls_badge(),
  },
  {
    path: `/${orgId}/academics/exams/reports`,
    title: m.academics_exams_center_reports_title(),
    detail: m.academics_exams_center_reports_badge(),
  },
]);

</script>

<div class="h-full w-full overflow-y-auto bg-background p-6 select-none font-sans text-start">
  <div class="mb-5 flex justify-end">
    <Button
      variant="lime"
      class="h-9 px-4 text-xs font-semibold"
      onclick={() => isCreateSheetOpen = true}
    >
      {m.academics_exams_center_add_session()}
    </Button>
  </div>

  {#if isLoading}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {#each Array(4) as _}
        <Skeleton class="h-32 w-full rounded-xl" />
      {/each}
    </div>
  {:else if error}
    <div class="flex flex-col items-center gap-3 rounded-xl border border-border/60 bg-card p-8 text-center">
      <p class="text-sm text-destructive">{extractErrorMessage(error)}</p>
      <Button variant="outline" onclick={refetchAll} class="h-9 px-4 text-xs">
        {m.academics_exams_center_retry()}
      </Button>
    </div>
  {:else}
    <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
      {#each directoryItems as item (item.path + item.title)}
        <Card class="shadow-xs transition-colors hover:border-primary/40">
          <a
            href={localizeHref(item.path)}
            class="flex min-h-32 flex-col justify-between p-5"
          >
            <span class="text-sm font-semibold text-foreground">{item.title}</span>
            <span class="text-xs text-muted-foreground">{item.detail}</span>
          </a>
        </Card>
      {/each}
    </div>
  {/if}
</div>

<!-- Exam Session Creation Side Sheet -->
<SessionCreateSheet
  bind:open={isCreateSheetOpen}
  {orgId}
  onSuccess={refetchAll}
/>
