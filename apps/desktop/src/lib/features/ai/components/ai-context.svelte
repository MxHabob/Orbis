<script lang="ts">
import { aiContext } from "$lib/stores";
import type { AiPageContext } from "$lib/stores/ai-context.svelte";
import { onMount } from "svelte";

let props: AiPageContext = $props();

onMount(() => {
  aiContext.setContext(props);
  return () => {
    aiContext.clearContext();
  };
});
</script>
