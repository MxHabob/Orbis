<!--
@component MainViewport
A multi-panel split view that manages resizing of panels.
Inspired by GitButler's MainViewport.

Accepts `left`, `middle`, and optionally `right` snippets.
Each side panel is resizable via drag handles, and widths
are persisted to localStorage.

@example
```svelte
<MainViewport
  name="students"
  leftWidth={{ default: 280, min: 220 }}
  rightWidth={{ default: 300, min: 200 }}
>
  {#snippet left()}...{/snippet}
  {#snippet middle()}...{/snippet}
  {#snippet right()}...{/snippet}
</MainViewport>
```
-->
<script lang="ts">
	import Resizer from './Resizer.svelte';
	import SashLayer from './SashLayer.svelte';
	import type { Snippet } from 'svelte';

	type PanelSize = {
		default: number;
		min: number;
	};

	let {
		name,
		left,
		leftWidth,
		middle,
		right,
		rightWidth = { default: 0, min: 0 },
	}: {
		name: string;
		left?: Snippet;
		leftWidth: PanelSize;
		middle: Snippet;
		right?: Snippet;
		rightWidth?: PanelSize;
	} = $props();

	// Convert px to rem using root font-size
	function pxToRem(px: number): number {
		if (typeof document === 'undefined') return px / 16;
		const base = parseFloat(getComputedStyle(document.documentElement).fontSize);
		return px / base;
	}

	// Panel preferred widths (in rem)
	const initialLeftWidth = leftWidth.default;
	const initialRightWidth = rightWidth.default;
	let leftPreferredWidth = $state(pxToRem(initialLeftWidth));
	let rightPreferredWidth = $state(pxToRem(initialRightWidth));

	// Panel DOM refs
	let leftDiv = $state<HTMLDivElement>();
	let rightDiv = $state<HTMLDivElement>();

	// Computed constraints (in rem)
	const leftMinWidth = $derived(pxToRem(leftWidth.min));
	const leftMaxWidth = $derived(pxToRem(leftWidth.min * 3));
	const rightMinWidth = $derived(right ? pxToRem(rightWidth.min) : 0);
	const rightMaxWidth = $derived(right ? pxToRem(rightWidth.min * 3) : 0);

	// Final computed widths
	const finalLeftWidth = $derived(
		left ? Math.max(leftMinWidth, Math.min(leftMaxWidth, leftPreferredWidth)) : 0
	);
	const finalRightWidth = $derived(
		right ? Math.max(rightMinWidth, Math.min(rightMaxWidth, rightPreferredWidth)) : 0
	);
</script>

<SashLayer>
	<div class="main-viewport">
		{#if left}
			<div
				class="left-section"
				bind:this={leftDiv}
				style:width="{finalLeftWidth}rem"
				style:min-width="{leftMinWidth}rem"
			>
				<div class="section-content panel-border">
					{@render left()}
				</div>
				<Resizer
					viewport={leftDiv}
					direction="right"
					minWidth={leftMinWidth}
					maxWidth={leftMaxWidth}
					defaultValue={pxToRem(leftWidth.default)}
					persistId="viewport-{name}-left"
					onWidth={(w) => (leftPreferredWidth = w)}
				/>
			</div>
		{/if}

		<div class="main-section panel-border" class:no-left={!left}>
			{@render middle()}
		</div>

		{#if right}
			<div
				class="right-section"
				bind:this={rightDiv}
				style:width="{finalRightWidth}rem"
				style:min-width="{rightMinWidth}rem"
			>
				<Resizer
					viewport={rightDiv}
					direction="left"
					minWidth={rightMinWidth}
					maxWidth={rightMaxWidth}
					defaultValue={pxToRem(rightWidth.default)}
					persistId="viewport-{name}-right"
					onWidth={(w) => (rightPreferredWidth = w)}
				/>
				<div class="section-content panel-border">
					{@render right()}
				</div>
			</div>
		{/if}
	</div>
</SashLayer>

<style>
	.main-viewport {
		display: flex;
		position: relative;
		flex: 1;
		align-items: stretch;
		width: 100%;
		height: 100%;
		gap: 8px;
	}

	.left-section {
		display: flex;
		position: relative;
		flex-shrink: 0;
		flex-direction: column;
		height: 100%;
	}

	.section-content {
		display: flex;
		flex-direction: column;
		height: 100%;
		overflow: hidden;
	}

	.main-section {
		display: flex;
		position: relative;
		flex: 1;
		flex-direction: column;
		height: 100%;
		min-width: 0;
		overflow: hidden;
	}

	.main-section.no-left {
		/* No gap offset needed */
	}

	.right-section {
		display: flex;
		position: relative;
		flex-shrink: 0;
		flex-direction: column;
		height: 100%;
	}
</style>
