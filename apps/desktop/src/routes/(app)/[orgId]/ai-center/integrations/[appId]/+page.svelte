<script lang="ts">
  import { page } from "$app/state";
  import { IntegrationDetails } from "$lib/features/ai";

  const orgId = $derived(page.params.orgId || "");
  const appId = $derived(page.params.appId || "");
</script>

<svelte:head>
  <title>Integration Details | Orbis</title>
</svelte:head>

<IntegrationDetails {orgId} {appId} />
