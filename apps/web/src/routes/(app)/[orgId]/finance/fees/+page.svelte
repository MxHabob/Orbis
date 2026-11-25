<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { FeesWorkspace } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.common_sidebar_fees()} | Orbis</title>
</svelte:head>

<FeesWorkspace orgId={data.orgId} />
