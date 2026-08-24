<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { createQuery } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import SchedulePlanWorkspace from "$lib/features/academics/components/schedule-plan-workspace.svelte";
import { commands } from "$lib/core/ipc";
import { localizeHref } from "$lib/paraglide/runtime";
import { ArrowLeft, ArrowRight, CalendarDays, GraduationCap } from "@lucide/svelte";

let { data }: { data: { orgId: string } } = $props();

let activePeriodId = $state("");
let selectedClassId = $state("");

const periodsQuery = createQuery(() => ({
  queryKey: ["periods", data.orgId],
  queryFn: () => commands.calendar.listAcademicPeriods(),
  enabled: !!data.orgId,
  staleTime: 10 * 60 * 1000,
}));
const classesQuery = createQuery(() => ({
  queryKey: ["classes", data.orgId],
  queryFn: () => commands.academic.listClasses(),
  enabled: !!data.orgId,
  staleTime: 5 * 60 * 1000,
}));
const subjectsQuery = createQuery(() => ({
  queryKey: ["subjects", data.orgId],
  queryFn: () => commands.academic.listSubjects(),
  enabled: !!data.orgId,
  staleTime: 10 * 60 * 1000,
}));

const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);

$effect(() => {
  if (periods.length > 0 && !activePeriodId) {
    const active = periods.find((period: any) => period.status === "active") ?? periods[0];
    activePeriodId = String(active.id);
  }
});

$effect(() => {
  if (classes.length > 0 && !selectedClassId) {
    selectedClassId = String(classes[0].id);
  }
});

const isLoading = $derived(
  periodsQuery.isLoading || classesQuery.isLoading || subjectsQuery.isLoading,
);

function backToSchedule() {
  goto(localizeHref(`/${data.orgId}/academics/schedule`));
}
</script>

<div class="h-full w-full overflow-hidden bg-background" dir="auto">
  <div class="h-full w-full overflow-y-auto p-4 sm:p-6 lg:p-8">
    <div class="mx-auto flex w-full max-w-7xl flex-col gap-5">
      <header class="flex flex-col gap-4 border-b border-border/70 pb-5 sm:flex-row sm:items-start sm:justify-between">
        <div class="flex items-start gap-3 text-start">
          <Button variant="outline" size="icon" class="h-9 w-9 shrink-0 cursor-pointer" onclick={backToSchedule} aria-label={m.common_action_back()}>
            <ArrowLeft size={15} class="rtl:rotate-180" />
          </Button>
          <div>
            <div class="mb-1 flex items-center gap-2 text-primary">
              <CalendarDays size={16} />
              <span class="text-[10px] font-bold uppercase tracking-wider">{m.common_schedule_ai_auto_scheduler()}</span>
            </div>
            <h1 class="text-xl font-bold tracking-tight text-foreground sm:text-2xl">{m.common_schedule_ai_auto_scheduler()}</h1>
          </div>
        </div>
        <Button variant="ghost" class="h-9 shrink-0 cursor-pointer text-xs" onclick={backToSchedule}>
          <ArrowRight size={14} class="rtl:rotate-180" />
          {m.common_action_back()}
        </Button>
      </header>

      <section class="grid grid-cols-1 gap-3 md:grid-cols-2" aria-label={m.common_schedule_ai_auto_scheduler()}>
        <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3">
          <CalendarDays size={17} class="text-primary" />
          <div class="min-w-0 flex-1 text-start">
            <span class="block text-[10px] font-semibold uppercase tracking-wider text-muted-foreground">{m.common_word_semester()}</span>
            <PeriodSelect bind:value={activePeriodId} periods={periods} class="mt-1" disabled={isLoading} />
          </div>
        </div>
        <div class="flex items-center gap-3 rounded-lg border border-border/70 bg-card p-3">
          <GraduationCap size={17} class="text-brand-lime" />
          <div class="min-w-0 flex-1 text-start">
            <span class="block text-[10px] font-semibold uppercase tracking-wider text-muted-foreground">{m.common_schedule_page_sel_class()}</span>
            <ClassSelect bind:value={selectedClassId} classes={classes} class="mt-1" disabled={isLoading} />
          </div>
        </div>
      </section>

      {#if isLoading}
        <div class="min-h-96 animate-pulse rounded-xl border border-border bg-card/60"></div>
      {:else}
        <SchedulePlanWorkspace
          periodId={activePeriodId}
          selectedClassId={selectedClassId}
          {subjects}
          standalone={true}
          previewDraft={false}
          activeJob={null}
          onApply={() => {}}
        />
      {/if}
    </div>
  </div>
</div>
