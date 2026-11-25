<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import FeeTemplatesManager from "$lib/features/settings/fee-templates-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_fee_templates()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <FeeTemplatesManager {orgId} />
</div>
