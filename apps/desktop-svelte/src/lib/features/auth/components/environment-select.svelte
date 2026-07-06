<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui-svelte/select";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";
import * as m from "$lib/paraglide/messages";

let {
  selectedBranch = $bindable("org-sanaa-model"),
  selectedYear = $bindable(""),
} = $props<{
  selectedBranch?: string;
  selectedYear?: string;
}>();

let academicYears = $state<{ value: string; label: string }[]>([]);

async function loadYears() {
  const orgId = selectedBranch || "org-sanaa-model";
  try {
    const years = await commands.academicYears.list(orgId);
    academicYears = years.map((y) => ({
      value: y.year.id,
      label: y.year.name_ar || y.year.name,
    }));
    if (academicYears.length > 0) {
      const activeYear = years.find((y) => y.is_active_in_app);
      if (activeYear) {
        selectedYear = activeYear.year.id;
      } else {
        selectedYear = academicYears[0].value;
      }
    }
  } catch (err) {
    console.warn("Failed to load academic years for org:", orgId, err);
    academicYears = [];
    selectedYear = "";
  }
}

onMount(() => {
  loadYears();
});
</script>

<div class="flex flex-col gap-1 text-start">
	<h3 class="text-sm font-semibold text-foreground select-none">
		{m.login_env_title()}
	</h3>
</div>

<!-- Academic Year Select -->
<div class="flex flex-col gap-2 text-start">
	<Select
		type="single"
		value={selectedYear}
		onValueChange={(v: string | undefined) => (selectedYear = v ?? "")}
	>
		<SelectTrigger class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input w-full flex justify-between items-center px-3">
			{academicYears.find(y => y.value === selectedYear)?.label || selectedYear}
		</SelectTrigger>
		<SelectContent>
			{#each academicYears as y}
				<SelectItem value={y.value}>{y.label}</SelectItem>
			{/each}
		</SelectContent>
	</Select>
</div>
