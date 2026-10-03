<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { PeriodsManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_periods ? m.settings_nav_periods() : "Academic Periods"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <PeriodsManager {orgId} />
</div>
