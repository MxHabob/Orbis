<script lang="ts">
  import * as m from "$lib/paraglide/messages";
  import { page } from "$app/state";
  import { AcademicsReports } from "$lib/features/academics";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.ui_page_academics_reports()} | Orbis</title>
</svelte:head>

<AcademicsReports {orgId} />
