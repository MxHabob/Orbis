<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { AiLiveCockpit } from "$lib/features/ai";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.ai_center_live_title()} | Orbis</title>
</svelte:head>

<AiLiveCockpit {orgId} />
