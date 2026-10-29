<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { SectionsDirectory } from "$lib/features/academics";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_search_group_classes()} | Orbis</title>
</svelte:head>

<SectionsDirectory {orgId} />
