<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import UsersDirectory from "$lib/features/settings/users-directory.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_users()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <UsersDirectory {orgId} />
</div>
