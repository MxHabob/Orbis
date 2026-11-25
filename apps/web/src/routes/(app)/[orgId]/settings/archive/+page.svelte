<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import ArchiveManager from "$lib/features/settings/archive-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_archive()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ArchiveManager {orgId} />
</div>
