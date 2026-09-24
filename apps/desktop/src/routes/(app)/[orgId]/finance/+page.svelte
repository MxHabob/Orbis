<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { FinanceOverview } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.common_sidebar_finance()} | Orbis</title>
</svelte:head>

<FinanceOverview orgId={data.orgId} />
