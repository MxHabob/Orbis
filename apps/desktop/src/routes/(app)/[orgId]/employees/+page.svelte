<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { EmployeeDirectory } from "$lib/features/employees";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.employees_nav_title ? m.employees_nav_title() : "Employee Affairs"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <EmployeeDirectory {orgId} />
</div>
