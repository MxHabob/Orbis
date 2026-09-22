<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Button } from "@orbis/ui/button";
import { Check, Settings } from "@orbis/icons";
import MetricsFilter from "./metrics-filter.svelte";

let {
  orgId,
  isEditing = $bindable(false),
  period = $bindable("this-year"),
  selectedTerm = $bindable("all"),
  selectedClass = $bindable("all"),
}: {
  orgId: string;
  isEditing: boolean;
  period: string;
  selectedTerm: string;
  selectedClass: string;
} = $props();
</script>

<div class="flex items-center justify-between w-full border-b border-border/40 pb-4 mb-6 pt-2 select-none">
	<div class="flex items-center gap-2">
		<Button
			variant="outline"
			class="gap-1.5 px-3 h-8 text-xs font-medium cursor-pointer bg-background border-border hover:bg-muted/40 h-8"
			onclick={() => isEditing = !isEditing}
		>
			{#if isEditing}
				<Check size={14} class="text-emerald-500" />
				<span>{m.common_schedule_ai_status_done()}</span>
			{:else}
				<Settings size={14} class="text-muted-foreground" />
				<span>{m.reports_metrics_customize()}</span>
			{/if}
		</Button>

		<MetricsFilter
			{orgId}
			bind:period
			bind:selectedTerm
			bind:selectedClass
		/>
	</div>
</div>
