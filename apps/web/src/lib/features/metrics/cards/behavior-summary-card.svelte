<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { createQuery } from "@tanstack/svelte-query";
import { createPeriodsQuery } from "$lib/features/academics/queries";
import { api } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { ArrowRight } from "@orbis/icons";

let {
  orgId,
  selectedClass = "all",
  selectedTerm = "all",
  period = "this-year",
  isEditing = false,
}: {
  orgId: string;
  selectedClass?: string;
  selectedTerm?: string;
  period?: string;
  isEditing?: boolean;
} = $props();

const recordsQuery = createQuery(() => ({
  queryKey: ["behavior-summary-records", orgId],
  queryFn: () => api.listAllBehavior(),
  enabled: !!orgId,
}));
const periodsQuery = createPeriodsQuery(() => orgId);
const enrollmentQuery = createQuery(() => {
  const classId = selectedClass;
  const termId = selectedTerm;
  return {
    queryKey: ["behavior-summary-enrollment", orgId, classId, termId],
    enabled: !!orgId && (classId !== "all" || termId !== "all"),
    queryFn: async () => {
      const students = new Set<string>();
      const enrollmentIds = new Set<string>();
      const pageSize = 100;
      for (let page = 1; ; page++) {
        const batch = await api.listEnrollments({
          class_id: classId === "all" ? undefined : classId,
          period_id: termId === "all" ? undefined : termId,
          page,
          page_size: pageSize,
        });
        const previousSize = enrollmentIds.size;
        for (const enrollment of batch) {
          enrollmentIds.add(enrollment.id);
          students.add(enrollment.student_id);
        }
        // Local clients may return the full list without applying pagination.
        if (batch.length < pageSize || enrollmentIds.size === previousSize) return students;
      }
    },
  };
});

const isLoading = $derived(recordsQuery.isLoading || periodsQuery.isLoading || enrollmentQuery.isLoading);
const failed = $derived(recordsQuery.isError || periodsQuery.isError || enrollmentQuery.isError);
let hoveredIndex = $state<number | null>(null);

const summary = $derived.by(() => {
  if (failed) return null;
  const selectedPeriod = periodsQuery.data?.find((item) => item.id === selectedTerm);
  if (selectedTerm !== "all" && !selectedPeriod) return null;
  const cutoff = new Date();
  if (period === "3-months" || period === "6-months") {
    cutoff.setMonth(cutoff.getMonth() - (period === "3-months" ? 3 : 6));
  } else {
    cutoff.setMonth(0, 1);
  }
  cutoff.setHours(0, 0, 0, 0);
  const lowerBound = period === "all" ? -Infinity : cutoff.getTime();
  const termStart = selectedPeriod ? Date.parse(selectedPeriod.start_date) : -Infinity;
  const termEnd = selectedPeriod ? new Date(selectedPeriod.end_date).setHours(23, 59, 59, 999) : Infinity;
  const scoped = selectedClass !== "all" || selectedTerm !== "all";
  const records = (recordsQuery.data ?? []).filter((record) => {
    const date = Date.parse(record.created_at);
    return (!scoped || enrollmentQuery.data?.has(record.student_id)) &&
      date >= Math.max(lowerBound, termStart) && date <= termEnd;
  });
  const positive = records.filter((record) => record.record_type === "positive").length;
  const negative = records.filter((record) => record.record_type === "negative").length;
  return records.length ? { total_records: records.length, positive, negative, other: records.length - positive - negative } : null;
});

const displayItems = $derived([
  { name: "positive", label: m.academics_behavior_type_positive(), value: summary?.positive ?? 0, color: "bg-primary" },
  { name: "negative", label: m.academics_behavior_type_negative(), value: summary?.negative ?? 0, color: "bg-destructive" },
  { name: "other", label: m.common_rooms_type_other(), value: summary?.other ?? 0, color: "bg-muted-foreground" },
].map((item) => ({ ...item, percent: Math.round(item.value * 100 / (summary?.total_records || 1)) })));
</script>

