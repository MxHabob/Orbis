<script lang="ts">
import { page } from "$app/state";
import * as m from "$lib/paraglide/messages";
import { AiOverview } from "$lib/features/ai";

const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.ai_center_overview_health()} | Orbis</title>
</svelte:head>

<AiOverview {orgId} />
