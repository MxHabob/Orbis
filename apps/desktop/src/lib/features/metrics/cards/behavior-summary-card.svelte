<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Skeleton } from "@orbis/ui";
import { onMount } from "svelte";
import { api } from "$lib/api";
import DragIndicator from "../components/drag-indicator.svelte";

import { localizeHref } from "$lib/paraglide/runtime";
import { ArrowRight } from "@lucide/svelte";

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

let summary = $state<any>(null);
let isLoading = $state(true);

// Interactive Hover State
let hoveredIndex = $state<number | null>(null);

async function loadData() {
  isLoading = true;
  try {
    // Query stats to extract behavior details
    const res = await api.getDashboard();
    if (
      res &&
      res.behavior_summary &&
      ((res.behavior_summary as any).critical_count ||
        (res.behavior_summary as any).major_count ||
        (res.behavior_summary as any).minor_count)
    ) {
      summary = res.behavior_summary as any;
    } else {
      summary = null;
    }
  } catch (e) {
    console.error("Error loading behavior reports:", e);
    summary = null;
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

const listItems = $derived.by(() => {
  if (!summary) return [];
  const critical = summary.critical_count ?? 0;
  const major = summary.major_count ?? 0;
  const minor = summary.minor_count ?? 0;
  const total = critical + major + minor || 1;

  return [
    {
      name: "critical",
      label: m.common_override_risk_critical(),
      value: critical,
      percent: Math.round((critical / total) * 100),
      color: "bg-destructive",
    },
    {
      name: "major",
      label: m.reports_behavior_major(),
      value: major,
      percent: Math.round((major / total) * 100),
      color: "bg-orange-500",
    },
    {
      name: "minor",
      label: m.reports_behavior_minor(),
      value: minor,
      percent: Math.round((minor / total) * 100),
      color: "bg-primary",
    },
  ];
});

// Determine display items list (uses placeholders when empty)
const displayItems = $derived(
  summary && listItems.length > 0
    ? listItems
    : [
        {
          name: "critical",
          label: m.common_override_risk_critical(),
          value: 0,
          percent: 0,
          color: "bg-destructive",
        },
        {
          name: "major",
          label: m.reports_behavior_major(),
          value: 0,
          percent: 0,
          color: "bg-orange-500",
        },
        {
          name: "minor",
          label: m.reports_behavior_minor(),
          value: 0,
          percent: 0,
          color: "bg-primary",
        },
      ],
);
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
        {m.reports_behavior_severity_title()} {#if summary}• Total: <span class="text-foreground font-mono">{summary.total_records}</span>{/if}
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
            {item.name}
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
          <span class="font-bold text-xs text-foreground/90">{m.reports_behavior_empty_title()}</span>
          <span class="text-[9px] text-muted-foreground max-w-[280px] leading-relaxed">
            {m.reports_behavior_empty_desc()}
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
        <span>{m.reports_behavior_view_all ? m.reports_behavior_view_all() : "View all incidents"}</span>
        <ArrowRight size={10} class="transition-transform group-hover/link:translate-x-0.5 rtl:rotate-180" />
      </a>
    </div>
  {/if}
</div>
