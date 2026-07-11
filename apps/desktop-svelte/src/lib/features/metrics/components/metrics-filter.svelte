<script lang="ts">
import { Button } from "@orbis/ui-svelte/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuLabel,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@orbis/ui-svelte/dropdown-menu";
import { Filter, ChevronDown } from "lucide-svelte";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import * as m from "$lib/paraglide/messages";
import { getLocale } from "$lib/paraglide/runtime";

let {
  orgId,
  period = $bindable("this-year"),
  selectedTerm = $bindable("all"),
  selectedClass = $bindable("all"),
}: {
  orgId: string;
  period: string;
  selectedTerm: string;
  selectedClass: string;
} = $props();

// Loaded Options
let terms = $state<any[]>([]);
let classes = $state<any[]>([]);

async function loadOptions() {
  try {
    // Load Terms (Cycles of type 'term')
    const cycles = await commands.academic.listCycles(orgId, {
      cycle_type: "term",
    });
    terms = cycles || [];

    // Load Classes
    const cl = await commands.academic.listClasses(orgId);
    classes = cl || [];
  } catch (e) {
    console.error("Error loading filter options:", e);
  }
}

onMount(() => {
  loadOptions();
});

// Get Labels for indicators
const periodLabel = $derived(() => {
  switch (period) {
    case "3-months":
      return m.metrics_period_3months();
    case "6-months":
      return m.metrics_period_6months();
    case "this-year":
      return m.metrics_period_this_year();
    default:
      return m.metrics_period_all();
  }
});

const termName = $derived(() => {
  if (selectedTerm === "all") return m.metrics_all_terms();
  const term = terms.find((t) => t.id === selectedTerm);
  if (!term) return m.metrics_all_terms();
  return getLocale() === "ar" && term.name_ar
    ? term.name_ar
    : term.name || term.name_en || term.name_ar || "";
});

const className = $derived(() => {
  if (selectedClass === "all") return m.metrics_all_classes();
  const c = classes.find((i) => i.id === selectedClass);
  if (!c) return m.metrics_all_classes();
  return getLocale() === "ar" && c.name_ar
    ? c.name_ar
    : c.name || c.name_en || c.name_ar || "";
});
</script>

<div class="flex items-center gap-2">
	<DropdownMenu>
		<DropdownMenuTrigger>
			{#snippet child({ props }: { props: any })}
				<Button {...props} variant="outline" class="gap-2 px-2.5 h-8 text-xs font-medium bg-background border-border hover:bg-muted/40 cursor-pointer">
					<Filter size={13} class="text-[#666]" />
					<span>{m.metrics_filters()}</span>
					<ChevronDown size={12} class="text-[#666] opacity-60" />
				</Button>
			{/snippet}
		</DropdownMenuTrigger>
		<DropdownMenuContent class="w-60 bg-popover border border-border rounded-xl shadow-lg" align="end" sideOffset={8}>
			<!-- PERIOD Section -->
			<DropdownMenuGroup>
				<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1 uppercase tracking-wider">
					{m.metrics_period()}
				</DropdownMenuLabel>
				<DropdownMenuRadioGroup value={period} onValueChange={(val) => period = val}>
					<DropdownMenuRadioItem value="3-months" class="text-xs cursor-pointer">{m.metrics_period_3months()}</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="6-months" class="text-xs cursor-pointer">{m.metrics_period_6months()}</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="this-year" class="text-xs cursor-pointer">{m.metrics_period_this_year()}</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">{m.metrics_period_all()}</DropdownMenuRadioItem>
				</DropdownMenuRadioGroup>
			</DropdownMenuGroup>

			{#if terms.length > 0}
				<DropdownMenuSeparator class="bg-border/60" />
				<DropdownMenuGroup>
					<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1 uppercase tracking-wider">
						{m.metrics_academic_term()}
					</DropdownMenuLabel>
					<DropdownMenuRadioGroup value={selectedTerm} onValueChange={(val) => selectedTerm = val}>
						<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">{m.metrics_all_terms()}</DropdownMenuRadioItem>
						{#each terms as term (term.id)}
							<DropdownMenuRadioItem value={term.id} class="text-xs cursor-pointer">
								{getLocale() === "ar" && term.name_ar ? term.name_ar : (term.name || term.name_en || term.name_ar || '')}
							</DropdownMenuRadioItem>
						{/each}
					</DropdownMenuRadioGroup>
				</DropdownMenuGroup>
			{/if}

			{#if classes.length > 0}
				<DropdownMenuSeparator class="bg-border/60" />
				<DropdownMenuGroup>
					<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1 uppercase tracking-wider">
						{m.metrics_class_grade()}
					</DropdownMenuLabel>
					<DropdownMenuRadioGroup value={selectedClass} onValueChange={(val) => selectedClass = val}>
						<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">{m.metrics_all_classes()}</DropdownMenuRadioItem>
						{#each classes as c (c.id)}
							<DropdownMenuRadioItem value={c.id} class="text-xs cursor-pointer">
								{getLocale() === "ar" && c.name_ar ? c.name_ar : (c.name || c.name_en || c.name_ar || '')}
							</DropdownMenuRadioItem>
						{/each}
					</DropdownMenuRadioGroup>
				</DropdownMenuGroup>
			{/if}
		</DropdownMenuContent>
	</DropdownMenu>

	<!-- Indicators -->
	<div class="hidden sm:flex items-center gap-1.5 text-[10px] text-muted-foreground bg-muted/40 px-2 py-1.5 border border-border/40 rounded-md leading-none h-8 select-none">
		<span class="font-medium text-foreground">{periodLabel()}</span>
		{#if selectedTerm !== "all"}
			<span class="text-muted-foreground/30">•</span>
			<span class="truncate max-w-[100px]">{termName()}</span>
		{/if}
		{#if selectedClass !== "all"}
			<span class="text-muted-foreground/30">•</span>
			<span class="truncate max-w-[100px]">{className()}</span>
		{/if}
	</div>
</div>
