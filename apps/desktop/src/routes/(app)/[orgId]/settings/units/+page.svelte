<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { AcademicUnitsManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_units()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <AcademicUnitsManager {orgId} />
</div>
