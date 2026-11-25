<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { DisbursementsManager } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.ui_page_finance_disbursements()} | Orbis</title>
</svelte:head>

<DisbursementsManager orgId={data.orgId} />
