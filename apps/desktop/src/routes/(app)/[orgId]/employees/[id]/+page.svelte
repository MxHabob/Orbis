<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { EmployeeProfile } from "$lib/features/employees";

const orgId = $derived(page.params.orgId || "");
const employeeId = $derived(page.params.id || "");
</script>

<svelte:head>
  <title>{m.employees_profile_title ? m.employees_profile_title() : "Employee Dossier"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <EmployeeProfile {orgId} {employeeId} />
</div>
