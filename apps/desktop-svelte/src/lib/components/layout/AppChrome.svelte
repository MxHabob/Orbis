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
		display: flex;
		flex-grow: 1;
		height: 100%;
		overflow: hidden;
	}

	.chrome-content {
		display: flex;
		flex-grow: 1;
		align-items: stretch;
		padding: 0 14px 14px 0;
		overflow: hidden;
	}
</style>
