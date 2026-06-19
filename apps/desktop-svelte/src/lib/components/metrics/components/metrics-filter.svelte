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
import { commands } from "$lib/services/ipc";

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
    const cycles = await commands.academics.getCycles(orgId, {
      cycle_type: "term",
    });
    terms = cycles || [];

    // Load Classes
    const cl = await commands.classes.getAll(orgId);
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
      return "3 months";
    case "6-months":
      return "6 months";
    case "this-year":
      return "This year";
    default:
      return "All time";
  }
});

const termName = $derived(() => {
  if (selectedTerm === "all") return "All Terms";
  const term = terms.find((t) => t.id === selectedTerm);
  return term ? term.name : "Selected Term";
});

const className = $derived(() => {
  if (selectedClass === "all") return "All Classes";
  const c = classes.find((i) => i.id === selectedClass);
  return c ? c.name : "Selected Class";
});
</script>

<div class="flex items-center gap-2">
	<DropdownMenu>
		<DropdownMenuTrigger>
			{#snippet child({ props })}
				<Button {...props} variant="outline" class="gap-2 px-2.5 h-8 text-xs font-medium bg-background border-border hover:bg-muted/40 cursor-pointer">
					<Filter size={13} class="text-[#666]" />
					<span>Filters</span>
					<ChevronDown size={12} class="text-[#666] opacity-60" />
				</Button>
			{/snippet}
		</DropdownMenuTrigger>
		<DropdownMenuContent class="w-60 bg-popover border border-border rounded-xl shadow-lg" align="end" sideOffset={8}>
			<!-- PERIOD Section -->
			<DropdownMenuGroup>
				<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1">
					PERIOD
				</DropdownMenuLabel>
				<DropdownMenuRadioGroup value={period} onValueChange={(val) => period = val}>
					<DropdownMenuRadioItem value="3-months" class="text-xs cursor-pointer">3 months</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="6-months" class="text-xs cursor-pointer">6 months</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="this-year" class="text-xs cursor-pointer">This year</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">All time</DropdownMenuRadioItem>
				</DropdownMenuRadioGroup>
			</DropdownMenuGroup>

			{#if terms.length > 0}
				<DropdownMenuSeparator class="bg-border/60" />
				<DropdownMenuGroup>
					<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1">
						ACADEMIC TERM
					</DropdownMenuLabel>
					<DropdownMenuRadioGroup value={selectedTerm} onValueChange={(val) => selectedTerm = val}>
						<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">All Terms</DropdownMenuRadioItem>
						{#each terms as term (term.id)}
							<DropdownMenuRadioItem value={term.id} class="text-xs cursor-pointer">{term.name}</DropdownMenuRadioItem>
						{/each}
					</DropdownMenuRadioGroup>
				</DropdownMenuGroup>
			{/if}

			{#if classes.length > 0}
				<DropdownMenuSeparator class="bg-border/60" />
				<DropdownMenuGroup>
					<DropdownMenuLabel class="text-[9px] text-muted-foreground font-normal px-2.5 py-1">
						CLASS / GRADE
					</DropdownMenuLabel>
					<DropdownMenuRadioGroup value={selectedClass} onValueChange={(val) => selectedClass = val}>
						<DropdownMenuRadioItem value="all" class="text-xs cursor-pointer">All Classes</DropdownMenuRadioItem>
						{#each classes as c (c.id)}
							<DropdownMenuRadioItem value={c.id} class="text-xs cursor-pointer">{c.name}</DropdownMenuRadioItem>
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
