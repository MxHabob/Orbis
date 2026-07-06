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
import { Activity, ExternalLink } from "lucide-svelte";
import { academicContext, workflowInbox } from "$lib/stores";
import * as m from "$lib/paraglide/messages";

const phaseLabel = $derived.by(() => {
  const phase = academicContext.cyclePhase;
  switch (phase) {
    case "pre_start": return m.phase_pre_start ? m.phase_pre_start() : "Pre-Start";
    case "active": return m.phase_active ? m.phase_active() : "Active";
    case "exams": return m.phase_exams ? m.phase_exams() : "Exams";
    case "grade_entry": return m.phase_grade_entry ? m.phase_grade_entry() : "Grade Entry";
    case "closed": return m.phase_closed ? m.phase_closed() : "Closed";
    default: return "";
  }
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

		<!-- Incident Ingestion Alerts (Sentry-like layout) -->
		<div class="p-4 border-b border-border flex flex-col gap-3">
			<div class="space-y-1">
				<h4 class="text-xs font-bold text-foreground leading-normal text-start">
					{m.sidebar_status_ingestion_stopped()}
				</h4>
				<a 
					href="https://status.orbis.edu" 
					target="_blank" 
					rel="noopener noreferrer" 
					class="inline-flex items-center gap-1 text-[10px] text-primary hover:underline font-medium text-start"
				>
					<span>{m.sidebar_status_incident_details()}</span>
					<ExternalLink size={10} />
				</a>
				<p class="text-[10px] text-muted-foreground text-start">
					{m.sidebar_status_incident_started({ hours: "6" })}
				</p>
			</div>

			<!-- Vertical Timeline -->
			<div class="relative ps-4 border-s border-border mt-2 space-y-4">
				<!-- Item 1 (Monitoring) -->
				<div class="relative text-start">
					<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-yellow-500 border border-background"></span>
					<div class="flex items-center gap-1.5 text-[10px]">
						<span class="font-bold text-yellow-500">{m.sidebar_status_monitoring_label()}</span>
						<span class="text-muted-foreground">(4h ago)</span>
					</div>
					<p class="text-[10px] text-muted-foreground leading-normal mt-0.5">
						{m.sidebar_status_monitoring_recovery()}
					</p>
				</div>

				<!-- Item 2 (Monitoring) -->
				<div class="relative text-start">
					<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-yellow-500 border border-background"></span>
					<div class="flex items-center gap-1.5 text-[10px]">
						<span class="font-bold text-yellow-500">{m.sidebar_status_monitoring_label()}</span>
						<span class="text-muted-foreground">(6h ago)</span>
					</div>
					<p class="text-[10px] text-muted-foreground leading-normal mt-0.5">
						{m.sidebar_status_monitoring_fixed()}
					</p>
				</div>

				<!-- Item 3 (Investigating) -->
				<div class="relative text-start">
					<span class="absolute -start-[21px] top-1 size-2 rounded-full bg-destructive border border-background"></span>
					<div class="flex items-center gap-1.5 text-[10px]">
						<span class="font-bold text-destructive">{m.sidebar_status_investigating_label()}</span>
						<span class="text-muted-foreground">(6h ago)</span>
					</div>
					<p class="text-[10px] text-muted-foreground leading-normal mt-0.5">
						{m.sidebar_status_investigating()}
					</p>
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
