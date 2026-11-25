<script lang="ts">
import TitleBar from "$lib/shell/title-bar.svelte";
import { isDesktopApp } from "$lib/api";
import { uiStore } from "$lib/stores";
import { printState } from "$lib/stores/print.svelte";
import { localeState } from "$lib/core/i18n";

let { children } = $props();
</script>

<div class="flex flex-col h-dvh w-dvw min-h-0 min-w-0 overflow-hidden bg-background">
	{#if isDesktopApp}
		{#key localeState.revision}
			<TitleBar />
		{/key}
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

{#if printState.isOpen}
	{#await import("$lib/shared/components/print-dialog.svelte") then { default: PrintDialog }}
		<PrintDialog
			bind:open={printState.isOpen}
			entityType={printState.entityType}
			documentType={printState.documentType}
			entityIds={printState.entityIds}
			entityLabels={printState.entityLabels}
			contexts={printState.contexts}
		/>
	{/await}
{/if}
