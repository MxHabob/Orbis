<script lang="ts">
import { isDesktopApp } from "$lib/api";
import TitleBar from "$lib/shell/title-bar.svelte";

let { children } = $props();
</script>

<div class="h-screen w-screen bg-background text-foreground flex flex-col overflow-hidden font-sans antialiased relative select-none">
	{#if isDesktopApp}
		<TitleBar />
	{/if}
	
	<!-- Background subtle ambient glow -->
	<div class="pointer-events-none absolute inset-0 overflow-hidden -z-10">
		<div class="absolute -top-[30%] -start-[10%] w-[50vw] h-[50vw] rounded-full bg-primary/5 blur-3xl"></div>
		<div class="absolute -bottom-[30%] -end-[10%] w-[50vw] h-[50vw] rounded-full bg-primary/5 blur-3xl"></div>
	</div>

	<main class="flex-1 flex items-center justify-center p-3 sm:p-4 lg:p-6 min-h-0 overflow-y-auto">
		{@render children()}
	</main>
</div>
