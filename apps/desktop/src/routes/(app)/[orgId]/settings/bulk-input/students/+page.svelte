<script lang="ts">
  import { BulkEntryStudio } from "$lib/features/bulk-entry";
</script>

<svelte:head>
  <title>الإدخال الجماعي للطلاب | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <BulkEntryStudio initialProfileId="students" />
</div>
