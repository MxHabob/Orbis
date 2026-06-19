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

<div class="sash-layer">
	{@render children()}
</div>

<style>
	.sash-layer {
		display: flex;
		position: relative;
		width: 100%;
		height: 100%;
	}
</style>
