<script lang="ts">
import { isDesktopApp } from "$lib/api";
import TitleBar from "$lib/shell/title-bar.svelte";

let { children } = $props();
</script>

<div class="setup-shell">
	{#if isDesktopApp}
		<TitleBar />
	{/if}
	<div class="setup-content">
		{@render children()}
	</div>
</div>

<style>
	.setup-shell {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
		background: linear-gradient(135deg, hsl(222 47% 8%) 0%, hsl(222 47% 14%) 100%);
	}

	.setup-content {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 0;
		overflow: auto;
		padding: 2rem;
	}
</style>
