<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import NotificationsManager from "$lib/features/settings/notifications-manager.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.notifications_settings_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <NotificationsManager {orgId} />
</div>
