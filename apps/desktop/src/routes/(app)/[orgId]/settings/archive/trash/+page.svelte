<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import TrashManager from "$lib/features/documents/trash-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>سلة المحذوفات | الأرشيف | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <TrashManager {orgId} />
</div>
