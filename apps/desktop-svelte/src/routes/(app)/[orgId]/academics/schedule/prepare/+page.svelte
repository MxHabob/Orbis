<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { goto } from "$app/navigation";
import { createQuery } from "@tanstack/svelte-query";
import { Button } from "@orbis/ui-svelte/button";
import PeriodSelect from "$lib/features/academics/components/period-select.svelte";
import ClassSelect from "$lib/features/academics/components/class-select.svelte";
import SchedulePlanWorkspace from "$lib/features/academics/components/schedule-plan-workspace.svelte";
import ScheduleBoard from "$lib/features/academics/components/schedule-board.svelte";
import Resizer from "$lib/shell/resizer.svelte";
import { commands } from "$lib/core/ipc";
import { getLocale, localizeHref } from "$lib/paraglide/runtime";
import {
  ArrowLeft,
} from "@lucide/svelte";

let { data }: { data: { orgId: string } } = $props();

let activePeriodId = $state("");
let selectedClassId = $state("");
let sidebarEl = $state<HTMLElement | undefined>(undefined);
let sidebarWidth = $state(18);
const sidebarMinWidth = 16;
const sidebarMaxWidth = 26;

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
const roomsQuery = createQuery(() => ({
  queryKey: ["rooms", data.orgId],
  queryFn: () => commands.scheduling.listRooms(),
  enabled: !!data.orgId,
  staleTime: 10 * 60 * 1000,
}));

const timeSlotsQuery = createQuery(() => ({
  queryKey: ["time-slots", data.orgId, activePeriodId],
  queryFn: () => commands.scheduling.listTimeSlots(),
  enabled: !!data.orgId && !!activePeriodId,
  staleTime: 10 * 60 * 1000,
}));

const lessonsQuery = createQuery(() => ({
  queryKey: ["prepare-schedule", data.orgId, activePeriodId, selectedClassId],
  queryFn: () => commands.scheduling.listLessonsByClass(activePeriodId, selectedClassId),
  enabled: !!data.orgId && !!activePeriodId && !!selectedClassId,
  staleTime: 2 * 60 * 1000,
}));

const periods = $derived(periodsQuery.data ?? []);
const classes = $derived(classesQuery.data ?? []);
const subjects = $derived(subjectsQuery.data ?? []);
const rooms = $derived(roomsQuery.data ?? []);
const timeSlots = $derived(
  (timeSlotsQuery.data ?? []).filter(
    (slot: any) => String(slot.period_id) === String(activePeriodId),
  ),
);
const lessons = $derived(lessonsQuery.data ?? []);
const weekdays = $derived.by(() => {
  const configuredDays = new Set<number>();
  for (const slot of timeSlots) {
    for (const value of String(slot.applicable_days ?? "").split(",")) {
      const day = Number(value.trim());
      if (Number.isInteger(day) && day >= 1 && day <= 7) configuredDays.add(day);
    }
  }
  for (const lesson of lessons) {
    const day = Number(lesson.day_of_week);
    if (Number.isInteger(day) && day >= 1 && day <= 7) configuredDays.add(day);
  }
  const days = configuredDays.size > 0 ? [...configuredDays] : [1, 2, 3, 4, 5];
  const weekOrder = [7, 1, 2, 3, 4, 5, 6];
  return days.sort((a, b) => weekOrder.indexOf(a) - weekOrder.indexOf(b)).map((day) => ({
    value: day,
    label: new Intl.DateTimeFormat(getLocale(), { weekday: "short" }).format(new Date(Date.UTC(2023, 0, day))),
  }));
});

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
  periodsQuery.isLoading || classesQuery.isLoading || subjectsQuery.isLoading || roomsQuery.isLoading,
);

const isBoardLoading = $derived(isLoading || timeSlotsQuery.isLoading || lessonsQuery.isLoading);

function getSubjectColor(subjectId: string) {
  const subject = subjects.find((item: any) => String(item.id) === String(subjectId));
  const savedColor = subject?.color || subject?.color_hex || subject?.accent_color;
  if (savedColor) return savedColor;

  const hash = String(subjectId)
    .split("")
    .reduce((total, character) => (total * 31 + character.charCodeAt(0)) >>> 0, 0);
  return `hsl(${hash % 360} 65% 52%)`;
}

