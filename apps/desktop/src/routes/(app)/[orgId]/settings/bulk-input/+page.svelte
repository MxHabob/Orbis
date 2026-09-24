<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  import BulkInputWorkspace from "$lib/features/bulk-input/bulk-input-workspace.svelte";

  let { data }: { data: { orgId: string } } = $props();
  const orgId = $derived(data?.orgId || page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_bulk_hub_title?.() || "Bulk Import"} - Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <BulkInputWorkspace {orgId} />
</div>
