<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { GroupDetailView } from "$lib/features/groups";

const orgId = $derived(page.params.orgId || "");
const groupId = $derived(page.params.id || "");
</script>

<svelte:head>
  <title>{m.groups_nav_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <GroupDetailView {orgId} {groupId} />
</div>
