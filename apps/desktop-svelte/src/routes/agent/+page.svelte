<script lang="ts">
import { onMount } from "svelte";
import AskOrbisSidebar from "$lib/features/ai-agent/ui/views/sidebar.svelte";

onMount(() => {
	import("@tauri-apps/api/window")
		.then(({ getCurrentWindow }) => getCurrentWindow().show())
		.catch(() => {
			// The route also works in a regular browser during development.
		});
});
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-card text-card-foreground">
	<div class="flex-1 flex flex-col min-h-0 overflow-hidden relative">
		<AskOrbisSidebar isStandalone={true} />
	</div>
</div>
