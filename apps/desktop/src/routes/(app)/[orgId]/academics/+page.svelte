<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { AcademicsOverview } from "$lib/features/academics";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.academics_overview_title()} | Orbis</title>
</svelte:head>

<AcademicsOverview {orgId} />
