<!--
@component AppChrome
The outermost layout shell for the authenticated application.
Follows GitButler's "Chrome Shell" pattern:

  ┌──────────────────────────────────┐
  │           AppHeader              │
  ├──┬───────────────────────────────┤
  │  │                               │
  │S │       chrome-content          │
  │i │     (page via slot)           │
  │d │                               │
  │e │                               │
  │b │                               │
  │a │                               │
  │r │                               │
  │  │                               │
  └──┴───────────────────────────────┘
-->
<script lang="ts">
import AppSidebar from "./app-sidebar.svelte";
import { uiStore } from "$lib/stores";
import { bootstrapApp } from "$lib/core/bootstrap";
import type { Snippet } from "svelte";
import { slide } from "svelte/transition";
import { page } from "$app/state";
import { onMount } from "svelte";
import AskOrbisSidebar from "$lib/features/ai-agent/ui/views/sidebar.svelte";

let {
  orgId,
  orgName,
  children,
}: {
  orgId: string;
  orgName?: string;
  children: Snippet;
} = $props();

// Bootstrap academic context, workflow inbox, and Tauri event listeners.
// Runs once per session — idempotent, non-blocking.
onMount(() => {
  bootstrapApp(orgId);
});

function handleKeyDown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === "/") {
    e.preventDefault();
    uiStore.toggleAskOrbis();
  }
}
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="flex h-full max-w-full bg-background mx-1 pb-2">
	{#if !uiStore.focusMode}
		<div transition:slide={{ axis: 'x', duration: 200 }} class="overflow-hidden h-full flex shrink-0">
			<AppSidebar {orgId} {orgName} />
		</div>
	{/if}
	<div class="flex flex-1 flex-col overflow-hidden relative pb-4">
		<div
			class="flex flex-1 min-w-0 items-stretch overflow-hidden"
		>
			<div class="flex flex-1 min-w-0 items-stretch overflow-hidden">
				{@render children()}
			</div>
		</div>
		{#if uiStore.showAskOrbis}
			{#key page.url.searchParams.get("ask_orbis")}
				<AskOrbisSidebar />
			{/key}
		{/if}
	</div>
</div>

<!-- Lazy-load UpgradeDialog only when shown -->
{#if uiStore.showUpgradeDialog}
	{#await import("./upgrade-dialog.svelte") then { default: UpgradeDialog }}
		<UpgradeDialog bind:open={uiStore.showUpgradeDialog} />
	{/await}
{/if}

