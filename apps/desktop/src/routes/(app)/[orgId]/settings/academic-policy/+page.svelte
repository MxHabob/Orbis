<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { AcademicPolicyManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_grading_schemes ? m.settings_nav_grading_schemes() : "Academic Policy"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicPolicyManager {orgId} />
</div>
