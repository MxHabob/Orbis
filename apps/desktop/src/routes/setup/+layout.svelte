<script lang="ts">
import { isDesktopApp } from "$lib/api";
import TitleBar from "$lib/shell/title-bar.svelte";

let { children } = $props();
</script>

<div class="h-screen w-screen bg-background text-foreground flex flex-col overflow-hidden font-sans antialiased relative select-none">
	{#if isDesktopApp}
		<TitleBar />
	{/if}
	<main class="flex-1 flex items-center justify-center p-4 sm:p-6 lg:p-8 min-h-0 overflow-y-auto">
		{@render children()}
	</main>
</div>
