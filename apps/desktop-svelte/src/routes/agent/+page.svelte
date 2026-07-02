<script lang="ts">
import AskOrbisSidebar from "$lib/components/layout/AskOrbisSidebar.svelte";
import { onMount } from "svelte";
import { authStore } from "$lib/stores";

onMount(() => {
  authStore.restore();
});
</script>

<div class="h-screen w-screen bg-card flex flex-col overflow-hidden">
  <AskOrbisSidebar isStandalone={true} />
</div>
