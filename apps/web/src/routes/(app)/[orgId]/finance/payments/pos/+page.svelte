<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { PosTerminal } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.finance_pos_quick_pos_payment_register()} | Orbis</title>
</svelte:head>

<PosTerminal orgId={data.orgId} />
