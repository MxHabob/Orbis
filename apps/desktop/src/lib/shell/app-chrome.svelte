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
import type { Snippet } from "svelte";
import { slide } from "svelte/transition";
import { page } from "$app/state";
import { onMount } from "svelte";
import AskOrbisSidebar from "$lib/features/ai/views/sidebar.svelte";
import { useQueryClient } from "@tanstack/svelte-query";

let {
  orgId,
  orgName,
  children,
}: {
  orgId: string;
  orgName?: string;
  children: Snippet;
} = $props();

const queryClient = useQueryClient();

function handleKeyDown(e: KeyboardEvent) {
  // Ctrl/Cmd + / -> Toggle Ask Orbis AI
  if ((e.ctrlKey || e.metaKey) && e.key === "/") {
    e.preventDefault();
    uiStore.toggleAskOrbis();
    return;
  }

  // Ctrl/Cmd + K -> Toggle global quick search
  if ((e.ctrlKey || e.metaKey) && (e.key === "k" || e.key === "K")) {
    e.preventDefault();
    uiStore.toggleSearch();
    return;
  }

  // Ctrl/Cmd + S -> Save action for active form
  if ((e.ctrlKey || e.metaKey) && (e.key === "s" || e.key === "S")) {
    e.preventDefault();
    window.dispatchEvent(new CustomEvent("orbis:save"));
    return;
  }

  // F5 -> Invalidate active queries / soft refresh
  if (e.key === "F5") {
    e.preventDefault();
    queryClient.invalidateQueries();
    return;
  }
}
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="flex h-full max-w-full bg-background">
	{#if !uiStore.focusMode}
		<div transition:slide={{ axis: 'x', duration: 200 }} class="overflow-hidden h-full flex shrink-0">
			<AppSidebar {orgId} {orgName} />
		</div>
	{/if}
	<div class="flex flex-1 flex-col overflow-hidden relative">
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
