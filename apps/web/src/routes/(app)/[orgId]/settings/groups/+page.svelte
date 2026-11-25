<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { GroupsManager } from "$lib/features/groups";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.groups_nav_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <GroupsManager {orgId} />
</div>
