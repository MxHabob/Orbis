<script lang="ts">
  import { page } from "$app/state";
  import { AcademicsReports } from "$lib/features/academics";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>التقارير الأكاديمية | Orbis</title>
</svelte:head>

<AcademicsReports {orgId} />
