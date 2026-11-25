<script lang="ts">
import * as m from "$lib/paraglide/messages";
import AcademicOutcomePolicyManager from "$lib/features/settings/academic-outcome-policy-manager.svelte";
</script>

<svelte:head>
  <title>{m.common_nav_academic_policy()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicOutcomePolicyManager />
</div>
