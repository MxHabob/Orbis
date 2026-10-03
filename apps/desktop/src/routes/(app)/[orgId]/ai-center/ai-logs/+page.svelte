<script lang="ts">
  import { page } from "$app/state";
  import * as m from "$lib/paraglide/messages";
  import { AiLogs } from "$lib/features/ai";

  const orgId = $derived(page.params.orgId || "");
</script>

<svelte:head>
  <title>{m.ai_center_logs_title()} | Orbis</title>
</svelte:head>

<AiLogs {orgId} />