<div class="card-tactile h-full relative group min-h-[235px] p-4 flex flex-col justify-between font-sans shadow-sm">
  {#if isLoading}
    <div class="grow w-full relative flex items-center justify-center">
      <Skeleton class="h-full w-full rounded-lg animate-pulse" />
    </div>
  {:else}
    <!-- Top info bar with DragIndicator if editing, and small Releases-style label on the right -->
    <div class="flex items-center justify-between w-full h-5 select-none text-[9px] font-bold text-muted-foreground/80">
      <div class="flex items-center gap-2">
        {#if isEditing}
          <DragIndicator />
        {/if}
      </div>
      <div class="flex items-center gap-1.5 font-sans font-extrabold uppercase tracking-wider text-[8px] text-muted-foreground/75">
        <span class="h-1.5 w-1.5 rounded-full bg-rose-500 {summary ? 'animate-pulse' : ''}"></span>
        {m.academics_behavior_records_title()} {#if summary}• Total: <span class="text-foreground font-mono">{summary.total_records}</span>{/if}
      </div>
    </div>

    <!-- Main Content Area: Sentry-style horizontal ratios listing or Blurred Empty State -->
    <div class="flex flex-col gap-4 mt-2 grow justify-center w-full px-2 min-h-[140px] relative">
      {#each displayItems as item, idx}
        <div
          role="presentation"
          class="flex items-center gap-4 text-[10px] w-full group/row cursor-pointer transition-opacity duration-200 hover:bg-foreground/5 p-1 rounded-md"
          style="opacity: {hoveredIndex === null || hoveredIndex === idx ? 1 : 0.45}"
          onmouseenter={() => { if (summary) hoveredIndex = idx; }}
          onmouseleave={() => { hoveredIndex = null; }}
        >
          <!-- Category Label -->
          <span class="w-[60px] font-bold text-muted-foreground text-start lowercase tracking-tight select-none">
            {item.label}
          </span>

          <!-- Progress Bar Bar (Tactile Sunken Track) -->
          <div class="grow h-2.5 bg-muted/40 border border-border/40 rounded-md overflow-hidden relative select-none shadow-[inset_0_1px_2.5px_rgba(0,0,0,0.08)] dark:shadow-[inset_0_1px_3px_rgba(0,0,0,0.35)]">
            <!-- Progress Fill (Rendered ONLY when summary is loaded) -->
            {#if summary}
              <div
                class="h-full rounded-md transition-all duration-500 ease-out {item.color} shadow-[0_1.5px_0_0_rgba(0,0,0,0.15)] border-b border-black/10"
                style="width: {item.percent}%"
              ></div>
            {/if}
          </div>

          <!-- Value Percentage Ratios -->
          <span class="w-[85px] text-end font-extrabold font-mono text-foreground select-none">
            {item.percent}% <span class="text-muted-foreground font-bold font-mono">({item.value})</span>
          </span>
        </div>
      {/each}

      <!-- Backdrop Blur Overlay (No borders, no alert icons, frosted glass blur effect) -->
      {#if !summary}
        <div class="absolute inset-0 flex flex-col items-center justify-center text-center select-none p-4 gap-1.5 backdrop-blur-[3px] bg-background/35 z-10 transition-all duration-300 rounded-xl">
          <span class="font-bold text-xs text-foreground/90">{failed ? m.common_action_failed() : m.academics_behavior_empty_title()}</span>
          <span class="text-[9px] text-muted-foreground max-w-[280px] leading-relaxed">
            {failed ? "" : m.academics_behavior_empty_desc()}
          </span>
        </div>
      {/if}
    </div>

    <!-- Footer link matching Screenshot 1 -->
    <div class="border-t border-border/40 pt-2 flex items-center justify-start w-full select-none text-[9px] font-bold">
      <a
        href={localizeHref(`/${orgId}/students`)}
        class="text-primary hover:underline flex items-center gap-1 group/link"
      >
        <span>{m.reports_behavior_view_all()}</span>
        <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
      </a>
    </div>
  {/if}
</div>
