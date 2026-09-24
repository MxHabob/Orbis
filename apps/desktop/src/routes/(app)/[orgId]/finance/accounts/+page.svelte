<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AccountsManager } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.common_sidebar_ledgers ? m.common_sidebar_ledgers() : "الحسابات"} | Orbis</title>
</svelte:head>

<AccountsManager orgId={data.orgId} />
