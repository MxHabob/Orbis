<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { PaymentsManager } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.common_sidebar_payments()} | Orbis</title>
</svelte:head>

<PaymentsManager orgId={data.orgId} />
