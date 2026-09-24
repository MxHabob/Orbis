<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { JournalEntries } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>قيود اليومية المحاسبية | Orbis</title>
</svelte:head>

<JournalEntries orgId={data.orgId} />
