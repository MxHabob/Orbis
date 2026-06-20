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
</script>

<div class="flex h-screen max-w-full bg-background mx-1">
	<AppSidebar {orgId} {orgName} />
	<div class="flex flex-1 flex-col overflow-hidden">
		<AppHeader {orgId} {orgName} />
		<div class="flex flex-1 min-w-0 items-stretch overflow-hidden px-3.5 pb-3.5">
			{@render children()}
		</div>
	</div>
</div>
