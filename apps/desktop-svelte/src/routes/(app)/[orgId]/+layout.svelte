<!--
  [orgId] Layout — wraps all authenticated pages with the Chrome Shell.
  This is the equivalent of GitButler's [projectId]/+layout.svelte.
-->
<script lang="ts">
import AppChrome from "$lib/shell/app-chrome.svelte";
import type { Snippet } from "svelte";
import type { LayoutData } from "./$types";

let { data, children }: { data: LayoutData; children: Snippet } = $props();
</script>

<AppChrome orgId={data.orgId}>
	{@render children()}
</AppChrome>