function backToSchedule() {
  goto(localizeHref(`/${data.orgId}/academics/schedule`));
}
</script>

<div
  class="h-full w-full overflow-hidden"
>
  <div class="flex h-full min-h-0 flex-col overflow-auto scrollbar-hide lg:flex-row">
    <aside
      bind:this={sidebarEl}
      class="planner-sidebar relative w-full shrink-0 border-b border-border bg-muted/10 p-3 text-start sm:p-4 lg:border-b-0 lg:border-e lg:p-4 lg:overflow-y-auto scrollbar-hide"
      style={`--planner-sidebar-width: ${sidebarWidth}rem;`}
    >
      <Resizer
        viewport={sidebarEl}
        direction="right"
        minWidth={sidebarMinWidth}
        maxWidth={sidebarMaxWidth}
        defaultValue={18}
        collapseThreshold={sidebarMinWidth}
        collapsedWidth={sidebarMinWidth}
        persistId="schedule-prepare-sidebar-width"
        onWidth={(width) => (sidebarWidth = width)}
      />
      <div class="mb-4 flex items-center gap-2">
        <Button variant="ghost" size="icon" class="h-8 w-8" onclick={backToSchedule} aria-label={m.common_action_back()}>
          <ArrowLeft size={15} class="rtl:rotate-180" />
        </Button>
        <div class="text-xs font-bold">
          {m.academics_schedule_prepare_title()}
        </div>
      </div>
      <div class="mb-4 space-y-3">
        <div><p class="mb-1 text-[10px] font-semibold text-muted-foreground">{m.common_word_semester()}</p><PeriodSelect bind:value={activePeriodId} periods={periods} disabled={isLoading} /></div>
        <div><p class="mb-1 text-[10px] font-semibold text-muted-foreground">{m.common_schedule_page_sel_class()}</p><ClassSelect bind:value={selectedClassId} classes={classes} disabled={isLoading} /></div>
        <div class="flex items-center justify-between border-t border-border pt-3 text-[10px] text-muted-foreground">
          <span title={m.academics_schedule_prepare_lessons_count({ count: lessons.length })}>
            {m.academics_schedule_prepare_lessons_count({ count: lessons.length })}
          </span>
          <span title={m.academics_schedule_prepare_periods_count({ count: timeSlots.length })}>
            {m.academics_schedule_prepare_periods_count({ count: timeSlots.length })}
          </span>
        </div>
      </div>
      <div class="border-t border-border pt-4">
        <SchedulePlanWorkspace periodId={activePeriodId} selectedClassId={selectedClassId} {subjects} standalone={true} embedded={true} previewDraft={false} activeJob={null} onApply={() => lessonsQuery.refetch()} />
      </div>
    </aside>

    <main class="flex min-h-130 min-w-0 flex-1 flex-col overflow-hidden bg-background p-3 sm:p-4">
      <ScheduleBoard fitViewport={true} isLoading={isBoardLoading} error={lessonsQuery.error ? String(lessonsQuery.error) : null} {timeSlots} {weekdays} {lessons} draftLessons={[]} previewDraft={false} viewMode="class" getSubjectName={(id) => subjects.find((s: any) => String(s.id) === String(id))?.name_ar || subjects.find((s: any) => String(s.id) === String(id))?.name || m.common_schedule_page_unknown_subject()} getTeacherName={() => ""} getClassName={() => ""} getRoomName={(id) => rooms.find((room: any) => String(room.id) === String(id))?.name_ar || rooms.find((room: any) => String(room.id) === String(id))?.name || ""} {getSubjectColor} onCellClick={() => {}} onDeleteLesson={() => {}} onMoveLesson={() => {}} onRetry={() => lessonsQuery.refetch()} />
    </main>
  </div>
</div>

<style>
  .planner-sidebar {
    width: 100%;
  }

  @media (min-width: 1024px) {
    .planner-sidebar {
      width: var(--planner-sidebar-width);
    }
  }
</style>
