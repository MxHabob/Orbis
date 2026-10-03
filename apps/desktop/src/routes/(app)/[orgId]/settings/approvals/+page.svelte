<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import { ApprovalsManager } from "$lib/features/settings";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.approvals_title ? m.approvals_title() : "Approvals"} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ApprovalsManager {orgId} />
</div>
