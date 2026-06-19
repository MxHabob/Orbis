<!--
@component SashLayer
Positioning wrapper for resizable panel layouts.
Provides the relative positioning context that Resizer handles need.
Inspired by GitButler's SashLayer pattern.
-->
<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<div class="relative flex h-full w-full">
	{@render children()}
</div>
