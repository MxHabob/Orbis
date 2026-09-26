<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { EmployeeForm } from "$lib/features/employees";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.employees_new_title ? m.employees_new_title() : "New Employee"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <EmployeeForm {orgId} mode="create" />
</div>
