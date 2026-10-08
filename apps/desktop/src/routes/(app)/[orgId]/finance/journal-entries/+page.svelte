<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { JournalEntries } from "$lib/features/finance";

let { data }: { data: { orgId: string } } = $props();
</script>

<svelte:head>
  <title>{m.ui_page_finance_journal_entries()} | Orbis</title>
</svelte:head>

<JournalEntries orgId={data.orgId} />
