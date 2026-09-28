<script lang="ts">
import { DisbursementsManager } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>سندات الصرف والمدفوعات | Orbis</title>
</svelte:head>

<DisbursementsManager orgId={data.orgId} />
