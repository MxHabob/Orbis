<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import RoomsManager from "$lib/features/settings/rooms-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_rooms()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <RoomsManager {orgId} />
</div>
