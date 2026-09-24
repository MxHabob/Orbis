<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  import BulkStudentImport from "$lib/features/bulk-input/bulk-student-import.svelte";

  let { data }: { data: { orgId: string } } = $props();
  const orgId = $derived(data?.orgId || page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_bulk_card_students_title?.() || "Bulk Student Import"} - Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <BulkStudentImport {orgId} />
</div>
