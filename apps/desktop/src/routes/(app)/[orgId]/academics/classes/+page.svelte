<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { ClassesDirectory } from "$lib/features/academics";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.common_card_active_classes()} | Orbis</title>
</svelte:head>

<ClassesDirectory {orgId} />
