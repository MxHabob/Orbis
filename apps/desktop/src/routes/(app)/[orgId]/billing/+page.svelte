<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import LicenseBillingManager from "$lib/features/settings/license-billing-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.license_nav_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <LicenseBillingManager {orgId} />
</div>
