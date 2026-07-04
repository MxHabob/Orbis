<script lang="ts">
import { Label } from "@orbis/ui-svelte/label";
import { Select, SelectTrigger, SelectContent, SelectItem } from "@orbis/ui-svelte/select";
import * as m from "$lib/paraglide/messages";

let { selectedBranch = $bindable("main_branch"), selectedYear = $bindable("2026_2027") } = $props<{
  selectedBranch?: string;
  selectedYear?: string;
}>();

const branches = $derived([
  { value: "main_branch", label: m.login_default_branch() },
  { value: "secondary_branch", label: m.login_other_branch() }
]);

const academicYears = $derived([
  { value: "2026_2027", label: m.login_current_year() },
  { value: "2027_2028", label: m.login_next_year() }
]);
</script>

<div class="flex flex-col gap-1 text-start mb-2">
	<h3 class="text-sm font-semibold text-foreground select-none">
		{m.login_env_title()}
	</h3>
	<p class="text-xs text-muted-foreground leading-normal select-none">
		{m.login_env_desc()}
	</p>
</div>

<!-- Branch Select -->
<div class="flex flex-col gap-2 text-start">
	<Label class="text-xs font-semibold text-foreground select-none uppercase tracking-wider">
		{m.login_select_org()}
	</Label>
	<Select
		type="single"
		value={selectedBranch}
		onValueChange={(v: string | undefined) => (selectedBranch = v ?? "")}
	>
		<SelectTrigger class="h-10 text-xs focus-visible:ring-primary bg-background/50 border-input w-full flex justify-between items-center px-3">
			{branches.find(b => b.value === selectedBranch)?.label || selectedBranch}
		</SelectTrigger>
		<SelectContent>
			{#each branches as b}
				<SelectItem value={b.value}>{b.label}</SelectItem>
			{/each}
		</SelectContent>
	</Select>
</div>

<!-- Academic Year Select -->
<div class="flex flex-col gap-2 text-start mt-2">
	<Label class="text-xs font-semibold text-foreground select-none uppercase tracking-wider">
		{m.login_select_year()}
	</Label>
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
