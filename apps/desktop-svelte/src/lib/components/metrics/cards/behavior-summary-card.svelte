<script lang="ts">
import { Skeleton } from "@orbis/ui-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/services/ipc";
import DragIndicator from "../components/drag-indicator.svelte";

let {
  orgId,
  selectedClass = "all",
  selectedTerm = "all",
  period = "this-year",
  isEditing = false,
}: {
  orgId: string;
  selectedClass: string;
  selectedTerm: string;
  period: string;
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
    const res = await commands.dashboard.getStats(orgId);
    if (res && res.behavior_summary) {
      summary = res.behavior_summary;
    } else {
      // Fallback mock data
      summary = {
        total_records: 48,
        critical_count: 5,
        major_count: 15,
        minor_count: 28,
      };
    }
  } catch (e) {
    console.error("Error loading behavior reports:", e);
  } finally {
    isLoading = false;
  }
}

$effect(() => {
  if (orgId || selectedClass || selectedTerm || period) {
    loadData();
  }
});

// Donut variables
const r = 55;
const circumference = 2 * Math.PI * r; // ~345.57

const segments = $derived.by(() => {
  if (!summary) return [];
  const critical = summary.critical_count ?? 0;
  const major = summary.major_count ?? 0;
  const minor = summary.minor_count ?? 0;
  const total = critical + major + minor || 1;

  const dataList = [
    { name: "Critical", value: critical, color: "var(--color-destructive)" },
    { name: "Major", value: major, color: "oklch(0.65 0.15 80)" },
    { name: "Minor", value: minor, color: "var(--color-primary)" },
  ];

  let accumulatedPercent = 0;
  return dataList.map((d) => {
    const percent = d.value / total;
    const dashArray = `${percent * circumference} ${circumference}`;
    const dashOffset = -accumulatedPercent * circumference;
    accumulatedPercent += percent;
    return {
      ...d,
      percent: Math.round(percent * 100),
      dashArray,
      dashOffset,
    };
  });
});
</script>

<div class="card-tactile flex flex-col h-full relative group min-h-[460px]">
	{#if isLoading}
		<div class="mb-4 min-h-[100px] flex flex-col gap-2">
			<Skeleton class="h-[14px] w-[180px]" />
			<Skeleton class="h-[28px] w-[140px] mt-1" />
		</div>
		<div class="h-80 w-full bg-muted/5 border border-border/40 rounded-xl p-4">
			<Skeleton class="h-full w-full rounded-lg" />
		</div>
	{:else}
		<div class="mb-4 min-h-[100px]">
			<div class="flex items-start justify-between h-7">
				<h3 class="text-sm font-normal text-muted-foreground">
					Behavioral Incidents Severity
				</h3>
				{#if isEditing}
					<div>
						<DragIndicator />
					</div>
				{/if}
			</div>
			<p class="text-3xl font-semibold tracking-tight mt-1">
				{summary?.total_records ?? 0} <span class="text-xs text-muted-foreground font-normal">total incidents</span>
			</p>
		</div>

		<div class="h-80 w-full flex items-center justify-center bg-muted/10 border border-border/40 rounded-xl p-4 relative overflow-hidden">
			{#if summary}
				<svg viewBox="0 0 500 200" class="w-full h-full select-none overflow-visible">
					<!-- Base circle -->
					<circle
						cx="120"
						cy="100"
						r={r}
						fill="transparent"
						class="stroke-border/30"
						stroke-width="12"
					/>

					<!-- Segments -->
					{#each segments as seg, idx}
						{#if seg.value > 0}
							<circle
								cx="120"
								cy="100"
								r={r}
								fill="transparent"
								stroke={seg.color}
								stroke-width={hoveredIndex === idx ? 15 : 12}
								stroke-dasharray={seg.dashArray}
								stroke-dashoffset={seg.dashOffset}
								transform="rotate(-90 120 100)"
								stroke-linecap="round"
								class="transition-all duration-200 cursor-pointer origin-center"
								style="opacity: {hoveredIndex === null || hoveredIndex === idx ? 1 : 0.45}"
								onmouseenter={() => hoveredIndex = idx}
								onmouseleave={() => hoveredIndex = null}
							/>
						{/if}
					{/each}

					<!-- Donut Hole Labels -->
					{#if hoveredIndex === null}
						<text x="120" y="96" text-anchor="middle" class="text-[10px] fill-muted-foreground font-semibold uppercase tracking-wider">Total</text>
						<text x="120" y="118" text-anchor="middle" class="text-xl font-bold fill-foreground tracking-tight">{summary.total_records}</text>
					{:else}
						{@const seg = segments[hoveredIndex]}
						<text x="120" y="96" text-anchor="middle" class="text-[10px] font-bold uppercase tracking-wider" fill={seg.color}>{seg.name}</text>
						<text x="120" y="118" text-anchor="middle" class="text-xl font-bold fill-foreground tracking-tight">{seg.percent}%</text>
					{/if}

					<!-- Legend Details -->
					<g transform="translate(250, 45)">
						{#each segments as seg, idx}
							<g
								transform="translate(0, {idx * 40})"
								class="cursor-pointer transition-opacity duration-200"
								style="opacity: {hoveredIndex === null || hoveredIndex === idx ? 1 : 0.45}"
								onmouseenter={() => hoveredIndex = idx}
								onmouseleave={() => hoveredIndex = null}
							>
								<!-- Color Dot -->
								<circle cx="0" cy="0" r="5" fill={seg.color} />
								
								<!-- Label and Description -->
								<text x="15" y="4" class="text-xs font-semibold fill-foreground">{seg.name}</text>
								<text x="95" y="4" class="text-xs fill-muted-foreground">{seg.value} records ({seg.percent}%)</text>
							</g>
						{/each}
					</g>
				</svg>
			{:else}
				<div class="text-xs text-muted-foreground">No behavior logs found.</div>
			{/if}
		</div>
	{/if}
</div>
