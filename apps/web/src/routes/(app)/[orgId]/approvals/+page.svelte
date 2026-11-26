<script lang="ts">
import * as m from "$lib/paraglide/messages";
import { page } from "$app/state";
import ApprovalsManager from "$lib/features/settings/approvals-manager.svelte";

const orgId = $derived(page.params.orgId || "");
const requestedTab = $derived(page.url.searchParams.get("tab"));
const initialTab = $derived(
  requestedTab === "mine"
    ? "mine"
    : requestedTab === "history" || requestedTab === "log"
      ? "history"
      : "pending",
);
</script>

<svelte:head>
  <title>{m.approvals_title()} | Orbis</title>
</svelte:head>

<div class="h-full w-full overflow-hidden">
  <ApprovalsManager {orgId} {initialTab} />
</div>
