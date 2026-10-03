<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { CurrenciesManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_currencies()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <CurrenciesManager {orgId} />
</div>
