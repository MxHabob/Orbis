<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { ArchiveManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
const folderId = $derived(page.params.folderId || null);
</script>

<svelte:head>
  <title>{m.settings_nav_archive ? m.settings_nav_archive() : "Archive"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ArchiveManager {orgId} {folderId} />
</div>
