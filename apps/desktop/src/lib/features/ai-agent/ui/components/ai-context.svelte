<script lang="ts">
import { aiContext } from "$lib/stores";
import type { AiPageContext } from "$lib/stores/ai-context.svelte";

let { domain, pageName, recordId, metadata }: AiPageContext = $props();

$effect(() => {
  aiContext.setContext({ domain, pageName, recordId, metadata });
  return () => {
    aiContext.clearContext();
  };
});
</script>
