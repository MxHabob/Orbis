<script lang="ts">
import { isDesktopApp } from "$lib/core/ipc";
import TitleBar from "$lib/shell/title-bar.svelte";
import { uiStore } from "$lib/stores";

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

<!-- Lazy-load SearchModal only when opened (saves ~15KB from initial bundle) -->
{#if uiStore.showSearch}
	{#await import("$lib/shell/search-modal.svelte") then { default: SearchModal }}
		<SearchModal />
	{/await}
{/if}
