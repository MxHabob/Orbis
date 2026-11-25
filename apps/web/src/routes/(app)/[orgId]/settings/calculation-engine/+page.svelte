<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import AcademicPolicyManager from "$lib/features/settings/academic-policy-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_calculation_engine()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicPolicyManager {orgId} />
</div>
