<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { ProviderAuth } from "$lib/features/auth";

const provider = $derived(page.params.provider?.toLowerCase() || "google");
const providerName = $derived(provider.charAt(0).toUpperCase() + provider.slice(1));
</script>

<svelte:head>
  <title>{m.auth_provider_title({ provider: providerName })} | Orbis</title>
</svelte:head>

<ProviderAuth />

