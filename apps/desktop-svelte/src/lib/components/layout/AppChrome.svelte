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
import AppHeader from "./AppHeader.svelte";
import AppSidebar from "./AppSidebar.svelte";
import AskOrbisSidebar from "./AskOrbisSidebar.svelte";
import UpgradeDialog from "./UpgradeDialog.svelte";
import { uiStore } from "$lib/stores";
import type { Snippet } from "svelte";

let {
  orgId,
  orgName,
  children,
}: {
  orgId: string;
  orgName?: string;
  children: Snippet;
} = $props();

function handleKeyDown(e: KeyboardEvent) {
  if ((e.ctrlKey || e.metaKey) && e.key === "/") {
    e.preventDefault();
    uiStore.toggleAskOrbis();
  }
}
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="flex h-screen max-w-full bg-background mx-1">
	<AppSidebar {orgId} {orgName} />
	<div class="flex flex-1 flex-col overflow-hidden">
		<AppHeader {orgId} {orgName} />
		<div
			class="flex flex-1 min-w-0 items-stretch overflow-hidden px-3.5 pb-3.5"
			class:gap-3.5={uiStore.showAskOrbis}
		>
			<div class="flex flex-1 min-w-0 items-stretch overflow-hidden">
				{@render children()}
			</div>
			{#if uiStore.showAskOrbis}
				<AskOrbisSidebar />
			{/if}
		</div>
	</div>
</div>

{#if uiStore.showUpgradeDialog}
	<UpgradeDialog />
{/if}
