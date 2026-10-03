<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import DatabaseManagementView from "$lib/features/settings/database/database-management-view.svelte";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.settings_nav_database()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <DatabaseManagementView {orgId} />
</div>
