<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import { Badge } from "@orbis/ui-svelte/badge";
import {
  Popover,
  PopoverTrigger,
  PopoverContent,
} from "@orbis/ui-svelte/popover";
import {
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from "@orbis/ui-svelte";
import { Activity } from "lucide-svelte";
import { academicContext, workflowInbox } from "$lib/stores";
import * as m from "$lib/paraglide/messages";
import { l, formatDate } from "$lib/core/i18n";

function getPhaseLabel(phase: string) {
  switch (phase) {
    case "pre_start": return m.phase_pre_start ? m.phase_pre_start() : "Pre-Start";
    case "active": return m.phase_active ? m.phase_active() : "Active";
    case "exams": return m.phase_exams ? m.phase_exams() : "Exams";
    case "grade_entry": return m.phase_grade_entry ? m.phase_grade_entry() : "Grade Entry";
    case "closed": return m.phase_closed ? m.phase_closed() : "Closed";
    default: return "";
  }
}

const phaseLabel = $derived(getPhaseLabel(academicContext.cyclePhase));

const nextCycle = $derived.by(() => {
  const active = academicContext.activeCycle;
  const cycles = academicContext.allCycles;
  if (!cycles || cycles.length === 0) return null;

  // Chronologically sorted cycles. Let's sort them by start_date ascending.
  const sorted = [...cycles].sort((a, b) => a.start_date.localeCompare(b.start_date));

  if (active) {
    const activeIndex = sorted.findIndex(c => c.id === active.id);
    if (activeIndex !== -1 && activeIndex + 1 < sorted.length) {
      return sorted[activeIndex + 1];
    }
  }

  // If no active cycle, look for the first cycle that starts in the future
  const nowStr = new Date().toISOString().split("T")[0];
  const upcoming = sorted.find(c => c.start_date > nowStr);
  if (upcoming) return upcoming;

  return null;
});
</script>

<Popover>
	<PopoverTrigger>
		{#snippet child({ props })}
			<Tooltip>
				<TooltipTrigger>
					<Button 
						{...props}
						variant="outline" 
						size="icon-sm" 
						class="relative cursor-pointer rounded-[10px]"
						aria-label={m.sidebar_status_title ? m.sidebar_status_title() : "System Status"}
					>
						<Activity size={16} strokeWidth={1.8} class="text-foreground" />
						{#if workflowInbox.hasActions}
							<span class="absolute -top-0.5 -end-0.5 w-1.5 h-1.5 rounded-full bg-destructive border border-background animate-pulse"></span>
						{:else}
							<span class="absolute -top-0.5 -end-0.5 w-1.5 h-1.5 rounded-full bg-primary border border-background"></span>
						{/if}
					</Button>
				</TooltipTrigger>
				<TooltipContent side="right" sideOffset={12}>
					{m.sidebar_status_title ? m.sidebar_status_title() : "System Status"}
				</TooltipContent>
			</Tooltip>
		{/snippet}
	</PopoverTrigger>
	<PopoverContent side="right" sideOffset={12} class="w-80 p-0 overflow-hidden bg-card border-border shadow-xl rounded-xl">
		<!-- Header -->
		<div class="px-4 py-3 border-b border-border bg-muted/30">
			<p class="text-xs font-semibold text-foreground">{m.sidebar_status_title ? m.sidebar_status_title() : "System Status"}</p>
		</div>

		<!-- Academic Cycles Timeline -->
		<div class="p-4 border-b border-border flex flex-col gap-3">
			<h4 class="text-xs font-bold text-foreground leading-normal text-start">
				{m.sidebar_status_timeline_title ? m.sidebar_status_timeline_title() : "Academic Cycles Timeline"}
			</h4>

			<!-- Vertical Timeline -->
			<div class="relative ps-4 border-s border-border mt-2 space-y-5">
				<!-- Active Cycle -->
				<div class="relative text-start">
					{#if academicContext.activeCycle}
						<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-primary border border-background animate-pulse"></span>
						<div class="flex items-center justify-between gap-1.5">
							<span class="font-bold text-foreground text-[11px] leading-tight">
								{l(academicContext.activeCycle.name_ar, academicContext.activeCycle.name)}
							</span>
							<Badge variant="lime" class="text-[9px] h-3.5 px-1 shrink-0">
								{m.sidebar_status_active_cycle ? m.sidebar_status_active_cycle() : "Active"}
							</Badge>
						</div>
						<p class="text-[10px] text-muted-foreground leading-normal mt-1">
							{formatDate(academicContext.activeCycle.start_date)} - {formatDate(academicContext.activeCycle.end_date)}
						</p>
						{#if getPhaseLabel(academicContext.cyclePhase)}
							<p class="text-[9px] text-primary/80 font-medium mt-0.5">
								{m.sidebar_status_phase ? m.sidebar_status_phase() : "Phase"}: {getPhaseLabel(academicContext.cyclePhase)}
							</p>
						{/if}
					{:else}
						<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-muted border border-background"></span>
						<span class="font-bold text-muted-foreground text-[11px]">
							{m.sidebar_status_no_active_cycle ? m.sidebar_status_no_active_cycle() : "No active cycle"}
						</span>
					{/if}
				</div>

				<!-- Next Cycle -->
				<div class="relative text-start">
					{#if nextCycle}
						<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-muted-foreground/60 border border-background"></span>
						<div class="flex items-center justify-between gap-1.5">
							<span class="font-bold text-foreground text-[11px] leading-tight">
								{l(nextCycle.name_ar, nextCycle.name)}
							</span>
							<Badge variant="secondary" class="text-[9px] h-3.5 px-1 shrink-0">
								{m.sidebar_status_next_cycle ? m.sidebar_status_next_cycle() : "Next"}
							</Badge>
						</div>
						<p class="text-[10px] text-muted-foreground leading-normal mt-1">
							{m.sidebar_status_starts_on ? m.sidebar_status_starts_on({ date: formatDate(nextCycle.start_date) }) : `Starts on ${formatDate(nextCycle.start_date)}`}
						</p>
					{:else}
						<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-muted border border-background"></span>
						<span class="font-medium text-muted-foreground text-[11px]">
							{m.sidebar_status_no_next_cycle ? m.sidebar_status_no_next_cycle() : "No upcoming cycle"}
						</span>
					{/if}
				</div>
			</div>
		</div>

		<!-- School Metadata / Status -->
		<div class="p-4 flex flex-col gap-2.5 bg-muted/10">
			<!-- Academic Phase -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.sidebar_status_phase ? m.sidebar_status_phase() : "Phase"}</span>
				{#if phaseLabel}
					<Badge variant="secondary" class="text-[10px] h-4 px-1.5">{phaseLabel}</Badge>
				{:else}
					<span class="text-muted-foreground">—</span>
				{/if}
			</div>
			<!-- Year Status -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.sidebar_status_year ? m.sidebar_status_year() : "Year"}</span>
				<span class="font-medium text-foreground">{academicContext.yearLabel || "—"}</span>
			</div>
			<!-- Pending Approvals -->
			<div class="flex items-center justify-between text-xs">
				<span class="text-muted-foreground">{m.approvals_tab_pending ? m.approvals_tab_pending() : "Pending"}</span>
				{#if workflowInbox.hasActions}
					<Badge variant="destructive" class="text-[10px] h-4 px-1.5">{workflowInbox.totalCount}</Badge>
				{:else}
					<span class="text-primary text-[10px] font-medium">✓</span>
				{/if}
			</div>
			<!-- Read-Only Mode -->
			{#if academicContext.isReadOnly}
				<div class="flex items-center gap-1.5 text-[10px] text-muted-foreground bg-muted/50 rounded px-2 py-1 mt-1 justify-center">
					<span>🔒</span>
					<span>{m.year_archived_banner ? m.year_archived_banner() : "Read-only mode"}</span>
				</div>
			{/if}
		</div>
	</PopoverContent>
</Popover>
