<script lang="ts">
import AskOrbisSidebar from "$lib/features/ai/views/sidebar.svelte";
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-card text-card-foreground">
	<div class="flex-1 flex flex-col min-h-0 overflow-hidden relative">
		<AskOrbisSidebar isStandalone={true} />
	</div>
</div>
