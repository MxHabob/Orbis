<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { Label } from "@orbis/ui-svelte/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@orbis/ui-svelte/select";
import { onMount } from "svelte";
import { commands } from "$lib/core/ipc";

let { selectedBranch = $bindable(""), selectedYear = $bindable("") } = $props<{
  selectedBranch?: string;
  selectedYear?: string;
}>();

let academicYears = $state<{ value: string; label: string }[]>([]);

async function loadYears() {
  let orgId = selectedBranch;

  try {
    const org = await commands.organization.getMainOrganization();
    if (org && org.id) {
      orgId = org.id;
      selectedBranch = org.id;
    }
  } catch (err) {
    console.warn("Failed to fetch main organization:", err);
  }

  // Use getSetupStatus or fallback to hardcoded default if empty
  try {
    if (!orgId) {
      orgId = "org-sanaa-model"; // Fallback identifier
    }
  } catch (err) {
    console.warn("Failed to fetch main organization:", err);
  }

  try {
    const years = await commands.platform.listYears();
    // @ts-ignore - y is typed based on OrbisCommands which might be outdated
    academicYears = years.map((y: any) => ({
      value: y.id,
      label: y.name_ar || y.name,
    }));
    if (academicYears.length > 0) {
      // @ts-ignore
      const activeYear = years.find((y: any) => y.is_active_in_app);
      if (activeYear) {
        selectedYear = activeYear.id;
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
		{m.auth_login_env_title()}
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
