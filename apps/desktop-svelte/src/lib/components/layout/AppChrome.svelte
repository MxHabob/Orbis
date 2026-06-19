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

<div class="flex h-screen max-w-full flex-1 flex-col bg-muted">
	<AppHeader {orgId} {orgName} />
	<div class="grid flex-1 grid-flow-col grid-cols-[auto_minmax(0,1fr)] overflow-hidden">
		<AppSidebar {orgId} />
		<div class="flex min-w-0 items-stretch overflow-hidden px-3.5 pb-3.5 box-border">
			{@render children()}
		</div>
	</div>
</div>
