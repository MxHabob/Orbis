<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { AccountStatement } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>كشف الحساب المالي | Orbis</title>
</svelte:head>

<AccountStatement orgId={data.orgId} />
