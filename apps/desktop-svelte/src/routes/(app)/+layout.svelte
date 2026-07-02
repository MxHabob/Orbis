<script lang="ts">
import { isDesktopApp } from "$lib/services/ipc";
import TitleBar from "$lib/components/layout/TitleBar.svelte";
import SearchModal from "$lib/components/layout/SearchModal.svelte";

let { children } = $props();
</script>

<div class="flex flex-col h-screen w-screen overflow-hidden bg-background">
	{#if isDesktopApp()}
		<TitleBar />
	{/if}
	<div class="flex-1 flex flex-col min-h-0 overflow-hidden relative">
		{@render children()}
	</div>
</div>

<SearchModal />
