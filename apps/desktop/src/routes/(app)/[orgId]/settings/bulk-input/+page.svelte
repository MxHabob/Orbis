<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { BulkEntryStudio } from "$lib/features/bulk-entry";
</script>

<svelte:head>
  <title>{m.ui_page_bulk_entry()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <BulkEntryStudio />
</div>
