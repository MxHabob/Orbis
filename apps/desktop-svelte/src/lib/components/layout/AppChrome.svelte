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
	import AppHeader from './AppHeader.svelte';
	import AppSidebar from './AppSidebar.svelte';
	import type { Snippet } from 'svelte';

	let {
		orgId,
		orgName,
		children,
	}: {
		orgId: string;
		orgName?: string;
		children: Snippet;
	} = $props();
</script>

<div class="chrome">
	<AppHeader {orgId} {orgName} />
	<div class="chrome-body">
		<AppSidebar {orgId} />
		<div class="chrome-content">
			{@render children()}
		</div>
	</div>
</div>

<style>
	.chrome {
		display: flex;
		flex: 1;
		flex-direction: column;
		max-width: 100%;
		height: 100vh;
		background-color: var(--chrome-bg);
	}

	.chrome-body {
		display: grid;
		grid-template-columns: auto minmax(0, 1fr);
		grid-auto-flow: column;
		flex: 1 1 0%;
		overflow: hidden;
	}

	.chrome-content {
		display: flex;
		min-width: 0;
		align-items: stretch;
		padding-inline: 14px;
		padding-block-end: 14px;
		overflow: hidden;
		box-sizing: border-box;
	}
</style>
